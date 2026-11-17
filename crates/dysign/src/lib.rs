//! 台子：Rust + Unicorn 跑 libmetasec_ml.so，进程内出信封头。
//!
//! 由 `crates/dysign/src/main.rs` 的探针演进而来。资源目录须含
//! `libmetasec_ml.so`、`rootfs/`、`supers.tsv.gz`、`device-seq-cold.jsonl`。

use std::collections::HashMap;
use std::io::Read;
use std::path::{Path, PathBuf};

use unidroid::{Emulator, Handle, JniCall, JniHandler, JniOp, JniValue, Trace};

pub mod encrypt;

pub const PKG: &str = "com.ss.android.ugc.aweme";
pub const APP_VERSION: &str = "32.2.6";
/// 无 `aid` 时的唯一回包值（Java 封装层的常量）。
pub const X_NEPTUNE: &str = "-8|50:51:59:20:21:30:40:47:49:39:22:29";
const FILES: &str = "/data/user/0/com.ss.android.ugc.aweme/files";
const MSDATA: &str = ".msdata";
const BRIDGE: &str = "ms/bd/c/m";
const SIG: &str = "a(IIJLjava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;";

struct Env {
    model: String,
    sdk: String,
    ctx: Handle,
    /// 最近一次 native 调用返回的数值（对象句柄在调用作用域内有效，须当场抄出）。
    last_num: Option<i64>,
    /// 设备侧裸指针 → 本机分配地址（重放期学到，签名期复用）。
    sub: HashMap<i64, i64>,
    /// 按 JNI cmd 累计的回调次数与耗时（诊断探针，每次签名开头清零）。
    spent: HashMap<i32, (u32, u128)>,
}

impl JniHandler for Env {
    fn import(&mut self, _emu: &mut Emulator, call: &unidroid::ImportCall<'_>) -> Option<u64> {
        let v = match call.name {
            "socket" => 42u64,
            "connect" | "setsockopt" | "fcntl" | "ioctl" => 0,
            "pthread_mutex_lock" | "pthread_mutex_unlock" => return None,
            _ => return None,
        };
        Some(v)
    }
    fn member(&mut self, emu: &mut Emulator, call: &JniCall<'_>) -> Option<JniValue> {
        if std::env::var("JNI_TRACE").is_ok() {
            eprintln!(
                "jni {} {}->{}{} static={} args={}",
                call.op.name(),
                call.class,
                call.name,
                call.signature,
                call.is_static,
                call.args.len()
            );
        }
        match (call.op, call.is_static, call.class, call.name) {
            (JniOp::Call, true, "com/bytedance/mobsec/metasec/ml/MS", "b") => {
                let cmd = call.args.first().and_then(as_i32).unwrap_or(0);
                let sel = call.args.get(1).and_then(as_i32).unwrap_or(0);
                let arg = call.args.get(2).and_then(as_i64).unwrap_or(0);
                let op = obj_arg(call.args.get(3));
                let obj = obj_arg(call.args.get(4));
                let r = self.dispatch(emu, cmd, sel, arg, op, obj);
                if let Some(JniValue::Object(h)) = &r {
                    if let Some(v) = boxed(emu, *h) {
                        self.last_num = Some(v);
                    }
                }
                r
            }
            (JniOp::Call, true, "java/lang/System", "getProperty") => {
                let key = arg_str(emu, call, 0)?;
                let v = match key.as_str() {
                    "ro.product.model" => self.model.clone(),
                    "ro.build.version.sdk" => self.sdk.clone(),
                    "java.vm.version" => "2.1.0".into(),
                    "os.arch" => "aarch64".into(),
                    "java.vm.name" => "Dalvik".into(),
                    _ => String::new(),
                };
                Some(emu.new_string(&v).into())
            }
            (JniOp::Call, true, "java/lang/Thread", "currentThread") => {
                Some(emu.new_object("java/lang/Thread", "main").into())
            }
            (JniOp::Call, false, "java/lang/Thread", "getStackTrace") => {
                let inner = emu.new_string("ak");
                Some(
                    emu.new_object_array("java/lang/StackTraceElement", &[inner])
                        .ok()?
                        .into(),
                )
            }
            (JniOp::Call, true, "java/lang/Thread", "getId") => Some(JniValue::Long(1)),
            (JniOp::Call, false, "java/lang/Thread", "getName") => {
                Some(emu.new_string("main").into())
            }
            (JniOp::Call, false, "java/lang/Thread", "toString") => {
                Some(emu.new_string("Thread[main,5,main]").into())
            }
            (JniOp::Call, true, "java/lang/Integer", "valueOf") => {
                let v = call.args.first().and_then(as_i32).unwrap_or(0);
                self.last_num = Some(v as i64);
                Some(emu.new_object("java/lang/Integer", v).into())
            }
            (JniOp::Call, true, "java/lang/Long", "valueOf") => {
                let v = call.args.first().and_then(as_i64).unwrap_or(0);
                Some(emu.new_object("java/lang/Long", v).into())
            }
            (JniOp::Call, true, "java/lang/Boolean", "valueOf") => {
                let v = matches!(call.args.first(), Some(JniValue::Boolean(true)));
                Some(emu.new_object("java/lang/Boolean", v).into())
            }
            (JniOp::Call, true, "java/lang/String", "valueOf") => {
                let v = call.args.first().and_then(as_i32).unwrap_or(0);
                Some(emu.new_string(&v.to_string()).into())
            }
            (JniOp::Call, false, "java/lang/Integer", "intValue") => emu
                .user_value::<i32>(call.receiver)
                .map(|v| JniValue::Int(*v)),
            (JniOp::Call, false, "java/lang/Long", "longValue") => emu
                .user_value::<i64>(call.receiver)
                .map(|v| JniValue::Long(*v)),
            (JniOp::Call, false, "java/lang/Boolean", "booleanValue") => emu
                .user_value::<bool>(call.receiver)
                .map(|v| JniValue::Boolean(*v)),
            (JniOp::Call, false, "android/content/Context", "getPackageName")
            | (JniOp::Call, false, "java/lang/String", "getPackageName") => {
                Some(emu.new_string(PKG).into())
            }
            (JniOp::Call, false, "android/content/Context", "getApplicationContext") => {
                Some(JniValue::Object(Handle(0)))
            }
            (JniOp::Call, false, "android/content/Context", "getFilesDir")
            | (JniOp::Call, false, "android/content/Context", "getDataDir") => {
                Some(emu.new_string(FILES).into())
            }
            (JniOp::Call, false, "android/content/Context", "getCacheDir") => {
                Some(emu.new_string(&format!("{FILES}/cache")).into())
            }
            _ => None,
        }
    }
}

