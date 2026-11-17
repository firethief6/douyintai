//! 采集流水线：提交、取任务、执行与归档。提交只写表，执行由单循环完成。
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use dycore::api::Api;
use tauri::AppHandle;
use tauri::Manager as _;

use crate::db;
use crate::rec;

const KINDS: [&str; 5] = ["collection", "recheck", "backfill", "download", "record"];
const ASSET_KINDS: [&str; 5] = ["video", "cover", "music", "gallery", "record"];
const ACTIVE: &str = "('queued','running')";
const TASK_COLUMNS: &str = "id,kind,endpoint,params,target_key,state,attempts,max_attempts,batch_id,outcome,error_code,created_at,started_at,finished_at,request_id,identity,upstream_url,retry_at,output_path,source,progress,COALESCE((SELECT d.bytes FROM downloads d WHERE d.id=json_extract(tasks.params,'$.download_id')),0),COALESCE((SELECT d.cover FROM downloads d WHERE d.id=json_extract(tasks.params,'$.download_id')),json_extract(tasks.params,'$.display_image')),acknowledged_at,archived_at,deleted_at,keep_until,trigger,plan_id,result_summary";
const LIBRARY_UNION: &str = "SELECT 'asset' source,a.id raw_id,CASE a.kind WHEN 'gallery' THEN 'image' WHEN 'video' THEN 'video' ELSE 'other' END item_type,a.kind asset_kind,a.title,a.author,a.target_key,a.cover cover,'' thumb_path,a.path path,a.size,a.created_at,NULL record_state,'{}' metrics,0 comments_requested,0 comment_count,0 comment_target,NULL comment_outcome,0 asset_count FROM assets a WHERE a.kind<>'cover' UNION ALL SELECT 'record',r.id,'comment','comment',r.title,r.author_name,r.target_key,r.cover,r.thumb_path,t.output_path,NULL,r.created_at,r.state,r.metrics,CASE WHEN (SELECT COUNT(*) FROM captured_comments c WHERE c.record_id=r.id)>0 OR COALESCE(json_extract(t.params,'$.comments'),0)!=0 THEN 1 ELSE 0 END,(SELECT COUNT(*) FROM captured_comments c WHERE c.record_id=r.id),COALESCE(json_extract(t.params,'$.max_comments'),0),t.outcome,(SELECT COUNT(*) FROM assets a WHERE a.record_id=r.id AND a.kind!='cover') FROM capture_records r LEFT JOIN tasks t ON t.id=r.task_id";

#[derive(Debug, Clone, Serialize)]
pub struct PipelineErr {
    pub status: u16,
    pub code: String,
    pub message: String,
}

impl PipelineErr {
    fn new(status: u16, code: &str, message: impl Into<String>) -> Self {
        Self {
            status,
            code: code.into(),
            message: message.into(),
        }
    }
}

