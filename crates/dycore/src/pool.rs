//! 本机身份池：只存用户自己生成的号，按低水位补位、按健康度租用。
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

const MAX_USES: u32 = 620;
const SPACING: Duration = Duration::from_secs(5);
const SOFT_COOL: u64 = 60;
const MAX_COOL: u64 = 8 * 3600;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Identity {
    pub device_id: String,
    pub iid: String,
    #[serde(default)]
    pub device_token: String,
    #[serde(default)]
    pub brand: String,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub resolution: String,
    #[serde(default)]
    pub dpi: String,
    #[serde(default)]
    pub android_id: String,
    #[serde(default)]
    pub wifi_mac: String,
    #[serde(default)]
    pub bt_mac: String,
    #[serde(default)]
    pub used: u32,
    #[serde(default)]
    pub cdid: String,
    #[serde(default)]
    pub search_state: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub first_launch: String,
    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub last_used_at: i64,
    #[serde(default)]
    pub cool_until: i64,
    #[serde(default)]
    pub consecutive_fail: u32,
    #[serde(default)]
    pub miss: u32,
    #[serde(default)]
    pub retired: bool,
    #[serde(default)]
    pub dead: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    pub target: usize,
    pub total: usize,
    pub available: usize,
    pub cooling: usize,
    pub retired: usize,
    pub dead: usize,
    pub creating: usize,
    pub paused: bool,
    pub last_error: String,
}

pub struct Pool {
    path: PathBuf,
    all: Vec<Identity>,
    target: usize,
    creating: usize,
    paused: bool,
    last_error: String,
    cursor: usize,
    last_lease: Option<Instant>,
    current: Option<(String, Instant)>,
    /// 运行期软惩罚；空包命中后先避开，成功即清。
    soft: HashSet<String>,
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// 新池没有文件时，顺手删掉作者机器留下的旧状态索引。
pub fn cleanup_legacy_state() {
    if let Some(dir) = dirs::config_dir() {
        let _ = std::fs::remove_file(dir.join("douyintai/pool-state.json"));
    }
}

pub fn default_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::env::temp_dir().join("douyintai"))
        .join("douyintai")
        .join("identity-pool.jsonl")
}