/// 拆箱：native 用 Integer/Long 装箱返回的句柄取回数值。
fn boxed(emu: &Emulator, h: Handle) -> Option<i64> {
    match emu.unboxed(h).ok()? {
        JniValue::Int(x) => Some(x as i64),
        JniValue::Long(x) => Some(x),
        JniValue::Short(x) => Some(x as i64),
        JniValue::Byte(x) => Some(x as i64),
        _ => None,
    }
}

fn as_i32(v: &JniValue) -> Option<i32> {
    match v {
        JniValue::Int(i) => Some(*i),
        JniValue::Short(i) => Some(*i as i32),
        JniValue::Long(i) => Some(*i as i32),
        _ => None,
    }
}
fn as_i64(v: &JniValue) -> Option<i64> {
    match v {
        JniValue::Long(i) => Some(*i),
        JniValue::Int(i) => Some(*i as i64),
        _ => None,
    }
}
fn obj_arg(v: Option<&JniValue>) -> Option<Handle> {
    match v {
        Some(JniValue::Object(h)) if h.0 != 0 => Some(*h),
        _ => None,
    }
}
fn arg_str(emu: &Emulator, call: &JniCall<'_>, i: usize) -> Option<String> {
    match call.args.get(i) {
        Some(JniValue::Object(h)) if h.0 != 0 => emu.string(*h).ok(),
        _ => None,
    }
}

