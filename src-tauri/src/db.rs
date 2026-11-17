//! 本机 SQLite：设置、资料库、采集任务与计划。前端保留同步内存缓存，写入同时落库。
use std::path::PathBuf;

use rusqlite::{params, Connection};
use tauri::Manager;

pub fn path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("douyintai.sqlite3"))
}

pub(crate) fn open(app: &tauri::AppHandle) -> Result<Connection, String> {
    open_path(&path(app)?)
}

/// 打开指定库并补齐结构。迁移一次建齐后续表，新增只加列不改名。
pub(crate) fn open_path(path: &std::path::Path) -> Result<Connection, String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let backup = path.with_extension("sqlite3.bak");
    let version: i64 = std::fs::metadata(path)
        .ok()
        .and_then(|_| Connection::open(path).ok())
        .and_then(|c| c.query_row("PRAGMA user_version", [], |r| r.get(0)).ok())
        .unwrap_or(0);
    if version < 2 && path.exists() {
        std::fs::copy(path, &backup).map_err(|e| e.to_string())?;
    }
    let c = Connection::open(path).map_err(|e| e.to_string())?;
    c.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .map_err(|e| e.to_string())?;
    let migrated = c
        .execute_batch(
            "BEGIN IMMEDIATE;
         CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);
         CREATE TABLE IF NOT EXISTS rows (
           bucket TEXT NOT NULL, id TEXT NOT NULL, json TEXT NOT NULL, ord INTEGER NOT NULL DEFAULT 0,
           PRIMARY KEY (bucket, id)
         );
         CREATE INDEX IF NOT EXISTS rows_bucket_ord ON rows(bucket, ord DESC);
         CREATE TABLE IF NOT EXISTS tasks (
           id            TEXT PRIMARY KEY,
           kind          TEXT NOT NULL,
           endpoint      TEXT NOT NULL,
           params        TEXT NOT NULL,
           target_key    TEXT,
           state         TEXT NOT NULL,
           attempts      INTEGER NOT NULL DEFAULT 0,
           max_attempts  INTEGER NOT NULL DEFAULT 6,
           batch_id      TEXT,
           outcome       TEXT,
           error_code    TEXT,
           created_at    INTEGER NOT NULL,
           started_at    INTEGER,
           finished_at   INTEGER,
           request_id    TEXT,
           identity      TEXT,
           upstream_url  TEXT,
           retry_at      INTEGER,
           output_path   TEXT,
           source        TEXT,
           parent_id     TEXT,
           progress      REAL NOT NULL DEFAULT 0,
           priority      INTEGER NOT NULL DEFAULT 0,
           result_summary TEXT,
           acknowledged_at INTEGER,
           archived_at   INTEGER,
           deleted_at    INTEGER,
           keep_until    INTEGER,
           trigger       TEXT NOT NULL DEFAULT 'user',
           plan_id       TEXT
         );
         CREATE INDEX IF NOT EXISTS tasks_batch ON tasks(batch_id);
         CREATE INDEX IF NOT EXISTS tasks_state ON tasks(state, created_at);
         CREATE TABLE IF NOT EXISTS snapshots (
           id          INTEGER PRIMARY KEY,
           target_key  TEXT NOT NULL,
           aweme_id    TEXT,
           fetched_at  INTEGER NOT NULL,
           metrics     TEXT NOT NULL,
           state       TEXT NOT NULL DEFAULT 'live',
           last_checked INTEGER
         );
         CREATE INDEX IF NOT EXISTS snapshots_target_time ON snapshots(target_key, fetched_at);
         CREATE TABLE IF NOT EXISTS watches (
           id            TEXT PRIMARY KEY,
           kind          TEXT NOT NULL,
           target_key    TEXT NOT NULL,
           interval_sec  INTEGER NOT NULL,
           next_due_at   INTEGER NOT NULL,
           fail_streak   INTEGER NOT NULL DEFAULT 0,
           enabled       INTEGER NOT NULL DEFAULT 1,
           last_error    TEXT
         );
         CREATE INDEX IF NOT EXISTS watches_due ON watches(next_due_at);
         CREATE TABLE IF NOT EXISTS downloads (
           id          TEXT PRIMARY KEY,
           task_id     TEXT,
           target_key  TEXT NOT NULL,
           asset_key   TEXT NOT NULL DEFAULT '',
           url         TEXT NOT NULL,
           path        TEXT,
           bytes       INTEGER,
           state       TEXT NOT NULL,
           pinned      INTEGER NOT NULL DEFAULT 0,
           attempts    INTEGER NOT NULL DEFAULT 0,
           created_at  INTEGER NOT NULL,
           finished_at INTEGER,
           error       TEXT,
           title       TEXT,
           author       TEXT,
           asset_kind  TEXT NOT NULL DEFAULT 'video',
           ext         TEXT,
           expected_bytes INTEGER,
           partial_path TEXT,
           cover       TEXT,
           record_id   TEXT,
           asset_id    TEXT,
           progress    REAL NOT NULL DEFAULT 0,
           speed       INTEGER NOT NULL DEFAULT 0,
           priority    INTEGER NOT NULL DEFAULT 0,
           task_deleted_at INTEGER
         );
         CREATE UNIQUE INDEX IF NOT EXISTS downloads_target ON downloads(target_key);
         CREATE TABLE IF NOT EXISTS identity_health (
           device_id TEXT PRIMARY KEY,
           consecutive_fails INTEGER NOT NULL DEFAULT 0,
           cooled_until INTEGER,
           last_error TEXT
         );
         CREATE TABLE IF NOT EXISTS breaker_events (
           id INTEGER PRIMARY KEY,
           endpoint TEXT NOT NULL,
           identity TEXT NOT NULL,
           risk INTEGER NOT NULL,
           at INTEGER NOT NULL
         );
         CREATE INDEX IF NOT EXISTS breaker_events_at ON breaker_events(endpoint, at);
         CREATE TABLE IF NOT EXISTS breakers (
           endpoint TEXT PRIMARY KEY,
           tripped_at INTEGER NOT NULL,
           next_probe_at INTEGER NOT NULL
         );
         CREATE TABLE IF NOT EXISTS targets (
           key         TEXT PRIMARY KEY,
           kind        TEXT NOT NULL,
           sec_uid     TEXT,
           uid         TEXT,
           aweme_id    TEXT,
           room_id     TEXT,
           title       TEXT,
           nickname    TEXT,
           avatar      TEXT,
           cover       TEXT,
           updated_at  INTEGER NOT NULL
         );
         CREATE TABLE IF NOT EXISTS plans (
           id            TEXT PRIMARY KEY,
           kind          TEXT NOT NULL,
           target_key    TEXT NOT NULL,
           interval_sec  INTEGER NOT NULL,
           params        TEXT NOT NULL,
           enabled       INTEGER NOT NULL DEFAULT 1,
           next_due_at   INTEGER NOT NULL,
           fail_streak   INTEGER NOT NULL DEFAULT 0,
           last_task_id  TEXT,
           last_error    TEXT,
           created_at    INTEGER NOT NULL
         );
         CREATE UNIQUE INDEX IF NOT EXISTS plans_enabled_target ON plans(kind, target_key) WHERE enabled=1;
         CREATE INDEX IF NOT EXISTS plans_due ON plans(enabled, next_due_at);
         CREATE TABLE IF NOT EXISTS record_sessions (
           id          TEXT PRIMARY KEY,
           plan_id     TEXT,
           target_key  TEXT NOT NULL,
           room_id     TEXT,
           dir         TEXT NOT NULL,
           state       TEXT NOT NULL,
           bytes       INTEGER NOT NULL DEFAULT 0,
           max_bytes   INTEGER,
           max_secs    INTEGER,
           retain_bytes INTEGER,
           started_at  INTEGER NOT NULL,
           finished_at INTEGER,
           error       TEXT
         );
         CREATE UNIQUE INDEX IF NOT EXISTS record_sessions_active_room
           ON record_sessions(room_id) WHERE state IN ('queued','running');
         CREATE TABLE IF NOT EXISTS capture_records (
           id             TEXT PRIMARY KEY,
           target_key     TEXT NOT NULL,
           task_id        TEXT,
           title          TEXT,
           author_key     TEXT,
           author_name    TEXT,
           cover          TEXT,
           thumb_path     TEXT,
           metrics        TEXT NOT NULL,
           metrics_history TEXT NOT NULL DEFAULT '[]',
           state          TEXT NOT NULL,
           created_at     INTEGER NOT NULL,
           last_checked   INTEGER
         );
         CREATE INDEX IF NOT EXISTS capture_records_time ON capture_records(created_at DESC);
         CREATE INDEX IF NOT EXISTS capture_records_state ON capture_records(state, created_at DESC);
         CREATE TABLE IF NOT EXISTS captured_comments (
           record_id   TEXT NOT NULL,
           cid         TEXT NOT NULL,
           parent_cid  TEXT,
           level       INTEGER NOT NULL,
           author_key  TEXT,
           author_name TEXT,
           content     TEXT NOT NULL,
           likes       INTEGER,
           ip_label    TEXT,
           created_at  INTEGER,
           raw         TEXT NOT NULL,
           PRIMARY KEY (record_id, cid)
         );
         CREATE INDEX IF NOT EXISTS captured_comments_order ON captured_comments(record_id, level, created_at);
         CREATE TABLE IF NOT EXISTS assets (
           id          TEXT PRIMARY KEY,
           target_key  TEXT NOT NULL,
           kind        TEXT NOT NULL,
           task_id     TEXT,
           record_id   TEXT,
           title       TEXT,
           author      TEXT,
           cover       TEXT,
           path        TEXT NOT NULL,
           size        INTEGER,
           missing     INTEGER NOT NULL DEFAULT 0,
           created_at  INTEGER NOT NULL
         );
         CREATE UNIQUE INDEX IF NOT EXISTS assets_target ON assets(target_key);
         CREATE UNIQUE INDEX IF NOT EXISTS assets_path ON assets(path);
         CREATE INDEX IF NOT EXISTS assets_kind_time ON assets(kind, created_at DESC);
         CREATE INDEX IF NOT EXISTS assets_author_time ON assets(author, created_at DESC);
         CREATE INDEX IF NOT EXISTS assets_record ON assets(record_id);
         INSERT OR IGNORE INTO capture_records(id,target_key,task_id,title,author_name,metrics,state,created_at,last_checked)
           SELECT 'legacy-'||id,target_key,NULL,COALESCE(aweme_id,target_key),NULL,metrics,state,fetched_at,last_checked FROM snapshots;
         INSERT OR IGNORE INTO plans(id,kind,target_key,interval_sec,params,enabled,next_due_at,fail_streak,last_error,created_at)
           SELECT id,
             CASE WHEN kind IN ('live','live_auto') THEN 'live_auto' ELSE 'author_new' END,
             target_key,
             CASE WHEN kind IN ('live','live_auto') THEN MAX(interval_sec,60) ELSE MAX(interval_sec,300) END,
             '{}',enabled,next_due_at,fail_streak,last_error,strftime('%s','now') FROM watches;
         DROP TABLE IF EXISTS snapshots;
         DROP TABLE IF EXISTS watches;
         PRAGMA user_version=2;
         COMMIT;",
        );
    if let Err(e) = migrated {
        drop(c);
        if backup.exists() {
            let _ = std::fs::copy(&backup, path);
        }
        return Err(e.to_string());
    }
    for (name, ty) in [
        ("request_id", "TEXT"),
        ("identity", "TEXT"),
        ("upstream_url", "TEXT"),
        ("retry_at", "INTEGER"),
        ("output_path", "TEXT"),
        ("source", "TEXT"),
        ("parent_id", "TEXT"),
        ("progress", "REAL NOT NULL DEFAULT 0"),
        ("priority", "INTEGER NOT NULL DEFAULT 0"),
        ("result_summary", "TEXT"),
        ("acknowledged_at", "INTEGER"),
        ("archived_at", "INTEGER"),
        ("deleted_at", "INTEGER"),
        ("keep_until", "INTEGER"),
        ("trigger", "TEXT NOT NULL DEFAULT 'user'"),
        ("plan_id", "TEXT"),
    ] {
        let exists = c
            .prepare("PRAGMA table_info(tasks)")
            .and_then(|mut st| {
                st.query_map([], |r| r.get::<_, String>(1))
                    .map(|rows| rows.flatten().any(|x| x == name))
            })
            .unwrap_or(false);
        if !exists {
            let _ = c.execute(&format!("ALTER TABLE tasks ADD COLUMN {name} {ty}"), []);
        }
    }
    for sql in [
        "CREATE INDEX IF NOT EXISTS tasks_visible ON tasks(state, created_at, id)",
        "CREATE INDEX IF NOT EXISTS tasks_history ON tasks(created_at, id)",
        "CREATE INDEX IF NOT EXISTS tasks_plan ON tasks(plan_id, created_at)",
        "CREATE INDEX IF NOT EXISTS tasks_deleted ON tasks(deleted_at)",
    ] {
        let _ = c.execute(sql, []);
    }
    for (name, ty) in [
        ("finished_at", "INTEGER"),
        ("error", "TEXT"),
        ("title", "TEXT"),
        ("author", "TEXT"),
        ("asset_kind", "TEXT NOT NULL DEFAULT 'video'"),
        ("ext", "TEXT"),
        ("expected_bytes", "INTEGER"),
        ("asset_key", "TEXT NOT NULL DEFAULT ''"),
        ("partial_path", "TEXT"),
        ("cover", "TEXT"),
        ("record_id", "TEXT"),
        ("asset_id", "TEXT"),
        ("progress", "REAL NOT NULL DEFAULT 0"),
        ("speed", "INTEGER NOT NULL DEFAULT 0"),
        ("priority", "INTEGER NOT NULL DEFAULT 0"),
    ] {
        let exists = c
            .prepare("PRAGMA table_info(downloads)")
            .and_then(|mut st| {
                st.query_map([], |r| r.get::<_, String>(1))
                    .map(|rows| rows.flatten().any(|x| x == name))
            })
            .unwrap_or(false);
        if !exists {
            let _ = c.execute(&format!("ALTER TABLE downloads ADD COLUMN {name} {ty}"), []);
        }
    }
    let _ = c.execute(
        "UPDATE downloads SET asset_key=target_key WHERE asset_key IS NULL OR asset_key=''",
        [],
    );
    {
        let (name, ty) = ("task_deleted_at", "INTEGER");
        let exists = c
            .prepare("PRAGMA table_info(downloads)")
            .and_then(|mut st| {
                st.query_map([], |r| r.get::<_, String>(1))
                    .map(|rows| rows.flatten().any(|x| x == name))
            })
            .unwrap_or(false);
        if !exists {
            let _ = c.execute(&format!("ALTER TABLE downloads ADD COLUMN {name} {ty}"), []);
        }
    }
    {
        let (name, ty) = ("retain_bytes", "INTEGER");
        let exists = c
            .prepare("PRAGMA table_info(record_sessions)")
            .and_then(|mut st| {
                st.query_map([], |r| r.get::<_, String>(1))
                    .map(|rows| rows.flatten().any(|x| x == name))
            })
            .unwrap_or(false);
        if !exists {
            let _ = c.execute(
                &format!("ALTER TABLE record_sessions ADD COLUMN {name} {ty}"),
                [],
            );
        }
    }
    Ok(c)
}