impl Pool {
    pub fn load(path: PathBuf, target: usize) -> Self {
        let mut all = std::fs::read_to_string(&path)
            .ok()
            .map(|s| {
                s.lines()
                    .filter_map(|l| serde_json::from_str::<Identity>(l).ok())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        // 旧版曾把作者号随包塞进池子；本机生成的号都有 created_at，作者号没有。
        all.retain(|i| i.created_at > 0);
        let pool = Self {
            path,
            all,
            target: target.clamp(1, 128),
            creating: 0,
            paused: false,
            last_error: String::new(),
            cursor: rand::thread_rng().gen_range(0..1000),
            last_lease: None,
            current: None,
            soft: HashSet::new(),
        };
        pool.save();
        pool
    }

    pub fn len(&self) -> usize {
        self.all.len()
    }

    pub fn is_empty(&self) -> bool {
        self.all.is_empty()
    }

    pub fn target(&self) -> usize {
        self.target
    }

    pub fn set_target(&mut self, target: usize) {
        self.target = target.clamp(1, 128);
        self.normalize();
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    fn usable(&self, id: &Identity) -> bool {
        !id.retired
            && !id.dead
            && id.used < MAX_USES
            && id.cool_until <= now()
            && id.search_state != "burned"
            && id.status != "burned"
    }

    pub fn usable_count(&self) -> usize {
        self.all.iter().filter(|i| self.usable(i)).count()
    }

    pub fn has_usable(&self) -> bool {
        self.all.iter().any(|i| self.usable(i))
    }

    pub fn status(&self) -> Status {
        Status {
            target: self.target,
            total: self.all.len(),
            available: self.usable_count(),
            cooling: self
                .all
                .iter()
                .filter(|i| !i.retired && !i.dead && i.cool_until > now())
                .count(),
            retired: self.all.iter().filter(|i| i.retired).count(),
            dead: self.all.iter().filter(|i| i.dead).count(),
            creating: self.creating,
            paused: self.paused,
            last_error: self.last_error.clone(),
        }
    }

    pub fn needs_identity(&mut self) -> bool {
        self.normalize();
        if self.paused {
            return false;
        }
        let live = self.all.iter().filter(|i| !i.retired && !i.dead).count();
        // 目标是可用数；冷却期间临时多备，死号最多把文件撑到两倍。
        (self.usable_count() + self.creating) < self.target
            && (live + self.creating) < self.target.saturating_mul(2)
    }

    /// 预占一个创建名额；调用方必须在结束时 finish/fail。
    pub fn begin_create(&mut self) -> bool {
        if !self.needs_identity() {
            return false;
        }
        self.creating += 1;
        true
    }

    pub fn finish_create(&mut self, mut id: Identity) {
        self.creating = self.creating.saturating_sub(1);
        id.created_at = now();
        id.retired = false;
        id.dead = false;
        id.used = 0;
        id.cool_until = 0;
        id.consecutive_fail = 0;
        self.all.retain(|i| i.device_id != id.device_id);
        self.all.push(id);
        self.last_error.clear();
        self.normalize();
        self.save();
    }

    pub fn fail_create(&mut self, error: impl Into<String>) {
        self.creating = self.creating.saturating_sub(1);
        self.last_error = error.into();
    }

    pub fn clear(&mut self) {
        self.all.clear();
        self.creating = 0;
        self.last_error.clear();
        self.current = None;
        self.soft.clear();
        self.save();
    }

    fn normalize(&mut self) {
        let t = now();
        let mut changed = false;
        for id in &mut self.all {
            if id.used >= MAX_USES && !id.retired {
                id.retired = true;
                changed = true;
            }
            if id.cool_until > 0 && id.cool_until <= t && !id.retired && !id.dead {
                id.cool_until = 0;
                changed = true;
            }
        }
        if changed {
            self.save();
        }
    }

    pub fn lease(&mut self) -> Result<Identity> {
        self.lease_with(true)
    }

    pub fn lease_fast(&mut self) -> Result<Identity> {
        self.lease_with(false)
    }

    pub fn lease_filtered(
        &mut self,
        spaced: bool,
        allow: &dyn Fn(&str) -> bool,
    ) -> Result<Identity> {
        self.lease_with_filter(spaced, Some(allow))
    }

    fn lease_with(&mut self, spaced: bool) -> Result<Identity> {
        self.lease_with_filter(spaced, None)
    }

    fn lease_with_filter(
        &mut self,
        spaced: bool,
        allow: Option<&dyn Fn(&str) -> bool>,
    ) -> Result<Identity> {
        if let Ok(want) = std::env::var("DYCORE_DEVICE_ID") {
            if let Some(id) = self
                .all
                .iter()
                .find(|i| i.device_id == want && allow.is_none_or(|f| f(&i.device_id)))
                .cloned()
            {
                self.mark_used(&id.device_id);
                return Ok(id);
            }
        }
        if let Some((device_id, since)) = &self.current {
            if since.elapsed() < Duration::from_secs(30)
                && allow.is_none_or(|f| f(device_id))
                && self
                    .all
                    .iter()
                    .find(|i| &i.device_id == device_id)
                    .is_some_and(|i| self.usable(i))
            {
                let id = self
                    .all
                    .iter()
                    .find(|i| &i.device_id == device_id)
                    .cloned()
                    .expect("current");
                self.mark_used(&id.device_id);
                return Ok(id);
            }
        }
        if let Some(t) = self.last_lease.filter(|_| spaced) {
            let left = SPACING.saturating_sub(t.elapsed());
            if !left.is_zero() {
                std::thread::sleep(left);
            }
        }
        let mut live: Vec<&Identity> = self
            .all
            .iter()
            .filter(|i| self.usable(i) && allow.is_none_or(|f| f(&i.device_id)))
            .collect();
        if live.is_empty() {
            return Err(Error::msg("身份池已耗尽"));
        }
        live.sort_by_key(|i| (i.miss, i.used, i.last_used_at));
        let window = live.iter().take_while(|i| i.miss == 0).count().clamp(1, 16);
        let pick = live[rand::thread_rng().gen_range(0..window)].clone();
        self.mark_used(&pick.device_id);
        self.cursor = self.cursor.wrapping_add(1);
        self.last_lease = Some(Instant::now());
        self.current = Some((pick.device_id.clone(), Instant::now()));
        Ok(pick)
    }

    fn mark_used(&mut self, device_id: &str) {
        if let Some(id) = self.all.iter_mut().find(|i| i.device_id == device_id) {
            id.used += 1;
            id.last_used_at = now();
            if id.used >= MAX_USES {
                id.retired = true;
            }
            self.save();
        }
    }

    pub fn release(&mut self, device_id: &str) {
        if self.soft.iter().any(|s| s == device_id)
            && self
                .all
                .iter()
                .find(|i| i.device_id == device_id)
                .is_some_and(|i| i.miss >= 2)
        {
            self.soft.remove(device_id);
        }
    }

    pub fn success(&mut self, device_id: &str) {
        if let Some(id) = self.all.iter_mut().find(|i| i.device_id == device_id) {
            let mut changed = false;
            if id.consecutive_fail != 0 {
                id.consecutive_fail = 0;
                changed = true;
            }
            if id.cool_until != 0 && id.cool_until <= now() {
                id.cool_until = 0;
                changed = true;
            }
            if id.miss != 0 {
                id.miss = 0;
                changed = true;
            }
            if changed {
                self.save();
            }
        }
        self.soft.remove(device_id);
    }

    /// 风控类失败：指数冷却，达到 5 次标死。
    pub fn fail(&mut self, device_id: &str) -> u64 {
        let mut secs = SOFT_COOL;
        if let Some(id) = self.all.iter_mut().find(|i| i.device_id == device_id) {
            id.consecutive_fail += 1;
            secs = SOFT_COOL
                .saturating_mul(1u64 << id.consecutive_fail.min(7))
                .min(MAX_COOL);
            if id.consecutive_fail >= 5 {
                id.dead = true;
            } else {
                id.cool_until = now() + secs as i64;
            }
        }
        if self.current.as_ref().is_some_and(|(i, _)| i == device_id) {
            self.current = None;
        }
        self.save();
        secs
    }

    pub fn cool(&mut self, device_id: &str, secs: u64) {
        if let Some(id) = self.all.iter_mut().find(|i| i.device_id == device_id) {
            id.cool_until = now() + secs as i64;
        }
        if self.current.as_ref().is_some_and(|(i, _)| i == device_id) {
            self.current = None;
        }
        self.save();
    }

    pub fn drop_current(&mut self) {
        self.current = None;
    }

    pub fn mark_miss(&mut self, device_id: &str) {
        if let Some(id) = self.all.iter_mut().find(|i| i.device_id == device_id) {
            id.miss += 1;
            if id.miss >= 2 {
                self.soft.insert(device_id.to_string());
            }
        }
        self.current = None;
        self.save();
    }

    pub fn mark_hit(&mut self, device_id: &str) {
        self.success(device_id);
    }

    pub fn retire(&mut self, device_id: &str) {
        if let Some(id) = self.all.iter_mut().find(|i| i.device_id == device_id) {
            id.retired = true;
        }
        if self.current.as_ref().is_some_and(|(i, _)| i == device_id) {
            self.current = None;
        }
        self.save();
    }

    fn save(&self) {
        if let Some(dir) = self.path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let mut out = String::new();
        for id in &self.all {
            if let Ok(line) = serde_json::to_string(id) {
                out.push_str(&line);
                out.push('\n');
            }
        }
        let tmp = self.path.with_extension("jsonl.tmp");
        if std::fs::write(&tmp, out).is_ok() {
            set_private(&tmp);
            let _ = std::fs::rename(&tmp, &self.path);
        }
    }
}

#[cfg(unix)]
fn set_private(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
}

#[cfg(not(unix))]
fn set_private(_path: &Path) {}

/// 运行期租约守卫：请求结束统一释放当前号。
pub struct Lease {
    pub identity: Identity,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp() -> PathBuf {
        std::env::temp_dir().join(format!("dy-pool-{}.jsonl", uuid::Uuid::new_v4()))
    }

    fn identity(n: usize) -> Identity {
        Identity {
            device_id: format!("{n}"),
            iid: format!("{}", n + 1),
            brand: "xiaomi".into(),
            model: "2201123C".into(),
            resolution: "1080x2400".into(),
            dpi: "419".into(),
            first_launch: "1".into(),
            ..Identity::default()
        }
    }

    #[test]
    fn drops_packaged_seed_on_load() {
        let path = temp();
        let seed = Identity {
            device_id: "old".into(),
            ..Identity::default()
        };
        let local = Identity {
            device_id: "new".into(),
            created_at: 1,
            ..Identity::default()
        };
        std::fs::write(
            &path,
            format!(
                "{}\n{}\n",
                serde_json::to_string(&seed).unwrap(),
                serde_json::to_string(&local).unwrap()
            ),
        )
        .unwrap();
        let p = Pool::load(path, 2);
        assert_eq!(p.len(), 1);
        assert_eq!(p.status().available, 1);
    }

    #[test]
    fn soft_identity_stays_usable() {
        let mut p = Pool::load(temp(), 1);
        p.finish_create(identity(1));
        p.mark_miss("1");
        p.mark_miss("1");
        assert_eq!(p.usable_count(), 1);
    }

    #[test]
    fn cooled_identity_is_not_usable() {
        let mut p = Pool::load(temp(), 4);
        p.finish_create(identity(1));
        let n = p.usable_count();
        p.cool("1", 60);
        assert!(p.usable_count() < n);
    }

    #[test]
    fn refills_to_target_then_stops() {
        let mut p = Pool::load(temp(), 3);
        while p.begin_create() {
            let n = p.len() + 1;
            p.finish_create(identity(n));
        }
        assert_eq!(p.len(), 3);
        assert!(!p.begin_create());
    }

    #[test]
    fn retired_opens_a_slot_and_survives_reload() {
        let path = temp();
        let mut p = Pool::load(path.clone(), 2);
        p.clear();
        p.finish_create(identity(1));
        p.finish_create(identity(2));
        p.retire("1");
        assert!(p.status().available < 2);
        assert!(p.needs_identity());
        drop(p);
        let p = Pool::load(path, 2);
        assert_eq!(p.status().retired, 1);
    }

    #[test]
    fn five_failures_mark_dead() {
        let mut p = Pool::load(temp(), 1);
        p.finish_create(identity(1));
        for _ in 0..4 {
            p.fail("1");
        }
        assert_eq!(p.status().dead, 0);
        p.fail("1");
        assert_eq!(p.status().dead, 1);
    }
}