impl Env {
    fn dispatch(
        &mut self,
        emu: &mut Emulator,
        cmd: i32,
        sel: i32,
        arg: i64,
        op: Option<Handle>,
        obj: Option<Handle>,
    ) -> Option<JniValue> {
        let arg = self.sub.get(&arg).copied().unwrap_or(arg);
        if cmd == 0x10003 {
            return Some(emu.new_string(&format!("{FILES}/{MSDATA}")).into());
        }
        if cmd == 0x2000001 || cmd == 0x2000002 || cmd == 0x2000003 {
            return Some(emu.new_object("java/lang/Boolean", true).into());
        }
        if cmd == 0x1000011 {
            return Some(emu.new_string(APP_VERSION).into());
        }
        if cmd == 0x1000022 {
            return Some(JniValue::Object(Handle(0)));
        }
        if std::env::var("DYSIGN_CMD_TRACE").is_ok() {
            eprintln!("  [cbt] cmd=0x{cmd:x} sel={sel} arg=0x{arg:x}");
        }
        let t0 = std::time::Instant::now();
        let r = emu.call_static_jni_method(
            self,
            BRIDGE,
            SIG,
            &[
                JniValue::Int(cmd),
                JniValue::Int(sel),
                JniValue::Long(arg),
                JniValue::Object(op.unwrap_or(Handle(0))),
                JniValue::Object(obj.unwrap_or(Handle(0))),
            ],
        );
        let spent = self.spent.entry(cmd).or_insert((0, 0));
        *spent = (spent.0 + 1, spent.1 + t0.elapsed().as_nanos());
        match &r {
            Ok(JniValue::Object(h)) => {
                if let Some(v) = boxed(emu, *h) {
                    self.last_num = Some(v);
                }
            }
            Ok(JniValue::Int(x)) => self.last_num = Some(*x as i64),
            Ok(JniValue::Long(x)) => self.last_num = Some(*x),
            _ => {}
        }
        if std::env::var("DYSIGN_CMD_TRACE").is_ok() {
            let bv = match &r {
                Ok(JniValue::Object(h)) => boxed(emu, *h),
                _ => None,
            };
            eprintln!(
                "  [cbr] cmd=0x{cmd:x} sel={sel} -> {r:?} boxed={bv:?} last={:?}",
                self.last_num
            );
        }
        if let Err(e) = &r {
            if std::env::var("DYSIGN_DEBUG").is_ok() {
                eprintln!("  [cb] cmd=0x{cmd:x} 失败: {e}");
            }
        }
        r.ok()
    }
}

/// 一个台子实例。非 `Send`：调用方应在固定线程里持有。
pub struct Signer {
    emu: Emulator,
    env: Env,
    base: u64,
    traced: bool,
}