#[tauri::command]
pub fn db_get_settings(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let c = open(&app)?;
    let mut st = c
        .prepare("SELECT value FROM settings WHERE key='main'")
        .map_err(|e| e.to_string())?;
    let mut rows = st.query([]).map_err(|e| e.to_string())?;
    rows.next()
        .map_err(|e| e.to_string())?
        .map(|r| r.get::<_, String>(0))
        .transpose()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn db_set_settings(app: tauri::AppHandle, json: String) -> Result<(), String> {
    open(&app)?
        .execute("INSERT INTO settings(key,value) VALUES('main',?1) ON CONFLICT(key) DO UPDATE SET value=excluded.value", params![json])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_json(app: &tauri::AppHandle, bucket: &str) -> Vec<String> {
    let Ok(c) = open(app) else { return Vec::new() };
    let Ok(mut st) = c.prepare("SELECT json FROM rows WHERE bucket=?1 ORDER BY ord DESC") else {
        return Vec::new();
    };
    let Ok(rows) = st.query_map(params![bucket], |r| r.get::<_, String>(0)) else {
        return Vec::new();
    };
    rows.flatten().collect()
}

pub fn put_json(
    app: &tauri::AppHandle,
    bucket: &str,
    id: &str,
    json: &str,
    ord: i64,
) -> Result<(), String> {
    open(app)?
        .execute("INSERT INTO rows(bucket,id,json,ord) VALUES(?1,?2,?3,?4) ON CONFLICT(bucket,id) DO UPDATE SET json=excluded.json, ord=excluded.ord", params![bucket, id, json, ord])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn del(app: &tauri::AppHandle, bucket: &str, id: &str) -> Result<(), String> {
    open(app)?
        .execute(
            "DELETE FROM rows WHERE bucket=?1 AND id=?2",
            params![bucket, id],
        )
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn db_load_rows(app: tauri::AppHandle, bucket: String) -> Result<Vec<String>, String> {
    Ok(load_json(&app, &bucket))
}

#[tauri::command]
pub fn db_put_row(
    app: tauri::AppHandle,
    bucket: String,
    id: String,
    json: String,
    ord: i64,
) -> Result<(), String> {
    put_json(&app, &bucket, &id, &json, ord)
}

#[tauri::command]
pub fn db_del_row(app: tauri::AppHandle, bucket: String, id: String) -> Result<(), String> {
    del(&app, &bucket, &id)
}

#[tauri::command]
pub fn db_clear_rows(app: tauri::AppHandle, bucket: String) -> Result<(), String> {
    open(&app)?
        .execute("DELETE FROM rows WHERE bucket=?1", params![bucket])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_snapshots_and_watches_migrate_to_new_tables() {
        let path =
            std::env::temp_dir().join(format!("dy-db-migrate-{}.sqlite3", uuid::Uuid::new_v4()));
        {
            let c = Connection::open(&path).unwrap();
            c.execute_batch(
                "CREATE TABLE snapshots (id INTEGER PRIMARY KEY,target_key TEXT NOT NULL,aweme_id TEXT,fetched_at INTEGER NOT NULL,metrics TEXT NOT NULL,state TEXT NOT NULL DEFAULT 'live',last_checked INTEGER);
                 CREATE TABLE watches (id TEXT PRIMARY KEY,kind TEXT NOT NULL,target_key TEXT NOT NULL,interval_sec INTEGER NOT NULL,next_due_at INTEGER NOT NULL,fail_streak INTEGER NOT NULL DEFAULT 0,enabled INTEGER NOT NULL DEFAULT 1,last_error TEXT);
                 INSERT INTO snapshots(id,target_key,aweme_id,fetched_at,metrics,state,last_checked) VALUES(1,'video:9','9',10,'{\"digg_count\":3}','live',0);
                 INSERT INTO watches(id,kind,target_key,interval_sec,next_due_at) VALUES('w1','author','user:sec',60,99);
                 PRAGMA user_version=1;",
            ).unwrap();
        }
        let c = open_path(&path).unwrap();
        let snap: (String, String) = c
            .query_row(
                "SELECT target_key,metrics FROM capture_records WHERE id='legacy-1'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(snap.0, "video:9");
        assert!(snap.1.contains("digg_count"));
        let plan: (String, i64) = c
            .query_row(
                "SELECT kind,interval_sec FROM plans WHERE id='w1'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        assert_eq!(plan, ("author_new".into(), 300));
        let legacy: i64 = c.query_row("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('snapshots','watches')", [], |r| r.get(0)).unwrap();
        assert_eq!(legacy, 0);
        assert!(path.with_extension("sqlite3.bak").exists());
        drop(c);
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(path.with_extension("sqlite3.bak"));
    }
}