type PResult<T> = std::result::Result<T, PipelineErr>;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub target: Option<String>,
    pub link: Option<String>,
    pub kind: Option<String>,
    pub endpoint: Option<String>,
    pub params: Option<Value>,
    pub max_attempts: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Submit {
    pub target: Option<String>,
    pub link: Option<String>,
    pub kind: Option<String>,
    pub endpoint: Option<String>,
    pub params: Option<Value>,
    pub max_attempts: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Batch {
    pub items: Vec<Item>,
    pub batch_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Accepted {
    pub index: usize,
    pub ok: bool,
    pub id: Option<String>,
    pub state: Option<String>,
    pub error_code: Option<String>,
    pub message: Option<String>,
    pub deduped: bool,
}

#[derive(Debug, Serialize)]
pub struct BatchResult {
    pub batch_id: String,
    pub results: Vec<Accepted>,
    pub accepted: usize,
    pub rejected: usize,
    pub deduped: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DayPoint {
    pub day: String,
    pub submitted: i64,
    pub succeeded: i64,
    pub failed: i64,
    pub bytes: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct KindPoint {
    pub kind: String,
    pub active: i64,
    pub done: i64,
    pub failed: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OutcomePoint {
    pub outcome: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Dashboard {
    pub day: String,
    pub submitted: i64,
    pub completed: i64,
    pub succeeded: i64,
    pub failed: i64,
    pub gone: i64,
    pub avg_duration_ms: i64,
    pub queue_depth: i64,
    pub disk_bytes: i64,
    pub pool_available: i64,
    pub breakers: i64,
    pub series: Vec<DayPoint>,
    pub kinds: Vec<KindPoint>,
    pub outcomes: Vec<OutcomePoint>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Stats {
    pub day: String,
    pub submitted: i64,
    pub completed: i64,
    pub succeeded: i64,
    pub failed: i64,
    pub gone: i64,
    pub avg_duration_ms: i64,
    pub queue_depth: i64,
    pub disk_bytes: i64,
    pub pool_available: i64,
    pub breakers: i64,
}

#[derive(Debug, Deserialize)]
pub struct RecheckReq {
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct DownloadReq {
    pub target: Option<String>,
    pub target_key: Option<String>,
    pub asset_kind: Option<String>,
    pub url: Option<String>,
    pub pinned: Option<bool>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub ext: Option<String>,
    pub expected_bytes: Option<i64>,
    pub record_id: Option<String>,
    pub parent_id: Option<String>,
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadView {
    pub id: String,
    pub task_id: Option<String>,
    pub target_key: String,
    pub asset_key: String,
    pub asset_kind: String,
    pub url: String,
    pub path: Option<String>,
    pub partial_path: Option<String>,
    pub bytes: Option<i64>,
    pub expected_bytes: Option<i64>,
    pub progress: f64,
    pub speed: i64,
    pub state: String,
    pub pinned: bool,
    pub attempts: i64,
    pub created_at: i64,
    pub finished_at: Option<i64>,
    pub error: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub record_id: Option<String>,
    pub asset_id: Option<String>,
    pub task_deleted_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssetView {
    pub id: String,
    pub target_key: String,
    pub kind: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub cover: Option<String>,
    pub path: String,
    pub dir: String,
    pub size: Option<i64>,
    pub missing: bool,
    pub task_id: Option<String>,
    pub record_id: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AssetPage {
    pub items: Vec<AssetView>,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DirView {
    pub path: String,
    pub name: String,
    pub count: i64,
    pub bytes: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DirListing {
    pub root: String,
    pub items: Vec<DirView>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordView {
    pub id: String,
    pub target_key: String,
    pub task_id: Option<String>,
    pub title: Option<String>,
    pub author_key: Option<String>,
    pub author_name: Option<String>,
    pub cover: Option<String>,
    pub thumb_path: Option<String>,
    pub metrics: Value,
    pub metrics_history: Value,
    pub state: String,
    pub created_at: i64,
    pub last_checked: Option<i64>,
    pub comment_count: i64,
    pub asset_count: i64,
    pub comments_requested: bool,
    pub comment_target: i64,
    pub comment_outcome: Option<String>,
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommentView {
    pub cid: String,
    pub parent_cid: Option<String>,
    pub level: i64,
    pub author_key: Option<String>,
    pub author_name: Option<String>,
    pub content: String,
    pub likes: Option<i64>,
    pub ip_label: Option<String>,
    pub created_at: Option<i64>,
    pub raw: Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct CommentPage {
    pub items: Vec<CommentView>,
    pub next: Option<String>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordSummary {
    pub count: i64,
    pub live: i64,
    pub deleted: i64,
}

#[derive(Debug, Deserialize)]
pub struct PlanReq {
    pub kind: Option<String>,
    pub target: Option<String>,
    pub target_key: Option<String>,
    pub interval_sec: Option<i64>,
    pub params: Option<Value>,
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanView {
    pub id: String,
    pub kind: String,
    pub target_key: String,
    pub interval_sec: i64,
    pub params: Value,
    pub enabled: bool,
    pub next_due_at: i64,
    pub fail_streak: i64,
    pub last_task_id: Option<String>,
    pub last_error: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordSessionView {
    pub id: String,
    pub plan_id: Option<String>,
    pub target_key: String,
    pub room_id: Option<String>,
    pub dir: String,
    pub state: String,
    pub bytes: i64,
    pub started_at: i64,
    pub finished_at: Option<i64>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BackfillReq {
    pub target: Option<String>,
    pub pages: Option<i64>,
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskView {
    pub id: String,
    pub kind: String,
    pub endpoint: String,
    pub params: Value,
    pub target_key: Option<String>,
    pub state: String,
    pub attempts: i64,
    pub max_attempts: i64,
    pub batch_id: Option<String>,
    pub outcome: Option<String>,
    pub error_code: Option<String>,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub finished_at: Option<i64>,
    pub request_id: Option<String>,
    pub identity: Option<String>,
    pub upstream_url: Option<String>,
    pub retry_at: Option<i64>,
    pub output_path: Option<String>,
    pub source: Option<String>,
    pub progress: f64,
    pub bytes: i64,
    pub cover: Option<String>,
    pub acknowledged_at: Option<i64>,
    pub archived_at: Option<i64>,
    pub deleted_at: Option<i64>,
    pub keep_until: Option<i64>,
    pub trigger: String,
    pub plan_id: Option<String>,
    pub result_summary: Option<Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TaskFilter {
    pub kind: Option<String>,
    pub q: Option<String>,
    pub source: Option<String>,
    pub state: Option<String>,
    pub batch_id: Option<String>,
    pub plan_id: Option<String>,
    pub kept: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AssetFilter {
    pub kind: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub source: Option<String>,
    pub record: Option<String>,
    pub dir: Option<String>,
    pub direct: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AssetActionReq {
    pub action: String,
    pub ids: Option<Vec<String>>,
    pub filter: Option<AssetFilter>,
    pub file: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AssetActionResult {
    pub count: usize,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LibraryFilter {
    pub kind: Option<String>,
    pub q: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LibraryView {
    pub id: String,
    pub source: String,
    pub item_type: String,
    pub asset_kind: String,
    pub title: Option<String>,
    pub author: Option<String>,
    pub target_key: String,
    pub cover: Option<String>,
    pub thumb_path: Option<String>,
    pub path: Option<String>,
    pub size: Option<i64>,
    pub created_at: i64,
    pub missing: bool,
    pub record_state: Option<String>,
    pub metrics: Value,
    pub comments_requested: bool,
    pub comment_count: i64,
    pub comment_target: i64,
    pub comment_outcome: Option<String>,
    pub asset_count: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct LibraryPage {
    pub items: Vec<LibraryView>,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct LibrarySummary {
    pub total: i64,
    pub video: i64,
    pub image: i64,
    pub comment: i64,
    pub other: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LibraryActionReq {
    pub action: String,
    pub ids: Option<Vec<String>>,
    pub filter: Option<LibraryFilter>,
    pub file: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecordActionReq {
    pub action: String,
    pub ids: Option<Vec<String>>,
    pub state: Option<String>,
    pub kind: Option<String>,
    pub q: Option<String>,
    pub from: Option<i64>,
    pub to: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TaskActionReq {
    pub action: String,
    pub ids: Option<Vec<String>>,
    pub scope: Option<String>,
    pub filter: Option<TaskFilter>,
    pub file: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TaskActionResult {
    pub count: usize,
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaskSummary {
    pub active: i64,
    pub pending: i64,
    pub recent: i64,
    pub history: i64,
    pub deleted: i64,
    pub kept: i64,
}

struct ResumeRow {
    state: String,
    key: String,
    tid: Option<String>,
    kind: String,
    target: String,
    record_id: Option<String>,
    pinned: i64,
    title: Option<String>,
    author: Option<String>,
}

struct Normalized {
    kind: String,
    endpoint: String,
    params: Value,
    target_key: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Ok,
    Gone,
    Risk,
    Network,
    Param,
}

impl Outcome {
    fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Gone => "gone",
            Self::Risk => "risk",
            Self::Network => "network",
            Self::Param => "param",
        }
    }
}

/// 先读平台信封，再读 HTTP 状态。空包（-99999）是风控，不是网络。
pub fn classify_value(v: &Value) -> Outcome {
    let code = v.get("status_code").and_then(Value::as_i64).unwrap_or(0);
    let msg = v.get("status_msg").and_then(Value::as_str).unwrap_or("");
    if v.get("filter_detail").is_some_and(|f| !f.is_null()) {
        return Outcome::Gone;
    }
    if matches!(code, 404 | 30001) {
        return Outcome::Gone;
    }
    if matches!(code, 403 | 429 | 2483 | 2484 | -99999) {
        return Outcome::Risk;
    }
    if code != 0 {
        return match classify_text(msg) {
            Outcome::Ok => Outcome::Param,
            other => other,
        };
    }
    if v.get("items")
        .and_then(Value::as_array)
        .is_some_and(|a| a.is_empty())
        && !msg.is_empty()
        && is_risk_msg(msg)
    {
        return Outcome::Risk;
    }
    Outcome::Ok
}

fn comments_complete(target: i64, count: i64, total: i64, exhausted: bool) -> bool {
    if target > 0 {
        count >= target || (total > 0 && count >= total)
    } else {
        (total > 0 && count >= total) || (exhausted && total <= 0)
    }
}

pub fn classify_error(status: u16, message: &str) -> Outcome {
    if status == 404 {
        return Outcome::Gone;
    }
    if matches!(status, 403 | 429) {
        return Outcome::Risk;
    }
    if matches!(status, 0 | 502 | 503 | 504) {
        return Outcome::Network;
    }
    if status == 400 {
        return Outcome::Param;
    }
    classify_text(message)
}

fn is_risk_msg(m: &str) -> bool {
    let m = m.to_ascii_lowercase();
    [
        "频繁",
        "限流",
        "too many",
        "风控",
        "gate",
        "签名",
        "校验",
        "forbidden",
        "denied",
        "-99999",
    ]
    .iter()
    .any(|n| m.contains(n))
}

fn classify_text(m: &str) -> Outcome {
    if is_risk_msg(m) {
        Outcome::Risk
    } else if m.contains("已删除") || m.contains("不存在") {
        Outcome::Gone
    } else {
        Outcome::Network
    }
}

pub struct Pipeline {
    db: PathBuf,
    api: Mutex<Option<Arc<Api>>>,
    wake: AtomicBool,
    last_identity: Mutex<Option<String>>,
    last_request_id: Mutex<Option<String>>,
    last_upstream_url: Mutex<Option<String>>,
    app: OnceLock<AppHandle>,
}

#[derive(Default)]
struct Trace {
    identity: Option<String>,
    request_id: Option<String>,
    upstream_url: Option<String>,
}

impl Pipeline {
    pub fn new(db: PathBuf) -> Self {
        Self {
            db,
            api: Mutex::new(None),
            wake: AtomicBool::new(false),
            last_identity: Mutex::new(None),
            last_request_id: Mutex::new(None),
            last_upstream_url: Mutex::new(None),
            app: OnceLock::new(),
        }
    }

    pub fn attach_app(&self, app: AppHandle) {
        let _ = self.app.set(app);
    }

    pub fn attach(&self, api: Arc<Api>) {
        *self.api.lock().unwrap_or_else(|e| e.into_inner()) = Some(api);
    }

    /// 一次性迁移旧下载记录：先备份已由 db::open_path 完成，失败时原地索引，不丢记录。
    pub fn migrate_assets_once(&self) -> PResult<()> {
        let conn = self.conn()?;
        let done: Option<String> = conn
            .query_row(
                "SELECT value FROM settings WHERE key='assets_migrated'",
                [],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if done.as_deref() == Some("2") {
            self.repair_asset_links()?;
            return self.backfill_download_task_titles();
        }
        let root = self.download_root();
        let mut legacy: Vec<Value> = conn
            .prepare("SELECT json FROM rows WHERE bucket='lib:downloaded' ORDER BY ord")
            .and_then(|mut st| {
                st.query_map([], |r| r.get::<_, String>(0))?
                    .collect::<Result<Vec<_>, _>>()
            })
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .into_iter()
            .filter_map(|s| serde_json::from_str(&s).ok())
            .collect();
        let mut downloads = conn
            .prepare("SELECT id,target_key,path,bytes,state,title,author,asset_kind,record_id FROM downloads WHERE state='done' AND asset_id IS NULL")
            .and_then(|mut st| st.query_map(params![], |r| {
                Ok(serde_json::json!({
                    "id": format!("download-{}", r.get::<_, String>(0)?),
                    "target_key": r.get::<_, String>(1)?,
                    "path": r.get::<_, Option<String>>(2)?,
                    "size": r.get::<_, Option<i64>>(3)?,
                    "title": r.get::<_, Option<String>>(5)?,
                    "author": r.get::<_, Option<String>>(6)?,
                    "kind": r.get::<_, Option<String>>(7)?,
                    "record_id": r.get::<_, Option<String>>(8)?,
                    "source_id": r.get::<_, String>(0)?,
                }))
            })?.collect::<Result<Vec<_>,_>>())
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        legacy.append(&mut downloads);
        for row in legacy {
            let Some(source) = row.get("path").and_then(Value::as_str) else {
                continue;
            };
            if source.is_empty() {
                continue;
            }
            let Ok(key) =
                normalize_download_target(row.get("id").and_then(Value::as_str).unwrap_or(source))
            else {
                continue;
            };
            let target = row
                .get("target_key")
                .and_then(Value::as_str)
                .unwrap_or(&key)
                .to_string();
            let dup: Option<(String, String, bool)> = conn
                .query_row(
                    "SELECT id,path,missing FROM assets WHERE target_key=?1 OR path=?2 LIMIT 1",
                    params![target, source],
                    |r| Ok((r.get(0)?, r.get(1)?, r.get::<_, i64>(2)? != 0)),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if let Some((aid, asset_path, missing)) = dup {
                let src = std::path::PathBuf::from(source);
                if missing && src.is_file() {
                    let dest = std::path::PathBuf::from(&asset_path);
                    if let Some(parent) = dest.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    if std::fs::rename(&src, &dest).is_ok()
                        || (std::fs::copy(&src, &dest).is_ok()
                            && std::fs::remove_file(&src).is_ok())
                    {
                        let size = std::fs::metadata(&dest)
                            .map(|m| m.len() as i64)
                            .unwrap_or_default();
                        let _ = conn.execute(
                            "UPDATE assets SET missing=0,size=?2 WHERE id=?1",
                            params![aid, size],
                        );
                    }
                }
                if let Some(did) = row.get("source_id").and_then(Value::as_str) {
                    let _ = conn.execute(
                        "UPDATE downloads SET asset_id=?2,path=?3 WHERE id=?1",
                        params![did, aid, asset_path],
                    );
                }
                continue;
            }
            let title = row.get("title").and_then(Value::as_str).unwrap_or("");
            let author = row.get("author").and_then(Value::as_str).unwrap_or("");
            let kind = legacy_asset_kind(
                source,
                row.get("kind").and_then(Value::as_str).unwrap_or("video"),
            );
            let src = std::path::PathBuf::from(source);
            let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("bin");
            let id = row
                .get("id")
                .and_then(Value::as_str)
                .unwrap_or("legacy")
                .to_string();
            let dest = self.asset_path(&kind, &target, &id, title, author, ext);
            let mut indexed = src.clone();
            if src.exists() && !src.starts_with(&root) {
                if let Some(parent) = dest.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                if std::fs::rename(&src, &dest).is_ok()
                    || (std::fs::copy(&src, &dest).is_ok() && std::fs::remove_file(&src).is_ok())
                {
                    indexed = dest;
                }
            }
            let size = std::fs::metadata(&indexed)
                .ok()
                .map(|m| m.len() as i64)
                .or_else(|| row.get("size").and_then(Value::as_i64));
            conn.execute(
                "INSERT OR IGNORE INTO assets(id,target_key,kind,task_id,record_id,title,author,path,size,missing,created_at) VALUES(?1,?2,?3,NULL,?4,?5,?6,?7,?8,?9,?10)",
                params![Uuid::new_v4().to_string(), target, kind, row.get("record_id").and_then(Value::as_str), title, author, indexed.to_string_lossy(), size, (!indexed.is_file()) as i64, now()],
            ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if let Some(did) = row.get("source_id").and_then(Value::as_str) {
                if let Some(aid) = conn
                    .query_row(
                        "SELECT id FROM assets WHERE path=?1",
                        params![indexed.to_string_lossy()],
                        |r| r.get::<_, String>(0),
                    )
                    .optional()
                    .unwrap_or(None)
                {
                    let _ = conn.execute(
                        "UPDATE downloads SET asset_id=?2,path=?3 WHERE id=?1",
                        params![did, aid, indexed.to_string_lossy()],
                    );
                }
            }
        }
        drop(conn);
        self.repair_asset_links()?;
        self.backfill_download_task_titles()?;
        self.conn()?.execute("INSERT INTO settings(key,value) VALUES('assets_migrated','2') ON CONFLICT(key) DO UPDATE SET value='2'", [])
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    /// 给旧下载任务补上可读标题，任务列表不再只显示目标键。
    fn backfill_download_task_titles(&self) -> PResult<()> {
        let conn = self.conn()?;
        let rows: Vec<(String, Option<String>, Option<String>)> = conn
            .prepare(
                "SELECT t.id,d.title,d.author FROM tasks t JOIN downloads d ON d.id=json_extract(t.params,'$.download_id') WHERE t.kind='download' AND json_extract(t.params,'$.display_title') IS NULL",
            )
            .and_then(|mut st| {
                st.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?
                    .collect::<Result<Vec<_>, _>>()
            })
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        for (id, title, author) in rows {
            conn.execute(
                "UPDATE tasks SET params=json_set(params,'$.display_title',?2,'$.display_subtitle',?3) WHERE id=?1",
                params![id, title, author],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        }
        Ok(())
    }

    /// 修复旧记录未指向迁移后资产的问题，并纠正按扩展名可确定的资产类型。
    fn repair_asset_links(&self) -> PResult<()> {
        let conn = self.conn()?;
        let rows = conn
            .prepare(
                "SELECT d.id,d.path,a.id,a.path,a.missing FROM downloads d JOIN assets a ON a.target_key=d.target_key WHERE d.state='done' AND (d.asset_id IS NULL OR d.path<>a.path)",
            )
            .and_then(|mut st| {
                st.query_map([], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, Option<String>>(1)?,
                        r.get::<_, String>(2)?,
                        r.get::<_, String>(3)?,
                        r.get::<_, i64>(4)? != 0,
                    ))
                })?
                .collect::<Result<Vec<_>, _>>()
            })
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        for (did, source, aid, asset_path, missing) in rows {
            if missing {
                if let Some(src) = source.filter(|v| std::path::Path::new(v).is_file()) {
                    let dest = std::path::PathBuf::from(&asset_path);
                    if let Some(parent) = dest.parent() {
                        let _ = std::fs::create_dir_all(parent);
                    }
                    if std::fs::rename(&src, &dest).is_ok()
                        || (std::fs::copy(&src, &dest).is_ok()
                            && std::fs::remove_file(&src).is_ok())
                    {
                        let size = std::fs::metadata(&dest)
                            .map(|m| m.len() as i64)
                            .unwrap_or_default();
                        let _ = conn.execute(
                            "UPDATE assets SET missing=0,size=?2 WHERE id=?1",
                            params![aid, size],
                        );
                    }
                }
            }
            let _ = conn.execute(
                "UPDATE downloads SET asset_id=?2,path=?3 WHERE id=?1",
                params![did, aid, asset_path],
            );
        }
        let kinds = conn
            .prepare("SELECT id,kind,path FROM assets")
            .and_then(|mut st| {
                st.query_map([], |r| {
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, String>(2)?,
                    ))
                })?
                .collect::<Result<Vec<_>, _>>()
            })
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        for (id, current, path) in kinds {
            let kind = legacy_asset_kind(&path, &current);
            if kind != current {
                let _ = conn.execute("UPDATE assets SET kind=?2 WHERE id=?1", params![id, kind]);
                let _ = conn.execute(
                    "UPDATE downloads SET asset_kind=?2 WHERE asset_id=?1",
                    params![id, kind],
                );
            }
        }
        Ok(())
    }

    pub fn wake(&self) {
        self.wake.store(true, Ordering::SeqCst);
    }

    pub fn take_wake(&self) -> bool {
        self.wake.swap(false, Ordering::SeqCst)
    }

    pub fn conn(&self) -> PResult<Connection> {
        db::open_path(&self.db).map_err(|e| PipelineErr::new(500, "db", e.to_string()))
    }

    pub fn submit_one(&self, item: Submit) -> PResult<Accepted> {
        let batch = Batch {
            items: vec![Item {
                target: item.target,
                link: item.link,
                kind: item.kind,
                endpoint: item.endpoint,
                params: item.params,
                max_attempts: item.max_attempts,
            }],
            batch_id: None,
        };
        let mut out = self.submit_batch(batch)?;
        Ok(out.results.remove(0))
    }

    pub fn submit_batch(&self, batch: Batch) -> PResult<BatchResult> {
        if batch.items.is_empty() {
            return Err(PipelineErr::new(400, "empty_batch", "批次没有条目"));
        }
        if batch.items.len() > 500 {
            return Err(PipelineErr::new(400, "batch_too_large", "单批最多 500 条"));
        }
        let batch_id = batch.batch_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        let mut conn = self.conn()?;
        let tx = conn
            .transaction()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let now = now();
        let mut results = Vec::with_capacity(batch.items.len());
        let mut seen = std::collections::HashMap::<String, String>::new();
        let (mut accepted, mut rejected, mut deduped) = (0, 0, 0);

        for (index, raw) in batch.items.into_iter().enumerate() {
            let max_attempts = raw.max_attempts;
            let parsed = match normalize(raw) {
                Ok(v) => v,
                Err(e) => {
                    rejected += 1;
                    results.push(Accepted {
                        index,
                        ok: false,
                        id: None,
                        state: None,
                        error_code: Some(e.code),
                        message: Some(e.message),
                        deduped: false,
                    });
                    continue;
                }
            };
            if let Some(id) = seen.get(&parsed.target_key) {
                deduped += 1;
                accepted += 1;
                results.push(Accepted {
                    index,
                    ok: true,
                    id: Some(id.clone()),
                    state: Some("queued".into()),
                    error_code: None,
                    message: None,
                    deduped: true,
                });
                continue;
            }
            let existing: Option<String> = tx
                .query_row(
                    &format!(
                        "SELECT id FROM tasks WHERE target_key=?1 AND state IN {ACTIVE} LIMIT 1"
                    ),
                    params![parsed.target_key],
                    |r| r.get(0),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if let Some(id) = existing {
                deduped += 1;
                accepted += 1;
                seen.insert(parsed.target_key.clone(), id.clone());
                results.push(Accepted {
                    index,
                    ok: true,
                    id: Some(id),
                    state: Some("queued".into()),
                    error_code: None,
                    message: None,
                    deduped: true,
                });
                continue;
            }
            let id = Uuid::new_v4().to_string();
            let plan_id = parsed
                .params
                .get("plan_id")
                .and_then(Value::as_str)
                .map(str::to_string);
            tx.execute(
                "INSERT INTO tasks(id,kind,endpoint,params,target_key,state,attempts,max_attempts,batch_id,created_at,trigger,plan_id)
                 VALUES(?1,?2,?3,?4,?5,'queued',0,?6,?7,?8,?9,?10)",
                params![
                    id,
                    parsed.kind,
                    parsed.endpoint,
                    parsed.params.to_string(),
                    parsed.target_key,
                    raw_max(max_attempts),
                    batch_id,
                    now,
                    if plan_id.is_some() { "plan" } else { "user" },
                    plan_id
                ],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            seen.insert(parsed.target_key.clone(), id.clone());
            accepted += 1;
            results.push(Accepted {
                index,
                ok: true,
                id: Some(id),
                state: Some("queued".into()),
                error_code: None,
                message: None,
                deduped: false,
            });
        }
        tx.commit()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        self.wake();
        Ok(BatchResult {
            batch_id,
            results,
            accepted,
            rejected,
            deduped,
        })
    }

    pub fn add_plan(&self, req: PlanReq) -> PResult<PlanView> {
        let kind = req.kind.unwrap_or_else(|| "author_new".into());
        let raw = req.target.or(req.target_key).unwrap_or_default();
        let (target_key, params) = normalize_plan(&kind, &raw, req.params)?;
        let interval = plan_interval(&kind, req.interval_sec.unwrap_or(0));
        let id = Uuid::new_v4().to_string();
        let now = now();
        let conn = self.conn()?;
        let exists: Option<String> = conn
            .query_row(
                "SELECT id FROM plans WHERE kind=?1 AND target_key=?2 AND enabled=1",
                params![kind, target_key],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if let Some(id) = exists {
            return self.get_plan(&id);
        }
        conn.execute(
            "INSERT INTO plans(id,kind,target_key,interval_sec,params,enabled,next_due_at,created_at) VALUES(?1,?2,?3,?4,?5,?6,?7,?8)",
            params![id, kind, target_key, interval, params.to_string(), req.enabled.unwrap_or(true) as i64, now + 1, now],
        ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        self.wake();
        self.get_plan(&id)
    }

    pub fn list_plans(&self) -> PResult<Vec<PlanView>> {
        let conn = self.conn()?;
        let mut st = conn.prepare("SELECT id,kind,target_key,interval_sec,params,enabled,next_due_at,fail_streak,last_task_id,last_error,created_at FROM plans ORDER BY enabled DESC,next_due_at,created_at")
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let rows = st
            .query_map([], row_plan)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(rows.flatten().collect())
    }

    pub fn get_plan(&self, id: &str) -> PResult<PlanView> {
        self.conn()?
            .query_row("SELECT id,kind,target_key,interval_sec,params,enabled,next_due_at,fail_streak,last_task_id,last_error,created_at FROM plans WHERE id=?1", params![id], row_plan)
            .optional().map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .ok_or_else(|| PipelineErr::new(404, "not_found", "计划不存在"))
    }

    pub fn update_plan(&self, id: &str, req: PlanReq) -> PResult<PlanView> {
        let old = self.get_plan(id)?;
        let interval = plan_interval(&old.kind, req.interval_sec.unwrap_or(old.interval_sec));
        let params = req.params.unwrap_or(old.params);
        let enabled = req.enabled.unwrap_or(old.enabled);
        let conn = self.conn()?;
        if enabled {
            let conflict: Option<String> = conn
                .query_row(
                    "SELECT id FROM plans WHERE kind=?1 AND target_key=?2 AND enabled=1 AND id<>?3",
                    params![old.kind, old.target_key, id],
                    |r| r.get(0),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if conflict.is_some() {
                return Err(PipelineErr::new(
                    409,
                    "duplicate_plan",
                    "同一目标已有启用计划",
                ));
            }
        }
        conn.execute(
            "UPDATE plans SET interval_sec=?2,params=?3,enabled=?4,next_due_at=CASE WHEN ?4=1 THEN MIN(next_due_at,?5) ELSE next_due_at END WHERE id=?1",
            params![id, interval, params.to_string(), enabled as i64, now() + interval],
        ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        self.wake();
        self.get_plan(id)
    }

    pub fn set_plan_enabled(&self, id: &str, enabled: bool) -> PResult<PlanView> {
        let p = self.get_plan(id)?;
        let conn = self.conn()?;
        if enabled {
            let conflict: Option<String> = conn
                .query_row(
                    "SELECT id FROM plans WHERE kind=?1 AND target_key=?2 AND enabled=1 AND id<>?3",
                    params![p.kind, p.target_key, id],
                    |r| r.get(0),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if conflict.is_some() {
                return Err(PipelineErr::new(
                    409,
                    "duplicate_plan",
                    "同一目标已有启用计划",
                ));
            }
        }
        conn.execute(
            "UPDATE plans SET enabled=?2,next_due_at=CASE WHEN ?2=1 THEN ?3 ELSE next_due_at END WHERE id=?1",
            params![id, enabled as i64, now() + 1],
        ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        self.wake();
        self.get_plan(id)
    }

    pub fn del_plan(&self, id: &str) -> PResult<()> {
        let n = self
            .conn()?
            .execute("DELETE FROM plans WHERE id=?1", params![id])
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if n == 0 {
            return Err(PipelineErr::new(404, "not_found", "计划不存在"));
        }
        Ok(())
    }

    pub fn dispatch_plans(&self) -> PResult<usize> {
        let due = self
            .conn()?
            .prepare("SELECT id,kind,target_key,interval_sec,params FROM plans WHERE enabled=1 AND next_due_at<=?1 ORDER BY next_due_at LIMIT 50")
            .and_then(|mut st| {
                st.query_map(params![now()], |r| {
                    Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?, r.get::<_, i64>(3)?, r.get::<_, String>(4)?))
                })?.collect::<Result<Vec<_>, _>>()
            })
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let mut n = 0;
        for (id, kind, target, interval, raw_params) in due {
            let params: Value =
                serde_json::from_str(&raw_params).unwrap_or_else(|_| serde_json::json!({}));
            if kind == "author_new" {
                let mut task_params = params.as_object().cloned().unwrap_or_default();
                task_params.insert("plan_id".into(), Value::String(id.clone()));
                task_params.insert("target_type".into(), Value::String("user".into()));
                let accepted = self.submit_one(Submit {
                    target: Some(target.clone()),
                    link: None,
                    kind: Some("collection".into()),
                    endpoint: None,
                    params: Some(Value::Object(task_params)),
                    max_attempts: None,
                })?;
                if accepted.ok {
                    n += 1;
                    if let Some(tid) = accepted.id {
                        self.conn()?
                            .execute(
                                "UPDATE plans SET last_task_id=?2,next_due_at=?3 WHERE id=?1",
                                params![id, tid, now() + interval],
                            )
                            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
                    }
                }
                continue;
            }
            if kind != "live_auto" {
                continue;
            }
            let room = record_room(&target);
            let active: Option<String> = self
                .conn()?
                .query_row(
                    "SELECT id FROM record_sessions WHERE room_id=?1 AND state IN ('queued','running') LIMIT 1",
                    params![room],
                    |r| r.get(0),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if let (Some(sid), Some(app)) = (active.as_deref(), self.app.get()) {
                if !rec::get(app, sid)
                    .is_some_and(|s| matches!(s.state.as_str(), "queued" | "running"))
                {
                    self.conn()?.execute(
                        "UPDATE record_sessions SET state='failed',finished_at=?2,error='进程重启，会话中断' WHERE id=?1",
                        params![sid, now()],
                    ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
                }
            }
            let active: Option<String> = self
                .conn()?
                .query_row(
                    "SELECT id FROM record_sessions WHERE room_id=?1 AND state IN ('queued','running') LIMIT 1",
                    params![room],
                    |r| r.get(0),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            let pending: Option<String> = self
                .conn()?
                .query_row(
                    "SELECT id FROM downloads WHERE asset_key LIKE ?1 AND asset_kind='record' AND state IN ('queued','running','paused') LIMIT 1",
                    params![format!("record:{room}:%")],
                    |r| r.get(0),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if active.is_none() && pending.is_none() {
                let last_end: Option<i64> = self
                    .conn()?
                    .query_row(
                        "SELECT finished_at FROM record_sessions WHERE room_id=?1 AND state IN ('done','failed') AND finished_at IS NOT NULL ORDER BY finished_at DESC LIMIT 1",
                        params![room],
                        |r| r.get(0),
                    )
                    .optional()
                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
                if last_end.is_some_and(|v| v > now() - 600) {
                    self.conn()?
                        .execute(
                            "UPDATE plans SET next_due_at=?2 WHERE id=?1",
                            params![id, now() + interval],
                        )
                        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
                    continue;
                }
                let api = self
                    .api
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .clone()
                    .ok_or_else(|| PipelineErr::new(503, "backend", "执行后端未就绪"))?;
                match api.call(&format!("/live/{room}/pull"), &Value::Null) {
                    Ok(v) => {
                        let url = v.get("url").and_then(Value::as_str).unwrap_or_default();
                        if !url.is_empty() {
                            let rec_limit = self
                                .settings_value()
                                .get("recordConcurrency")
                                .and_then(Value::as_u64)
                                .unwrap_or(2)
                                .clamp(1, 4);
                            let active_count: i64 = self.conn()?.query_row(
                                "SELECT COUNT(*) FROM record_sessions WHERE state IN ('queued','running')",
                                [],
                                |r| r.get(0),
                            ).unwrap_or(0);
                            let extra = serde_json::json!({
                                "plan_id": id,
                                "max_secs": params.get("max_secs").cloned(),
                                "max_bytes": params.get("max_bytes").cloned(),
                                "retain_bytes": params.get("retain_bytes").cloned(),
                                "miss_reason": if active_count >= rec_limit as i64 { "concurrency" } else { "" }
                            });
                            let d = self.submit_download(DownloadReq {
                                target: Some(format!("room:{room}")),
                                target_key: None,
                                asset_kind: Some("record".into()),
                                url: Some(url.into()),
                                pinned: Some(true),
                                title: params
                                    .get("title")
                                    .and_then(Value::as_str)
                                    .map(str::to_string),
                                author: params
                                    .get("author")
                                    .and_then(Value::as_str)
                                    .map(str::to_string),
                                cover: params
                                    .get("cover")
                                    .and_then(Value::as_str)
                                    .map(str::to_string),
                                ext: Some("m3u8".into()),
                                expected_bytes: None,
                                record_id: Some(id.clone()),
                                parent_id: Some(id.clone()),
                                params: Some(extra),
                            })?;
                            if let Some(tid) = d.task_id {
                                self.conn()?.execute("UPDATE plans SET last_task_id=?2,last_error=NULL WHERE id=?1", params![id, tid])
                                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
                                n += 1;
                            }
                        }
                    }
                    Err(e) => {
                        let _ = self.conn()?.execute(
                            "UPDATE plans SET last_error=?2 WHERE id=?1",
                            params![id, e.message],
                        );
                    }
                }
            }
            self.conn()?
                .execute(
                    "UPDATE plans SET next_due_at=?2 WHERE id=?1",
                    params![id, now() + interval],
                )
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        }
        Ok(n)
    }

    fn plan_result(&self, task: &TaskView, ok: bool, error: Option<&str>) {
        let Some(pid) = task.params.get("plan_id").and_then(Value::as_str) else {
            return;
        };
        let Ok(conn) = self.conn() else { return };
        if ok {
            let _ = conn.execute(
                "UPDATE plans SET fail_streak=0,last_error=NULL,last_task_id=?2 WHERE id=?1",
                params![pid, task.id],
            );
        } else {
            let streak: i64 = conn
                .query_row(
                    "SELECT fail_streak FROM plans WHERE id=?1",
                    params![pid],
                    |r| r.get(0),
                )
                .unwrap_or(0)
                + 1;
            let _ = conn.execute(
                "UPDATE plans SET fail_streak=?2,last_error=?3,last_task_id=?4 WHERE id=?1",
                params![pid, streak, error, task.id],
            );
        }
    }

    pub fn submit_backfill(&self, req: BackfillReq) -> PResult<Accepted> {
        let target = req.target.unwrap_or_default();
        let pages = req.pages.unwrap_or(5).clamp(1, 50);
        let tk = split_target(&target).0;
        let tk = if tk.is_empty() {
            if let Ok(u) = url::Url::parse(&target) {
                if u.path().split('/').any(|x| x == "user") {
                    "user".to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            tk
        };
        if tk != "user" {
            return Err(PipelineErr::new(
                400,
                "invalid_target",
                "回溯目标必须是用户",
            ));
        }
        let mut params = match req.params {
            Some(Value::Object(m)) => m,
            Some(_) => return Err(PipelineErr::new(400, "invalid_params", "params 必须是对象")),
            None => serde_json::Map::new(),
        };
        params.insert("pages".into(), Value::from(pages));
        self.submit_one(Submit {
            target: Some(target),
            link: None,
            kind: Some("backfill".into()),
            endpoint: None,
            params: Some(Value::Object(params)),
            max_attempts: Some(3),
        })
    }

    fn push_record_filters(
        sql: &mut String,
        args: &mut Vec<String>,
        state: Option<&str>,
        kind: Option<&str>,
        q: Option<&str>,
        from: Option<i64>,
        to: Option<i64>,
    ) {
        if let Some(v) = state.filter(|v| !v.is_empty()) {
            sql.push_str(" AND r.state=?");
            args.push(v.into());
        }
        if let Some(v) = kind.filter(|v| !v.is_empty()) {
            sql.push_str(" AND r.target_key LIKE ?");
            args.push(format!("{v}:%"));
        }
        if let Some(v) = q.map(str::trim).filter(|v| !v.is_empty()) {
            sql.push_str(
                " AND (r.title LIKE ? ESCAPE '\\' OR COALESCE(r.author_name,'') LIKE ? ESCAPE '\\' OR r.target_key LIKE ? ESCAPE '\\')",
            );
            let like = like_pattern(v);
            args.push(like.clone());
            args.push(like.clone());
            args.push(like);
        }
        if let Some(v) = from {
            sql.push_str(" AND r.created_at>=?");
            args.push(v.to_string());
        }
        if let Some(v) = to {
            sql.push_str(" AND r.created_at<?");
            args.push(v.to_string());
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn list_records(
        &self,
        state: Option<&str>,
        kind: Option<&str>,
        q: Option<&str>,
        from: Option<i64>,
        to: Option<i64>,
        cursor: Option<&str>,
        limit: usize,
    ) -> PResult<Vec<RecordView>> {
        let conn = self.conn()?;
        let mut sql = String::from(
            "SELECT r.id,r.target_key,r.task_id,r.title,r.author_key,r.author_name,r.cover,r.thumb_path,r.metrics,r.metrics_history,r.state,r.created_at,r.last_checked,
             (SELECT COUNT(*) FROM captured_comments c WHERE c.record_id=r.id),(SELECT COUNT(*) FROM assets a WHERE a.record_id=r.id AND a.kind!='cover'),
             COALESCE(json_extract(t.params,'$.comments'),0),COALESCE(json_extract(t.params,'$.max_comments'),0),t.outcome,t.output_path
             FROM capture_records r LEFT JOIN tasks t ON t.id=r.task_id WHERE 1=1",
        );
        let mut args: Vec<String> = Vec::new();
        Self::push_record_filters(&mut sql, &mut args, state, kind, q, from, to);
        if let Some(v) = cursor.filter(|v| !v.is_empty()) {
            let (ts, id) = v.split_once('|').unwrap_or((v, ""));
            sql.push_str(
                " AND (r.created_at<CAST(? AS INTEGER) OR (r.created_at=CAST(? AS INTEGER) AND r.id<?))",
            );
            args.push(ts.into());
            args.push(ts.into());
            args.push(id.into());
        }
        sql.push_str(" ORDER BY r.created_at DESC,r.id DESC LIMIT ?");
        let n = limit.clamp(1, 100);
        args.push((n + 1).to_string());
        let mut st = conn
            .prepare(&sql)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let refs: Vec<&dyn rusqlite::ToSql> =
            args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        let rows = st
            .query_map(refs.as_slice(), row_record)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let mut items: Vec<RecordView> = rows.flatten().collect();
        items.truncate(n);
        Ok(items)
    }

    pub fn record_summary(
        &self,
        state: Option<&str>,
        kind: Option<&str>,
        q: Option<&str>,
        from: Option<i64>,
        to: Option<i64>,
    ) -> PResult<RecordSummary> {
        let conn = self.conn()?;
        let mut sql = String::from(
            "SELECT COUNT(*), COALESCE(SUM(r.state='live'),0) FROM capture_records r WHERE 1=1",
        );
        let mut args: Vec<String> = Vec::new();
        Self::push_record_filters(&mut sql, &mut args, state, kind, q, from, to);
        let refs: Vec<&dyn rusqlite::ToSql> =
            args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        let (count, live): (i64, i64) = conn
            .query_row(&sql, refs.as_slice(), |r| Ok((r.get(0)?, r.get(1)?)))
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(RecordSummary {
            count,
            live,
            deleted: count - live,
        })
    }

    pub fn record_action(&self, req: RecordActionReq) -> PResult<AssetActionResult> {
        if req.action != "delete" {
            return Err(PipelineErr::new(
                400,
                "invalid_action",
                format!("未知动作 {}", req.action),
            ));
        }
        let ids = match req.ids {
            Some(ids) => ids,
            None => {
                let conn = self.conn()?;
                let mut sql = String::from("SELECT r.id FROM capture_records r WHERE 1=1");
                let mut args: Vec<String> = Vec::new();
                Self::push_record_filters(
                    &mut sql,
                    &mut args,
                    req.state.as_deref(),
                    req.kind.as_deref(),
                    req.q.as_deref(),
                    req.from,
                    req.to,
                );
                sql.push_str(" ORDER BY r.created_at DESC,r.id DESC LIMIT 5000");
                let mut st = conn
                    .prepare(&sql)
                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
                let refs: Vec<&dyn rusqlite::ToSql> =
                    args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
                let rows = st
                    .query_map(refs.as_slice(), |r| r.get::<_, String>(0))
                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
                rows.flatten().collect()
            }
        };
        let mut count = 0;
        for id in ids {
            if self.delete_record(&id).is_ok() {
                count += 1;
            }
        }
        Ok(AssetActionResult { count })
    }

    pub fn get_record(&self, id: &str) -> PResult<RecordView> {
        self.conn()?
            .query_row(
                "SELECT r.id,r.target_key,r.task_id,r.title,r.author_key,r.author_name,r.cover,r.thumb_path,r.metrics,r.metrics_history,r.state,r.created_at,r.last_checked,
                 (SELECT COUNT(*) FROM captured_comments c WHERE c.record_id=r.id),(SELECT COUNT(*) FROM assets a WHERE a.record_id=r.id AND a.kind!='cover'),
                 COALESCE(json_extract(t.params,'$.comments'),0),COALESCE(json_extract(t.params,'$.max_comments'),0),t.outcome,t.output_path
                 FROM capture_records r LEFT JOIN tasks t ON t.id=r.task_id WHERE r.id=?1",
                params![id],
                row_record,
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .ok_or_else(|| PipelineErr::new(404, "not_found", "采集记录不存在"))
    }

    pub fn list_record_comments(
        &self,
        id: &str,
        cursor: Option<&str>,
        level: Option<i64>,
        parent: Option<&str>,
        q: Option<&str>,
        limit: usize,
    ) -> PResult<CommentPage> {
        let conn = self.conn()?;
        let cols =
            "cid,parent_cid,level,author_key,author_name,content,likes,ip_label,created_at,raw";
        let query = q.map(str::trim).filter(|v| !v.is_empty());
        let mut sql = format!("SELECT {cols} FROM captured_comments WHERE record_id=?1");
        let mut args: Vec<String> = vec![id.into()];
        if let Some(v) = query {
            sql.push_str(
                " AND (content LIKE ? ESCAPE '\\' OR COALESCE(author_name,'') LIKE ? ESCAPE '\\' OR COALESCE(author_key,'') LIKE ? ESCAPE '\\')",
            );
            let like = like_pattern(v);
            args.push(like.clone());
            args.push(like.clone());
            args.push(like);
        } else if let Some(v) = level {
            sql.push_str(" AND level=?");
            args.push(v.to_string());
        } else if parent.filter(|v| !v.is_empty()).is_none() {
            sql.push_str(" AND (parent_cid IS NULL OR parent_cid='')");
        }
        if query.is_none() {
            if let Some(v) = parent.filter(|v| !v.is_empty()) {
                sql.push_str(" AND parent_cid=?");
                args.push(v.into());
            }
        }
        let base_args = args.clone();
        if let Some(v) = cursor.filter(|v| !v.is_empty()) {
            let (ts, cid) = v.split_once('|').unwrap_or((v, ""));
            sql.push_str(" AND (COALESCE(created_at,0)<CAST(? AS INTEGER) OR (COALESCE(created_at,0)=CAST(? AS INTEGER) AND cid<?))");
            args.push(ts.into());
            args.push(ts.into());
            args.push(cid.into());
        }
        let n = limit.clamp(1, 100);
        sql.push_str(" ORDER BY COALESCE(created_at,0) DESC,cid LIMIT ?");
        args.push((n + 1).to_string());
        let mut st = conn
            .prepare(&sql)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let refs: Vec<&dyn rusqlite::ToSql> =
            args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        let rows = st
            .query_map(refs.as_slice(), row_comment)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let mut roots: Vec<CommentView> = rows.flatten().collect();
        let next = if roots.len() > n {
            roots.truncate(n);
            roots
                .last()
                .map(|c| format!("{}|{}", c.created_at.unwrap_or(0), c.cid))
        } else {
            None
        };

        let mut count_sql =
            String::from("SELECT COUNT(*) FROM captured_comments WHERE record_id=?1");
        if query.is_some() {
            count_sql.push_str(
                " AND (content LIKE ? ESCAPE '\\' OR COALESCE(author_name,'') LIKE ? ESCAPE '\\' OR COALESCE(author_key,'') LIKE ? ESCAPE '\\')",
            );
        } else if level.is_some() || parent.filter(|v| !v.is_empty()).is_some() {
            count_sql.push_str(" AND 1=1");
            if level.is_some() {
                count_sql.push_str(" AND level=?");
            } else if parent.filter(|v| !v.is_empty()).is_some() {
                count_sql.push_str(" AND parent_cid=?");
            }
        } else {
            count_sql.push_str(" AND (parent_cid IS NULL OR parent_cid='')");
        }
        let count_refs: Vec<&dyn rusqlite::ToSql> = base_args
            .iter()
            .map(|v| v as &dyn rusqlite::ToSql)
            .collect();
        let total: i64 = conn
            .query_row(&count_sql, count_refs.as_slice(), |r| r.get(0))
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;

        if query.is_none()
            && level.is_none()
            && parent.filter(|v| !v.is_empty()).is_none()
            && !roots.is_empty()
        {
            let roots_json =
                serde_json::to_string(&roots.iter().map(|r| r.cid.as_str()).collect::<Vec<_>>())
                    .unwrap_or_else(|_| "[]".into());
            let mut ds = conn
                .prepare(
                    "WITH RECURSIVE descendants(cid) AS (
                       SELECT cid FROM captured_comments WHERE record_id=?1 AND parent_cid IN (SELECT value FROM json_each(?2))
                       UNION
                       SELECT c.cid FROM captured_comments c JOIN descendants d ON c.parent_cid=d.cid WHERE c.record_id=?1
                     )
                     SELECT c.cid,c.parent_cid,c.level,c.author_key,c.author_name,c.content,c.likes,c.ip_label,c.created_at,c.raw
                     FROM captured_comments c JOIN descendants d ON d.cid=c.cid
                     ORDER BY c.level,c.created_at,c.cid",
                )
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            let children = ds
                .query_map(params![id, roots_json], row_comment)
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            roots.extend(children.flatten());
        }
        Ok(CommentPage {
            items: roots,
            next,
            total,
        })
    }

    pub fn recheck(&self, req: RecheckReq) -> PResult<BatchResult> {
        let limit = req.limit.unwrap_or(20).clamp(1, 200);
        let rows = {
            let conn = self.conn()?;
            let mut st = conn.prepare(
                "SELECT id,target_key FROM capture_records WHERE state='live' ORDER BY COALESCE(last_checked,0),created_at LIMIT ?1"
            ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            let rows = st
                .query_map(params![limit], |r| {
                    Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
                })
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            rows.flatten().collect::<Vec<_>>()
        };
        let batch = Uuid::new_v4().to_string();
        let mut results = Vec::new();
        for (index, (sid, key)) in rows.into_iter().enumerate() {
            let accepted = self.submit_one(Submit {
                target: Some(key),
                link: None,
                kind: Some("recheck".into()),
                endpoint: None,
                params: Some(serde_json::json!({"record_id": sid})),
                max_attempts: Some(4),
            })?;
            results.push(Accepted { index, ..accepted });
        }
        let accepted = results.iter().filter(|x| x.ok).count();
        Ok(BatchResult {
            batch_id: batch,
            rejected: results.len() - accepted,
            deduped: results.iter().filter(|x| x.deduped).count(),
            accepted,
            results,
        })
    }

    pub fn recheck_record(&self, id: &str) -> PResult<Accepted> {
        let r = self.get_record(id)?;
        if !r.target_key.starts_with("video:") {
            return Err(PipelineErr::new(400, "invalid_target", "仅作品记录可复查"));
        }
        self.submit_one(Submit {
            target: Some(r.target_key),
            link: None,
            kind: Some("recheck".into()),
            endpoint: None,
            params: Some(serde_json::json!({"record_id": id})),
            max_attempts: Some(4),
        })
    }

    pub fn delete_record(&self, id: &str) -> PResult<()> {
        let r = self.get_record(id)?;
        let conn = self.conn()?;
        if let Some(task_id) = r.task_id {
            let ts = now();
            let _ = conn.execute(
                "UPDATE tasks SET deleted_at=?2,archived_at=COALESCE(archived_at,?2),acknowledged_at=COALESCE(acknowledged_at,?2) WHERE id=?1",
                params![task_id, ts],
            );
        }
        let _ = conn.execute(
            "DELETE FROM captured_comments WHERE record_id=?1",
            params![id],
        );
        let _ = conn.execute(
            "UPDATE assets SET record_id=NULL WHERE record_id=?1",
            params![id],
        );
        if let Some(thumb) = r.thumb_path {
            let _ = std::fs::remove_file(thumb);
        }
        let n = conn
            .execute("DELETE FROM capture_records WHERE id=?1", params![id])
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if n == 0 {
            return Err(PipelineErr::new(404, "not_found", "采集记录不存在"));
        }
        Ok(())
    }

    fn mark_snapshot(&self, task: &TaskView, deleted: bool) -> PResult<()> {
        let Some(rid) = task.params.get("record_id").and_then(Value::as_str) else {
            return Ok(());
        };
        self.conn()?.execute(
            "UPDATE capture_records SET last_checked=?2,state=CASE WHEN ?3 THEN 'deleted' ELSE 'live' END WHERE id=?1",
            params![rid, now(), deleted as i64],
        ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    pub fn submit_download(&self, req: DownloadReq) -> PResult<DownloadView> {
        let kind = req.asset_kind.unwrap_or_else(|| "video".into());
        if !ASSET_KINDS.contains(&kind.as_str()) {
            return Err(PipelineErr::new(400, "invalid_asset_kind", "未知资产类型"));
        }
        let raw = req.target.unwrap_or_default();
        let base_key = normalize_download_target(&raw)?;
        let id_part = digits19(&raw).unwrap_or(&base_key);
        let asset_key = req
            .target_key
            .filter(|v| !v.trim().is_empty())
            .unwrap_or_else(|| match kind.as_str() {
                "video" => base_key.clone(),
                "record" => format!("record:{id_part}:{}", now()),
                other => format!("{base_key}:{other}"),
            });
        let mut conn = self.conn()?;
        // 同一房间同一时间只保留一个录制会话；重复提交返回正在进行的任务。
        if kind == "record" {
            if let Some(did) = conn
                .query_row(
                    "SELECT id FROM downloads WHERE asset_key LIKE ?1 AND asset_kind='record' AND state IN ('queued','running','paused') ORDER BY created_at DESC LIMIT 1",
                    params![format!("record:{id_part}:%")],
                    |r| r.get::<_, String>(0),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            {
                return self.get_download(&did);
            }
        }
        if let Some(path) = conn
            .query_row(
                "SELECT path FROM assets WHERE target_key=?1",
                params![asset_key],
                |r| r.get::<_, String>(0),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
        {
            if let Some(v) = find_download(&conn, &asset_key)? {
                return Ok(v);
            }
            let did = Uuid::new_v4().to_string();
            conn.execute(
                "INSERT OR IGNORE INTO downloads(id,target_key,asset_key,url,path,bytes,state,created_at,title,author,asset_kind,cover,record_id) VALUES(?1,?2,?2,'',?3,?4,'done',?5,?6,?7,?8,?9,?10)",
                params![did, asset_key, path, 0, now(), req.title, req.author, kind, req.cover, req.record_id],
            ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            return self.get_download(&did);
        }
        if let Some(v) = find_download(&conn, &asset_key)? {
            return Ok(v);
        }
        let url = req.url.unwrap_or_default();
        let ext = clean_ext(req.ext.as_deref().unwrap_or(match kind.as_str() {
            "video" | "record" => "mp4",
            "cover" | "gallery" => "jpg",
            "music" => "mp3",
            _ => "bin",
        }));
        let path = self.asset_path(
            &kind,
            &asset_key,
            id_part,
            req.title.as_deref().unwrap_or(""),
            req.author.as_deref().unwrap_or(""),
            &ext,
        );
        let force_endpoint = req
            .params
            .as_ref()
            .and_then(|v| v.get("force_endpoint"))
            .and_then(Value::as_str)
            .filter(|v| !v.trim().is_empty())
            .map(str::to_string);
        let (endpoint, target) = if kind == "video" && url.is_empty() {
            let vid = base_key.trim_start_matches("video:");
            (format!("/video/{vid}/download"), base_key.clone())
        } else if kind == "record" {
            let room = raw
                .trim_start_matches("room:")
                .trim_start_matches("live:")
                .to_string();
            (
                force_endpoint.unwrap_or_else(|| format!("/live/{room}/pull")),
                format!("room:{room}"),
            )
        } else {
            if url.is_empty() {
                return Err(PipelineErr::new(400, "no_url", "非视频资产需要直链"));
            }
            (String::new(), base_key.clone())
        };
        let did = Uuid::new_v4().to_string();
        let tid = Uuid::new_v4().to_string();
        let tx = conn
            .transaction()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.execute(
            "INSERT INTO downloads(id,task_id,target_key,asset_key,url,path,bytes,expected_bytes,state,pinned,created_at,title,author,asset_kind,ext,cover,record_id) VALUES(?1,?2,?3,?3,?4,?5,0,?6,'queued',?7,?8,?9,?10,?11,?12,?13,?14)",
            params![did, tid, asset_key, url, path.to_string_lossy(), req.expected_bytes, req.pinned.unwrap_or(false) as i64, now(), req.title, req.author, kind, ext, req.cover, req.record_id],
        ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let mut task_params = serde_json::json!({
            "download_id": did,
            "target": target,
            "asset_key": asset_key,
            "asset_kind": kind,
            "display_title": req.title,
            "display_subtitle": req.author,
        });
        if let (Some(extra), Some(dst)) = (req.params, task_params.as_object_mut()) {
            if let Some(extra) = extra.as_object() {
                dst.extend(extra.clone());
            }
        }
        tx.execute(
            "INSERT INTO tasks(id,kind,endpoint,params,target_key,state,attempts,max_attempts,created_at,parent_id,source,priority) VALUES(?1,'download',?2,?3,?4,'queued',0,4,?5,?6,'download',?7)",
            params![tid, endpoint, task_params.to_string(), format!("download:{asset_key}"), now(), req.parent_id, req.pinned.unwrap_or(false) as i64],
        ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.commit()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        self.wake();
        self.get_download(&did)
    }

    pub fn list_downloads(&self, limit: usize) -> PResult<Vec<DownloadView>> {
        let conn = self.conn()?;
        let mut st = conn.prepare("SELECT id,task_id,target_key,asset_key,asset_kind,url,path,partial_path,bytes,expected_bytes,progress,speed,state,pinned,attempts,created_at,finished_at,error,title,author,cover,record_id,asset_id,task_deleted_at FROM downloads ORDER BY pinned DESC, priority DESC, created_at DESC LIMIT ?1")
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let rows = st
            .query_map(params![limit.clamp(1, 1000) as i64], row_download)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r.map_err(|e| PipelineErr::new(500, "db", e.to_string()))?);
        }
        Ok(out)
    }

    pub fn pause_download(&self, id: &str) -> PResult<DownloadView> {
        let conn = self.conn()?;
        let (state, tid): (String, Option<String>) = conn
            .query_row(
                "SELECT state,task_id FROM downloads WHERE id=?1",
                params![id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .ok_or_else(|| PipelineErr::new(404, "not_found", "下载不存在"))?;
        if !matches!(state.as_str(), "queued" | "running") {
            return Err(PipelineErr::new(
                409,
                "not_pausable",
                format!("状态 {state}"),
            ));
        }
        conn.execute(
            "UPDATE downloads SET state='paused',speed=0 WHERE id=?1",
            params![id],
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if let Some(tid) = tid {
            let _ = conn.execute(
                "UPDATE tasks SET state='paused',started_at=NULL WHERE id=?1 AND state IN ('queued','running')",
                params![tid],
            );
        }
        self.get_download(id)
    }

    pub fn resume_download(&self, id: &str) -> PResult<DownloadView> {
        let conn = self.conn()?;
        let row: ResumeRow = conn
            .query_row(
                "SELECT d.state,d.target_key,d.task_id,d.asset_kind,d.record_id,d.pinned,d.title,d.author FROM downloads d WHERE id=?1",
                params![id],
                |r| {
                    Ok(ResumeRow {
                        state: r.get(0)?,
                        key: r.get(1)?,
                        tid: r.get(2)?,
                        kind: r.get(3)?,
                        target: r.get(1)?,
                        record_id: r.get(4)?,
                        pinned: r.get(5)?,
                        title: r.get(6)?,
                        author: r.get(7)?,
                    })
                },
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .ok_or_else(|| PipelineErr::new(404, "not_found", "下载不存在"))?;
        let ResumeRow {
            state,
            key,
            tid,
            kind,
            target,
            record_id: parent,
            pinned,
            title,
            author,
        } = row;
        let endpoint = String::new();
        if state != "paused" {
            return Err(PipelineErr::new(409, "not_paused", format!("状态 {state}")));
        }
        let endpoint = if kind == "video" && endpoint.is_empty() {
            format!("/video/{}/download", target.trim_start_matches("video:"))
        } else if kind == "record" {
            format!("/live/{}/pull", record_room(&target))
        } else {
            endpoint
        };
        conn.execute(
            "UPDATE downloads SET state='queued',finished_at=NULL,error=NULL WHERE id=?1",
            params![id],
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let revivable = tid
            .as_deref()
            .map(|tid| {
                conn.execute(
                    "UPDATE tasks SET state='queued',started_at=NULL,finished_at=NULL,error_code=NULL WHERE id=?1 AND state='paused'",
                    params![tid],
                )
                .unwrap_or(0)
                    > 0
            })
            .unwrap_or(false);
        if !revivable {
            let ntid = Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO tasks(id,kind,endpoint,params,target_key,state,attempts,max_attempts,created_at,parent_id,source,priority) VALUES(?1,'download',?2,?3,?4,'queued',0,4,?5,?6,'download',?7)",
                params![ntid, endpoint, serde_json::json!({"download_id": id, "target": target, "asset_key": key, "asset_kind": kind, "display_title": title, "display_subtitle": author}).to_string(), format!("download:{key}"), now(), parent, pinned],
            ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            conn.execute(
                "UPDATE downloads SET task_id=?2 WHERE id=?1",
                params![id, ntid],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        }
        self.wake();
        self.get_download(id)
    }

    pub fn cancel_download(&self, id: &str) -> PResult<DownloadView> {
        let conn = self.conn()?;
        let (state, tid): (String, Option<String>) = conn
            .query_row(
                "SELECT state,task_id FROM downloads WHERE id=?1",
                params![id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .ok_or_else(|| PipelineErr::new(404, "not_found", "下载不存在"))?;
        if !matches!(state.as_str(), "queued" | "running" | "paused") {
            return Err(PipelineErr::new(
                409,
                "not_cancelable",
                format!("状态 {state}"),
            ));
        }
        conn.execute(
            "UPDATE downloads SET state='canceled',finished_at=?2,error='canceled',speed=0 WHERE id=?1",
            params![id, now()],
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if let Some(tid) = tid {
            let _ = conn.execute(
                "UPDATE tasks SET state='canceled',error_code='cancelled',finished_at=?2 WHERE id=?1 AND state IN ('queued','running','paused')",
                params![tid, now()],
            );
        }
        self.get_download(id)
    }

    pub fn delete_download(&self, id: &str, trash_file: bool) -> PResult<()> {
        let conn = self.conn()?;
        let (state, key, tid, path, asset_id): (
            String,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
        ) = conn
            .query_row(
                "SELECT state,target_key,task_id,path,asset_id FROM downloads WHERE id=?1",
                params![id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .ok_or_else(|| PipelineErr::new(404, "not_found", "下载不存在"))?;
        if !matches!(state.as_str(), "done" | "failed" | "gone" | "canceled") {
            return Err(PipelineErr::new(
                409,
                "active_download",
                format!("状态 {state}"),
            ));
        }
        if trash_file {
            if let Some(path) = &path {
                trash_with_parts(std::path::Path::new(path))?;
            }
        } else if let Some(path) = &path {
            let _ = std::fs::remove_file(partial_path(std::path::Path::new(path)));
        }
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let ts = now();
        if let Some(tid) = tid {
            let _ = tx.execute(
                "UPDATE tasks SET deleted_at=?2,archived_at=COALESCE(archived_at,?2),acknowledged_at=COALESCE(acknowledged_at,?2) WHERE id=?1",
                params![tid, ts],
            );
        }
        let _ = tx.execute(
            "UPDATE tasks SET deleted_at=?2,archived_at=COALESCE(archived_at,?2),acknowledged_at=COALESCE(acknowledged_at,?2) WHERE target_key=?1",
            params![format!("download:{key}"), ts],
        );
        if let Some(aid) = asset_id {
            let _ = tx.execute("DELETE FROM assets WHERE id=?1", params![aid]);
        }
        tx.execute("DELETE FROM downloads WHERE id=?1", params![id])
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.commit()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    pub fn get_download(&self, id: &str) -> PResult<DownloadView> {
        let conn = self.conn()?;
        conn.query_row("SELECT id,task_id,target_key,asset_key,asset_kind,url,path,partial_path,bytes,expected_bytes,progress,speed,state,pinned,attempts,created_at,finished_at,error,title,author,cover,record_id,asset_id,task_deleted_at FROM downloads WHERE id=?1", params![id], row_download)
            .optional().map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .ok_or_else(|| PipelineErr::new(404, "not_found", "下载不存在"))
    }

    pub fn retry_download(&self, id: &str) -> PResult<DownloadView> {
        let conn = self.conn()?;
        let (state, key, tid, title, author): (
            String,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
        ) = conn
            .query_row(
                "SELECT state,target_key,task_id,title,author FROM downloads WHERE id=?1",
                params![id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .ok_or_else(|| PipelineErr::new(404, "not_found", "下载不存在"))?;
        if !matches!(state.as_str(), "failed" | "gone" | "canceled") {
            return Err(PipelineErr::new(
                409,
                "not_retryable",
                format!("状态 {state}"),
            ));
        }
        conn.execute(
            "UPDATE downloads SET state='queued',finished_at=NULL,error=NULL,progress=0,speed=0 WHERE id=?1",
            params![id],
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let live: Option<String> = conn.query_row("SELECT id FROM tasks WHERE target_key=?1 AND state IN ('queued','running') LIMIT 1", params![format!("download:{key}")], |r| r.get(0)).optional().map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if live.is_none() {
            let ntid = Uuid::new_v4().to_string();
            let kind: String = conn
                .query_row(
                    "SELECT asset_kind FROM downloads WHERE id=?1",
                    params![id],
                    |r| r.get(0),
                )
                .unwrap_or_else(|_| "video".into());
            let endpoint = if kind == "video" {
                format!("/video/{}/download", key.trim_start_matches("video:"))
            } else if kind == "record" {
                format!(
                    "/live/{}/pull",
                    key.trim_start_matches("record:")
                        .split(':')
                        .next()
                        .unwrap_or("")
                )
            } else {
                String::new()
            };
            conn.execute("INSERT INTO tasks(id,kind,endpoint,params,target_key,state,max_attempts,created_at,source) VALUES(?1,'download',?2,?3,?4,'queued',4,?5,'download')",
                params![ntid, endpoint, serde_json::json!({"download_id": id, "target": key, "asset_key": key, "asset_kind": kind, "display_title": title, "display_subtitle": author}).to_string(), format!("download:{key}"), now()])
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            conn.execute(
                "UPDATE downloads SET task_id=?2 WHERE id=?1",
                params![id, ntid],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        } else if let (Some(old), Some(_)) = (tid, live.as_ref()) {
            let _ = conn.execute(
                "UPDATE tasks SET state='queued',error_code=NULL,started_at=NULL WHERE id=?1",
                params![old],
            );
        }
        self.wake();
        self.get_download(id)
    }

    pub fn pin_download(&self, id: &str, pinned: bool) -> PResult<DownloadView> {
        self.conn()?
            .execute(
                "UPDATE downloads SET pinned=?2,priority=?2 WHERE id=?1",
                params![id, pinned as i64],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        self.get_download(id)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn list_assets(
        &self,
        filter: &AssetFilter,
        hide_cover: bool,
        cursor: Option<&str>,
        limit: usize,
    ) -> PResult<AssetPage> {
        let conn = self.conn()?;
        let mut sql = String::from(
            "SELECT id,target_key,kind,title,author,cover,path,size,task_id,record_id,created_at,COALESCE((SELECT c.path FROM assets c WHERE c.kind='cover' AND c.target_key=assets.target_key||':cover'),'') FROM assets WHERE 1=1",
        );
        let mut args: Vec<String> = Vec::new();
        apply_asset_filter(&mut sql, &mut args, filter, hide_cover);
        apply_asset_cursor(&mut sql, &mut args, cursor);
        sql.push_str(" ORDER BY created_at DESC,id DESC LIMIT ?");
        let n = limit.clamp(1, 100);
        args.push((n + 1).to_string());
        let mut st = conn
            .prepare(&sql)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let refs: Vec<&dyn rusqlite::ToSql> =
            args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        let rows = st
            .query_map(refs.as_slice(), row_asset)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let mut items: Vec<AssetView> = rows
            .flatten()
            .map(|mut a| {
                a.missing = !std::path::Path::new(&a.path).is_file();
                a
            })
            .collect();
        let next = if items.len() > n {
            items.truncate(n);
            items.last().map(|a| format!("{}|{}", a.created_at, a.id))
        } else {
            None
        };
        Ok(AssetPage { items, next })
    }

    pub fn asset_summary(&self, filter: &AssetFilter) -> PResult<i64> {
        let conn = self.conn()?;
        let mut sql = String::from("SELECT COUNT(*) FROM assets WHERE 1=1");
        let mut args: Vec<String> = Vec::new();
        apply_asset_filter(&mut sql, &mut args, filter, true);
        let refs: Vec<&dyn rusqlite::ToSql> =
            args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        conn.query_row(&sql, refs.as_slice(), |r| r.get(0))
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))
    }

    pub fn asset_summary_direct(&self, filter: &AssetFilter) -> PResult<i64> {
        let conn = self.conn()?;
        let mut sql = String::from("SELECT COUNT(*) FROM assets WHERE 1=1");
        let mut args: Vec<String> = Vec::new();
        let mut filter = filter.clone();
        filter.direct = Some("1".into());
        apply_asset_filter(&mut sql, &mut args, &filter, true);
        let refs: Vec<&dyn rusqlite::ToSql> =
            args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        conn.query_row(&sql, refs.as_slice(), |r| r.get(0))
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))
    }

    pub fn list_library(
        &self,
        filter: &LibraryFilter,
        cursor: Option<&str>,
        limit: usize,
    ) -> PResult<LibraryPage> {
        let conn = self.conn()?;
        let mut sql = format!("SELECT * FROM ({LIBRARY_UNION}) x WHERE 1=1");
        let mut args: Vec<String> = Vec::new();
        apply_library_filter(&mut sql, &mut args, filter);
        apply_library_cursor(&mut sql, &mut args, cursor);
        sql.push_str(" ORDER BY created_at DESC,source DESC,raw_id DESC LIMIT ?");
        let n = limit.clamp(1, 100);
        args.push((n + 1).to_string());
        let mut st = conn
            .prepare(&sql)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let refs: Vec<&dyn rusqlite::ToSql> =
            args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        let rows = st
            .query_map(refs.as_slice(), row_library)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let mut items: Vec<LibraryView> = rows.flatten().collect();
        for item in &mut items {
            if let Some(path) = item.path.as_deref() {
                item.missing = !std::path::Path::new(path).is_file();
            }
        }
        let next = if items.len() > n {
            items.truncate(n);
            items.last().map(|i| {
                format!(
                    "{}|{}|{}",
                    i.created_at,
                    i.source,
                    i.id.split_once(':').map(|(_, v)| v).unwrap_or(&i.id)
                )
            })
        } else {
            None
        };
        Ok(LibraryPage { items, next })
    }

    pub fn library_summary(&self, filter: &LibraryFilter) -> PResult<LibrarySummary> {
        let conn = self.conn()?;
        let mut sql = format!("SELECT item_type,COUNT(*) FROM ({LIBRARY_UNION}) x WHERE 1=1");
        let mut args: Vec<String> = Vec::new();
        apply_library_filter(&mut sql, &mut args, filter);
        sql.push_str(" GROUP BY item_type");
        let refs: Vec<&dyn rusqlite::ToSql> =
            args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        let mut st = conn
            .prepare(&sql)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let rows = st
            .query_map(refs.as_slice(), |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?))
            })
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let mut out = LibrarySummary::default();
        for (kind, count) in rows.flatten() {
            out.total += count;
            match kind.as_str() {
                "video" => out.video = count,
                "image" => out.image = count,
                "comment" => out.comment = count,
                _ => out.other += count,
            }
        }
        Ok(out)
    }

    pub fn library_action(&self, req: LibraryActionReq) -> PResult<AssetActionResult> {
        if req.action != "delete" {
            return Err(PipelineErr::new(400, "invalid_action", "未知动作"));
        }
        let trash = req.file.as_deref() == Some("trash");
        let rows: Vec<(String, String, Option<String>)> = match req.ids {
            Some(ids) => {
                let conn = self.conn()?;
                let mut out = Vec::new();
                for id in ids {
                    let Some((source, raw_id)) = id.split_once(':') else {
                        continue;
                    };
                    let path = if source == "asset" {
                        conn.query_row(
                            "SELECT path FROM assets WHERE id=?1",
                            params![raw_id],
                            |r| r.get(0),
                        )
                        .optional()
                        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
                    } else if source == "record" {
                        conn.query_row(
                            "SELECT t.output_path FROM capture_records r LEFT JOIN tasks t ON t.id=r.task_id WHERE r.id=?1",
                            params![raw_id],
                            |r| r.get(0),
                        )
                        .optional()
                        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
                        .flatten()
                    } else {
                        None
                    };
                    out.push((source.into(), raw_id.into(), path));
                }
                out
            }
            None => {
                let filter = req.filter.unwrap_or_default();
                let conn = self.conn()?;
                let mut sql =
                    format!("SELECT source,raw_id,path FROM ({LIBRARY_UNION}) x WHERE 1=1");
                let mut args: Vec<String> = Vec::new();
                apply_library_filter(&mut sql, &mut args, &filter);
                sql.push_str(" ORDER BY created_at DESC,source DESC,raw_id DESC LIMIT 5000");
                let mut st = conn
                    .prepare(&sql)
                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
                let refs: Vec<&dyn rusqlite::ToSql> =
                    args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
                let out: Vec<(String, String, Option<String>)> = st
                    .query_map(refs.as_slice(), |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
                    .flatten()
                    .collect();
                out
            }
        };
        let mut count = 0;
        for (source, id, path) in rows.into_iter().take(5000) {
            let ok = if source == "asset" {
                self.delete_asset(&id, trash).is_ok()
            } else {
                if trash {
                    if let Some(path) = path
                        .as_deref()
                        .filter(|p| std::path::Path::new(p).is_file())
                    {
                        let _ = trash_with_parts(std::path::Path::new(path));
                    }
                }
                self.delete_record(&id).is_ok()
            };
            if ok {
                count += 1;
            }
        }
        Ok(AssetActionResult { count })
    }

    pub fn asset_action(&self, req: AssetActionReq) -> PResult<AssetActionResult> {
        if req.action != "delete" {
            return Err(PipelineErr::new(
                400,
                "invalid_action",
                format!("未知动作 {}", req.action),
            ));
        }
        let ids = match req.ids {
            Some(ids) => ids,
            None => {
                let filter = req.filter.unwrap_or_default();
                let conn = self.conn()?;
                let mut sql = String::from("SELECT id FROM assets WHERE 1=1");
                let mut args: Vec<String> = Vec::new();
                apply_asset_filter(&mut sql, &mut args, &filter, true);
                sql.push_str(" ORDER BY created_at DESC,id DESC LIMIT 5000");
                let mut st = conn
                    .prepare(&sql)
                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
                let refs: Vec<&dyn rusqlite::ToSql> =
                    args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
                let rows = st
                    .query_map(refs.as_slice(), |r| r.get::<_, String>(0))
                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
                rows.flatten().collect()
            }
        };
        let trash = req.file.as_deref() == Some("trash");
        let mut count = 0;
        for id in ids.into_iter().take(5000) {
            if self.delete_asset(&id, trash).is_ok() {
                count += 1;
            }
        }
        Ok(AssetActionResult { count })
    }

    pub fn asset_dirs(&self, dir: Option<&str>) -> PResult<DirListing> {
        let root = self.download_root();
        let base = dir
            .filter(|v| !v.is_empty())
            .map(std::path::PathBuf::from)
            .filter(|v| v.starts_with(&root))
            .unwrap_or_else(|| root.clone());
        let prefix = base.to_string_lossy().trim_end_matches('/').to_string();
        let conn = self.conn()?;
        let paths = conn
            .prepare("SELECT path,size FROM assets WHERE path LIKE ?")
            .and_then(|mut st| {
                st.query_map(params![format!("{prefix}/%")], |r| {
                    Ok((r.get::<_, String>(0)?, r.get::<_, Option<i64>>(1)?))
                })?
                .collect::<Result<Vec<_>, _>>()
            })
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let mut map = std::collections::BTreeMap::<String, (i64, i64)>::new();
        for (path, size) in paths {
            let parent = std::path::Path::new(&path)
                .parent()
                .unwrap_or_else(|| std::path::Path::new(&root));
            let Ok(rel) = parent.strip_prefix(&base) else {
                continue;
            };
            let Some(first) = rel.components().next() else {
                continue;
            };
            let child = base.join(first.as_os_str()).to_string_lossy().to_string();
            let e = map.entry(child).or_default();
            e.0 += 1;
            e.1 += size.unwrap_or(0);
        }
        Ok(DirListing {
            root: root.to_string_lossy().to_string(),
            items: map
                .into_iter()
                .map(|(path, (count, bytes))| DirView {
                    name: path.rsplit('/').next().unwrap_or(&path).to_string(),
                    path,
                    count,
                    bytes,
                })
                .collect(),
        })
    }

    /// 列出目录直属文件；目录视图不递归展开子目录里的文件。
    pub fn list_assets_direct(
        &self,
        filter: &AssetFilter,
        hide_cover: bool,
        cursor: Option<&str>,
        limit: usize,
    ) -> PResult<AssetPage> {
        let conn = self.conn()?;
        let mut sql = String::from(
            "SELECT id,target_key,kind,title,author,cover,path,size,task_id,record_id,created_at,COALESCE((SELECT c.path FROM assets c WHERE c.kind='cover' AND c.target_key=assets.target_key||':cover'),'') FROM assets WHERE 1=1",
        );
        let mut args: Vec<String> = Vec::new();
        apply_asset_filter(&mut sql, &mut args, filter, hide_cover);
        sql.push_str(" ORDER BY created_at DESC,id DESC");
        let mut st = conn
            .prepare(&sql)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let refs: Vec<&dyn rusqlite::ToSql> =
            args.iter().map(|v| v as &dyn rusqlite::ToSql).collect();
        let rows = st
            .query_map(refs.as_slice(), row_asset)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let after = cursor.and_then(|c| c.split_once('|'));
        let mut items: Vec<AssetView> = rows
            .flatten()
            .map(|mut a| {
                a.missing = !std::path::Path::new(&a.path).is_file();
                a
            })
            .filter(|a| match after {
                Some((ts, id)) => {
                    let ts = ts.parse::<i64>().unwrap_or(0);
                    a.created_at < ts || (a.created_at == ts && a.id.as_str() < id)
                }
                None => true,
            })
            .collect();
        let next = if items.len() > limit.clamp(1, 100) {
            let n = limit.clamp(1, 100);
            items.truncate(n);
            items.last().map(|a| format!("{}|{}", a.created_at, a.id))
        } else {
            None
        };
        Ok(AssetPage { items, next })
    }

    pub fn get_asset(&self, id: &str) -> PResult<AssetView> {
        let mut a = self
            .conn()?
            .query_row("SELECT id,target_key,kind,title,author,cover,path,size,task_id,record_id,created_at,COALESCE((SELECT c.path FROM assets c WHERE c.kind='cover' AND c.target_key=assets.target_key||':cover'),'') FROM assets WHERE id=?1", params![id], row_asset)
            .optional().map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .ok_or_else(|| PipelineErr::new(404, "not_found", "资产不存在"))?;
        a.missing = !std::path::Path::new(&a.path).is_file();
        Ok(a)
    }

    pub fn asset_images(&self, id: &str) -> PResult<Vec<String>> {
        let asset = self.get_asset(id)?;
        if !matches!(asset.kind.as_str(), "gallery" | "cover") || asset.missing {
            return Ok(Vec::new());
        }
        let path = std::path::PathBuf::from(&asset.path);
        let stem = path
            .file_stem()
            .and_then(|v| v.to_str())
            .unwrap_or("gallery");
        let dir = path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join(stem);
        if !dir.is_dir() && path.extension().and_then(|v| v.to_str()) == Some("zip") {
            std::fs::create_dir_all(&dir)
                .map_err(|e| PipelineErr::new(500, "io", e.to_string()))?;
            let file = std::fs::File::open(&path)
                .map_err(|e| PipelineErr::new(500, "io", e.to_string()))?;
            let mut archive = zip::ZipArchive::new(file)
                .map_err(|e| PipelineErr::new(500, "zip", e.to_string()))?;
            for i in 0..archive.len() {
                let Ok(mut entry) = archive.by_index(i) else {
                    continue;
                };
                if entry.is_dir() {
                    continue;
                }
                let Ok(name) = entry.name() else {
                    continue;
                };
                let name = std::path::Path::new(name.as_ref())
                    .file_name()
                    .and_then(|v| v.to_str())
                    .unwrap_or_default();
                let ext = std::path::Path::new(name)
                    .extension()
                    .and_then(|v| v.to_str())
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                if !matches!(
                    ext.as_str(),
                    "jpg" | "jpeg" | "png" | "webp" | "gif" | "avif"
                ) {
                    continue;
                }
                let dest = dir.join(name);
                if dest.is_file() {
                    continue;
                }
                let mut out = std::fs::File::create(&dest)
                    .map_err(|e| PipelineErr::new(500, "io", e.to_string()))?;
                std::io::copy(&mut entry, &mut out)
                    .map_err(|e| PipelineErr::new(500, "io", e.to_string()))?;
            }
        }
        let mut images: Vec<String> = std::fs::read_dir(&dir)
            .into_iter()
            .flatten()
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                matches!(
                    p.extension()
                        .and_then(|v| v.to_str())
                        .unwrap_or_default()
                        .to_ascii_lowercase()
                        .as_str(),
                    "jpg" | "jpeg" | "png" | "webp" | "gif" | "avif"
                )
            })
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        images.sort();
        Ok(images)
    }

    pub fn delete_asset(&self, id: &str, trash_file: bool) -> PResult<()> {
        let a = self.get_asset(id)?;
        if trash_file && !a.missing {
            trash_with_parts(std::path::Path::new(&a.path))?;
        }
        let conn = self.conn()?;
        if a.kind != "cover" {
            let cover: Option<String> = conn
                .query_row(
                    "SELECT path FROM assets WHERE kind='cover' AND target_key=?1",
                    params![format!("{}:cover", a.target_key)],
                    |r| r.get(0),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if let (true, Some(path)) = (trash_file, cover.as_deref()) {
                if std::path::Path::new(path).is_file() {
                    let _ = trash_with_parts(std::path::Path::new(path));
                }
            }
            conn.execute(
                "DELETE FROM assets WHERE kind='cover' AND target_key=?1",
                params![format!("{}:cover", a.target_key)],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        }
        conn.execute("DELETE FROM assets WHERE id=?1", params![id])
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    fn settings_value(&self) -> Value {
        self.conn()
            .ok()
            .and_then(|c| {
                c.query_row("SELECT value FROM settings WHERE key='main'", [], |r| {
                    r.get::<_, String>(0)
                })
                .optional()
                .ok()
                .flatten()
            })
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or(Value::Null)
    }

    fn setting_str(&self, key: &str, fallback: &str) -> String {
        self.settings_value()
            .get(key)
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .unwrap_or(fallback)
            .to_string()
    }

    fn download_root(&self) -> PathBuf {
        crate::paths::expand(&self.setting_str("downloadDir", "~/Videos/抖音台"))
    }

    #[allow(dead_code)]
    pub fn downloads_dir(&self) -> PathBuf {
        self.download_root()
    }

    fn asset_path(
        &self,
        kind: &str,
        target: &str,
        id: &str,
        title: &str,
        author: &str,
        ext: &str,
    ) -> PathBuf {
        let template = if kind == "record" {
            "直播/{author}/{date}_{desc}_{id}".to_string()
        } else {
            self.setting_str("naming", "{author}/{date}_{desc}_{id}")
        };
        let date = file_date(now());
        let base = template
            .split('/')
            .filter(|s| !s.is_empty())
            .map(|part| {
                safe_name(
                    &part
                        .replace(
                            "{author}",
                            if author.is_empty() {
                                "未知作者"
                            } else {
                                author
                            },
                        )
                        .replace("{date}", &date)
                        .replace("{desc}", if title.is_empty() { target } else { title })
                        .replace("{id}", id),
                    80,
                )
            })
            .collect::<PathBuf>();
        let mut path = self.download_root().join(base);
        if kind == "record" {
            let stamp = target.rsplit_once(':').map(|(_, s)| s).unwrap_or("");
            if !stamp.is_empty() {
                let stem = path
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "record".into());
                path.set_file_name(format!("{stem}_{stamp}"));
            }
            return unique_dir(path);
        }
        path.set_extension(clean_ext(ext));
        unique_dest(path)
    }

    fn download_state(&self, id: &str) -> String {
        self.conn()
            .ok()
            .and_then(|c| {
                c.query_row(
                    "SELECT state FROM downloads WHERE id=?1",
                    params![id],
                    |r| r.get::<_, String>(0),
                )
                .optional()
                .ok()
                .flatten()
            })
            .unwrap_or_default()
    }

    #[allow(clippy::type_complexity)]
    fn download_task(&self, task: &TaskView) -> PResult<String> {
        let did = task
            .params
            .get("download_id")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let kind = task
            .params
            .get("asset_kind")
            .and_then(Value::as_str)
            .unwrap_or("video")
            .to_string();
        if kind == "record" {
            return self.record_download(task, &did);
        }
        let (stored_url, path, target, title, author, cover, record_id, expected): (
            String,
            String,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<i64>,
        ) = self
            .conn()?
            .query_row(
                "SELECT url,path,target_key,title,author,cover,record_id,expected_bytes FROM downloads WHERE id=?1",
                params![did],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?, r.get(7)?)),
            )
            .map_err(|e| PipelineErr::new(404, "not_found", e.to_string()))?;
        self.conn()?
            .execute(
                "UPDATE downloads SET state='running',attempts=attempts+1,error=NULL WHERE id=?1",
                params![did],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let url = if stored_url.is_empty() {
            let api = self
                .api
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .clone()
                .ok_or_else(|| PipelineErr::new(503, "backend", "执行后端未就绪"))?;
            let out = match api.call(&task.endpoint, &task.params) {
                Ok(v) => v,
                Err(e) => {
                    self.capture_identity(&api);
                    let current = self.download_state(&did);
                    if matches!(current.as_str(), "paused" | "canceled") {
                        return Ok(current);
                    }
                    let state = if task.attempts >= task.max_attempts {
                        "failed"
                    } else {
                        "queued"
                    };
                    let _ = self.fail_download(&did, state, &e.message);
                    return Err(PipelineErr::new(e.status, "upstream", e.message));
                }
            };
            self.capture_identity(&api);
            out.get("url")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .to_string()
        } else {
            stored_url
        };
        let current = self.download_state(&did);
        if matches!(current.as_str(), "paused" | "canceled") {
            return Ok(current);
        }
        if url.is_empty() {
            let _ = self.fail_download(&did, "failed", "直链为空");
            return Err(PipelineErr::new(502, "no_url", "直链为空"));
        }
        let path = PathBuf::from(path);
        self.conn()?
            .execute(
                "UPDATE downloads SET url=?2,partial_path=?3 WHERE id=?1",
                params![did, url, partial_path(&path).to_string_lossy()],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let started = Instant::now();
        let mut last = Instant::now();
        let mut last_bytes = 0i64;
        let transfer = fetch_file_resume(
            &url,
            &path,
            || {
                let st = self.download_state(&did);
                st != "running"
            },
            |done, total| {
                let elapsed = last.elapsed().as_millis().max(1) as i64;
                let speed = (done - last_bytes).max(0) * 1000 / elapsed;
                if elapsed >= 500 {
                    last = Instant::now();
                    last_bytes = done;
                }
                let progress = if total > 0 {
                    done as f64 / total as f64
                } else {
                    0.0
                };
                let _ = self.conn().map(|c| {
                    c.execute(
                        "UPDATE downloads SET bytes=?2,progress=?3,speed=?4 WHERE id=?1",
                        params![did, done, progress, speed],
                    )
                });
                let _ = self.conn().map(|c| {
                    c.execute(
                        "UPDATE tasks SET progress=?2 WHERE id=?1",
                        params![task.id, progress],
                    )
                });
            },
        );
        let current = self.download_state(&did);
        if matches!(current.as_str(), "paused" | "canceled") {
            self.conn()?
                .execute(
                    "UPDATE downloads SET bytes=?2,speed=0 WHERE id=?1",
                    params![did, part_size(&path)],
                )
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            return Ok(current);
        }
        let bytes = match transfer {
            Ok(Transfer::Done(n)) => n,
            Ok(Transfer::Aborted) => part_size(&path),
            Err(e) => {
                let state = if task.attempts >= task.max_attempts {
                    "failed"
                } else {
                    "queued"
                };
                let _ = self.fail_download(&did, state, &e);
                return Err(PipelineErr::new(0, "network", e));
            }
        };
        if expected.unwrap_or(0) > 0 && bytes != expected.unwrap_or(0) {
            let _ = self.fail_download(&did, "queued", "文件大小与预检不一致");
            return Err(PipelineErr::new(0, "size_mismatch", "文件大小与预检不一致"));
        }
        let aid = Uuid::new_v4().to_string();
        let conn = self.conn()?;
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.execute(
            "DELETE FROM assets WHERE target_key=?1 OR path=?2",
            params![target, path.to_string_lossy()],
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.execute(
            "INSERT INTO assets(id,target_key,kind,task_id,record_id,title,author,cover,path,size,created_at) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
            params![aid, target, kind, task.id, record_id, title, author, cover, path.to_string_lossy(), bytes, now()],
        ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.execute(
            "UPDATE downloads SET state='done',bytes=?2,expected_bytes=COALESCE(expected_bytes,?2),progress=1,speed=0,finished_at=?3,error=NULL,asset_id=?4 WHERE id=?1",
            params![did, bytes, now(), aid],
        ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.commit()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let _ = self.cleanup_downloads();
        crate::logging::info(
            "dl",
            format!("{did} 完成 {}B {}ms", bytes, started.elapsed().as_millis()),
        );
        Ok("ok".into())
    }

    fn record_download(&self, task: &TaskView, did: &str) -> PResult<String> {
        let app = self
            .app
            .get()
            .cloned()
            .ok_or_else(|| PipelineErr::new(503, "backend", "应用句柄未就绪"))?;
        let (path, target, asset_key, title, author, plan_id): (
            String,
            String,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
        ) = self
            .conn()?
            .query_row(
                "SELECT d.path,d.target_key,d.asset_key,d.title,d.author,d.record_id FROM downloads d WHERE id=?1",
                params![did],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)),
            )
            .map_err(|e| PipelineErr::new(404, "not_found", e.to_string()))?;
        let active: Option<String> = self
            .conn()?
            .query_row(
                "SELECT id FROM record_sessions WHERE room_id=?1 AND state IN ('queued','running') LIMIT 1",
                params![record_room(&target)],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if let Some(sid) = active {
            let running = rec::get(&app, &sid)
                .is_some_and(|s| matches!(s.state.as_str(), "queued" | "running"));
            if running {
                let msg = "这个房间已经在录";
                let _ = self.fail_download(did, "failed", msg);
                return Err(PipelineErr::new(409, "room_busy", msg));
            }
            self.conn()?
                .execute(
                    "UPDATE record_sessions SET state='failed',finished_at=?2,error='进程重启，会话中断' WHERE id=?1",
                    params![sid, now()],
                )
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        }
        self.conn()?
            .execute(
                "UPDATE downloads SET state='running',attempts=attempts+1,error=NULL WHERE id=?1",
                params![did],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let api = self
            .api
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .ok_or_else(|| PipelineErr::new(503, "backend", "执行后端未就绪"))?;
        let forced = task
            .params
            .get("force_url")
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .map(|v| serde_json::json!({"url": v}));
        let out = if let Some(v) = forced {
            v
        } else {
            match api.call(&task.endpoint, &task.params) {
                Ok(v) => {
                    self.capture_identity(&api);
                    v
                }
                Err(e) => {
                    let missed = task.params.get("miss_reason").and_then(Value::as_str)
                        == Some("concurrency");
                    let message = if missed {
                        format!("漏录 · 并发已满（{}）", e.message)
                    } else {
                        e.message
                    };
                    let _ = self.fail_download(did, "failed", &message);
                    return Err(PipelineErr::new(e.status, "upstream", message));
                }
            }
        };
        let url = out
            .get("url")
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .or_else(|| out.get("hls").and_then(Value::as_str))
            .or_else(|| out.get("flv").and_then(Value::as_str))
            .unwrap_or_default()
            .to_string();
        if url.is_empty() {
            let _ = self.fail_download(did, "failed", "拉流地址为空");
            return Err(PipelineErr::new(502, "no_url", "拉流地址为空"));
        }
        let sid = Uuid::new_v4().to_string();
        let room = record_room(&target);
        self.conn()?
            .execute(
                "INSERT INTO record_sessions(id,plan_id,target_key,room_id,dir,state,max_bytes,max_secs,started_at) VALUES(?1,?2,?3,?4,?5,'queued',?6,?7,?8)",
                params![sid, plan_id, target, room, path, 20i64 * 1024 * 1024 * 1024, 4i64 * 3600, now()],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let max_secs = task
            .params
            .get("max_secs")
            .and_then(Value::as_u64)
            .unwrap_or(4 * 3600);
        let max_bytes = task
            .params
            .get("max_bytes")
            .and_then(Value::as_u64)
            .unwrap_or(20 * 1024 * 1024 * 1024);
        let retain_bytes = task
            .params
            .get("retain_bytes")
            .and_then(Value::as_u64)
            .unwrap_or(20 * 1024 * 1024 * 1024);
        self.conn()?
            .execute(
                "UPDATE record_sessions SET max_secs=?2,max_bytes=?3,retain_bytes=?4 WHERE id=?1",
                params![sid, max_secs as i64, max_bytes as i64, retain_bytes as i64],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let refresh_api = api.clone();
        let refresh_endpoint = task.endpoint.clone();
        let refresh_params = task.params.clone();
        let opts = rec::Options {
            id: sid.clone(),
            dir: PathBuf::from(&path),
            max_secs: Some(max_secs),
            max_bytes: Some(max_bytes),
            retain_bytes: Some(retain_bytes),
            refresh: Some(std::sync::Arc::new(move || {
                refresh_api
                    .call(&refresh_endpoint, &refresh_params)
                    .map_err(|e| e.message)
                    .and_then(|v| {
                        ["url", "hls", "flv"]
                            .iter()
                            .find_map(|k| v.get(*k).and_then(Value::as_str))
                            .filter(|u| !u.is_empty())
                            .map(str::to_string)
                            .ok_or_else(|| "刷新后无拉流地址".to_string())
                    })
            })),
        };
        if let Err(e) = rec::start_opts(app.clone(), url.clone(), room, opts) {
            let _ = self.conn()?.execute(
                "UPDATE record_sessions SET state='failed',finished_at=?2,error=?3 WHERE id=?1",
                params![sid, now(), e],
            );
            let _ = self.fail_download(did, "failed", &e);
            return Err(PipelineErr::new(500, "record", e));
        }
        self.conn()?
            .execute(
                "UPDATE record_sessions SET state='running' WHERE id=?1",
                params![sid],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        crate::notify::send(&app, "直播录制开始", title.as_deref().unwrap_or(&target));
        loop {
            std::thread::sleep(Duration::from_secs(1));
            let state = self.download_state(did);
            if matches!(state.as_str(), "paused" | "canceled") {
                let mgr = app.state::<rec::Manager>();
                mgr.stop(&sid);
                let until = Instant::now() + Duration::from_secs(15);
                while Instant::now() < until {
                    if rec::get(&app, &sid)
                        .is_some_and(|s| !matches!(s.state.as_str(), "queued" | "running"))
                    {
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(200));
                }
                return Ok(state);
            }
            let Some(session) = rec::get(&app, &sid) else {
                let _ = self.fail_download(did, "failed", "录制会话丢失");
                return Err(PipelineErr::new(500, "record", "录制会话丢失"));
            };
            self.conn()?
                .execute(
                    "UPDATE record_sessions SET bytes=?2 WHERE id=?1",
                    params![sid, session.bytes],
                )
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            self.conn()?
                .execute(
                    "UPDATE downloads SET bytes=?2,progress=?3 WHERE id=?1",
                    params![
                        did,
                        session.bytes,
                        (session.bytes as f64 / max_bytes.max(1) as f64).min(1.0)
                    ],
                )
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if session.state == "running" || session.state == "queued" {
                continue;
            }
            let finished = now();
            self.conn()?
                .execute(
                    "UPDATE record_sessions SET state=?2,finished_at=?3,error=?4,bytes=?5 WHERE id=?1",
                    params![sid, if session.state == "fail" || session.state == "failed" { "failed" } else { "done" }, finished, session.err, session.bytes],
                )
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if session.state == "fail" || session.state == "failed" {
                let _ = self.fail_download(did, "failed", &session.err);
                crate::notify::send(&app, "直播录制失败", &session.err);
                return Err(PipelineErr::new(0, "record", session.err));
            }
            let index = PathBuf::from(&session.dir).join("index.m3u8");
            let aid = Uuid::new_v4().to_string();
            let conn = self.conn()?;
            let tx = conn
                .unchecked_transaction()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            tx.execute(
                "DELETE FROM assets WHERE target_key=?1 OR path=?2",
                params![asset_key, index.to_string_lossy()],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            tx.execute(
                "INSERT INTO assets(id,target_key,kind,task_id,title,author,path,size,created_at) VALUES(?1,?2,'record',?3,?4,?5,?6,?7,?8)",
                params![aid, asset_key, task.id, title, author, index.to_string_lossy(), session.bytes, now()],
            ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            tx.execute(
                "UPDATE downloads SET state='done',url=?2,path=?3,bytes=?4,progress=1,speed=0,finished_at=?5,asset_id=?6 WHERE id=?1",
                params![did, url, index.to_string_lossy(), session.bytes, now(), aid],
            ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            tx.commit()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            crate::notify::send(
                &app,
                "直播录制结束",
                if session.err.is_empty() {
                    title.as_deref().unwrap_or(&target)
                } else {
                    &session.err
                },
            );
            return Ok("ok".into());
        }
    }

    pub fn list_recordings(&self) -> PResult<Vec<RecordSessionView>> {
        let conn = self.conn()?;
        let mut st = conn.prepare("SELECT id,plan_id,target_key,room_id,dir,state,bytes,started_at,finished_at,error FROM record_sessions ORDER BY started_at DESC LIMIT 500")
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let rows = st
            .query_map([], |r| {
                Ok(RecordSessionView {
                    id: r.get(0)?,
                    plan_id: r.get(1)?,
                    target_key: r.get(2)?,
                    room_id: r.get(3)?,
                    dir: r.get(4)?,
                    state: r.get(5)?,
                    bytes: r.get(6)?,
                    started_at: r.get(7)?,
                    finished_at: r.get(8)?,
                    error: r.get(9)?,
                })
            })
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(rows.flatten().collect())
    }

    pub fn stop_recording(&self, id: &str) -> PResult<()> {
        let app = self
            .app
            .get()
            .cloned()
            .ok_or_else(|| PipelineErr::new(503, "backend", "应用句柄未就绪"))?;
        if !app.state::<rec::Manager>().stop(id) {
            return Err(PipelineErr::new(404, "not_found", "录制会话未运行"));
        }
        Ok(())
    }

    fn fail_download(&self, id: &str, state: &str, error: &str) -> PResult<()> {
        self.conn()?
            .execute(
                "UPDATE downloads SET state=?2,error=?3,speed=0,finished_at=CASE WHEN ?2='failed' THEN ?4 ELSE NULL END WHERE id=?1",
                params![id, state, error, now()],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    fn cleanup_downloads(&self) -> PResult<()> {
        let conn = self.conn()?;
        let cap: i64 = conn
            .query_row(
                "SELECT CAST(value AS INTEGER) FROM settings WHERE key='download_capacity_bytes'",
                [],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .unwrap_or(20 * 1024 * 1024 * 1024);
        if cap <= 0 {
            return Ok(());
        }
        let mut total: i64 = conn
            .query_row(
                "SELECT COALESCE(SUM(bytes),0) FROM downloads WHERE state='done'",
                [],
                |r| r.get(0),
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if total <= cap {
            return Ok(());
        }
        let mut st = conn.prepare("SELECT id,path,bytes FROM downloads WHERE state='done' AND pinned=0 ORDER BY created_at ASC").map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let rows = st
            .query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, Option<String>>(1)?,
                    r.get::<_, Option<i64>>(2)?,
                ))
            })
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        for row in rows {
            if total <= cap {
                break;
            }
            let (id, path, bytes) = row.map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            if let Some(p) = path {
                let _ = std::fs::remove_file(p);
            }
            total -= bytes.unwrap_or(0);
            conn.execute("UPDATE downloads SET state='gone' WHERE id=?1", params![id])
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        }
        Ok(())
    }

    pub fn stats(&self) -> PResult<Stats> {
        let conn = self.conn()?;
        let since = now() - 86_400;
        let one = |sql: &str, p: &[&dyn rusqlite::ToSql]| -> i64 {
            conn.query_row(sql, p, |r| r.get(0)).unwrap_or(0)
        };
        let submitted = one("SELECT COUNT(*) FROM tasks WHERE created_at>=?1", &[&since]);
        let completed = one(
            "SELECT COUNT(*) FROM tasks WHERE finished_at>=?1 AND state IN ('done','dead')",
            &[&since],
        );
        let succeeded = one(
            "SELECT COUNT(*) FROM tasks WHERE finished_at>=?1 AND outcome='ok'",
            &[&since],
        );
        let gone = one(
            "SELECT COUNT(*) FROM tasks WHERE finished_at>=?1 AND outcome='gone'",
            &[&since],
        );
        let failed = one("SELECT COUNT(*) FROM tasks WHERE finished_at>=?1 AND outcome IN ('risk','network','param')", &[&since]);
        let avg_duration_ms = one("SELECT COALESCE(AVG((finished_at-started_at)*1000),0) FROM tasks WHERE finished_at>=?1 AND started_at IS NOT NULL", &[&since]);
        let queue_depth = one(
            "SELECT COUNT(*) FROM tasks WHERE state IN ('queued','running')",
            &[],
        );
        let disk_bytes = one(
            "SELECT COALESCE(SUM(bytes),0) FROM downloads WHERE state='done'",
            &[],
        );
        let breakers = one("SELECT COUNT(*) FROM breakers", &[]);
        let pool_available = self
            .api
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .map(|a| a.pool_available() as i64)
            .unwrap_or(0);
        Ok(Stats {
            day: chrono_day(),
            submitted,
            completed,
            succeeded,
            failed,
            gone,
            avg_duration_ms,
            queue_depth,
            disk_bytes,
            pool_available,
            breakers,
        })
    }

    pub fn dashboard(&self) -> PResult<Dashboard> {
        let conn = self.conn()?;
        let one = |sql: &str, p: &[&dyn rusqlite::ToSql]| -> i64 {
            conn.query_row(sql, p, |r| r.get(0)).unwrap_or(0)
        };
        let today: String = conn
            .query_row("SELECT date('now','localtime')", [], |r| r.get(0))
            .unwrap_or_default();
        let submitted = one(
            "SELECT COUNT(*) FROM tasks WHERE date(created_at,'unixepoch','localtime')=?1",
            &[&today],
        );
        let completed = one(
            "SELECT COUNT(*) FROM tasks WHERE date(finished_at,'unixepoch','localtime')=?1 AND state IN ('done','dead')",
            &[&today],
        );
        let succeeded = one(
            "SELECT COUNT(*) FROM tasks WHERE date(finished_at,'unixepoch','localtime')=?1 AND outcome='ok'",
            &[&today],
        );
        let gone = one(
            "SELECT COUNT(*) FROM tasks WHERE date(finished_at,'unixepoch','localtime')=?1 AND outcome='gone'",
            &[&today],
        );
        let failed = one(
            "SELECT COUNT(*) FROM tasks WHERE date(finished_at,'unixepoch','localtime')=?1 AND outcome IN ('risk','network','param')",
            &[&today],
        );
        let avg_duration_ms = one(
            "SELECT COALESCE(AVG((finished_at-started_at)*1000),0) FROM tasks WHERE date(finished_at,'unixepoch','localtime')=?1 AND started_at IS NOT NULL",
            &[&today],
        );
        let queue_depth = one(
            "SELECT COUNT(*) FROM tasks WHERE state IN ('queued','running')",
            &[],
        );
        let disk_bytes = one(
            "SELECT COALESCE(SUM(bytes),0) FROM downloads WHERE state='done'",
            &[],
        );
        let breakers = one("SELECT COUNT(*) FROM breakers", &[]);
        let pool_available = self
            .api
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .as_ref()
            .map(|a| a.pool_available() as i64)
            .unwrap_or(0);

        let mut series = Vec::with_capacity(14);
        for offset in (0..14).rev() {
            let day: String = conn
                .query_row(
                    "SELECT date('now','localtime',?1)",
                    params![format!("-{offset} days")],
                    |r| r.get(0),
                )
                .unwrap_or_default();
            series.push(DayPoint {
                day: day.clone(),
                submitted: one(
                    "SELECT COUNT(*) FROM tasks WHERE date(created_at,'unixepoch','localtime')=?1",
                    &[&day],
                ),
                succeeded: one(
                    "SELECT COUNT(*) FROM tasks WHERE date(finished_at,'unixepoch','localtime')=?1 AND outcome='ok'",
                    &[&day],
                ),
                failed: one(
                    "SELECT COUNT(*) FROM tasks WHERE date(finished_at,'unixepoch','localtime')=?1 AND outcome IN ('risk','network','param')",
                    &[&day],
                ),
                bytes: one(
                    "SELECT COALESCE(SUM(bytes),0) FROM downloads WHERE state='done' AND date(finished_at,'unixepoch','localtime')=?1",
                    &[&day],
                ),
            });
        }

        let mut kinds = Vec::new();
        if let Ok(mut st) = conn.prepare(
            "SELECT kind,              SUM(CASE WHEN state IN ('queued','running','paused') THEN 1 ELSE 0 END),              SUM(CASE WHEN state='done' THEN 1 ELSE 0 END),              SUM(CASE WHEN state IN ('failed','dead') THEN 1 ELSE 0 END)              FROM tasks GROUP BY kind",
        ) {
            let rows = st.query_map([], |r| {
                Ok(KindPoint {
                    kind: r.get(0)?,
                    active: r.get(1)?,
                    done: r.get(2)?,
                    failed: r.get(3)?,
                })
            });
            if let Ok(rows) = rows {
                kinds.extend(rows.flatten());
            }
        }

        let mut outcomes = Vec::new();
        if let Ok(mut st) = conn.prepare(
            "SELECT outcome, COUNT(*) FROM tasks WHERE date(finished_at,'unixepoch','localtime')>=date('now','localtime','-13 days') AND outcome IS NOT NULL GROUP BY outcome ORDER BY COUNT(*) DESC",
        ) {
            let rows = st.query_map([], |r| {
                Ok(OutcomePoint {
                    outcome: r.get(0)?,
                    count: r.get(1)?,
                })
            });
            if let Ok(rows) = rows {
                outcomes.extend(rows.flatten());
            }
        }

        Ok(Dashboard {
            day: today,
            submitted,
            completed,
            succeeded,
            failed,
            gone,
            avg_duration_ms,
            queue_depth,
            disk_bytes,
            pool_available,
            breakers,
            series,
            kinds,
            outcomes,
        })
    }

    fn maintenance(&self) {
        if let Ok(conn) = self.conn() {
            let now = now();
            let _ = conn.execute(
                "UPDATE tasks SET archived_at=?1 WHERE deleted_at IS NULL AND archived_at IS NULL AND ((state='done' AND finished_at<?2) OR (state='canceled' AND finished_at<?3) OR (state='dead' AND acknowledged_at IS NOT NULL AND finished_at<?4))",
                params![now, now - 86_400, now - 3_600, now - 3_600],
            );
            let purge = "(keep_until IS NOT NULL AND keep_until>?1 AND deleted_at IS NULL) = 0 AND ((deleted_at IS NOT NULL AND deleted_at<?2) OR (state='done' AND finished_at<?3) OR (state='canceled' AND finished_at<?4) OR (state='dead' AND acknowledged_at IS NOT NULL AND finished_at<?5))";
            for sql in [
                "UPDATE downloads SET task_id=NULL,task_deleted_at=COALESCE(task_deleted_at,?1) WHERE task_id IN (SELECT id FROM tasks WHERE",
                "UPDATE capture_records SET task_id=NULL WHERE task_id IN (SELECT id FROM tasks WHERE",
                "UPDATE assets SET task_id=NULL WHERE task_id IN (SELECT id FROM tasks WHERE",
                "UPDATE plans SET last_task_id=NULL WHERE last_task_id IN (SELECT id FROM tasks WHERE",
            ] {
                let full = format!("{sql} {purge})");
                let _ = conn.execute(
                    &full,
                    params![now, now - 7 * 86_400, now - 30 * 86_400, now - 7 * 86_400, now - 30 * 86_400],
                );
            }
            let _ = conn.execute(
                &format!("DELETE FROM tasks WHERE {purge}"),
                params![
                    now,
                    now - 7 * 86_400,
                    now - 30 * 86_400,
                    now - 7 * 86_400,
                    now - 30 * 86_400
                ],
            );
            let _ = conn.execute(
                "DELETE FROM downloads WHERE task_id IS NULL AND task_deleted_at IS NOT NULL AND task_deleted_at<?1",
                params![now - 7 * 86_400],
            );
            let _ = conn.execute("DELETE FROM breaker_events WHERE at<?1", params![now - 300]);
        }
    }

    fn download_of_task(&self, id: &str) -> Option<String> {
        self.get(id).ok().and_then(|t| {
            t.params
                .get("download_id")
                .and_then(Value::as_str)
                .map(str::to_string)
        })
    }

    pub fn pause_task(&self, id: &str) -> PResult<TaskView> {
        if let Some(did) = self.download_of_task(id) {
            self.pause_download(&did)?;
            return self.get(id);
        }
        self.conn()?
            .execute(
                "UPDATE tasks SET state='paused',started_at=NULL WHERE id=?1 AND state IN ('queued','running')",
                params![id],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        self.get(id)
    }

    pub fn resume_task(&self, id: &str) -> PResult<TaskView> {
        if let Some(did) = self.download_of_task(id) {
            self.resume_download(&did)?;
            return self.get(id);
        }
        self.conn()?
            .execute(
                "UPDATE tasks SET state='queued',started_at=NULL,finished_at=NULL,error_code=NULL WHERE id=?1 AND state='paused'",
                params![id],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        self.wake();
        self.get(id)
    }

    pub fn retry_task(&self, id: &str) -> PResult<TaskView> {
        if let Some(did) = self.download_of_task(id) {
            self.retry_download(&did)?;
            self.conn()?
                .execute(
                    "UPDATE tasks SET state='queued',attempts=0,started_at=NULL,finished_at=NULL,error_code=NULL,outcome=NULL,retry_at=NULL,acknowledged_at=NULL,archived_at=NULL,deleted_at=NULL WHERE id=?1",
                    params![id],
                )
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            return self.get(id);
        }
        self.conn()?
            .execute(
                "UPDATE tasks SET state='queued',attempts=0,started_at=NULL,finished_at=NULL,error_code=NULL,outcome=NULL,retry_at=NULL,acknowledged_at=NULL,archived_at=NULL,deleted_at=NULL WHERE id=?1 AND state IN ('dead','failed','canceled')",
                params![id],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        self.wake();
        self.get(id)
    }

    pub fn acknowledge_task(&self, id: &str) -> PResult<()> {
        let now = now();
        self.conn()?
            .execute(
                "UPDATE tasks SET acknowledged_at=?2,archived_at=COALESCE(archived_at,?2) WHERE id=?1 AND state='dead'",
                params![id, now],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    pub fn keep_task(&self, id: &str, keep: bool) -> PResult<()> {
        self.conn()?
            .execute(
                "UPDATE tasks SET keep_until=?2 WHERE id=?1",
                params![
                    id,
                    if keep {
                        Some(now() + 365 * 86_400)
                    } else {
                        None
                    }
                ],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    pub fn delete_task(&self, id: &str, trash_file: bool) -> PResult<()> {
        let task = self.get(id)?;
        if task.deleted_at.is_some() {
            return Ok(());
        }
        if matches!(task.state.as_str(), "queued" | "running" | "paused") {
            let _ = self.cancel(id);
        }
        if let Some(did) = task.params.get("download_id").and_then(Value::as_str) {
            if trash_file {
                if let Ok(d) = self.get_download(did) {
                    if let Some(path) = d.path.as_deref() {
                        let _ = trash_with_parts(std::path::Path::new(path));
                    }
                }
                self.conn()?
                    .execute(
                        "UPDATE downloads SET state='gone',task_deleted_at=?2 WHERE id=?1",
                        params![did, now()],
                    )
                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            } else {
                self.conn()?
                    .execute(
                        "UPDATE downloads SET task_deleted_at=?2 WHERE id=?1",
                        params![did, now()],
                    )
                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            }
        } else if trash_file {
            if let Some(path) = task.output_path {
                let p = std::path::Path::new(&path);
                if p.exists() {
                    crate::trash::delete(p)
                        .map_err(|e| PipelineErr::new(500, "trash", e.to_string()))?;
                }
            }
        }
        let now = now();
        self.conn()?
            .execute(
                "UPDATE tasks SET deleted_at=?2,archived_at=COALESCE(archived_at,?2),acknowledged_at=COALESCE(acknowledged_at,?2) WHERE id=?1",
                params![id, now],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    pub fn cancel(&self, id: &str) -> PResult<()> {
        let conn = self.conn()?;
        let task = conn
            .query_row(
                "SELECT kind,params FROM tasks WHERE id=?1",
                params![id],
                |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if let Some((kind, params)) = &task {
            if kind == "download" {
                if let Some(did) = serde_json::from_str::<Value>(params).ok().and_then(|v| {
                    v.get("download_id")
                        .and_then(Value::as_str)
                        .map(str::to_string)
                }) {
                    let _ = self.cancel_download(&did);
                }
            }
        }
        let n = conn
            .execute(
                "UPDATE tasks SET state='canceled',error_code='cancelled',finished_at=?2 WHERE id=?1 AND state IN ('queued','running','paused')",
                params![id, now()],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if n == 0 {
            let state: Option<String> = conn
                .query_row("SELECT state FROM tasks WHERE id=?1", params![id], |r| {
                    r.get(0)
                })
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            match state {
                Some(s) => {
                    return Err(PipelineErr::new(409, "not_active", format!("任务状态 {s}")))
                }
                None => return Err(PipelineErr::new(404, "not_found", "任务不存在")),
            }
        }
        Ok(())
    }

    pub fn restore_task(&self, id: &str) -> PResult<()> {
        let task = self.get(id)?;
        if task.deleted_at.is_none() {
            return Ok(());
        }
        if let Some(did) = task.params.get("download_id").and_then(Value::as_str) {
            self.conn()?
                .execute(
                    "UPDATE downloads SET task_deleted_at=NULL WHERE id=?1",
                    params![did],
                )
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        }
        let archived = if matches!(task.state.as_str(), "done" | "canceled") {
            Some(now())
        } else {
            None
        };
        let acknowledged = if task.state == "dead" {
            None
        } else {
            task.acknowledged_at
        };
        self.conn()?
            .execute(
                "UPDATE tasks SET deleted_at=NULL,archived_at=?2,acknowledged_at=?3 WHERE id=?1",
                params![id, archived, acknowledged],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    pub fn purge_task(&self, id: &str, trash_file: bool) -> PResult<()> {
        let task = self.get(id)?;
        if task.deleted_at.is_none() {
            return Err(PipelineErr::new(409, "not_deleted", "任务尚未删除"));
        }
        if trash_file {
            if let Some(did) = task.params.get("download_id").and_then(Value::as_str) {
                if let Ok(d) = self.get_download(did) {
                    if let Some(path) = d.path.as_deref() {
                        let _ = trash_with_parts(std::path::Path::new(path));
                    }
                }
                self.conn()?
                    .execute(
                        "UPDATE downloads SET state='gone' WHERE id=?1",
                        params![did],
                    )
                    .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            } else if let Some(path) = task.output_path.as_deref() {
                let p = std::path::Path::new(path);
                if p.exists() {
                    let _ = crate::trash::delete(p);
                }
            }
        }
        let conn = self.conn()?;
        let tx = conn
            .unchecked_transaction()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        for sql in [
            "UPDATE downloads SET task_id=NULL WHERE task_id=?1",
            "UPDATE capture_records SET task_id=NULL WHERE task_id=?1",
            "UPDATE assets SET task_id=NULL WHERE task_id=?1",
            "UPDATE record_sessions SET plan_id=plan_id WHERE id IN (SELECT id FROM record_sessions WHERE id=?1)",
            "UPDATE plans SET last_task_id=NULL WHERE last_task_id=?1",
        ] {
            let _ = tx.execute(sql, params![id]);
        }
        tx.execute("DELETE FROM tasks WHERE id=?1", params![id])
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.commit()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    pub fn task_action(&self, req: TaskActionReq) -> PResult<TaskActionResult> {
        let mut ids = if let Some(ids) = req.ids {
            ids
        } else {
            let conn = self.conn()?;
            let now = now();
            let mut values: Vec<rusqlite::types::Value> = Vec::new();
            let mut where_sql = vec!["1=1".to_string()];
            apply_task_scope(&mut where_sql, &mut values, req.scope.as_deref(), now);
            if let Some(f) = req.filter.as_ref() {
                apply_task_filter(&mut where_sql, &mut values, f);
            }
            values.push(5000i64.into());
            let sql = format!(
                "SELECT id FROM tasks WHERE {} ORDER BY created_at DESC,id DESC LIMIT ?",
                where_sql.join(" AND ")
            );
            let mut st = conn
                .prepare(&sql)
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            let rows = st
                .query_map(rusqlite::params_from_iter(values.iter()), |r| {
                    r.get::<_, String>(0)
                })
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            rows.collect::<Result<Vec<_>, _>>()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
        };
        ids.truncate(5000);
        let trash = req.file.as_deref() == Some("trash");
        let mut changed = Vec::new();
        for id in ids {
            let result = match req.action.as_str() {
                "pause" => self.pause_task(&id).map(|_| ()),
                "cancel" => self.cancel(&id),
                "retry" => self.retry_task(&id).map(|_| ()),
                "ack" => self.acknowledge_task(&id),
                "keep" => self.keep_task(&id, true),
                "unkeep" => self.keep_task(&id, false),
                "delete" => self.delete_task(&id, trash),
                "restore" => self.restore_task(&id),
                "purge" => self.purge_task(&id, trash),
                other => {
                    return Err(PipelineErr::new(
                        400,
                        "invalid_action",
                        format!("未知动作 {other}"),
                    ))
                }
            };
            if result.is_ok() {
                changed.push(id);
            }
        }
        Ok(TaskActionResult {
            count: changed.len(),
            ids: changed,
        })
    }

    /// 启动恢复：所有 running 都是断点孤儿，与所有 queued 一起计一次尝试后重排。
    pub fn recover(&self) -> PResult<usize> {
        let mut conn = self.conn()?;
        let tx = conn
            .transaction()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        // 进程退出时任务与下载可能停在运行态，恢复时同步到同一终态，避免列表状态分叉。
        tx.execute(
            "UPDATE downloads SET state='failed',error='进程重启，任务中断',speed=0,finished_at=?1
             WHERE id IN (SELECT json_extract(params,'$.download_id') FROM tasks
                          WHERE state IN ('running','queued') AND json_extract(params,'$.download_id') IS NOT NULL
                            AND attempts+1>=max_attempts)",
            params![now()],
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.execute(
            "UPDATE downloads SET state='queued',error='进程重启，等待恢复',speed=0,finished_at=NULL
             WHERE id IN (SELECT json_extract(params,'$.download_id') FROM tasks
                          WHERE state IN ('running','queued') AND json_extract(params,'$.download_id') IS NOT NULL
                            AND attempts+1<max_attempts)",
            [],
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let n = tx
            .execute(
                "UPDATE tasks SET attempts=attempts+1,
                   state=CASE WHEN attempts+1>=max_attempts THEN 'dead' ELSE 'queued' END,
                   error_code=CASE WHEN attempts+1>=max_attempts THEN 'attempts_exhausted' ELSE 'recovered' END,
                   started_at=NULL
                 WHERE state='running'",
                [],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let m = tx
            .execute(
                "UPDATE tasks SET attempts=attempts+1,
                   state=CASE WHEN attempts+1>=max_attempts THEN 'dead' ELSE 'queued' END,
                   error_code=CASE WHEN attempts+1>=max_attempts THEN 'attempts_exhausted' ELSE 'recovered' END,
                   started_at=NULL
                 WHERE state='queued'",
                [],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        // 已处于终态的任务可能仍带着运行中的下载，按任务终态兜底校正。
        tx.execute(
            "UPDATE downloads SET state='failed',error='进程重启，任务中断',speed=0,finished_at=?1
             WHERE state IN ('queued','running') AND id IN (
               SELECT json_extract(params,'$.download_id') FROM tasks
               WHERE state='dead' AND json_extract(params,'$.download_id') IS NOT NULL)",
            params![now()],
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.execute(
            "UPDATE downloads SET state='queued',error='进程重启，等待恢复',speed=0,finished_at=NULL
             WHERE state IN ('queued','running') AND id IN (
               SELECT json_extract(params,'$.download_id') FROM tasks
               WHERE state='queued' AND json_extract(params,'$.download_id') IS NOT NULL)",
            [],
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let r = tx
            .execute(
                "UPDATE record_sessions SET state='failed',finished_at=?1,error='进程重启，会话中断' WHERE state IN ('queued','running')",
                params![now()],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        tx.commit()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(n + m + r)
    }

    /// 原子取一条优先级最高的 queued 任务。
    #[allow(dead_code)]
    pub fn claim(&self) -> PResult<Option<TaskView>> {
        self.claim_slot(true, true)
    }

    fn claim_slot(&self, allow_download: bool, allow_record: bool) -> PResult<Option<TaskView>> {
        let conn = self.conn()?;
        let id: Option<String> = conn
            .query_row(
                "SELECT id FROM tasks WHERE state='queued' AND (retry_at IS NULL OR retry_at<=?1)
                 AND (
                   (json_extract(params,'$.asset_kind')='record' AND ?2=1)
                   OR (COALESCE(json_extract(params,'$.asset_kind'),'video')!='record' AND ?3=1)
                 )
                 ORDER BY priority DESC,
                          CASE kind WHEN 'collection' THEN 0 WHEN 'download' THEN 1 WHEN 'recheck' THEN 2 ELSE 3 END,
                          created_at ASC LIMIT 1",
                params![now(), allow_record as i64, allow_download as i64],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let Some(id) = id else { return Ok(None) };
        let n = conn
            .execute(
                "UPDATE tasks SET state='running',started_at=?2,attempts=attempts+1,error_code=NULL,retry_at=NULL WHERE id=?1 AND state='queued'",
                params![id, now()],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        if n == 0 {
            return Ok(None);
        }
        self.get(&id).map(Some)
    }

    pub fn get(&self, id: &str) -> PResult<TaskView> {
        let conn = self.conn()?;
        let sql = format!("SELECT {TASK_COLUMNS} FROM tasks WHERE id=?1");
        conn.query_row(&sql, params![id], row_task)
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
            .ok_or_else(|| PipelineErr::new(404, "not_found", "任务不存在"))
    }

    pub fn list(
        &self,
        scope: Option<&str>,
        filter: Option<&TaskFilter>,
        cursor: Option<&str>,
        limit: usize,
    ) -> PResult<Vec<TaskView>> {
        let conn = self.conn()?;
        let now = now();
        let limit = limit.clamp(1, 200) as i64;
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        let mut where_sql = vec!["1=1".to_string()];
        apply_task_scope(&mut where_sql, &mut values, scope, now);
        if let Some(f) = filter {
            apply_task_filter(&mut where_sql, &mut values, f);
        }
        if let Some((created, id)) = cursor.and_then(|c| c.split_once('|')) {
            if let Ok(created) = created.parse::<i64>() {
                where_sql.push("(created_at<? OR (created_at=? AND id<?))".into());
                values.push(created.into());
                values.push(created.into());
                values.push(id.to_string().into());
            }
        }
        values.push(limit.into());
        let sql = format!(
            "SELECT {TASK_COLUMNS} FROM tasks WHERE {} ORDER BY created_at DESC,id DESC LIMIT ?",
            where_sql.join(" AND ")
        );
        let mut st = conn
            .prepare(&sql)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let rows = st
            .query_map(rusqlite::params_from_iter(values.iter()), row_task)
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))
    }

    pub fn task_summary(&self, filter: Option<&TaskFilter>) -> PResult<TaskSummary> {
        let conn = self.conn()?;
        let now = now();
        let mut extra = Vec::new();
        let mut values: Vec<rusqlite::types::Value> = Vec::new();
        if let Some(f) = filter {
            apply_task_filter(&mut extra, &mut values, f);
        }
        let suffix = if extra.is_empty() {
            String::new()
        } else {
            format!(" AND {}", extra.join(" AND "))
        };
        let one = |base: &str, mut vals: Vec<rusqlite::types::Value>| -> PResult<i64> {
            vals.extend(values.iter().cloned());
            let sql = format!("{base}{suffix}");
            conn.query_row(&sql, rusqlite::params_from_iter(vals.iter()), |r| r.get(0))
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))
        };
        Ok(TaskSummary {
            active: one(
                "SELECT COUNT(*) FROM tasks WHERE deleted_at IS NULL AND state IN ('queued','running','paused')",
                vec![],
            )?,
            pending: one(
                "SELECT COUNT(*) FROM tasks WHERE state='dead' AND acknowledged_at IS NULL AND deleted_at IS NULL",
                vec![],
            )?,
            recent: one(
                "SELECT COUNT(*) FROM tasks WHERE deleted_at IS NULL AND archived_at IS NULL AND ((state='done' AND finished_at>=?) OR (state='canceled' AND finished_at>=?) OR (state='dead' AND acknowledged_at IS NOT NULL AND finished_at>=?))",
                vec![(now - 86_400).into(), (now - 3_600).into(), (now - 7 * 86_400).into()],
            )?,
            history: one("SELECT COUNT(*) FROM tasks WHERE deleted_at IS NULL", vec![])?,
            deleted: one("SELECT COUNT(*) FROM tasks WHERE deleted_at IS NOT NULL", vec![])?,
            kept: one(
                "SELECT COUNT(*) FROM tasks WHERE deleted_at IS NULL AND keep_until>?",
                vec![now.into()],
            )?,
        })
    }
}

impl Pipeline {
    /// 熔断放行：未跳闸放行；跳闸后仅每 60 秒放一个探针。
    fn probe_allowed(&self, endpoint: &str) -> PResult<bool> {
        let conn = self.conn()?;
        let next: Option<i64> = conn
            .query_row(
                "SELECT next_probe_at FROM breakers WHERE endpoint=?1",
                params![endpoint],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        let Some(next) = next else { return Ok(true) };
        if now() < next {
            return Ok(false);
        }
        conn.execute(
            "UPDATE breakers SET next_probe_at=?2 WHERE endpoint=?1",
            params![endpoint, now() + 60],
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(true)
    }

    fn note_endpoint(&self, endpoint: &str, identity: Option<&str>, outcome: Outcome) {
        if outcome == Outcome::Param {
            return;
        }
        let Some(id) = identity else { return };
        let Ok(conn) = self.conn() else { return };
        let now = now();
        let _ = conn.execute("DELETE FROM breaker_events WHERE at<?1", params![now - 300]);
        let tripped = conn
            .query_row(
                "SELECT 1 FROM breakers WHERE endpoint=?1",
                params![endpoint],
                |r| r.get::<_, i64>(0),
            )
            .optional()
            .unwrap_or(None)
            .is_some();
        if outcome == Outcome::Ok && tripped {
            let _ = conn.execute("DELETE FROM breakers WHERE endpoint=?1", params![endpoint]);
            let _ = conn.execute(
                "DELETE FROM breaker_events WHERE endpoint=?1",
                params![endpoint],
            );
            return;
        }
        let _ = conn.execute(
            "INSERT INTO breaker_events(endpoint,identity,risk,at) VALUES(?1,?2,?3,?4)",
            params![endpoint, id, matches!(outcome, Outcome::Risk) as i64, now],
        );
        if let Ok((count, risks, ids)) = conn.query_row(
            "SELECT COUNT(*),COALESCE(SUM(risk),0),COUNT(DISTINCT identity) FROM breaker_events WHERE endpoint=?1 AND at>=?2",
            params![endpoint, now - 300],
            |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?, r.get::<_, i64>(2)?)),
        ) {
            if count >= 20 && ids >= 3 && risks as f64 / count as f64 > 0.6 {
                let _ = conn.execute(
                    "INSERT INTO breakers(endpoint,tripped_at,next_probe_at) VALUES(?1,?2,?3)
                     ON CONFLICT(endpoint) DO UPDATE SET tripped_at=?2,next_probe_at=?3",
                    params![endpoint, now, now + 60],
                );
                crate::logging::warn("breaker", format!("{endpoint} 跳闸"));
            }
        }
    }

    fn note_identity(&self, id: Option<&str>, outcome: Outcome, error: Option<&str>) {
        let Some(id) = id else { return };
        let Ok(conn) = self.conn() else { return };
        match outcome {
            Outcome::Risk => {
                let fails: i64 = conn
                    .query_row(
                        "SELECT consecutive_fails FROM identity_health WHERE device_id=?1",
                        params![id],
                        |r| r.get(0),
                    )
                    .optional()
                    .unwrap_or(None)
                    .unwrap_or(0)
                    + 1;
                let cool = (60i64 * 2i64.pow((fails - 1).min(8) as u32)).min(3600);
                let _ = conn.execute(
                    "INSERT INTO identity_health(device_id,consecutive_fails,cooled_until,last_error) VALUES(?1,?2,?3,?4)
                     ON CONFLICT(device_id) DO UPDATE SET consecutive_fails=?2,cooled_until=?3,last_error=?4",
                    params![id, fails, now() + cool, error],
                );
                if let Some(api) = self.api.lock().unwrap_or_else(|e| e.into_inner()).as_ref() {
                    api.cool_identity(id, cool as u64);
                }
            }
            Outcome::Ok => {
                let _ = conn.execute("UPDATE identity_health SET consecutive_fails=0,cooled_until=NULL,last_error=NULL WHERE device_id=?1", params![id]);
            }
            Outcome::Gone | Outcome::Network | Outcome::Param => {}
        }
    }

    fn defer_task(&self, id: &str) -> PResult<()> {
        self.conn()?.execute(
            "UPDATE tasks SET state='queued',attempts=MAX(attempts-1,0),started_at=NULL WHERE id=?1 AND state='running'",
            params![id],
        ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    fn finish_ok(&self, task: &TaskView, outcome: &str) -> PResult<()> {
        let id = &task.id;
        if matches!(outcome, "paused" | "canceled") {
            let (state, code) = if outcome == "paused" {
                ("paused", None)
            } else {
                ("dead", Some("cancelled"))
            };
            self.conn()?
                .execute(
                    "UPDATE tasks SET state=?2,error_code=?3,started_at=NULL,finished_at=?4 WHERE id=?1 AND state='running'",
                    params![id, state, code, if outcome == "canceled" { Some(now()) } else { None }],
                )
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            return Ok(());
        }
        let kind = match outcome {
            "gone" => Outcome::Gone,
            _ => Outcome::Ok,
        };
        let trace = self.take_trace();
        self.note_identity(trace.identity.as_deref(), kind, None);
        self.note_endpoint(&task.endpoint, trace.identity.as_deref(), kind);
        self.plan_result(task, true, None);
        self.conn()?
            .execute(
                "UPDATE tasks SET state='done',outcome=?2,finished_at=?3,request_id=COALESCE(?4,request_id),identity=COALESCE(?5,identity),upstream_url=COALESCE(?6,upstream_url) WHERE id=?1 AND state='running'",
                params![id, outcome, now(), trace.request_id, trace.identity, trace.upstream_url],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    fn finish_err(&self, task: &TaskView, status: u16, code: &str, message: &str) -> PResult<()> {
        let kind = classify_error(status, message);
        let terminal = matches!(kind, Outcome::Param)
            || status == 401
            || kind == Outcome::Gone
            || task.attempts >= task.max_attempts;
        let outcome = kind.as_str();
        let trace = self.take_trace();
        self.note_identity(trace.identity.as_deref(), kind, Some(message));
        self.note_endpoint(&task.endpoint, trace.identity.as_deref(), kind);
        let state = if kind == Outcome::Gone {
            "done"
        } else if terminal {
            "dead"
        } else {
            "queued"
        };
        if task.kind == "download" {
            if let Some(did) = task.params.get("download_id").and_then(Value::as_str) {
                let _ =
                    self.fail_download(did, if terminal { "failed" } else { "queued" }, message);
            }
        }
        self.conn()?
            .execute(
                "UPDATE tasks SET state=?2,outcome=?3,error_code=?4,finished_at=CASE WHEN ?2 IN ('done','dead') THEN ?5 ELSE NULL END,
                   started_at=CASE WHEN ?2='queued' THEN NULL ELSE started_at END,
                   retry_at=CASE WHEN ?2='queued' THEN ?9 ELSE NULL END,
                   request_id=COALESCE(?6,request_id),identity=COALESCE(?7,identity),upstream_url=COALESCE(?8,upstream_url)
                 WHERE id=?1 AND state='running'",
                params![task.id, state, outcome, code, now(), trace.request_id, trace.identity, trace.upstream_url, now() + 1i64.max((300i64 * 2i64.pow(task.attempts.min(4) as u32) / 1000).min(5))],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        self.plan_result(task, false, Some(message));
        crate::logging::warn("task", format!("{} {code}: {message}", task.id));
        Ok(())
    }

    fn capture_identity(&self, api: &Api) {
        *self.last_identity.lock().unwrap_or_else(|e| e.into_inner()) = api.last_identity();
        *self
            .last_request_id
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = api.last_request_id();
        *self
            .last_upstream_url
            .lock()
            .unwrap_or_else(|e| e.into_inner()) = api.last_upstream_url();
    }

    fn take_trace(&self) -> Trace {
        Trace {
            identity: self
                .last_identity
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .take(),
            request_id: self
                .last_request_id
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .take(),
            upstream_url: self
                .last_upstream_url
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .take(),
        }
    }

    fn execute(&self, task: &TaskView) -> PResult<String> {
        let api = self
            .api
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
            .ok_or_else(|| PipelineErr::new(503, "backend", "执行后端未就绪"))?;
        if !self.probe_allowed(&task.endpoint)? {
            return Err(PipelineErr::new(503, "breaker_open", "端点熔断中"));
        }
        if task.kind == "download" || task.kind == "record" {
            return self.download_task(task);
        }
        if task.kind == "recheck" {
            match api.call(&task.endpoint, &task.params) {
                Ok(value) => {
                    self.capture_identity(&api);
                    let gone = classify_value(&value) == Outcome::Gone;
                    self.mark_snapshot(task, gone)?;
                    if !gone {
                        self.record_snapshot(task, &value)?;
                    }
                    return Ok(if gone { "gone".into() } else { "ok".into() });
                }
                Err(e) if e.status == 404 => {
                    self.capture_identity(&api);
                    self.mark_snapshot(task, true)?;
                    return Ok("gone".into());
                }
                Err(e) => {
                    self.capture_identity(&api);
                    let _ = self.mark_snapshot(task, false);
                    return Err(PipelineErr::new(e.status, "upstream", e.message));
                }
            }
        }
        if task.kind == "backfill" {
            let pages = task
                .params
                .get("pages")
                .and_then(Value::as_i64)
                .unwrap_or(5)
                .clamp(1, 50);
            let mut cursor = String::new();
            for _ in 0..pages {
                let mut p = task.params.as_object().cloned().unwrap_or_default();
                p.insert("cursor".into(), Value::String(cursor.clone()));
                let value = api.call(&task.endpoint, &Value::Object(p)).map_err(|e| {
                    self.capture_identity(&api);
                    PipelineErr::new(e.status, "upstream", e.message)
                })?;
                self.capture_identity(&api);
                if classify_value(&value) != Outcome::Ok {
                    return Err(PipelineErr::new(
                        if classify_value(&value) == Outcome::Gone {
                            404
                        } else {
                            0
                        },
                        "upstream",
                        "回溯取数失败",
                    ));
                }
                if let Some(items) = value.get("items").and_then(Value::as_array) {
                    for item in items {
                        let rid = self.record_snapshot(task, item)?;
                        self.enqueue_item_assets(task, item, rid.as_deref())?;
                    }
                }
                let more = value
                    .get("meta")
                    .and_then(|m| m.get("has_more"))
                    .and_then(Value::as_bool)
                    .unwrap_or(false);
                let next = value
                    .get("meta")
                    .and_then(|m| m.get("cursor"))
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_string();
                if !more || next.is_empty() || next == cursor {
                    break;
                }
                cursor = next;
            }
            return Ok("ok".into());
        }
        let mut value = api.call(&task.endpoint, &task.params).map_err(|e| {
            self.capture_identity(&api);
            PipelineErr::new(e.status, "upstream", e.message)
        })?;
        self.capture_identity(&api);
        let verdict = classify_value(&value);
        match verdict {
            Outcome::Ok | Outcome::Gone => {}
            Outcome::Risk => {
                return Err(PipelineErr::new(
                    429,
                    "risk",
                    value
                        .get("status_msg")
                        .and_then(Value::as_str)
                        .unwrap_or("风控")
                        .to_string(),
                ))
            }
            Outcome::Network => return Err(PipelineErr::new(0, "network", "空包")),
            Outcome::Param => {
                return Err(PipelineErr::new(
                    400,
                    "param",
                    value
                        .get("status_msg")
                        .and_then(Value::as_str)
                        .unwrap_or("参数错误")
                        .to_string(),
                ))
            }
        }
        if let Some(items) = value.get("items").and_then(Value::as_array).cloned() {
            for item in items {
                let rid = self.record_snapshot(task, &item)?;
                self.enqueue_item_assets(task, &item, rid.as_deref())?;
            }
        } else {
            value = self.resolve_if_needed(&api, task, value)?;
            let record_id = self.record_snapshot(task, &value)?;
            self.enqueue_item_assets(task, &value, record_id.as_deref())?;
            let mut outcome = if verdict == Outcome::Gone {
                "gone".to_string()
            } else {
                "ok".to_string()
            };
            if task.kind == "collection" {
                if let Some(id) = value.get("aweme_id").and_then(Value::as_str) {
                    let id = id.to_string();
                    let mut payload = vec![
                        serde_json::json!({"type": "video", "fetched_at": now(), "data": value}),
                    ];
                    let total = value
                        .get("statistics")
                        .and_then(|s| s.get("comment_count"))
                        .and_then(Value::as_i64)
                        .unwrap_or(0);
                    let comments_outcome = self.collect_comments(
                        &api,
                        task,
                        record_id.as_deref(),
                        &id,
                        total,
                        &mut payload,
                    )?;
                    if verdict != Outcome::Gone {
                        outcome = comments_outcome;
                    }
                    self.write_payload(&id, task, &payload)?;
                }
            }
            return Ok(outcome);
        }
        Ok(if verdict == Outcome::Gone {
            "gone".into()
        } else {
            "ok".into()
        })
    }

    /// 裸目标先解析；已带目标类型时只确认可解析。
    fn resolve_if_needed(&self, api: &Api, task: &TaskView, value: Value) -> PResult<Value> {
        if value.get("aweme_id").is_some() {
            return Ok(value);
        }
        let target = task
            .params
            .get("target")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if target.starts_with("video:") {
            let id = target.trim_start_matches("video:");
            let probe = format!("/video/{id}");
            if probe != task.endpoint {
                return api
                    .call(&probe, &Value::Null)
                    .map_err(|e| PipelineErr::new(e.status, "upstream", e.message));
            }
        }
        Ok(value)
    }

    fn task_flag(task: &TaskView, key: &str) -> bool {
        task.params
            .get(key)
            .and_then(Value::as_bool)
            .unwrap_or(false)
    }

    fn first_url(v: Option<&Value>) -> Option<String> {
        match v {
            Some(Value::String(s)) if !s.is_empty() => Some(s.clone()),
            Some(Value::Array(a)) => a.iter().find_map(|x| match x {
                Value::String(s) if !s.is_empty() => Some(s.clone()),
                Value::Object(m) => m
                    .get("url_list")
                    .and_then(Value::as_array)
                    .and_then(|u| u.iter().find_map(Value::as_str))
                    .filter(|s| !s.is_empty())
                    .map(str::to_string),
                _ => None,
            }),
            Some(Value::Object(m)) => m
                .get("url_list")
                .and_then(Value::as_array)
                .and_then(|u| u.iter().find_map(Value::as_str))
                .filter(|s| !s.is_empty())
                .map(str::to_string),
            _ => None,
        }
    }

    /// 列表型任务逐条入库后，按任务参数排队资产与评论子任务。
    fn enqueue_item_assets(
        &self,
        task: &TaskView,
        value: &Value,
        record_id: Option<&str>,
    ) -> PResult<()> {
        let Some(id) = value.get("aweme_id").and_then(Value::as_str) else {
            return Ok(());
        };
        let target = format!("video:{id}");
        let title = value.get("desc").and_then(Value::as_str);
        let author = value
            .get("author")
            .and_then(|a| a.get("nickname"))
            .and_then(Value::as_str);
        let cover = value
            .get("cover")
            .and_then(Value::as_str)
            .map(str::to_string)
            .or_else(|| Self::first_url(value.get("cover")));
        let parent = Some(task.id.clone());

        if Self::task_flag(task, "save_video") {
            let direct = task
                .params
                .get("video_url")
                .and_then(Value::as_str)
                .filter(|_| {
                    task.params
                        .get("video_id")
                        .and_then(Value::as_str)
                        .map_or(true, |v| v == id)
                })
                .map(str::to_string);
            self.submit_download(DownloadReq {
                target: Some(target.clone()),
                target_key: None,
                asset_kind: Some("video".into()),
                url: direct,
                pinned: None,
                title: title.map(str::to_string),
                author: author.map(str::to_string),
                cover: cover.clone(),
                ext: task
                    .params
                    .get("video_ext")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                expected_bytes: None,
                record_id: record_id.map(str::to_string),
                parent_id: parent.clone(),
                params: None,
            })?;
        }

        if Self::task_flag(task, "save_gallery") {
            if let Some(images) = value.get("images").and_then(Value::as_array) {
                for (i, img) in images.iter().enumerate() {
                    let url = Self::first_url(img.get("download_url_list"))
                        .or_else(|| Self::first_url(img.get("url_list")));
                    let Some(url) = url else { continue };
                    self.submit_download(DownloadReq {
                        target: Some(target.clone()),
                        target_key: Some(format!("{target}:gallery:{i}")),
                        asset_kind: Some("gallery".into()),
                        url: Some(url),
                        pinned: None,
                        title: title.map(|t| format!("{t} · 第{}图", i + 1)),
                        author: author.map(str::to_string),
                        cover: cover.clone(),
                        ext: Some("jpg".into()),
                        expected_bytes: None,
                        record_id: record_id.map(str::to_string),
                        parent_id: parent.clone(),
                        params: None,
                    })?;
                }
            }
        }

        if Self::task_flag(task, "save_cover") {
            if let Some(url) = &cover {
                self.submit_download(DownloadReq {
                    target: Some(target.clone()),
                    target_key: Some(format!("{target}:cover")),
                    asset_kind: Some("cover".into()),
                    url: Some(url.clone()),
                    pinned: None,
                    title: title.map(str::to_string),
                    author: author.map(str::to_string),
                    cover: Some(url.clone()),
                    ext: Some("jpg".into()),
                    expected_bytes: None,
                    record_id: record_id.map(str::to_string),
                    parent_id: parent.clone(),
                    params: None,
                })?;
            }
        }

        if Self::task_flag(task, "save_music") {
            let url = Self::first_url(value.get("music").and_then(|m| m.get("play_url")));
            if let Some(url) = url {
                self.submit_download(DownloadReq {
                    target: Some(target.clone()),
                    target_key: Some(format!("{target}:music")),
                    asset_kind: Some("music".into()),
                    url: Some(url),
                    pinned: None,
                    title: title.map(str::to_string),
                    author: author.map(str::to_string),
                    cover: cover.clone(),
                    ext: Some("mp3".into()),
                    expected_bytes: None,
                    record_id: record_id.map(str::to_string),
                    parent_id: parent.clone(),
                    params: None,
                })?;
            }
        }

        if Self::task_flag(task, "comments") && task.endpoint.starts_with("/user/") {
            let child = self.submit_one(Submit {
                target: Some(target.clone()),
                link: None,
                kind: Some("collection".into()),
                endpoint: None,
                params: Some(serde_json::json!({
                    "target": target,
                    "target_type": "video",
                    "record_id": record_id,
                    "comments": true,
                    "level": task.params.get("level").cloned().unwrap_or(Value::from(1)),
                    "max_comments": task.params.get("max_comments").cloned().unwrap_or(Value::from(0)),
                    "max_replies": task.params.get("max_replies").cloned().unwrap_or(Value::from(0)),
                    "display_title": title,
                    "display_subtitle": author,
                })),
                max_attempts: None,
            })?;
            if let Some(cid) = child.id {
                let _ = self.conn()?.execute(
                    "UPDATE tasks SET parent_id=?2 WHERE id=?1",
                    params![cid, task.id],
                );
            }
        }
        Ok(())
    }

    fn collect_comments(
        &self,
        api: &Api,
        task: &TaskView,
        record_id: Option<&str>,
        id: &str,
        total: i64,
        payload: &mut Vec<Value>,
    ) -> PResult<String> {
        if !task
            .params
            .get("comments")
            .and_then(Value::as_bool)
            .unwrap_or(false)
        {
            return Ok("ok".into());
        }
        let level = task
            .params
            .get("level")
            .and_then(Value::as_i64)
            .unwrap_or(1);
        let limit = task
            .params
            .get("max_comments")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        let reply_limit = task
            .params
            .get("max_replies")
            .and_then(Value::as_i64)
            .unwrap_or(0);
        let target = if limit > 0 { limit } else { total.max(0) };
        let page_size = 20i64;
        let mut cursor = 0i64;
        let mut count = 0i64;
        let mut seen = std::collections::HashSet::new();
        let mut seen_order = std::collections::VecDeque::new();
        let mut calls = 0usize;
        let mut page_retries = 0usize;
        let mut stall_jumps = 0usize;
        let mut short_jumps = 0usize;
        let mut exhausted = false;
        let mut reason = String::new();
        let mut last_error: Option<PipelineErr> = None;

        while calls < 2000 {
            if target > 0 && count >= target {
                break;
            }
            calls += 1;
            let seen_query = seen_order.iter().cloned().collect::<Vec<_>>().join(",");
            let value = match api.call(
                &format!("/video/{id}/comments"),
                &serde_json::json!({"cursor": cursor, "count": page_size, "seen": seen_query}),
            ) {
                Ok(value) if classify_value(&value) == Outcome::Ok => value,
                Ok(value) => {
                    reason = format!("upstream_{}", classify_value(&value).as_str());
                    if page_retries < 4 {
                        page_retries += 1;
                        std::thread::sleep(Duration::from_millis(250 * page_retries as u64));
                        continue;
                    }
                    last_error = Some(PipelineErr::new(429, "upstream", "评论分页持续异常"));
                    break;
                }
                Err(e) => {
                    reason = "request_error".into();
                    last_error = Some(PipelineErr::new(e.status, "upstream", e.message));
                    if page_retries < 4 {
                        page_retries += 1;
                        std::thread::sleep(Duration::from_millis(250 * page_retries as u64));
                        continue;
                    }
                    break;
                }
            };
            let items = value
                .get("items")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            page_retries = 0;
            for item in &items {
                if target > 0 && count >= target {
                    break;
                }
                let cid = item.get("cid").and_then(Value::as_str).unwrap_or_default();
                if cid.is_empty() || !seen.insert(cid.to_string()) {
                    continue;
                }
                seen_order.push_back(cid.to_string());
                if seen_order.len() > 400 {
                    seen_order.pop_front();
                }
                count += 1;
                if let Some(rid) = record_id {
                    let _ = self.save_comment(rid, item, None, 1);
                }
                payload.push(serde_json::json!({"type": "comment", "data": item}));
                if level < 2 {
                    continue;
                }
                let mut rc = String::new();
                let mut replies = 0i64;
                for _ in 0..2000 {
                    if reply_limit > 0 && replies >= reply_limit {
                        break;
                    }
                    let r = match api.call(
                        &format!("/video/{id}/replies"),
                        &serde_json::json!({"comment": cid, "cursor": rc, "count": 50}),
                    ) {
                        Ok(r) => r,
                        Err(_) => break,
                    };
                    let list = r
                        .get("items")
                        .and_then(Value::as_array)
                        .cloned()
                        .unwrap_or_default();
                    for x in &list {
                        if reply_limit > 0 && replies >= reply_limit {
                            break;
                        }
                        replies += 1;
                        if let Some(rid) = record_id {
                            let _ = self.save_comment(rid, x, Some(cid), 2);
                        }
                        payload.push(serde_json::json!({"type": "reply", "data": x}));
                    }
                    let more = r
                        .get("meta")
                        .and_then(|m| m.get("has_more"))
                        .and_then(Value::as_bool)
                        .unwrap_or(false);
                    let next = r
                        .get("meta")
                        .and_then(|m| m.get("cursor"))
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string();
                    if !more || next.is_empty() || next == rc {
                        break;
                    }
                    rc = next;
                }
            }
            if target > 0 && count >= target {
                reason = "target_reached".into();
                break;
            }
            let more = value
                .get("meta")
                .and_then(|m| m.get("has_more"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let next = value
                .get("meta")
                .and_then(|m| m.get("cursor"))
                .and_then(Value::as_str)
                .and_then(|v| v.parse::<i64>().ok())
                .unwrap_or(cursor);
            if more && next > cursor {
                cursor = next;
                page_retries = 0;
                reason = "more".into();
                continue;
            }
            let wants_more = target == 0 || count < target;
            let unknown_more = total <= 0 && target > 0;
            if !more && wants_more && (count < total || unknown_more) && short_jumps < 3 {
                short_jumps += 1;
                cursor = cursor.saturating_add(page_size.max(1));
                reason = "upstream_short".into();
                std::thread::sleep(Duration::from_millis(600));
                continue;
            }
            if wants_more && more && stall_jumps < 40 {
                // 上游偶发返回重复页并让 cursor 停住。cursor 是偏移量，
                // 此时主动越过当前页，保证采集继续前进。
                stall_jumps += 1;
                page_retries = 0;
                cursor = cursor.saturating_add(page_size.max(1));
                reason = "cursor_jump".into();
                std::thread::sleep(Duration::from_millis(250));
                continue;
            }
            exhausted = true;
            reason = if !more {
                "upstream_end".into()
            } else {
                "cursor_stalled".into()
            };
            break;
        }

        let complete = comments_complete(target, count, total, exhausted);
        if complete && !exhausted {
            reason = "target_reached".into();
        }
        if !complete && reason.is_empty() {
            reason = "limit".into();
        }
        let summary = serde_json::json!({
            "comments_collected": count,
            "comments_target": limit.max(0),
            "comments_total": total,
            "comments_complete": complete,
            "comments_reason": reason,
        });
        let _ = self.conn()?.execute(
            "UPDATE tasks SET result_summary=?2 WHERE id=?1",
            params![task.id, summary.to_string()],
        );

        if count == 0 {
            if let Some(e) = last_error {
                return Err(e);
            }
        }
        if !complete {
            crate::logging::warn(
                "comments",
                format!(
                    "{id} 未达目标：{count}/{}，作品共 {} ({reason})",
                    target.max(0),
                    total.max(0)
                ),
            );
        }
        Ok(if complete { "ok" } else { "partial" }.into())
    }

    fn write_payload(&self, id: &str, task: &TaskView, payload: &[Value]) -> PResult<()> {
        use std::io::Write;
        let dir = self
            .db
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("data")
            .join(id);
        std::fs::create_dir_all(&dir).map_err(|e| PipelineErr::new(500, "io", e.to_string()))?;
        let path = dir.join(format!("{id}.jsonl"));
        let mut f =
            std::fs::File::create(&path).map_err(|e| PipelineErr::new(500, "io", e.to_string()))?;
        for row in payload {
            serde_json::to_writer(&mut f, row)
                .map_err(|e| PipelineErr::new(500, "json", e.to_string()))?;
            f.write_all(b"\n")
                .map_err(|e| PipelineErr::new(500, "io", e.to_string()))?;
        }
        self.conn()?
            .execute(
                "UPDATE tasks SET output_path=?2 WHERE id=?1",
                params![task.id, path.to_string_lossy()],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }

    fn record_snapshot(&self, task: &TaskView, value: &Value) -> PResult<Option<String>> {
        let Some(aweme_id) = value.get("aweme_id").and_then(Value::as_str) else {
            return Ok(None);
        };
        let target = format!("video:{aweme_id}");
        let metrics = value
            .get("statistics")
            .cloned()
            .unwrap_or_else(|| Value::Object(Default::default()));
        let title = value.get("desc").and_then(Value::as_str);
        let author = value.get("author").cloned().unwrap_or(Value::Null);
        let author_key = author
            .get("sec_uid")
            .and_then(Value::as_str)
            .or_else(|| author.get("uid").and_then(Value::as_str));
        let author_name = author.get("nickname").and_then(Value::as_str);
        let cover = value.get("cover").and_then(Value::as_str);
        let requested = task.params.get("record_id").and_then(Value::as_str);
        let rid = requested
            .map(str::to_string)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let mut thumb = None;
        if let (Some(app), Some(url)) = (self.app.get(), cover) {
            if let Ok(dir) = app.path().app_data_dir() {
                let dir = dir.join("cache").join("records");
                if std::fs::create_dir_all(&dir).is_ok() {
                    let ext = if url.contains(".png") { "png" } else { "jpg" };
                    let p = dir.join(format!("{rid}.{ext}"));
                    if !p.exists() {
                        let _ = fetch_file(url, &p, || false);
                    }
                    if p.is_file() {
                        thumb = Some(p.to_string_lossy().to_string());
                    }
                }
            }
        }
        let conn = self.conn()?;
        if let Some(old) = requested {
            let old_metrics: Option<String> = conn
                .query_row(
                    "SELECT metrics FROM capture_records WHERE id=?1",
                    params![old],
                    |r| r.get(0),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
            let mut hist: Vec<Value> = conn
                .query_row(
                    "SELECT metrics_history FROM capture_records WHERE id=?1",
                    params![old],
                    |r| r.get::<_, String>(0),
                )
                .optional()
                .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default();
            if let Some(m) = old_metrics {
                hist.push(serde_json::json!({"at": now(), "metrics": serde_json::from_str::<Value>(&m).unwrap_or(Value::Null)}));
            }
            conn.execute(
                "UPDATE capture_records SET task_id=?10,title=COALESCE(?2,title),author_key=COALESCE(?3,author_key),author_name=COALESCE(?4,author_name),cover=COALESCE(?5,cover),thumb_path=COALESCE(?6,thumb_path),metrics=?7,metrics_history=?8,state='live',last_checked=?9 WHERE id=?1",
                params![old, title, author_key, author_name, cover, thumb, metrics.to_string(), serde_json::to_string(&hist).unwrap_or_else(|_| "[]".into()), now(), task.id],
            ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        } else {
            conn.execute(
                "INSERT INTO capture_records(id,target_key,task_id,title,author_key,author_name,cover,thumb_path,metrics,state,created_at) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,'live',?10)",
                params![rid, target, task.id, title, author_key, author_name, cover, thumb, metrics.to_string(), now()],
            ).map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        }
        Ok(Some(rid))
    }

    fn save_comment(
        &self,
        record_id: &str,
        c: &Value,
        parent: Option<&str>,
        level: i64,
    ) -> PResult<()> {
        let cid = c.get("cid").and_then(Value::as_str).unwrap_or_default();
        if cid.is_empty() {
            return Ok(());
        }
        let user = c.get("user").cloned().unwrap_or(Value::Null);
        let author_key = user
            .get("sec_uid")
            .and_then(Value::as_str)
            .or_else(|| user.get("uid").and_then(Value::as_str));
        let author_name = user.get("nickname").and_then(Value::as_str);
        let content = c.get("text").and_then(Value::as_str).unwrap_or_default();
        let likes = c.get("digg_count").and_then(Value::as_i64);
        let ip = c.get("ip_label").and_then(Value::as_str);
        let created = c.get("create_time").and_then(Value::as_i64);
        self.conn()?
            .execute(
                "INSERT INTO captured_comments(record_id,cid,parent_cid,level,author_key,author_name,content,likes,ip_label,created_at,raw) VALUES(?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)
                 ON CONFLICT(record_id,cid) DO UPDATE SET content=excluded.content,likes=excluded.likes,raw=excluded.raw",
                params![record_id, cid, parent, level, author_key, author_name, content, likes, ip, created, c.to_string()],
            )
            .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
        Ok(())
    }
}

/// 进程内执行循环：采集串行，下载与录制按额度并发；任务仍在同一张表。
pub fn run_worker(p: Arc<Pipeline>) {
    match p.migrate_assets_once() {
        Ok(()) => {}
        Err(e) => crate::logging::error("migration", e.message),
    }
    match p.recover() {
        Ok(n) if n > 0 => crate::logging::warn("task", format!("恢复 {n} 条未完成任务")),
        Ok(_) => {}
        Err(e) => crate::logging::error("task", format!("启动恢复失败：{}", e.message)),
    }
    let mut last_watch = Instant::now() - Duration::from_secs(30);
    let mut active_downloads = 0usize;
    let mut active_records = 0usize;
    let mut active_other = 0usize;
    loop {
        if last_watch.elapsed() >= Duration::from_secs(30) {
            p.maintenance();
            match p.dispatch_plans() {
                Ok(n) if n > 0 => crate::logging::info("task", format!("计划提交 {n} 条")),
                Ok(_) => {}
                Err(e) => crate::logging::error("task", format!("计划派发失败：{}", e.message)),
            }
            last_watch = Instant::now();
        }
        let dl_limit = p
            .settings_value()
            .get("concurrency")
            .and_then(Value::as_u64)
            .unwrap_or(3)
            .clamp(1, 8) as usize;
        let rec_limit = p
            .settings_value()
            .get("recordConcurrency")
            .and_then(Value::as_u64)
            .unwrap_or(2)
            .clamp(1, 4) as usize;
        let allow_dl = active_downloads < dl_limit;
        let allow_rec = active_records < rec_limit;
        if allow_dl || allow_rec || active_other < 3 {
            match p.claim_slot(allow_dl, allow_rec) {
                Ok(Some(task)) => {
                    let is_record =
                        task.params.get("asset_kind").and_then(Value::as_str) == Some("record");
                    if task.kind == "download" {
                        if is_record {
                            active_records += 1;
                        } else {
                            active_downloads += 1;
                        }
                    } else {
                        active_other += 1;
                    }
                    crate::logging::info("task", format!("执行 {} {}", task.kind, task.endpoint));
                    let p2 = p.clone();
                    std::thread::spawn(move || {
                        match p2.execute(&task) {
                            Ok(outcome) => {
                                if let Err(e) = p2.finish_ok(&task, &outcome) {
                                    crate::logging::error(
                                        "task",
                                        format!("写完成状态失败：{}", e.message),
                                    );
                                }
                            }
                            Err(e) if e.code == "breaker_open" => {
                                let _ = p2.defer_task(&task.id);
                            }
                            Err(e) => {
                                let _ = p2.finish_err(&task, e.status, &e.code, &e.message);
                            }
                        }
                        p2.wake();
                    });
                    continue;
                }
                Ok(None) => {}
                Err(e) => crate::logging::error("task", format!("取任务失败：{}", e.message)),
            }
        }
        // 用活动数重新计算；spawn 线程通过 wake 让本循环快速返回。
        active_downloads = count_active(&p, "download", false).unwrap_or(active_downloads);
        active_records = count_active(&p, "record", true).unwrap_or(active_records);
        active_other = count_active(&p, "other", false).unwrap_or(active_other);
        let mut waited = 0;
        while waited < 150 {
            if p.take_wake() {
                break;
            }
            std::thread::sleep(Duration::from_millis(25));
            waited += 25;
        }
    }
}

fn count_active(p: &Pipeline, kind: &str, record: bool) -> PResult<usize> {
    let conn = p.conn()?;
    let n: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM tasks WHERE state='running' AND
             CASE
               WHEN ?1='other' THEN kind!='download'
               WHEN ?2=1 THEN json_extract(params,'$.asset_kind')='record'
               ELSE COALESCE(json_extract(params,'$.asset_kind'),'video')!='record'
             END",
            params![kind, record as i64],
            |r| r.get(0),
        )
        .map_err(|e| PipelineErr::new(500, "db", e.to_string()))?;
    Ok(n as usize)
}

fn row_download(r: &rusqlite::Row<'_>) -> rusqlite::Result<DownloadView> {
    Ok(DownloadView {
        id: r.get(0)?,
        task_id: r.get(1)?,
        target_key: r.get(2)?,
        asset_key: r.get(3)?,
        asset_kind: r.get(4)?,
        url: r.get(5)?,
        path: r.get(6)?,
        partial_path: r.get(7)?,
        bytes: r.get(8)?,
        expected_bytes: r.get(9)?,
        progress: r.get(10)?,
        speed: r.get(11)?,
        state: r.get(12)?,
        pinned: r.get::<_, i64>(13)? != 0,
        attempts: r.get(14)?,
        created_at: r.get(15)?,
        finished_at: r.get(16)?,
        error: r.get(17)?,
        title: r.get(18)?,
        author: r.get(19)?,
        cover: r.get(20)?,
        record_id: r.get(21)?,
        asset_id: r.get(22)?,
        task_deleted_at: r.get(23)?,
    })
}

fn row_record(r: &rusqlite::Row<'_>) -> rusqlite::Result<RecordView> {
    let metrics: String = r.get(8)?;
    let history: String = r.get(9)?;
    Ok(RecordView {
        id: r.get(0)?,
        target_key: r.get(1)?,
        task_id: r.get(2)?,
        title: r.get(3)?,
        author_key: r.get(4)?,
        author_name: r.get(5)?,
        cover: r.get(6)?,
        thumb_path: r.get(7)?,
        metrics: serde_json::from_str(&metrics).unwrap_or(Value::Object(Default::default())),
        metrics_history: serde_json::from_str(&history).unwrap_or_else(|_| serde_json::json!([])),
        state: r.get(10)?,
        created_at: r.get(11)?,
        last_checked: r.get(12)?,
        comment_count: r.get(13)?,
        asset_count: r.get(14)?,
        comments_requested: r.get::<_, i64>(15)? != 0,
        comment_target: r.get(16)?,
        comment_outcome: r.get(17)?,
        output_path: r.get(18)?,
    })
}

fn like_pattern(value: &str) -> String {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_");
    format!("%{escaped}%")
}

fn row_comment(r: &rusqlite::Row<'_>) -> rusqlite::Result<CommentView> {
    let raw: String = r.get(9)?;
    Ok(CommentView {
        cid: r.get(0)?,
        parent_cid: r.get(1)?,
        level: r.get(2)?,
        author_key: r.get(3)?,
        author_name: r.get(4)?,
        content: r.get(5)?,
        likes: r.get(6)?,
        ip_label: r.get(7)?,
        created_at: r.get(8)?,
        raw: serde_json::from_str(&raw).unwrap_or(Value::Null),
    })
}

fn row_plan(r: &rusqlite::Row<'_>) -> rusqlite::Result<PlanView> {
    let raw: String = r.get(4)?;
    Ok(PlanView {
        id: r.get(0)?,
        kind: r.get(1)?,
        target_key: r.get(2)?,
        interval_sec: r.get(3)?,
        params: serde_json::from_str(&raw).unwrap_or(Value::Object(Default::default())),
        enabled: r.get::<_, i64>(5)? != 0,
        next_due_at: r.get(6)?,
        fail_streak: r.get(7)?,
        last_task_id: r.get(8)?,
        last_error: r.get(9)?,
        created_at: r.get(10)?,
    })
}

fn plan_interval(kind: &str, value: i64) -> i64 {
    let allowed: &[i64] = if kind == "live_auto" {
        &[30, 60, 120]
    } else {
        &[300, 600, 1800, 3600]
    };
    if allowed.contains(&value) {
        value
    } else if kind == "live_auto" {
        60
    } else {
        600
    }
}

fn normalize_plan(kind: &str, raw: &str, params: Option<Value>) -> PResult<(String, Value)> {
    if kind != "author_new" && kind != "live_auto" {
        return Err(PipelineErr::new(
            400,
            "invalid_kind",
            "计划类型只能是 author_new/live_auto",
        ));
    }
    let text = raw.trim();
    let want_room = kind == "live_auto";
    let value = if want_room {
        text.trim_start_matches("room:")
            .trim_start_matches("live:")
            .trim_matches('/')
            .to_string()
    } else {
        text.trim_start_matches("user:")
            .trim_matches('/')
            .to_string()
    };
    if value.is_empty() {
        return Err(PipelineErr::new(400, "invalid_target", "计划目标为空"));
    }
    let key = if want_room {
        format!("room:{value}")
    } else {
        format!("user:{value}")
    };
    let params = match params {
        Some(Value::Object(m)) => Value::Object(m),
        Some(_) => return Err(PipelineErr::new(400, "invalid_params", "params 必须是对象")),
        None => serde_json::json!({}),
    };
    Ok((key, params))
}

fn row_asset(r: &rusqlite::Row<'_>) -> rusqlite::Result<AssetView> {
    let path: String = r.get(6)?;
    let dir = std::path::Path::new(&path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let local_cover: String = r.get(11)?;
    Ok(AssetView {
        id: r.get(0)?,
        target_key: r.get(1)?,
        kind: r.get(2)?,
        title: r.get(3)?,
        author: r.get(4)?,
        cover: if local_cover.is_empty() {
            r.get(5)?
        } else {
            Some(local_cover)
        },
        path,
        dir,
        size: r.get(7)?,
        missing: false,
        task_id: r.get(8)?,
        record_id: r.get(9)?,
        created_at: r.get(10)?,
    })
}

fn row_library(r: &rusqlite::Row<'_>) -> rusqlite::Result<LibraryView> {
    let source: String = r.get(0)?;
    let raw_id: String = r.get(1)?;
    let thumb_path: Option<String> = r.get(8)?;
    Ok(LibraryView {
        id: format!("{source}:{raw_id}"),
        source,
        item_type: r.get(2)?,
        asset_kind: r.get(3)?,
        title: r.get(4)?,
        author: r.get(5)?,
        target_key: r.get(6)?,
        cover: r.get(7)?,
        thumb_path: thumb_path.filter(|v| !v.is_empty()),
        path: r.get(9)?,
        size: r.get(10)?,
        created_at: r.get(11)?,
        missing: false,
        record_state: r.get(12)?,
        metrics: r
            .get::<_, String>(13)
            .ok()
            .and_then(|v| serde_json::from_str(&v).ok())
            .unwrap_or_else(|| serde_json::json!({})),
        comments_requested: r.get::<_, i64>(14)? != 0,
        comment_count: r.get(15)?,
        comment_target: r.get(16)?,
        comment_outcome: r.get(17)?,
        asset_count: r.get(18)?,
    })
}

fn find_download(conn: &Connection, key: &str) -> PResult<Option<DownloadView>> {
    conn.query_row("SELECT id,task_id,target_key,asset_key,asset_kind,url,path,partial_path,bytes,expected_bytes,progress,speed,state,pinned,attempts,created_at,finished_at,error,title,author,cover,record_id,asset_id,task_deleted_at FROM downloads WHERE target_key=?1", params![key], row_download)
        .optional().map_err(|e| PipelineErr::new(500, "db", e.to_string()))
}

enum Transfer {
    Done(i64),
    Aborted,
}

fn part_size(path: &std::path::Path) -> i64 {
    std::fs::metadata(partial_path(path))
        .map(|m| m.len() as i64)
        .unwrap_or(0)
}

fn partial_path(path: &std::path::Path) -> PathBuf {
    PathBuf::from(format!("{}.part", path.to_string_lossy()))
}

fn trash_with_parts(path: &std::path::Path) -> PResult<()> {
    for p in [path.to_path_buf(), partial_path(path)] {
        if p.exists() {
            crate::trash::delete(&p).map_err(|e| PipelineErr::new(500, "trash", e.to_string()))?;
        }
    }
    Ok(())
}

fn fetch_file(
    url: &str,
    path: &std::path::Path,
    should_stop: impl Fn() -> bool,
) -> Result<Transfer, String> {
    fetch_file_resume(url, path, should_stop, |_, _| {})
}

fn fetch_file_resume(
    url: &str,
    path: &std::path::Path,
    should_stop: impl Fn() -> bool,
    mut progress: impl FnMut(i64, i64),
) -> Result<Transfer, String> {
    use std::io::{Read, Write};
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let part = partial_path(path);
    let mut offset = part_size(path);
    let client = crate::net::blocking_builder()
        .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15")
        .connect_timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;
    let mut req = client.get(url).header("Referer", "https://www.douyin.com/");
    if offset > 0 {
        req = req.header("Range", format!("bytes={offset}-"));
    }
    let mut res = req.send().map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("HTTP {}", res.status()));
    }
    if offset > 0 && res.status() != reqwest::StatusCode::PARTIAL_CONTENT {
        offset = 0;
        let _ = std::fs::remove_file(&part);
    }
    let total = res
        .headers()
        .get("content-range")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.rsplit('/').next())
        .and_then(|v| v.parse::<i64>().ok())
        .or_else(|| res.content_length().map(|n| n as i64 + offset))
        .unwrap_or(0);
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(offset > 0)
        .write(true)
        .truncate(offset == 0)
        .open(&part)
        .map_err(|e| e.to_string())?;
    let mut buf = [0u8; 64 * 1024];
    let mut total_done = offset;
    while !should_stop() {
        let n = res.read(&mut buf).map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n]).map_err(|e| e.to_string())?;
        total_done += n as i64;
        progress(total_done, total);
    }
    file.flush().map_err(|e| e.to_string())?;
    drop(file);
    if should_stop() {
        return Ok(Transfer::Aborted);
    }
    if total > 0 && total_done != total {
        return Err(format!("传输不足：{total_done}/{total}"));
    }
    std::fs::rename(&part, path).map_err(|e| e.to_string())?;
    Ok(Transfer::Done(total_done))
}

fn record_room(key: &str) -> String {
    key.split(':')
        .nth(1)
        .unwrap_or(key)
        .trim_start_matches("room:")
        .to_string()
}

fn legacy_asset_kind(path: &str, current: &str) -> String {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|v| v.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match ext.as_str() {
        "mp3" | "m4a" | "aac" | "wav" | "flac" => "music",
        "zip" => "gallery",
        "jpg" | "jpeg" | "png" | "webp" | "gif" | "avif" => {
            if current == "cover" {
                "cover"
            } else {
                "gallery"
            }
        }
        "mp4" | "mov" | "m4v" | "ts" | "mkv" | "webm" => {
            if current == "record" {
                "record"
            } else {
                "video"
            }
        }
        _ if matches!(current, "video" | "cover" | "music" | "gallery" | "record") => current,
        _ => "video",
    }
    .to_string()
}

fn normalize_download_target(raw: &str) -> PResult<String> {
    let (kind, value) = split_target(raw);
    if kind == "video" && !value.is_empty() {
        return Ok(format!("video:{value}"));
    }
    if let Some(v) = digits19(raw) {
        return Ok(format!("video:{v}"));
    }
    if raw.starts_with("room:") || raw.starts_with("live:") {
        let v = raw
            .split_once(':')
            .map(|x| x.1)
            .unwrap_or_default()
            .trim()
            .to_string();
        if !v.is_empty() {
            return Ok(format!("room:{v}"));
        }
    }
    if !raw.is_empty() {
        return Ok(raw.to_string());
    }
    Err(PipelineErr::new(400, "invalid_target", "下载目标为空"))
}

fn clean_ext(ext: &str) -> String {
    let e = ext.trim().trim_start_matches('.').to_ascii_lowercase();
    if e.is_empty() || e.len() > 8 || !e.chars().all(|c| c.is_ascii_alphanumeric()) {
        "bin".into()
    } else {
        e
    }
}

fn safe_name(s: &str, max: usize) -> String {
    let mut out: String = s
        .chars()
        .map(|c| {
            if matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') || c.is_control() {
                '_'
            } else {
                c
            }
        })
        .collect();
    out = out.split_whitespace().collect::<Vec<_>>().join(" ");
    if out.chars().count() > max {
        out = out.chars().take(max).collect();
    }
    if out.trim().is_empty() {
        "untitled".into()
    } else {
        out
    }
}

fn unique_dir(path: PathBuf) -> PathBuf {
    if !path.exists() {
        return path;
    }
    let stem = path
        .file_name()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or_else(|| "dir".into());
    let parent = path.parent().map(|x| x.to_path_buf()).unwrap_or_default();
    for n in 2..10000 {
        let p = parent.join(format!("{stem} ({n})"));
        if !p.exists() {
            return p;
        }
    }
    path
}

fn unique_dest(path: PathBuf) -> PathBuf {
    if !path.exists() && !partial_path(&path).exists() {
        return path;
    }
    let ext = path
        .extension()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or_default();
    let stem = path
        .file_stem()
        .map(|x| x.to_string_lossy().to_string())
        .unwrap_or_else(|| "file".into());
    let parent = path.parent().map(|x| x.to_path_buf()).unwrap_or_default();
    for n in 2..10000 {
        let mut p = parent.join(format!("{stem} ({n})"));
        p.set_extension(&ext);
        if !p.exists() && !partial_path(&p).exists() {
            return p;
        }
    }
    path
}

fn file_date(secs: i64) -> String {
    let t = secs + 8 * 3600;
    let z = t.div_euclid(86_400) + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let y = y + i64::from(m <= 2);
    format!("{y:04}{m:02}{d:02}")
}

fn row_task(r: &rusqlite::Row<'_>) -> rusqlite::Result<TaskView> {
    let params: String = r.get(3)?;
    Ok(TaskView {
        id: r.get(0)?,
        kind: r.get(1)?,
        endpoint: r.get(2)?,
        params: serde_json::from_str(&params).unwrap_or(Value::Null),
        target_key: r.get(4)?,
        state: r.get(5)?,
        attempts: r.get(6)?,
        max_attempts: r.get(7)?,
        batch_id: r.get(8)?,
        outcome: r.get(9)?,
        error_code: r.get(10)?,
        created_at: r.get(11)?,
        started_at: r.get(12)?,
        finished_at: r.get(13)?,
        request_id: r.get(14)?,
        identity: r.get(15)?,
        upstream_url: r.get(16)?,
        retry_at: r.get(17)?,
        output_path: r.get(18)?,
        source: r.get(19)?,
        progress: r.get(20)?,
        bytes: r.get(21)?,
        cover: r.get(22)?,
        acknowledged_at: r.get(23)?,
        archived_at: r.get(24)?,
        deleted_at: r.get(25)?,
        keep_until: r.get(26)?,
        trigger: r.get(27)?,
        plan_id: r.get(28)?,
        result_summary: r
            .get::<_, Option<String>>(29)?
            .and_then(|s| serde_json::from_str(&s).ok()),
    })
}

fn apply_task_scope(
    where_sql: &mut Vec<String>,
    values: &mut Vec<rusqlite::types::Value>,
    scope: Option<&str>,
    now: i64,
) {
    match scope.unwrap_or("history") {
        "active" => {
            where_sql.push("deleted_at IS NULL AND state IN ('queued','running','paused')".into())
        }
        "pending" => {
            where_sql.push("deleted_at IS NULL AND state='dead' AND acknowledged_at IS NULL".into())
        }
        "recent" => {
            where_sql.push(
                "deleted_at IS NULL AND archived_at IS NULL AND ((state='done' AND finished_at>=?) OR (state='canceled' AND finished_at>=?) OR (state='dead' AND acknowledged_at IS NOT NULL AND finished_at>=?))".into(),
            );
            values.push((now - 86_400).into());
            values.push((now - 3_600).into());
            values.push((now - 7 * 86_400).into());
        }
        "deleted" => where_sql.push("deleted_at IS NOT NULL".into()),
        _ => where_sql.push("deleted_at IS NULL".into()),
    }
}

fn apply_task_filter(
    where_sql: &mut Vec<String>,
    values: &mut Vec<rusqlite::types::Value>,
    filter: &TaskFilter,
) {
    if let Some(state) = filter.state.as_deref().filter(|s| !s.is_empty()) {
        where_sql.push("state=?".into());
        values.push(state.to_string().into());
    }
    if let Some(kind) = filter.kind.as_deref().filter(|s| !s.is_empty()) {
        match kind {
            "download" => where_sql.push("kind='download'".into()),
            "json" => {
                where_sql.push("kind IN ('collection','recheck','backfill')".into());
            }
            "live" => where_sql.push("json_extract(params,'$.asset_kind')='record'".into()),
            "other" => where_sql.push(
                "COALESCE(json_extract(params,'$.asset_kind'),'') NOT IN ('video','cover','gallery','music','record') AND kind NOT IN ('collection','recheck','backfill')".into(),
            ),
            asset => {
                where_sql.push("json_extract(params,'$.asset_kind')=?".into());
                values.push(asset.to_string().into());
            }
        }
    }
    if let Some(q) = filter.q.as_deref().map(str::trim).filter(|q| !q.is_empty()) {
        let like = format!("%{q}%");
        where_sql.push("(COALESCE(json_extract(params,'$.display_title'),'') LIKE ? OR COALESCE(target_key,'') LIKE ? OR COALESCE(error_code,'') LIKE ?)".into());
        values.push(like.clone().into());
        values.push(like.clone().into());
        values.push(like.into());
    }
    if let Some(source) = filter.source.as_deref().filter(|s| !s.is_empty()) {
        where_sql.push("COALESCE(trigger,'user')=?".into());
        values.push(source.to_string().into());
    }
    if let Some(batch) = filter.batch_id.as_deref().filter(|s| !s.is_empty()) {
        where_sql.push("batch_id=?".into());
        values.push(batch.to_string().into());
    }
    if let Some(plan) = filter.plan_id.as_deref().filter(|s| !s.is_empty()) {
        where_sql.push("plan_id=?".into());
        values.push(plan.to_string().into());
    }
    if filter.kept == Some(true) {
        where_sql.push("keep_until>?".into());
        values.push(now().into());
    }
}

fn apply_asset_filter(
    sql: &mut String,
    args: &mut Vec<String>,
    filter: &AssetFilter,
    hide_cover: bool,
) {
    match filter.kind.as_deref().filter(|v| !v.is_empty()) {
        Some(kind) => {
            sql.push_str(" AND kind=?");
            args.push(kind.to_string());
        }
        None if hide_cover => sql.push_str(" AND kind<>'cover'"),
        None => {}
    }
    if let Some(v) = filter.title.as_deref().filter(|v| !v.is_empty()) {
        sql.push_str(" AND title LIKE ?");
        args.push(format!("%{v}%"));
    }
    if let Some(v) = filter.author.as_deref().filter(|v| !v.is_empty()) {
        sql.push_str(" AND author=?");
        args.push(v.to_string());
    }
    if let Some(v) = filter.from {
        sql.push_str(" AND created_at>=?");
        args.push(v.to_string());
    }
    if let Some(v) = filter.to {
        sql.push_str(" AND created_at<?");
        args.push(v.to_string());
    }
    if let Some(v) = filter.source.as_deref().filter(|v| !v.is_empty()) {
        sql.push_str(" AND task_id=?");
        args.push(v.to_string());
    }
    if let Some(v) = filter.record.as_deref().filter(|v| !v.is_empty()) {
        sql.push_str(" AND record_id=?");
        args.push(v.to_string());
    }
    if let Some(v) = filter.dir.as_deref().filter(|v| !v.is_empty()) {
        // Windows 落盘路径是反斜杠，统一折成 `/` 再比较。
        let dir = v.trim_end_matches(['/', '\\']).replace('\\', "/");
        sql.push_str(" AND REPLACE(path,'\\','/') LIKE ?");
        args.push(format!("{dir}/%"));
        if filter.direct.as_deref() == Some("1") {
            sql.push_str(" AND REPLACE(path,'\\','/') NOT LIKE ?");
            args.push(format!("{dir}/%/%"));
        }
    }
}

fn apply_asset_cursor(sql: &mut String, args: &mut Vec<String>, cursor: Option<&str>) {
    if let Some((ts, id)) = cursor.and_then(|c| c.split_once('|')) {
        if ts.parse::<i64>().is_ok() {
            sql.push_str(" AND (created_at<? OR (created_at=? AND id<?))");
            args.push(ts.to_string());
            args.push(ts.to_string());
            args.push(id.to_string());
        }
    }
}

fn apply_library_filter(sql: &mut String, args: &mut Vec<String>, filter: &LibraryFilter) {
    if let Some(kind) = filter.kind.as_deref().filter(|v| !v.is_empty()) {
        sql.push_str(" AND item_type=?");
        args.push(kind.to_string());
    }
    if let Some(q) = filter.q.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
        sql.push_str(
            " AND (COALESCE(title,'') LIKE ? OR COALESCE(author,'') LIKE ? OR target_key LIKE ?)",
        );
        let like = format!("%{q}%");
        args.push(like.clone());
        args.push(like.clone());
        args.push(like);
    }
    if let Some(from) = filter.from {
        sql.push_str(" AND created_at>=?");
        args.push(from.to_string());
    }
    if let Some(to) = filter.to {
        sql.push_str(" AND created_at<?");
        args.push(to.to_string());
    }
}

fn apply_library_cursor(sql: &mut String, args: &mut Vec<String>, cursor: Option<&str>) {
    let Some((ts, source, id)) = cursor.and_then(|c| {
        let mut parts = c.splitn(3, '|');
        Some((parts.next()?, parts.next()?, parts.next()?))
    }) else {
        return;
    };
    if ts.parse::<i64>().is_err() {
        return;
    }
    sql.push_str(" AND (created_at<? OR (created_at=? AND (source<? OR (source=? AND raw_id<?))))");
    args.push(ts.into());
    args.push(ts.into());
    args.push(source.into());
    args.push(source.into());
    args.push(id.into());
}

fn raw_max(v: Option<i64>) -> i64 {
    v.unwrap_or(6).clamp(1, 20)
}

fn chrono_day() -> String {
    day_key(now())
}

fn day_key(secs: i64) -> String {
    // 按 UTC 日界生成日期字符串，供统计接口使用。
    let days = secs / 86_400;
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn invalid(message: &str) -> PipelineErr {
    PipelineErr::new(400, "invalid_target", message)
}

fn digits19(s: &str) -> Option<&str> {
    s.split(|c: char| !c.is_ascii_digit())
        .find(|x| (15..=20).contains(&x.len()))
}

fn normalize(item: Item) -> PResult<Normalized> {
    let raw = item
        .target
        .or(item.link)
        .unwrap_or_default()
        .trim()
        .to_string();
    if raw.is_empty() {
        return Err(invalid("目标为空"));
    }
    let kind = item.kind.unwrap_or_else(|| "collection".into());
    if !KINDS.contains(&kind.as_str()) {
        return Err(PipelineErr::new(
            400,
            "invalid_kind",
            format!("未知任务类型 {kind}"),
        ));
    }

    let (mut target_kind, mut value) = split_target(&raw);
    if target_kind.is_empty() {
        if let Ok(u) = url::Url::parse(&raw) {
            let path = u.path().to_string();
            let segs: Vec<&str> = path.split('/').filter(|x| !x.is_empty()).collect();
            if let Some(pos) = segs.iter().position(|x| *x == "user") {
                if let Some(v) = segs.get(pos + 1) {
                    target_kind = "user".into();
                    value = (*v).into();
                }
            } else if let Some(pos) = segs.iter().position(|x| *x == "series" || *x == "mix") {
                target_kind = segs[pos].to_string();
                value = segs.get(pos + 1).copied().unwrap_or_default().into();
            } else {
                let candidate = u
                    .query_pairs()
                    .find(|(k, _)| k == "modal_id" || k == "aweme_id")
                    .map(|(_, v)| v.to_string())
                    .or_else(|| digits19(&raw).map(str::to_string));
                if let Some(v) = candidate {
                    target_kind = "video".into();
                    value = v;
                }
            }
        } else if let Some(v) = digits19(&raw) {
            target_kind = "video".into();
            value = v.to_string();
        }
    }

    if value.is_empty() {
        return Err(invalid("认不出目标"));
    }
    let endpoint = item.endpoint.filter(|x| !x.is_empty()).unwrap_or_default();
    let endpoint = if endpoint.is_empty() {
        match (kind.as_str(), target_kind.as_str()) {
            ("download", "video") => format!("/video/{value}/download"),
            ("backfill", "user") => format!("/user/{value}/posts"),
            (_, "video") => format!("/video/{value}"),
            (_, "user") => format!("/user/{value}/posts"),
            (_, "series") => format!("/series/{value}/episodes"),
            (_, "mix") => format!("/mix/{value}/videos"),
            _ => return Err(invalid("认不出目标类型")),
        }
    } else {
        if !endpoint.starts_with('/') {
            return Err(PipelineErr::new(400, "invalid_endpoint", "路由以 / 开头"));
        }
        endpoint
    };
    validate_endpoint(&kind, &endpoint)?;
    let key = target_key(&target_kind, &value);
    let mut body = match item.params {
        Some(Value::Object(m)) => m,
        Some(_) => return Err(PipelineErr::new(400, "invalid_params", "params 必须是对象")),
        None => serde_json::Map::new(),
    };
    body.entry("target")
        .or_insert(Value::String(format!("{target_kind}:{value}")));
    Ok(Normalized {
        kind,
        endpoint,
        params: Value::Object(body),
        target_key: key,
    })
}

fn split_target(raw: &str) -> (String, String) {
    if let Some((head, tail)) = raw.split_once(':') {
        let head = head.to_ascii_lowercase();
        let tail = tail.trim().trim_matches('/');
        let known = matches!(
            head.as_str(),
            "video" | "user" | "series" | "mix" | "collection" | "room" | "live"
        );
        if known && !tail.is_empty() {
            let head = if head == "collection" {
                "mix"
            } else if head == "live" {
                "room"
            } else {
                &head
            };
            let value = digits19(tail).unwrap_or(tail);
            return (head.to_string(), value.to_string());
        }
    }
    (String::new(), String::new())
}

fn target_key(kind: &str, value: &str) -> String {
    format!("{kind}:{value}")
}

fn validate_endpoint(kind: &str, endpoint: &str) -> PResult<()> {
    let segs: Vec<&str> = endpoint.trim_matches('/').split('/').collect();
    let ok = match kind {
        "download" => matches!(segs.as_slice(), ["video", id, "download"] if !id.is_empty()),
        "backfill" => matches!(segs.as_slice(), ["user", id, "posts"] if !id.is_empty()),
        "recheck" => matches!(segs.as_slice(), ["video", id] if !id.is_empty()),
        _ => {
            matches!(
                segs.as_slice(),
                ["video", id] if !id.is_empty()
            ) || matches!(segs.as_slice(), ["video", id, "comments"] if !id.is_empty())
                || matches!(segs.as_slice(), ["user", id, "posts"] if !id.is_empty())
                || matches!(segs.as_slice(), ["series", id, "episodes"] if !id.is_empty())
                || matches!(segs.as_slice(), ["mix", id, "videos"] if !id.is_empty())
        }
    };
    if ok {
        Ok(())
    } else {
        Err(PipelineErr::new(
            400,
            "invalid_endpoint",
            format!("{kind} 不支持路由 {endpoint}"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pipe_at(path: std::path::PathBuf) -> Pipeline {
        let p = Pipeline::new(path);
        let dir = std::env::temp_dir().join(format!("dy-dl-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&dir).unwrap();
        p.conn()
            .unwrap()
            .execute(
                "INSERT INTO settings(key,value) VALUES('main',?1) ON CONFLICT(key) DO UPDATE SET value=excluded.value",
                params![serde_json::json!({"downloadDir": dir.to_string_lossy(), "naming": "{author}/{date}_{desc}_{id}"}).to_string()],
            )
            .unwrap();
        p
    }

    fn pipe(name: &str) -> Pipeline {
        let path = std::env::temp_dir().join(format!("dy-pipe-{name}-{}.sqlite3", Uuid::new_v4()));
        pipe_at(path)
    }

    fn item(id: &str) -> Item {
        Item {
            target: Some(format!("video:{id}")),
            link: None,
            kind: None,
            endpoint: None,
            params: None,
            max_attempts: None,
        }
    }

    #[test]
    fn recovery_requeues_or_kills() {
        let p = pipe("recover");
        let c = p.conn().unwrap();
        let now = now();
        for (id, state, attempts, max, started) in [
            ("old-running", "running", 0i64, 6i64, now - 301),
            ("fresh-running", "running", 0, 6, now),
            ("queued-once", "queued", 0, 1, now),
            ("queued-ok", "queued", 0, 6, now),
        ] {
            c.execute(
                "INSERT INTO downloads(id,target_key,url,bytes,state,created_at) VALUES(?1,?1,'u',0,'queued',?2)",
                params![format!("d-{id}"), now],
            ).unwrap();
            c.execute(
                "INSERT INTO tasks(id,kind,endpoint,params,target_key,state,attempts,max_attempts,created_at,started_at) VALUES(?1,'collection','/video/1',?7,'video:1',?2,?3,?4,?5,?6)",
                params![id, state, attempts, max, now, started, serde_json::json!({"download_id": format!("d-{id}")}).to_string()],
            ).unwrap();
        }
        c.execute(
            "INSERT INTO record_sessions(id,room_id,target_key,dir,state,started_at) VALUES('rs1','7400000000000000001','room:7400000000000000001','/tmp/x','running',?1)",
            params![now],
        ).unwrap();
        p.recover().unwrap();
        let state = |id: &str| -> String {
            c.query_row("SELECT state FROM tasks WHERE id=?1", params![id], |r| {
                r.get(0)
            })
            .unwrap()
        };
        assert_eq!(state("old-running"), "queued");
        assert_eq!(state("fresh-running"), "queued");
        assert_eq!(state("queued-once"), "dead");
        assert_eq!(state("queued-ok"), "queued");
        let dl = |id: &str| -> String {
            c.query_row(
                "SELECT state FROM downloads WHERE id=?1",
                params![id],
                |r| r.get(0),
            )
            .unwrap()
        };
        assert_eq!(dl("d-old-running"), "queued");
        assert_eq!(dl("d-queued-once"), "failed");
        let session: String = c
            .query_row(
                "SELECT state FROM record_sessions WHERE id='rs1'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(session, "failed");
        let claimed = p.claim().unwrap().unwrap();
        assert_eq!(claimed.state, "running");
        assert!(claimed.attempts >= 2);
    }

    #[test]
    fn dashboard_reports_series_and_kind_counts() {
        let p = pipe("dashboard");
        let c = p.conn().unwrap();
        let t = now();
        c.execute("INSERT INTO tasks(id,kind,endpoint,params,state,attempts,created_at) VALUES('a','collection','/video/1','{}','running',1,?1)", params![t]).unwrap();
        c.execute("INSERT INTO tasks(id,kind,endpoint,params,state,attempts,created_at) VALUES('b','download','/video/2/download','{}','done',1,?1)", params![t]).unwrap();
        let d = p.dashboard().unwrap();
        assert_eq!(d.series.len(), 14);
        assert!(d
            .kinds
            .iter()
            .any(|k| k.kind == "collection" && k.active == 1));
        assert_eq!(d.queue_depth, 1);
    }

    #[test]
    fn stats_count_known_results_and_zero_after_clear() {
        let p = pipe("stats");
        let c = p.conn().unwrap();
        let t = now();
        for (i, state, outcome, fin) in [
            ("ok", "done", "ok", t),
            ("gone", "done", "gone", t),
            ("risk", "dead", "risk", t),
            ("net", "dead", "network", t),
        ] {
            c.execute("INSERT INTO tasks(id,kind,endpoint,params,state,attempts,created_at,started_at,finished_at,outcome) VALUES(?1,'collection','/video/1','{}',?2,1,?3,?3,?4,?5)",
                params![i, state, t - 5, fin, outcome]).unwrap();
        }
        c.execute("INSERT INTO tasks(id,kind,endpoint,params,state,created_at) VALUES('q','collection','/video/2','{}','queued',?1)", params![t]).unwrap();
        c.execute("INSERT INTO downloads(id,target_key,url,bytes,state,created_at) VALUES('d','video:1','u',123,'done',?1)", params![t]).unwrap();
        let s = p.stats().unwrap();
        assert_eq!(
            (
                s.submitted,
                s.completed,
                s.succeeded,
                s.failed,
                s.gone,
                s.queue_depth,
                s.disk_bytes
            ),
            (5, 4, 1, 2, 1, 1, 123)
        );
        c.execute("DELETE FROM tasks", []).unwrap();
        c.execute("DELETE FROM downloads", []).unwrap();
        let s = p.stats().unwrap();
        assert_eq!(
            (s.submitted, s.completed, s.queue_depth, s.disk_bytes),
            (0, 0, 0, 0)
        );
    }

    fn task_at(p: &Pipeline, id: &str, state: &str, created: i64, finished: Option<i64>) {
        p.conn()
            .unwrap()
            .execute(
                "INSERT INTO tasks(id,kind,endpoint,params,target_key,state,attempts,created_at,finished_at) VALUES(?1,'collection','/video/1','{}',?2,?3,1,?4,?5)",
                params![id, format!("video:{id}"), state, created, finished],
            )
            .unwrap();
    }

    #[test]
    fn cancel_uses_canceled_and_leaves_pending_empty() {
        let p = pipe("canceled");
        task_at(&p, "a", "running", now(), None);
        p.cancel("a").unwrap();
        assert_eq!(p.get("a").unwrap().state, "canceled");
        let summary = p.task_summary(None).unwrap();
        assert_eq!(
            (summary.active, summary.pending, summary.history),
            (0, 0, 1)
        );
    }

    #[test]
    fn task_list_pages_with_cursor_and_scope() {
        let p = pipe("cursor");
        let t = now();
        for i in 0..5 {
            task_at(&p, &format!("t{i}"), "done", t + i, Some(t + i));
        }
        let page1 = p.list(Some("recent"), None, None, 2).unwrap();
        assert_eq!(page1.len(), 2);
        let cursor = format!("{}|{}", page1[1].created_at, page1[1].id);
        p.conn()
            .unwrap()
            .execute(
                "UPDATE tasks SET result_summary='{\"comments_collected\":48}' WHERE id='t0'",
                [],
            )
            .unwrap();
        assert_eq!(
            p.get("t0").unwrap().result_summary.unwrap()["comments_collected"],
            48
        );
        let page2 = p.list(Some("recent"), None, Some(&cursor), 2).unwrap();
        assert_eq!(page2.len(), 2);
        assert!(page1.iter().all(|a| page2.iter().all(|b| a.id != b.id)));
        let filter = TaskFilter {
            kind: Some("json".into()),
            ..TaskFilter::default()
        };
        assert_eq!(
            p.list(Some("history"), Some(&filter), None, 20)
                .unwrap()
                .len(),
            5
        );
    }

    #[test]
    fn delete_is_recoverable_and_purge_detaches_outputs() {
        let p = pipe("task-lifecycle");
        let c = p.conn().unwrap();
        let t = now();
        c.execute(
            "INSERT INTO downloads(id,task_id,target_key,asset_key,url,bytes,state,created_at) VALUES('d','x','video:x','video:x','u',1,'done',?1)",
            params![t],
        )
        .unwrap();
        c.execute(
            "INSERT INTO tasks(id,kind,endpoint,params,target_key,state,attempts,created_at,finished_at) VALUES('x','download','/download','{\"download_id\":\"d\"}','download:video:x','done',1,?1,?1)",
            params![t],
        )
        .unwrap();
        c.execute(
            "INSERT INTO capture_records(id,target_key,task_id,metrics,state,created_at) VALUES('r','video:x','x','{}','live',?1)",
            params![t],
        )
        .unwrap();
        c.execute(
            "INSERT INTO assets(id,target_key,kind,task_id,path,created_at) VALUES('a','video:x','video','x','/tmp/a',?1)",
            params![t],
        )
        .unwrap();
        p.delete_task("x", false).unwrap();
        assert!(p.get("x").unwrap().deleted_at.is_some());
        let stamp: Option<i64> = c
            .query_row(
                "SELECT task_deleted_at FROM downloads WHERE id='d'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(stamp.is_some());
        assert_eq!(p.task_summary(None).unwrap().deleted, 1);
        p.restore_task("x").unwrap();
        assert!(p.get("x").unwrap().deleted_at.is_none());
        p.delete_task("x", false).unwrap();
        p.purge_task("x", false).unwrap();
        assert!(p.get("x").is_err());
        let tid: Option<String> = c
            .query_row("SELECT task_id FROM downloads WHERE id='d'", [], |r| {
                r.get(0)
            })
            .unwrap();
        let rid: Option<String> = c
            .query_row(
                "SELECT task_id FROM capture_records WHERE id='r'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        let aid: Option<String> = c
            .query_row("SELECT task_id FROM assets WHERE id='a'", [], |r| r.get(0))
            .unwrap();
        assert_eq!((tid, rid, aid), (None, None, None));
    }

    #[test]
    fn task_action_can_delete_entire_filtered_scope() {
        let p = pipe("bulk-action");
        let t = now();
        for i in 0..3 {
            task_at(&p, &format!("d{i}"), "done", t + i, Some(t + i));
        }
        let filter = TaskFilter::default();
        let out = p
            .task_action(TaskActionReq {
                action: "delete".into(),
                ids: None,
                scope: Some("recent".into()),
                filter: Some(filter),
                file: Some("keep".into()),
            })
            .unwrap();
        assert_eq!(out.count, 3);
        assert_eq!(p.task_summary(None).unwrap().deleted, 3);
    }

    #[test]
    fn maintenance_archives_and_purges_without_removing_outputs() {
        let p = pipe("maintenance");
        let t = now();
        task_at(
            &p,
            "old-done",
            "done",
            t - 40 * 86_400,
            Some(t - 40 * 86_400),
        );
        task_at(
            &p,
            "old-canceled",
            "canceled",
            t - 10 * 86_400,
            Some(t - 10 * 86_400),
        );
        task_at(&p, "recent-done", "done", t, Some(t));
        let c = p.conn().unwrap();
        for (id, stamp) in [("old-orphan", t - 8 * 86_400), ("new-orphan", t)] {
            c.execute(
                "INSERT INTO downloads(id,target_key,asset_key,url,state,created_at,task_deleted_at) VALUES(?1,?1,?1,'u','gone',?2,?2)",
                params![id, stamp],
            )
            .unwrap();
        }
        p.maintenance();
        assert!(p.get("old-done").is_err());
        assert!(p.get("old-canceled").is_err());
        assert!(p.get("recent-done").is_ok());
        let count = |id: &str| -> i64 {
            c.query_row(
                "SELECT COUNT(*) FROM downloads WHERE id=?1",
                params![id],
                |r| r.get(0),
            )
            .unwrap()
        };
        assert_eq!(count("old-orphan"), 0);
        assert_eq!(count("new-orphan"), 1);
    }

    #[test]
    fn breaker_needs_three_identities_and_probes() {
        let p = pipe("breaker");
        for i in 0..20 {
            let id = match i % 3 {
                0 => "a",
                1 => "b",
                _ => "c",
            };
            p.note_endpoint(
                "/hot",
                Some(id),
                if i < 15 { Outcome::Risk } else { Outcome::Ok },
            );
        }
        let tripped: Option<i64> = p
            .conn()
            .unwrap()
            .query_row(
                "SELECT next_probe_at FROM breakers WHERE endpoint='/hot'",
                [],
                |r| r.get(0),
            )
            .optional()
            .unwrap();
        assert!(tripped.is_some());
        assert!(!p.probe_allowed("/hot").unwrap());
        assert!(p.probe_allowed("/feed").unwrap());
        p.conn()
            .unwrap()
            .execute(
                "UPDATE breakers SET next_probe_at=1 WHERE endpoint='/hot'",
                [],
            )
            .unwrap();
        assert!(p.probe_allowed("/hot").unwrap());
        let next: i64 = p
            .conn()
            .unwrap()
            .query_row(
                "SELECT next_probe_at FROM breakers WHERE endpoint='/hot'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(next >= now() + 55);
        p.note_endpoint("/hot", Some("a"), Outcome::Ok);
        let still: Option<i64> = p
            .conn()
            .unwrap()
            .query_row(
                "SELECT next_probe_at FROM breakers WHERE endpoint='/hot'",
                [],
                |r| r.get(0),
            )
            .optional()
            .unwrap();
        assert!(still.is_none());
    }

    #[test]
    fn one_identity_jitter_does_not_trip() {
        let p = pipe("breaker-one");
        for _ in 0..20 {
            p.note_endpoint("/hot", Some("a"), Outcome::Risk);
        }
        let got: Option<i64> = p
            .conn()
            .unwrap()
            .query_row(
                "SELECT next_probe_at FROM breakers WHERE endpoint='/hot'",
                [],
                |r| r.get(0),
            )
            .optional()
            .unwrap();
        assert!(got.is_none());
    }

    #[test]
    fn comments_complete_respects_target_and_known_total() {
        assert!(comments_complete(100, 100, 3863, false));
        assert!(!comments_complete(100, 53, 3863, true));
        assert!(comments_complete(100, 80, 80, true));
        assert!(comments_complete(0, 80, 80, true));
        assert!(!comments_complete(0, 20, 80, false));
    }

    #[test]
    fn classification_prefers_envelope_and_gone_is_free() {
        assert_eq!(
            classify_value(&serde_json::json!({"status_code":-99999,"status_msg":"gate"})),
            Outcome::Risk
        );
        assert_eq!(
            classify_value(&serde_json::json!({"filter_detail":{"filter_reason":"deleted"}})),
            Outcome::Gone
        );
        assert_eq!(classify_error(404, ""), Outcome::Gone);
        assert_eq!(classify_error(0, "timeout"), Outcome::Network);

        let p = pipe("identity");
        *p.last_identity.lock().unwrap() = Some("dev1".into());
        p.note_identity(Some("dev1"), Outcome::Risk, Some("gate"));
        let fails: i64 = p
            .conn()
            .unwrap()
            .query_row(
                "SELECT consecutive_fails FROM identity_health WHERE device_id='dev1'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(fails, 1);
        *p.last_identity.lock().unwrap() = Some("dev1".into());
        p.note_identity(Some("dev1"), Outcome::Gone, Some("deleted"));
        let fails: i64 = p
            .conn()
            .unwrap()
            .query_row(
                "SELECT consecutive_fails FROM identity_health WHERE device_id='dev1'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(fails, 1);
    }

    #[test]
    fn fetch_file_abort_keeps_part() {
        use std::io::{Read, Write};
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            let mut req = [0u8; 1024];
            let _ = sock.read(&mut req);
            let _ = sock.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 100000000\r\n\r\n");
            let chunk = vec![7u8; 64 * 1024];
            while sock.write_all(&chunk).is_ok() {
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        });

        let dir = std::env::temp_dir().join(format!("dy-fetch-{}", Uuid::new_v4()));
        let path = dir.join("a.mp4");
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        let url = format!("http://{addr}/a.mp4");
        let p2 = path.clone();
        let worker =
            std::thread::spawn(move || fetch_file(&url, &p2, || flag.load(Ordering::Relaxed)));
        std::thread::sleep(std::time::Duration::from_millis(300));
        stop.store(true, Ordering::Relaxed);
        let out = worker.join().unwrap().unwrap();
        assert!(matches!(out, Transfer::Aborted));
        assert!(!path.exists());
        let part = partial_path(&path);
        assert!(std::fs::metadata(&part).unwrap().len() > 0);
        drop(server);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn record_download_dedupes_by_room() {
        let p = pipe("record-dedupe");
        let req = || DownloadReq {
            target: Some("room:7400000000000000002".into()),
            target_key: None,
            asset_kind: Some("record".into()),
            url: Some("https://example/x.m3u8".into()),
            pinned: Some(true),
            title: Some("录制".into()),
            author: None,
            cover: None,
            ext: Some("m3u8".into()),
            expected_bytes: None,
            record_id: Some("plan-1".into()),
            parent_id: Some("plan-1".into()),
            params: None,
        };
        let a = p.submit_download(req()).unwrap();
        let b = p.submit_download(req()).unwrap();
        assert_eq!(a.id, b.id);
        assert_eq!(p.list_downloads(10).unwrap().len(), 1);
        let tasks: i64 = p
            .conn()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM tasks", [], |r| r.get(0))
            .unwrap();
        assert_eq!(tasks, 1);
    }

    #[test]
    fn pause_resume_cancel_download_states() {
        let p = pipe("pause");
        let dir = p.downloads_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("7400000000000000009.mp4");
        std::fs::write(partial_path(&path), vec![0u8; 16]).unwrap();
        let d = p
            .submit_download(DownloadReq {
                target: Some("video:7400000000000000009".into()),
                target_key: None,
                asset_kind: Some("video".into()),
                url: Some("https://example/x".into()),
                pinned: None,
                title: Some("样例".into()),
                author: Some("作者".into()),
                cover: None,
                ext: Some("mp4".into()),
                expected_bytes: None,
                record_id: None,
                parent_id: None,
                params: None,
            })
            .unwrap();
        assert_eq!(d.title.as_deref(), Some("样例"));
        p.conn()
            .unwrap()
            .execute(
                "UPDATE downloads SET state='running' WHERE id=?1",
                params![d.id],
            )
            .unwrap();
        assert_eq!(p.pause_download(&d.id).unwrap().state, "paused");
        assert!(partial_path(&path).exists());
        assert_eq!(p.resume_download(&d.id).unwrap().state, "queued");
        p.conn()
            .unwrap()
            .execute(
                "UPDATE downloads SET state='running' WHERE id=?1",
                params![d.id],
            )
            .unwrap();
        assert_eq!(p.cancel_download(&d.id).unwrap().state, "canceled");
        assert_eq!(p.retry_download(&d.id).unwrap().state, "queued");
        let got = p.get_download(&d.id).unwrap();
        assert_eq!(got.author.as_deref(), Some("作者"));
    }

    #[test]
    fn downloads_dedupe_pin_and_cleanup_keep_row() {
        let p = pipe("downloads");
        let a = p
            .submit_download(DownloadReq {
                target: Some("video:7400000000000000001".into()),
                target_key: None,
                asset_kind: Some("video".into()),
                url: Some("https://example/x".into()),
                pinned: None,
                title: None,
                author: None,
                cover: None,
                ext: Some("mp4".into()),
                expected_bytes: None,
                record_id: None,
                parent_id: None,
                params: None,
            })
            .unwrap();
        let b = p
            .submit_download(DownloadReq {
                target: Some("video:7400000000000000001".into()),
                target_key: None,
                asset_kind: Some("video".into()),
                url: None,
                pinned: None,
                title: None,
                author: None,
                cover: None,
                ext: Some("mp4".into()),
                expected_bytes: None,
                record_id: None,
                parent_id: None,
                params: None,
            })
            .unwrap();
        assert_eq!(a.id, b.id);
        assert_eq!(p.list_downloads(10).unwrap().len(), 1);
        p.conn()
            .unwrap()
            .execute(
                "INSERT INTO assets(id,target_key,kind,path,created_at) VALUES('asset-hit','video:7400000000000000001','video','/tmp/already.mp4',1)",
                [],
            )
            .unwrap();
        let c = p
            .submit_download(DownloadReq {
                target: Some("video:7400000000000000001".into()),
                target_key: None,
                asset_kind: Some("video".into()),
                url: None,
                pinned: None,
                title: None,
                author: None,
                cover: None,
                ext: Some("mp4".into()),
                expected_bytes: None,
                record_id: Some("record-hit".into()),
                parent_id: None,
                params: None,
            })
            .unwrap();
        assert_eq!(a.id, c.id);
        p.pin_download(&a.id, true).unwrap();

        let dir = p.downloads_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let c = p.conn().unwrap();
        let now = now();
        for (id, pinned, bytes, at) in [
            ("old", 0, 8, now - 20),
            ("keep", 1, 8, now - 10),
            ("new", 0, 8, now),
        ] {
            let path = dir.join(format!("{id}.mp4"));
            std::fs::write(&path, vec![0u8; 8]).unwrap();
            c.execute("INSERT INTO downloads(id,target_key,url,path,bytes,state,pinned,created_at) VALUES(?1,?2,'u',?3,?4,'done',?5,?6)",
                params![id, format!("video:{id}"), path.to_string_lossy(), bytes, pinned, at]).unwrap();
        }
        c.execute("INSERT INTO settings(key,value) VALUES('download_capacity_bytes','16') ON CONFLICT(key) DO UPDATE SET value='16'", []).unwrap();
        p.cleanup_downloads().unwrap();
        assert!(!dir.join("old.mp4").exists());
        assert!(dir.join("keep.mp4").exists());
        let states: Vec<(String, String)> = p
            .list_downloads(10)
            .unwrap()
            .into_iter()
            .filter(|x| x.id == "old" || x.id == "keep")
            .map(|x| (x.id, x.state))
            .collect();
        assert!(states.iter().any(|(id, st)| id == "old" && st == "gone"));
        assert!(states.iter().any(|(id, st)| id == "keep" && st == "done"));
    }

    #[test]
    fn recheck_marks_deleted_and_keeps_row() {
        let p = pipe("recheck");
        let c = p.conn().unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,metrics,state,created_at,last_checked) VALUES('r1','video:1','{}','live',10,0)", []).unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,metrics,state,created_at,last_checked) VALUES('r2','video:2','{}','live',20,5)", []).unwrap();
        let out = p.recheck(RecheckReq { limit: Some(1) }).unwrap();
        assert_eq!(out.accepted, 1);
        let task = p.claim().unwrap().unwrap();
        assert_eq!(task.endpoint, "/video/1");
        p.mark_snapshot(&task, true).unwrap();
        let rows = p
            .list_records(None, None, None, None, None, None, 10)
            .unwrap();
        assert_eq!(rows.len(), 2);
        let gone = rows.iter().find(|x| x.target_key == "video:1").unwrap();
        assert_eq!(gone.state, "deleted");
        assert!(gone.last_checked.unwrap() > 0);
    }

    #[test]
    fn plan_dispatch_backs_off_and_backfill_is_separate() {
        let p = pipe("plan");
        let clamped = p
            .add_plan(PlanReq {
                kind: Some("author_new".into()),
                target: Some("user:sec1".into()),
                target_key: None,
                interval_sec: Some(299),
                params: None,
                enabled: None,
            })
            .unwrap();
        assert_eq!(clamped.interval_sec, 600);
        p.del_plan(&clamped.id).unwrap();
        let w = p
            .add_plan(PlanReq {
                kind: Some("author_new".into()),
                target: Some("user:sec1".into()),
                target_key: None,
                interval_sec: Some(300),
                params: Some(serde_json::json!({"save_video": true, "comments": true})),
                enabled: None,
            })
            .unwrap();
        p.conn()
            .unwrap()
            .execute("UPDATE plans SET next_due_at=1", [])
            .unwrap();
        assert_eq!(p.dispatch_plans().unwrap(), 1);
        let task = p.claim().unwrap().unwrap();
        assert_eq!(task.kind, "collection");
        assert_eq!(task.params["save_video"], true);
        assert_eq!(task.params["comments"], true);
        assert_eq!(task.params["plan_id"], w.id);
        p.plan_result(&task, false, Some("gone"));
        let (streak, due): (i64, i64) = p
            .conn()
            .unwrap()
            .query_row(
                "SELECT fail_streak,next_due_at FROM plans WHERE id=?1",
                params![w.id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(streak, 1);
        assert!(due >= now() + 55);
        let before = due;
        p.submit_backfill(BackfillReq {
            target: Some("user:sec1".into()),
            pages: Some(2),
            params: None,
        })
        .unwrap();
        let after: i64 = p
            .conn()
            .unwrap()
            .query_row(
                "SELECT next_due_at FROM plans WHERE id=?1",
                params![w.id],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn interactive_collection_preempts_backfill() {
        let p = pipe("priority");
        let c = p.conn().unwrap();
        for i in 0..20 {
            c.execute(
                "INSERT INTO tasks(id,kind,endpoint,params,target_key,state,created_at) VALUES(?1,'backfill','/user/u/posts','{}',?2,'queued',?3)",
                params![format!("b{i}"), format!("user:u{i}"), 10 + i],
            ).unwrap();
        }
        c.execute(
            "INSERT INTO tasks(id,kind,endpoint,params,target_key,state,created_at) VALUES('hot','collection','/video/1','{}','video:1','queued',100)",
            [],
        ).unwrap();
        assert_eq!(p.claim().unwrap().unwrap().id, "hot");
        let second = p.claim().unwrap().unwrap();
        assert_eq!(second.kind, "backfill");
    }

    #[test]
    fn cancel_only_affects_active() {
        let p = pipe("cancel");
        let out = p
            .submit_one(Submit {
                target: Some("video:7400000000000000001".into()),
                link: None,
                kind: None,
                endpoint: None,
                params: None,
                max_attempts: None,
            })
            .unwrap();
        p.cancel(&out.id.unwrap()).unwrap();
        assert!(p.cancel("missing").is_err());
    }

    #[test]
    fn batch_keeps_bad_items_and_dedupes() {
        let p = pipe("batch");
        let mut items = Vec::new();
        for i in 0..47 {
            items.push(item(&format!("74000000000000000{i:02}")));
        }
        for i in 0..3 {
            items.push(Item {
                target: Some(format!("不是链接{i}")),
                link: None,
                kind: None,
                endpoint: None,
                params: None,
                max_attempts: None,
            });
        }
        let out = p
            .submit_batch(Batch {
                items,
                batch_id: None,
            })
            .unwrap();
        assert_eq!(out.results.len(), 50);
        assert_eq!(out.rejected, 3);
        assert_eq!(out.accepted, 47);
        assert!(out.results[47..]
            .iter()
            .all(|x| x.error_code.as_deref() == Some("invalid_target")));
        let conn = p.conn().unwrap();
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM tasks WHERE state='queued'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(n, 47);

        let first = out.results[0].id.clone().unwrap();
        let again = p
            .submit_one(Submit {
                target: Some("video:7400000000000000000".into()),
                link: None,
                kind: None,
                endpoint: None,
                params: None,
                max_attempts: None,
            })
            .unwrap();
        assert_eq!(again.id.as_deref(), Some(first.as_str()));
        assert!(again.deduped);
        let n: i64 = conn
            .query_row("SELECT COUNT(*) FROM tasks", [], |r| r.get(0))
            .unwrap();
        assert_eq!(n, 47);
    }

    #[test]
    fn asset_template_expands_and_collision_suffixes() {
        let p = pipe("template");
        let root = p.downloads_dir();
        let first = p.asset_path(
            "video",
            "video:7400000000000000011",
            "7400000000000000011",
            "标题:样例",
            "作者/甲",
            "mp4",
        );
        assert!(first.starts_with(&root), "{}", first.display());
        let text = first.to_string_lossy();
        assert!(text.contains("作者_甲"), "{text}");
        assert!(text.contains("7400000000000000011.mp4"), "{text}");
        std::fs::create_dir_all(first.parent().unwrap()).unwrap();
        std::fs::write(&first, b"x").unwrap();
        let second = p.asset_path(
            "video",
            "video:7400000000000000011",
            "7400000000000000011",
            "标题:样例",
            "作者/甲",
            "mp4",
        );
        assert!(
            second.to_string_lossy().ends_with(" (2).mp4"),
            "{}",
            second.display()
        );
    }

    #[test]
    fn record_template_gets_unique_session_dir() {
        let p = pipe("record-template");
        let first = p.asset_path(
            "record",
            "record:7704618956774083378:1793882589",
            "7704618956774083378",
            "标题",
            "作者",
            "mp4",
        );
        let text = first.to_string_lossy();
        assert!(text.contains("直播"), "{text}");
        assert!(text.ends_with("7704618956774083378_1793882589"), "{text}");
        std::fs::create_dir_all(&first).unwrap();
        let same = p.asset_path(
            "record",
            "record:7704618956774083378:1793882589",
            "7704618956774083378",
            "标题",
            "作者",
            "mp4",
        );
        assert!(
            same.to_string_lossy().ends_with("_1793882589 (2)"),
            "{}",
            same.display()
        );
        let other = p.asset_path(
            "record",
            "record:7704618956774083378:1793883000",
            "7704618956774083378",
            "标题",
            "作者",
            "mp4",
        );
        assert!(
            other
                .to_string_lossy()
                .ends_with("7704618956774083378_1793883000"),
            "{}",
            other.display()
        );
    }

    #[test]
    fn legacy_migration_dedupes_and_falls_back_in_place() {
        let p = pipe("migrate-ok");
        let dir = p.downloads_dir();
        let src = std::env::temp_dir().join(format!("dy-old-{}.mp4", Uuid::new_v4()));
        std::fs::write(&src, b"legacy").unwrap();
        let row = serde_json::json!({"id":"7400000000000000021","title":"旧片","author":"旧作者","path":src.to_string_lossy(),"at":1});
        let c = p.conn().unwrap();
        c.execute("INSERT INTO rows(bucket,id,json,ord) VALUES('lib:downloaded','7400000000000000021',?1,1)", params![row.to_string()]).unwrap();
        c.execute(
            "INSERT INTO rows(bucket,id,json,ord) VALUES('lib:downloaded','dup',?1,2)",
            params![row.to_string()],
        )
        .unwrap();
        c.execute("INSERT INTO downloads(id,target_key,asset_key,url,path,bytes,state,asset_kind,created_at) VALUES('d1','video:7400000000000000021','video:7400000000000000021','',?1,6,'done','video',2)", params![src.to_string_lossy()]).unwrap();
        drop(c);
        p.migrate_assets_once().unwrap();
        let count: i64 = p
            .conn()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM assets", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
        assert!(!src.exists());
        let moved: String = p
            .conn()
            .unwrap()
            .query_row("SELECT path FROM assets", [], |r| r.get(0))
            .unwrap();
        assert!(moved.starts_with(dir.to_string_lossy().as_ref()), "{moved}");

        let q = pipe("migrate-fallback");
        let blocked = std::env::temp_dir().join(format!("dy-block-{}", Uuid::new_v4()));
        std::fs::write(&blocked, b"not-dir").unwrap();
        q.conn()
            .unwrap()
            .execute(
                "UPDATE settings SET value=?1 WHERE key='main'",
                params![serde_json::json!({"downloadDir": blocked.to_string_lossy()}).to_string()],
            )
            .unwrap();
        let src2 = std::env::temp_dir().join(format!("dy-old-{}.mp4", Uuid::new_v4()));
        std::fs::write(&src2, b"legacy").unwrap();
        let row2 =
            serde_json::json!({"id":"7400000000000000022","path":src2.to_string_lossy(),"at":1});
        q.conn().unwrap().execute("INSERT INTO rows(bucket,id,json,ord) VALUES('lib:downloaded','7400000000000000022',?1,1)", params![row2.to_string()]).unwrap();
        q.migrate_assets_once().unwrap();
        let kept: String = q
            .conn()
            .unwrap()
            .query_row("SELECT path FROM assets", [], |r| r.get(0))
            .unwrap();
        assert_eq!(kept, src2.to_string_lossy());
        assert!(src2.exists());
    }

    #[test]
    fn paused_state_and_part_survive_restart() {
        let path = std::env::temp_dir().join(format!("dy-restart-{}.sqlite3", Uuid::new_v4()));
        let p = pipe_at(path.clone());
        let d = p
            .submit_download(DownloadReq {
                target: Some("video:7400000000000000031".into()),
                target_key: None,
                asset_kind: Some("video".into()),
                url: Some("https://example/x".into()),
                pinned: None,
                title: None,
                author: None,
                cover: None,
                ext: Some("mp4".into()),
                expected_bytes: None,
                record_id: None,
                parent_id: None,
                params: None,
            })
            .unwrap();
        let part = partial_path(std::path::Path::new(d.path.as_deref().unwrap()));
        std::fs::create_dir_all(part.parent().unwrap()).unwrap();
        std::fs::write(&part, b"part").unwrap();
        p.pause_download(&d.id).unwrap();
        drop(p);
        let reopened = Pipeline::new(path);
        assert_eq!(reopened.get_download(&d.id).unwrap().state, "paused");
        assert!(part.exists());
        assert_eq!(reopened.resume_download(&d.id).unwrap().state, "queued");
    }

    #[test]
    fn canceled_download_remains_stopped_after_restart() {
        let path = std::env::temp_dir().join(format!("dy-cancel-{}.sqlite3", Uuid::new_v4()));
        let p = pipe_at(path.clone());
        let d = p
            .submit_download(DownloadReq {
                target: Some("video:7400000000000000041".into()),
                target_key: None,
                asset_kind: Some("video".into()),
                url: Some("https://example/x".into()),
                pinned: None,
                title: None,
                author: None,
                cover: None,
                ext: Some("mp4".into()),
                expected_bytes: None,
                record_id: None,
                parent_id: None,
                params: None,
            })
            .unwrap();
        p.cancel_download(&d.id).unwrap();
        drop(p);
        let reopened = Pipeline::new(path);
        assert_eq!(reopened.get_download(&d.id).unwrap().state, "canceled");
        let queued: i64 = reopened
            .conn()
            .unwrap()
            .query_row(
                "SELECT COUNT(*) FROM tasks WHERE state IN ('queued','running')",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(queued, 0);
    }

    #[test]
    fn assets_page_by_cursor_and_mark_missing() {
        let p = pipe("assets-page");
        let dir = p.downloads_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let live = dir.join("live.mp4");
        std::fs::write(&live, b"x").unwrap();
        let c = p.conn().unwrap();
        for i in 0..3 {
            let path = if i == 2 {
                dir.join("missing.mp4")
            } else {
                dir.join(format!("f{i}.mp4"))
            };
            c.execute("INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES(?1,?2,'video',?3,1,?4)", params![format!("a{i}"), format!("video:{i}"), path.to_string_lossy(), 100 + i]).unwrap();
        }
        drop(c);
        let one = p
            .list_assets(&AssetFilter::default(), false, None, 2)
            .unwrap();
        assert_eq!(one.items.len(), 2);
        assert!(one.next.is_some());
        let two = p
            .list_assets(&AssetFilter::default(), false, one.next.as_deref(), 2)
            .unwrap();
        assert_eq!(two.items.len(), 1);
        assert!(two.items[0].missing);
        assert!(p.get_asset("a0").unwrap().missing);
    }

    #[test]
    fn dir_filter_accepts_windows_separators() {
        let p = pipe("assets-win-dir");
        for (id, path) in [("a", r"C:\media\a.mp4"), ("b", r"C:\media\nested\b.mp4")] {
            p.conn()
                .unwrap()
                .execute(
                    "INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES(?1,?2,'video',?3,1,1)",
                    params![id, format!("video:{id}"), path],
                )
                .unwrap();
        }
        let filter = AssetFilter {
            dir: Some(r"C:\media".into()),
            direct: Some("1".into()),
            ..AssetFilter::default()
        };
        assert_eq!(p.asset_summary_direct(&filter).unwrap(), 1);
        let items = p.list_assets_direct(&filter, true, None, 10).unwrap().items;
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "a");
    }

    #[test]
    fn direct_assets_page_and_summary_stay_in_current_directory() {
        let p = pipe("assets-direct-page");
        let dir = p.downloads_dir();
        let nested = dir.join("nested");
        std::fs::create_dir_all(&nested).unwrap();
        for (id, kind, path) in [
            ("a", "video", dir.join("a.mp4")),
            ("b", "video", dir.join("b.mp4")),
            ("c", "video", nested.join("c.mp4")),
            ("cover", "cover", dir.join("a.jpg")),
        ] {
            std::fs::write(&path, b"x").unwrap();
            p.conn()
                .unwrap()
                .execute(
                    "INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES(?1,?2,?3,?4,1,1)",
                    params![id, format!("video:{id}"), kind, path.to_string_lossy()],
                )
                .unwrap();
        }
        let filter = AssetFilter {
            dir: Some(dir.to_string_lossy().to_string()),
            direct: Some("1".into()),
            ..AssetFilter::default()
        };
        assert_eq!(p.asset_summary_direct(&filter).unwrap(), 2);
        let first = p.list_assets_direct(&filter, true, None, 1).unwrap();
        assert_eq!(first.items.len(), 1);
        assert_ne!(first.items[0].id, "c");
        let second = p
            .list_assets_direct(&filter, true, first.next.as_deref(), 1)
            .unwrap();
        assert_eq!(second.items.len(), 1);
        assert_ne!(second.items[0].id, "c");
        let deleted = p
            .asset_action(AssetActionReq {
                action: "delete".into(),
                ids: None,
                filter: Some(filter),
                file: Some("keep".into()),
            })
            .unwrap();
        assert_eq!(deleted.count, 2);
        assert!(p.get_asset("c").is_ok());
        assert!(p.get_asset("cover").is_ok());
    }

    #[test]
    fn asset_list_hides_cover_and_delete_removes_pair() {
        let p = pipe("assets-cover");
        let dir = p.downloads_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let video = dir.join(format!("cover-{}.mp4", Uuid::new_v4()));
        let cover = dir.join(format!("cover-{}.jpg", Uuid::new_v4()));
        std::fs::write(&video, b"v").unwrap();
        std::fs::write(&cover, b"c").unwrap();
        let c = p.conn().unwrap();
        for (id, kind, key, path) in [
            ("v", "video", "video:1", video.to_string_lossy().to_string()),
            (
                "c",
                "cover",
                "video:1:cover",
                cover.to_string_lossy().to_string(),
            ),
        ] {
            c.execute(
                "INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES(?1,?2,?3,?4,1,1)",
                params![id, key, kind, path],
            )
            .unwrap();
        }
        drop(c);
        let page = p
            .list_assets(&AssetFilter::default(), true, None, 20)
            .unwrap();
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.items[0].kind, "video");
        assert_eq!(
            page.items[0].cover.as_deref(),
            Some(cover.to_string_lossy().as_ref())
        );
        let covers = AssetFilter {
            kind: Some("cover".into()),
            ..AssetFilter::default()
        };
        assert_eq!(
            p.list_assets(&covers, true, None, 20).unwrap().items.len(),
            1
        );
        p.delete_asset("v", false).unwrap();
        let left: i64 = p
            .conn()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM assets WHERE kind='cover'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(left, 0);
        let _ = std::fs::remove_file(video);
        let _ = std::fs::remove_file(cover);
    }

    #[test]
    fn comments_page_returns_complete_tree() {
        let p = pipe("comments");
        let c = p.conn().unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,metrics,state,created_at) VALUES('r','video:1','{}','live',1)", []).unwrap();
        for (cid, parent, level) in [
            ("root", None, 1),
            ("child", Some("root"), 2),
            ("grand", Some("child"), 3),
        ] {
            c.execute("INSERT INTO captured_comments(record_id,cid,parent_cid,level,content,raw) VALUES('r',?1,?2,?3,'x','{}')", params![cid, parent, level]).unwrap();
        }
        drop(c);
        let page = p
            .list_record_comments("r", None, None, None, None, 100)
            .unwrap();
        assert_eq!(page.items.len(), 3);
        assert!(page.next.is_none());
        assert!(page
            .items
            .iter()
            .any(|x| x.cid == "grand" && x.parent_cid.as_deref() == Some("child")));
    }

    #[test]
    fn comment_search_is_flat_and_paginates() {
        let p = pipe("comment-search");
        let c = p.conn().unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,metrics,state,created_at) VALUES('r','video:1','{}','live',1)", []).unwrap();
        for (cid, author, content, at) in [
            ("a", "甲", "第一条 美食", 10),
            ("b", "乙", "第二条 旅行", 20),
            ("c", "丙", "第三条 美食", 30),
        ] {
            c.execute("INSERT INTO captured_comments(record_id,cid,level,author_name,content,created_at,raw) VALUES('r',?1,1,?2,?3,?4,'{}')", params![cid, author, content, at]).unwrap();
        }
        drop(c);
        let page = p
            .list_record_comments("r", None, None, None, Some("美食"), 100)
            .unwrap();
        assert_eq!(page.total, 2);
        assert_eq!(page.items.len(), 2);
        assert!(page.items.iter().all(|c| c.content.contains("美食")));
        let by_author = p
            .list_record_comments("r", None, None, None, Some("乙"), 100)
            .unwrap();
        assert_eq!(by_author.items.len(), 1);
        assert_eq!(by_author.items[0].cid, "b");
        let first = p
            .list_record_comments("r", None, None, None, Some("美食"), 1)
            .unwrap();
        assert_eq!(first.items.len(), 1);
        assert!(first.next.is_some());
    }

    #[test]
    fn record_snapshot_reuses_requested_record() {
        let p = pipe("record-reuse");
        let c = p.conn().unwrap();
        c.execute(
            "INSERT INTO capture_records(id,target_key,title,metrics,metrics_history,state,created_at) VALUES('old','video:1','旧标题','{\"comment_count\":2}','[{\"at\":1,\"metrics\":{\"comment_count\":2}}]','live',1)",
            [],
        )
        .unwrap();
        c.execute(
            "INSERT INTO tasks(id,kind,endpoint,params,state,attempts,created_at) VALUES('t-reuse','collection','/video/1','{\"record_id\":\"old\"}','done',1,2)",
            [],
        )
        .unwrap();
        drop(c);
        let task = p.get("t-reuse").unwrap();
        let value = serde_json::json!({
            "aweme_id": "1",
            "desc": "新标题",
            "statistics": {"comment_count": 3},
            "author": {"sec_uid": "u1", "nickname": "作者"}
        });
        assert_eq!(
            p.record_snapshot(&task, &value).unwrap().as_deref(),
            Some("old")
        );
        let row = p.get_record("old").unwrap();
        assert_eq!(row.title.as_deref(), Some("新标题"));
        assert_eq!(row.task_id.as_deref(), Some("t-reuse"));
        assert_eq!(row.metrics_history.as_array().unwrap().len(), 2);
    }

    #[test]
    fn record_list_and_summary_honor_filters() {
        let p = pipe("record-filter");
        let c = p.conn().unwrap();
        c.execute("INSERT INTO tasks(id,kind,endpoint,params,state,attempts,created_at) VALUES('t1','collection','/video/1','{\"comments\":true,\"max_comments\":100}','done',1,10)", []).unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,task_id,title,author_name,metrics,state,created_at) VALUES('a','video:1','t1','猫咪','甲','{}','live',10)", []).unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,title,author_name,metrics,state,created_at) VALUES('b','video:2','狗狗','乙','{}','deleted',20)", []).unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,record_id,path,created_at) VALUES('v1','video:1','video','a','/tmp/v1.mp4',10)", []).unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,record_id,path,created_at) VALUES('c1','video:1:cover','cover','a','/tmp/c1.jpg',10)", []).unwrap();
        drop(c);
        let rows = p
            .list_records(None, None, Some("猫咪"), None, None, None, 20)
            .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].id, "a");
        assert_eq!(rows[0].asset_count, 1);
        assert!(rows[0].comments_requested);
        assert_eq!(rows[0].comment_target, 100);
        assert_eq!(
            p.record_summary(None, None, Some("狗"), None, None)
                .unwrap()
                .count,
            1
        );
        assert_eq!(
            p.record_summary(Some("deleted"), None, None, None, None)
                .unwrap()
                .deleted,
            1
        );

        let first = p
            .list_records(None, None, None, None, None, None, 1)
            .unwrap();
        assert_eq!(first.len(), 1);
        let cursor = format!("{}|{}", first[0].created_at, first[0].id);
        let second = p
            .list_records(None, None, None, None, None, Some(&cursor), 1)
            .unwrap();
        assert_eq!(second.len(), 1);
        assert_ne!(second[0].id, first[0].id);
    }

    #[test]
    fn record_bulk_delete_honors_ids_and_filter() {
        let p = pipe("record-delete");
        let c = p.conn().unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,title,metrics,state,created_at) VALUES('a','video:1','猫咪','{}','live',10)", []).unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,title,metrics,state,created_at) VALUES('b','video:2','狗狗','{}','live',20)", []).unwrap();
        c.execute("INSERT INTO captured_comments(record_id,cid,level,content,raw) VALUES('a','c1',1,'x','{}')", []).unwrap();
        drop(c);
        let one = p
            .record_action(RecordActionReq {
                action: "delete".into(),
                ids: Some(vec!["a".into()]),
                state: None,
                kind: None,
                q: None,
                from: None,
                to: None,
            })
            .unwrap();
        assert_eq!(one.count, 1);
        assert_eq!(
            p.record_summary(None, None, None, None, None)
                .unwrap()
                .count,
            1
        );
        let rest = p
            .record_action(RecordActionReq {
                action: "delete".into(),
                ids: None,
                state: None,
                kind: None,
                q: Some("狗".into()),
                from: None,
                to: None,
            })
            .unwrap();
        assert_eq!(rest.count, 1);
        assert_eq!(
            p.record_summary(None, None, None, None, None)
                .unwrap()
                .count,
            0
        );
    }

    #[test]
    fn enabled_plan_is_unique_per_target() {
        let p = pipe("plan-dup");
        let a = p
            .add_plan(PlanReq {
                kind: Some("author_new".into()),
                target: Some("user:abc".into()),
                target_key: None,
                interval_sec: Some(300),
                params: None,
                enabled: None,
            })
            .unwrap();
        let b = p
            .add_plan(PlanReq {
                kind: Some("author_new".into()),
                target: Some("user:abc".into()),
                target_key: None,
                interval_sec: Some(600),
                params: None,
                enabled: None,
            })
            .unwrap();
        assert_eq!(a.id, b.id);
        p.set_plan_enabled(&a.id, false).unwrap();
        let c = p
            .add_plan(PlanReq {
                kind: Some("author_new".into()),
                target: Some("user:abc".into()),
                target_key: None,
                interval_sec: Some(300),
                params: None,
                enabled: None,
            })
            .unwrap();
        assert_ne!(a.id, c.id);
        assert!(p.set_plan_enabled(&a.id, true).is_err());
    }

    #[test]
    fn fetch_file_resumes_with_range_and_appends() {
        use std::io::{Read, Write};
        use std::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            let mut req = [0u8; 2048];
            let n = sock.read(&mut req).unwrap();
            let req = String::from_utf8_lossy(&req[..n]).to_ascii_lowercase();
            assert!(req.contains("range: bytes=10-"), "{req}");
            sock.write_all(b"HTTP/1.1 206 Partial Content\r\nContent-Range: bytes 10-14/15\r\nContent-Length: 5\r\n\r\n").unwrap();
            sock.write_all(b"56789").unwrap();
        });
        let dir = std::env::temp_dir().join(format!("dy-resume-{}", Uuid::new_v4()));
        let path = dir.join("a.mp4");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(partial_path(&path), b"0123456789").unwrap();
        let url = format!("http://{addr}/a.mp4");
        let out = fetch_file(&url, &path, || false).unwrap();
        assert!(matches!(out, Transfer::Done(15)));
        assert_eq!(std::fs::read(&path).unwrap(), b"012345678956789");
        server.join().unwrap();
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    #[ignore = "实际写入 1GB，手工验收时运行"]
    fn pause_and_resume_one_gigabyte() {
        use std::io::{Read, Write};
        use std::net::TcpListener;
        use std::sync::atomic::AtomicU64;
        use std::sync::Arc as StdArc;

        const TOTAL: u64 = 1 << 30;
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let served = StdArc::new(AtomicU64::new(0));
        let server = {
            let served = served.clone();
            std::thread::spawn(move || {
                for sock in listener.incoming().flatten() {
                    let mut sock = sock;
                    let mut req = [0u8; 2048];
                    let Ok(n) = sock.read(&mut req) else { continue };
                    let req = String::from_utf8_lossy(&req[..n]).to_ascii_lowercase();
                    let start = req
                        .lines()
                        .find_map(|l| l.strip_prefix("range: bytes="))
                        .and_then(|v| v.split('-').next())
                        .and_then(|v| v.parse::<u64>().ok())
                        .unwrap_or(0);
                    let code = if start > 0 {
                        "206 Partial Content"
                    } else {
                        "200 OK"
                    };
                    let head = if start > 0 {
                        format!("HTTP/1.1 {code}\r\nContent-Range: bytes {start}-{}/{} \r\nContent-Length: {}\r\n\r\n", TOTAL - 1, TOTAL, TOTAL - start)
                    } else {
                        format!("HTTP/1.1 {code}\r\nContent-Length: {TOTAL}\r\n\r\n")
                    };
                    if sock.write_all(head.as_bytes()).is_err() {
                        continue;
                    }
                    let mut i = start;
                    let mut chunk = vec![0u8; 64 * 1024];
                    while i < TOTAL {
                        let want = (TOTAL - i).min(chunk.len() as u64) as usize;
                        for (j, b) in chunk[..want].iter_mut().enumerate() {
                            *b = ((i as usize + j) % 251) as u8;
                        }
                        if sock.write_all(&chunk[..want]).is_err() {
                            break;
                        }
                        i += want as u64;
                        served.fetch_add(want as u64, Ordering::Relaxed);
                    }
                }
            })
        };
        let dir = std::env::temp_dir().join(format!("dy-1g-{}", Uuid::new_v4()));
        let path = dir.join("one-gigabyte.mp4");
        let url = format!("http://{addr}/one.mp4");
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        let p2 = path.clone();
        let first_url = url.clone();
        let first = std::thread::spawn(move || {
            fetch_file(&first_url, &p2, || flag.load(Ordering::Relaxed))
        });
        let until = Instant::now() + Duration::from_secs(30);
        while Instant::now() < until {
            if part_size(&path) >= 8 * 1024 * 1024 {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        stop.store(true, Ordering::Relaxed);
        let out = first.join().unwrap().unwrap();
        assert!(matches!(out, Transfer::Aborted));
        let paused = part_size(&path);
        assert!(paused >= 8 * 1024 * 1024, "{paused}");

        let out = fetch_file(&url, &path, || false).unwrap();
        assert!(matches!(out, Transfer::Done(n) if n == TOTAL as i64));
        assert_eq!(std::fs::metadata(&path).unwrap().len(), TOTAL);
        assert!(served.load(Ordering::Relaxed) >= TOTAL);
        drop(server);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn live_plan_respects_ten_minute_cooldown() {
        let p = pipe("live-cool");
        let plan = p
            .add_plan(PlanReq {
                kind: Some("live_auto".into()),
                target: Some("room:123456".into()),
                target_key: None,
                interval_sec: Some(60),
                params: None,
                enabled: None,
            })
            .unwrap();
        let c = p.conn().unwrap();
        c.execute("INSERT INTO record_sessions(id,target_key,room_id,dir,state,started_at,finished_at) VALUES('s','room:123456','123456','/tmp','done',?1,?2)", params![now() - 20, now() - 10]).unwrap();
        c.execute(
            "UPDATE plans SET next_due_at=1 WHERE id=?1",
            params![plan.id],
        )
        .unwrap();
        drop(c);
        assert_eq!(p.dispatch_plans().unwrap(), 0);
        let due: i64 = p
            .conn()
            .unwrap()
            .query_row(
                "SELECT next_due_at FROM plans WHERE id=?1",
                params![plan.id],
                |r| r.get(0),
            )
            .unwrap();
        assert!(due > now() + 30);
    }

    fn library_filter(kind: Option<&str>, q: Option<&str>) -> LibraryFilter {
        LibraryFilter {
            kind: kind.map(str::to_string),
            q: q.map(str::to_string),
            from: None,
            to: None,
        }
    }

    #[test]
    fn library_union_hides_cover_and_keeps_counts_by_kind() {
        let p = pipe("library-union");
        let dir = p.downloads_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let video = dir.join("video.mp4");
        let image = dir.join("image.jpg");
        let cover = dir.join("cover.jpg");
        let json = dir.join("record.jsonl");
        for path in [&video, &image, &cover, &json] {
            std::fs::write(path, b"x").unwrap();
        }
        let c = p.conn().unwrap();
        c.execute(
            "INSERT INTO tasks(id,kind,endpoint,params,state,attempts,created_at,output_path) VALUES('t','collection','/video/1','{\"comments\":true,\"max_comments\":10}','done',1,30,?1)",
            params![json.to_string_lossy()],
        )
        .unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES('v','video:1','video',?1,1,10)", params![video.to_string_lossy()]).unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES('i','video:2','gallery',?1,1,20)", params![image.to_string_lossy()]).unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES('c','video:1:cover','cover',?1,1,10)", params![cover.to_string_lossy()]).unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,task_id,title,author_name,metrics,state,created_at) VALUES('r','video:1','t','猫','甲','{}','live',30)", []).unwrap();
        drop(c);

        let all = p
            .list_library(&library_filter(None, None), None, 50)
            .unwrap();
        assert_eq!(all.items.len(), 3);
        assert!(all.items.iter().all(|x| x.asset_kind != "cover"));
        let summary = p.library_summary(&library_filter(None, None)).unwrap();
        assert_eq!(summary.total, 3);
        assert_eq!((summary.video, summary.image, summary.comment), (1, 1, 1));

        let videos = p
            .list_library(&library_filter(Some("video"), None), None, 50)
            .unwrap();
        assert_eq!(videos.items.len(), 1);
        assert_eq!(videos.items[0].id, "asset:v");
        let comments = p
            .list_library(&library_filter(Some("comment"), Some("甲")), None, 50)
            .unwrap();
        assert_eq!(comments.items.len(), 1);
        assert_eq!(comments.items[0].id, "record:r");
        assert_eq!(
            comments.items[0].path.as_deref(),
            Some(json.to_str().unwrap())
        );

        let first = p
            .list_library(&library_filter(None, None), None, 1)
            .unwrap();
        assert_eq!(first.items.len(), 1);
        let mut seen = vec![first.items[0].id.clone()];
        let mut cursor = first.next.clone();
        while let Some(next) = cursor {
            let page = p
                .list_library(&library_filter(None, None), Some(&next), 1)
                .unwrap();
            for item in &page.items {
                assert!(!seen.contains(&item.id), "cursor repeated {}", item.id);
                seen.push(item.id.clone());
            }
            cursor = page.next;
        }
        assert_eq!(seen.len(), 3);
    }

    #[test]
    fn library_delete_keeps_or_trashes_only_target_files() {
        let p = pipe("library-delete");
        let dir = p.downloads_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let video = dir.join("video.mp4");
        let cover = dir.join("cover.jpg");
        let json = dir.join("record.jsonl");
        for path in [&video, &cover, &json] {
            std::fs::write(path, b"x").unwrap();
        }
        let c = p.conn().unwrap();
        c.execute(
            "INSERT INTO tasks(id,kind,endpoint,params,state,attempts,created_at,output_path) VALUES('t','collection','/video/1','{\"comments\":true}','done',1,30,?1)",
            params![json.to_string_lossy()],
        )
        .unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES('i','video:2','gallery',?1,1,20)", params![dir.join("image.jpg").to_string_lossy()]).unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES('v','video:1','video',?1,1,10)", params![video.to_string_lossy()]).unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES('c','video:1:cover','cover',?1,1,10)", params![cover.to_string_lossy()]).unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,task_id,title,metrics,state,created_at) VALUES('r','video:1','t','猫','{}','live',30)", []).unwrap();
        drop(c);

        let one = p
            .library_action(LibraryActionReq {
                action: "delete".into(),
                ids: Some(vec!["asset:v".into()]),
                filter: None,
                file: Some("keep".into()),
            })
            .unwrap();
        assert_eq!(one.count, 1);
        assert!(video.is_file());
        assert!(cover.is_file());
        let covers: i64 = p
            .conn()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM assets WHERE kind='cover'", [], |r| {
                r.get(0)
            })
            .unwrap();
        assert_eq!(covers, 0);
        assert!(json.is_file());
        assert_eq!(
            p.library_summary(&library_filter(Some("comment"), None))
                .unwrap()
                .comment,
            1
        );

        let two = p
            .library_action(LibraryActionReq {
                action: "delete".into(),
                ids: Some(vec!["record:r".into()]),
                filter: None,
                file: Some("keep".into()),
            })
            .unwrap();
        assert_eq!(two.count, 1);
        assert!(json.is_file());
        assert_eq!(
            p.library_summary(&library_filter(None, None))
                .unwrap()
                .total,
            1
        );

        let three = p
            .library_action(LibraryActionReq {
                action: "delete".into(),
                ids: None,
                filter: Some(library_filter(None, None)),
                file: Some("trash".into()),
            })
            .unwrap();
        assert_eq!(three.count, 1);
        assert_eq!(
            p.library_summary(&library_filter(None, None))
                .unwrap()
                .total,
            0
        );
    }

    #[test]
    fn library_record_trash_moves_json_only() {
        let p = pipe("library-record-trash");
        let dir = p.downloads_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let json = dir.join("record.jsonl");
        let video = dir.join("video.mp4");
        std::fs::write(&json, b"x").unwrap();
        std::fs::write(&video, b"x").unwrap();
        let c = p.conn().unwrap();
        c.execute(
            "INSERT INTO tasks(id,kind,endpoint,params,state,attempts,created_at,output_path) VALUES('t','collection','/video/1','{}','done',1,30,?1)",
            params![json.to_string_lossy()],
        )
        .unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES('v','video:1','video',?1,1,10)", params![video.to_string_lossy()]).unwrap();
        c.execute("INSERT INTO capture_records(id,target_key,task_id,title,metrics,state,created_at) VALUES('r','video:1','t','猫','{}','live',30)", []).unwrap();
        drop(c);
        p.library_action(LibraryActionReq {
            action: "delete".into(),
            ids: Some(vec!["record:r".into()]),
            filter: None,
            file: Some("trash".into()),
        })
        .unwrap();
        assert!(!json.exists());
        assert!(video.is_file());
        assert_eq!(
            p.library_summary(&library_filter(None, None))
                .unwrap()
                .total,
            1
        );
    }

    #[test]
    fn library_asset_trash_moves_video_and_cover() {
        let p = pipe("library-asset-trash");
        let dir = p.downloads_dir();
        std::fs::create_dir_all(&dir).unwrap();
        let video = dir.join("video.mp4");
        let cover = dir.join("cover.jpg");
        std::fs::write(&video, b"x").unwrap();
        std::fs::write(&cover, b"x").unwrap();
        let c = p.conn().unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES('v','video:1','video',?1,1,10)", params![video.to_string_lossy()]).unwrap();
        c.execute("INSERT INTO assets(id,target_key,kind,path,size,created_at) VALUES('c','video:1:cover','cover',?1,1,10)", params![cover.to_string_lossy()]).unwrap();
        drop(c);
        p.library_action(LibraryActionReq {
            action: "delete".into(),
            ids: Some(vec!["asset:v".into()]),
            filter: None,
            file: Some("trash".into()),
        })
        .unwrap();
        assert!(!video.exists());
        assert!(!cover.exists());
        assert_eq!(
            p.library_summary(&library_filter(None, None))
                .unwrap()
                .total,
            0
        );
    }
}