impl Signer {
    /// 载入资源、跑完冷启动序列。`res` 须为资源目录。
    pub fn new(res: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        unidroid::profile::validate_env()?;
        let mut gz = flate2::read::GzDecoder::new(std::fs::File::open(res.join("supers.tsv.gz"))?);
        let mut tsv = String::new();
        gz.read_to_string(&mut tsv)?;
        let mut supers = HashMap::new();
        for line in tsv.lines() {
            if let Some((k, v)) = line.split_once('\t') {
                supers.insert(k.to_string(), v.to_string());
            }
        }
        let mut builder = Emulator::builder();
        let cache_bytes = std::env::var("DYSIGN_CACHE_MB")
            .ok()
            .and_then(|mb| mb.parse::<u32>().ok())
            .map(|mb| mb * 1024 * 1024)
            .unwrap_or(64 * 1024 * 1024);
        let threads = match std::env::var("DYSIGN_THREADS").as_deref() {
            Ok("0") => false,
            Ok("1") => true,
            _ => std::env::var("DYSIGN_NOTHREADS").is_err(),
        };
        if let Ok(t) = std::env::var("DYSIGN_FIXED_TIME") {
            if let Ok(t) = t.parse() {
                builder = builder.fixed_clock(t);
            }
        }
        if let Ok(seed) = std::env::var("DYSIGN_RANDOM_SEED") {
            if let Ok(seed) = seed.parse() {
                builder = builder.random_seed(seed);
            }
        }
        let mut emu = builder
            .rootfs(res.join("rootfs"))
            .process_name(PKG)
            .system_property("ro.product.model", "2201123C")
            .system_property("ro.product.brand", "Xiaomi")
            .system_property("ro.build.version.sdk", "33")
            .translation_cache_size(cache_bytes)
            .trace(Trace::default())
            .threads(threads)
            .build()?;
        for (k, v) in supers.iter() {
            emu.superclass(k, v);
        }
        let lib = emu.load_library_without_constructors(res.join("libmetasec_ml.so"))?;
        if std::env::var("DYSIGN_COV").is_ok() {
            emu.set_trace(unidroid::Trace {
                coverage: vec!["libmetasec_ml.so".into()],
                ..Default::default()
            })?;
        }
        let mut env = Env {
            model: "2201123C".into(),
            sdk: "33".into(),
            ctx: Handle(0),
            last_num: None,
            sub: HashMap::new(),
            spent: HashMap::new(),
        };
        emu.run_constructors(lib, &mut env)?;
        emu.call_jni_onload(lib, &mut env)?;

        let base = emu
            .maps()?
            .into_iter()
            .find(|m| m.name.contains("libmetasec_ml.so"))
            .map(|m| m.start)
            .ok_or("找不到库映射")?;
        if std::env::var("DYSIGN_DEBUG").is_ok() {
            eprintln!("[base] 0x{base:x}");
            for m in emu.maps()? {
                eprintln!("[map] {:x}-{:x} {} {}", m.start, m.end, m.perms, m.name);
            }
        }
        patch_guard(&mut emu, base)?;
        emu.map_window(0x7e80_6000_0000, 0x7e80_7000_0000)?;
        env.ctx = emu.new_string("ctx");

        let text = std::fs::read_to_string(res.join("device-seq-cold.jsonl"))?;
        // 设备指针（如 0x4000002 的句柄）不能跨机复用：按录制里的返回值做「溯源替换」，
        // 与 Java 金标 replay 的 sub 表同口径。
        let limit: usize = std::env::var("REPLAY_N")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(usize::MAX);
        let mut done = 0usize;
        for line in text.lines() {
            if !line.contains("\"p\": \"call\"")
                || !line.contains("\"src\": \"ms.bd.c.m\"")
                || done >= limit
            {
                continue;
            }
            let v: serde_json::Value = serde_json::from_str(line)?;
            let cmd = match &v["cmd"] {
                serde_json::Value::Number(n) => n.as_i64().unwrap_or(0) as i32,
                serde_json::Value::String(s) => {
                    i64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap_or(0) as i32
                }
                _ => 0,
            };
            let sel = v["sel"].as_i64().unwrap_or(0) as i32;
            let arg = match &v["arg"] {
                serde_json::Value::String(s) => s.parse::<i64>().unwrap_or(0),
                serde_json::Value::Number(n) => n.as_i64().unwrap_or(0),
                _ => 0,
            };
            let op_s = v["op"].as_str().map(str::to_string);
            let obj_s = v["obj"].as_str().unwrap_or("null");
            let op = op_s.map(|s| emu.new_string(&s));
            let obj = if obj_s == "null" {
                None
            } else if let Some(hex) = obj_s
                .strip_prefix("bytes:")
                .map(|h| h.rsplit(':').next().unwrap_or(h))
            {
                let bytes = (0..hex.len() / 2)
                    .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16))
                    .collect::<Result<Vec<_>, _>>()?;
                Some(emu.new_byte_array(&bytes))
            } else if let Some(number) = obj_s.strip_prefix("num:") {
                Some(emu.new_object("java/lang/Integer", number.parse::<i32>().unwrap_or(0)))
            } else {
                Some(env.ctx)
            };
            // 替换只做一次，放在 dispatch 里：重放与签名期同口径。
            if std::env::var("DYSIGN_DEBUG").is_ok() {
                if let Some(v) = env.sub.get(&arg) {
                    eprintln!("  [sub] cmd=0x{cmd:x} arg=0x{arg:x} -> 0x{v:x}");
                }
            }
            env.last_num = None;
            let r = env.dispatch(&mut emu, cmd, sel, arg, op, obj);
            if std::env::var("DYSIGN_DEBUG").is_ok() {
                let exp = v["ret"].as_str().unwrap_or("");
                let got = match &r {
                    Some(JniValue::Long(x)) => format!("num:{x}"),
                    Some(JniValue::Int(x)) => format!("num:{x}"),
                    Some(JniValue::Object(_)) => "obj".into(),
                    Some(JniValue::Boolean(x)) => format!("bool:{x}"),
                    Some(JniValue::Void) => "null".into(),
                    None => "err".into(),
                    _ => "other".into(),
                };
                let same = (exp.starts_with("num:") && got == *exp)
                    || (exp.starts_with("str:") && got == "obj")
                    || (exp.starts_with("arr:") && got == "obj")
                    || (exp == "null" && got == "null");
                if !same {
                    eprintln!("  [replay] cmd=0x{cmd:x} arg=0x{arg:x} exp={exp} got={got}");
                }
            }
            let our: Option<i64> = env.last_num;
            if let Some(our) = our.filter(|x| *x != 0) {
                if let Some(exp) = v["ret"]
                    .as_str()
                    .and_then(|s| s.strip_prefix("num:"))
                    .and_then(|s| s.parse::<i64>().ok())
                    .filter(|e| *e != 0)
                {
                    if std::env::var("DYSIGN_DEBUG").is_ok() {
                        eprintln!("  [sub+] cmd=0x{cmd:x} dev=0x{exp:x} -> 0x{our:x}");
                    }
                    env.sub.insert(exp, our);
                }
            }
            done += 1;
        }
        dump_spent(&env.spent, "replay");
        Ok(Self {
            emu,
            env,
            base,
            traced: false,
        })
    }

    /// 切身份：device_id / iid 登记进库内状态。
    pub fn set_identity(
        &mut self,
        device_id: &str,
        iid: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (cmd, value) in [(0x2000002, device_id), (0x2000003, iid)] {
            let op = self.emu.new_string(value);
            let ctx = self.env.ctx;
            self.env
                .dispatch(&mut self.emu, cmd, 0, 0, Some(op), Some(ctx));
        }
        Ok(())
    }

    /// 出信封：`char* url, char* extra` → CRLF 分隔的 `name\\r\\nvalue` 串。
    pub fn sign(&mut self, url: &str, extra: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.env.spent.clear();
        if !self.traced {
            self.emu.hook_heap_writes()?;
            self.emu.hook_reads()?;
            self.emu.hook_args()?;
            self.emu.hook_ctrace()?;
            self.emu.hook_dump()?;
            self.emu.hook_patch()?;
            self.emu.hook_retlog()?;
            self.emu.hook_anchor()?;
            self.emu.hook_splog()?;
            self.emu.hook_snap_pc()?;
            self.emu.hook_execpages()?;
            if let Some(m) = self
                .emu
                .maps()?
                .into_iter()
                .find(|m| m.name.contains("libc.so"))
            {
                self.emu.hook_alloc(m.start)?;
            }
        }
        if !self.traced && std::env::var("DYSIGN_TRACE").is_ok() {
            let b = self.base;
            self.emu.hook_code_range(b + 0x33a90, b + 0x268ea8)?;
            self.traced = true;
        }
        let u = self.emu.malloc(url.len() + 1)?;
        self.emu.write_bytes(u, url.as_bytes())?;
        self.emu.write_bytes(u + url.len() as u64, &[0])?;
        let e = self.emu.malloc(extra.len() + 1)?;
        self.emu.write_bytes(e, extra.as_bytes())?;
        self.emu.write_bytes(e + extra.len() as u64, &[0])?;
        // 两参都是 C 串（Java 台子 cbModes=1 的口径）；std::string 形态（cbModes=2/3）实测返回 0。
        let t0 = std::time::Instant::now();
        let r = self
            .emu
            .call_function_with(&mut self.env, self.base + 0x16ad90, &[u, e])?;
        let call = t0.elapsed();
        if std::env::var("DYSIGN_DEBUG").is_ok() {
            eprintln!("  [ret] 0x{r:x} url=0x{u:x} extra=0x{e:x}");
        }
        if r == 0 {
            // 原生对「URL 无 aid」返回 0；Java 封装层（dex `ms.bd.c.m`）此时只吐一条
            // X-Neptune，值在 32.2.6 内恒定（真机抓包与台子逐字节一致）。照台子补上。
            return Ok(format!("X-Neptune\r\n{X_NEPTUNE}\r\n"));
        }
        let t1 = std::time::Instant::now();
        let mut out = Vec::new();
        for i in 0..8192u64 {
            let b = self.emu.read_bytes(r + i, 1)?;
            if b[0] == 0 {
                break;
            }
            out.push(b[0]);
        }
        let read = t1.elapsed();
        {
            eprintln!(
                "[phase] call={}ms read={}ms len={}",
                call.as_millis(),
                read.as_millis(),
                out.len()
            );
            let t = String::from_utf8_lossy(&out);
            let hv: Vec<(String, usize)> = t
                .split("\r\n")
                .collect::<Vec<_>>()
                .chunks(2)
                .map(|p| (p[0].to_string(), p.get(1).map(|v| v.len()).unwrap_or(0)))
                .collect();
            eprintln!(
                "  [heads] {:?}",
                hv.iter().map(|(k, v)| (k.clone(), *v)).collect::<Vec<_>>()
            );
            if let Ok(block) = std::str::from_utf8(&out) {
                eprintln!("  [hash] {:x}", md5ish(block.as_bytes()));
            }
            if std::env::var("DYSIGN_COV").is_ok() {
                let base = self.base;
                let mut c: Vec<_> = self
                    .emu
                    .coverage()
                    .into_iter()
                    .map(|b| (b.count, b.address))
                    .collect();
                c.sort_by_key(|x| std::cmp::Reverse(x.0));
                for (n, a) in c.iter().take(20) {
                    eprintln!("  [hot] 0x{:x} n={n}", a.wrapping_sub(base));
                }
                self.emu.reset_coverage();
            }
            dump_spent(&self.env.spent, "cmd");
        }
        Ok(String::from_utf8_lossy(&out).into_owned())
    }

    /// 直接写 guest 内存（锚点探针用）。
    pub fn poke(&mut self, addr: u64, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.emu.write_bytes(addr, bytes)?;
        Ok(())
    }
}

fn md5ish(b: &[u8]) -> u64 {
    let mut h: u64 = 1469598103934665603;
    for x in b {
        h ^= *x as u64;
        h = h.wrapping_mul(1099511628211);
    }
    h
}

/// 打印按 JNI cmd 累计的调用次数与耗时（诊断探针，取耗时前 10）。
fn dump_spent(spent: &HashMap<i32, (u32, u128)>, label: &str) {
    let mut v: Vec<_> = spent.iter().map(|(k, (n, ns))| (*k, *n, *ns)).collect();
    v.sort_by_key(|x| std::cmp::Reverse(x.2));
    for (cmd, n, ns) in v.iter().take(10) {
        eprintln!(
            "  [{label}] cmd=0x{cmd:x} n={n} total={:.1}ms avg={:.2}ms",
            *ns as f64 / 1e6,
            *ns as f64 / 1e6 / *n as f64
        );
    }
}
fn patch_guard(emu: &mut Emulator, base: u64) -> Result<(), Box<dyn std::error::Error>> {
    // libc++abi 的 guard 重入会 abort。Unicorn 的 LDARB/STLRB 不生效，换成 ldrb/strb，
    // 再把 __cxa_guard 的报错分支换成解锁并返回的桩。
    let mut stub_at = base + 0x370000;
    let stub: [u32; 11] = [
        0xd28a0800, 0xf2a206a0, 0xd2866801, 0xf2a20061, 0xd63f0020, 0xa9437bfd, 0xa9424ff4,
        0xa94157f6, 0xf84407f7, 0x52800020, 0xd65f03c0,
    ];
    let mut bytes = Vec::new();
    for w in stub {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    while emu.write_code(stub_at, &bytes).is_err() {
        stub_at += 0x10000;
        if stub_at > base + 0x800000 {
            return Err("找不到可写代码页".into());
        }
    }
    if std::env::var("DYSIGN_NOFMT").is_ok() {
        emu.patch_code(base + 0x4e648, &0xd65f03c0u32.to_le_bytes())?;
    }
    emu.patch_code(base + 0x250a28, &0x39400008u32.to_le_bytes())?;
    emu.patch_code(base + 0x250b8c, &0x39000015u32.to_le_bytes())?;
    let site = base + 0x250b2c;
    let delta = (stub_at as i64 - site as i64) >> 2;
    emu.patch_code(
        site,
        &(0x14000000u32 | ((delta as u32) & 0x03ff_ffff)).to_le_bytes(),
    )?;
    Ok(())
}

/// 默认资源目录（开发期 = crate resources；打包时由调用方给）。
pub fn default_resources() -> PathBuf {
    if let Ok(p) = std::env::var("DOUYINTAI_SIGN_RES") {
        return PathBuf::from(p);
    }
    let rel = PathBuf::from("crates/dysign/resources");
    if rel.join("libmetasec_ml.so").is_file() {
        return rel;
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources")
}
