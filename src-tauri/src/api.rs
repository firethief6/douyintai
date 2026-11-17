use std::sync::Arc;

use serde::Serialize;
use serde_json::{Map, Value};

use dycore::api::Api;

use crate::pipeline::{
    AssetActionReq, AssetFilter, BackfillReq, Batch, DownloadReq, LibraryActionReq, LibraryFilter,
    Pipeline, PipelineErr, PlanReq, RecheckReq, RecordActionReq, Submit, TaskActionReq, TaskFilter,
};

#[derive(Debug, Serialize)]
pub struct ApiErr {
    pub status: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<&'static str>,
}

impl From<PipelineErr> for ApiErr {
    fn from(e: PipelineErr) -> Self {
        Self {
            status: e.status,
            message: format!("{}: {}", e.code, e.message),
            kind: None,
        }
    }
}

#[derive(Clone)]
pub struct Backend {
    pub api: Arc<Api>,
    pub pipeline: Arc<Pipeline>,
}

fn value_arg(body: &Option<Value>) -> &Value {
    body.as_ref().unwrap_or(&Value::Null)
}

fn query_str(query: &Map<String, Value>, key: &str) -> Option<String> {
    query.get(key).and_then(Value::as_str).map(str::to_string)
}

fn asset_filter(query: &Map<String, Value>) -> AssetFilter {
    AssetFilter {
        kind: query_str(query, "kind"),
        title: query_str(query, "title"),
        author: query_str(query, "author"),
        from: query
            .get("from")
            .and_then(Value::as_str)
            .and_then(|x| x.parse().ok()),
        to: query
            .get("to")
            .and_then(Value::as_str)
            .and_then(|x| x.parse().ok()),
        source: query_str(query, "source"),
        record: query_str(query, "record"),
        dir: query_str(query, "dir"),
        direct: query_str(query, "direct"),
    }
}

fn library_filter(query: &Map<String, Value>) -> LibraryFilter {
    LibraryFilter {
        kind: query_str(query, "kind"),
        q: query_str(query, "q"),
        from: query
            .get("from")
            .and_then(Value::as_str)
            .and_then(|x| x.parse().ok()),
        to: query
            .get("to")
            .and_then(Value::as_str)
            .and_then(|x| x.parse().ok()),
    }
}

fn pipeline_call(
    p: &Pipeline,
    method: &str,
    path: &str,
    query: &Map<String, Value>,
    body: &Option<Value>,
) -> Result<Value, PipelineErr> {
    let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
    if method == "POST" || method == "DELETE" || method == "PATCH" {
        return match (method, parts.as_slice()) {
            ("POST", ["task"]) => {
                let item: Submit =
                    serde_json::from_value(value_arg(body).clone()).map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                let out = p.submit_one(item)?;
                serde_json::to_value(out).map_err(|e| PipelineErr {
                    status: 500,
                    code: "json".into(),
                    message: e.to_string(),
                })
            }
            ("POST", ["tasks"]) => {
                let batch: Batch =
                    serde_json::from_value(value_arg(body).clone()).map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                let out = p.submit_batch(batch)?;
                Ok(serde_json::to_value(out).unwrap_or(Value::Null))
            }
            ("DELETE", ["task", id]) => {
                let trash = query.get("file").and_then(Value::as_str) == Some("trash");
                p.delete_task(id, trash)?;
                Ok(serde_json::json!({"id": id, "deleted": true}))
            }
            ("POST", ["assets", "action"]) => {
                let req: AssetActionReq =
                    serde_json::from_value(value_arg(body).clone()).map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                Ok(serde_json::to_value(p.asset_action(req)?).unwrap_or(Value::Null))
            }
            ("POST", ["records", "action"]) => {
                let req: RecordActionReq = serde_json::from_value(value_arg(body).clone())
                    .map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                Ok(serde_json::to_value(p.record_action(req)?).unwrap_or(Value::Null))
            }
            ("POST", ["library", "action"]) => {
                let req: LibraryActionReq = serde_json::from_value(value_arg(body).clone())
                    .map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                Ok(serde_json::to_value(p.library_action(req)?).unwrap_or(Value::Null))
            }
            ("POST", ["tasks", "action"]) => {
                let req: TaskActionReq =
                    serde_json::from_value(value_arg(body).clone()).map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                Ok(serde_json::to_value(p.task_action(req)?).unwrap_or(Value::Null))
            }
            ("POST", ["task", id, "ack"]) => {
                p.acknowledge_task(id)?;
                Ok(serde_json::json!({"id": id, "acknowledged": true}))
            }
            ("POST", ["task", id, "keep"]) => {
                let keep = value_arg(body)
                    .get("keep")
                    .and_then(Value::as_bool)
                    .unwrap_or(true);
                p.keep_task(id, keep)?;
                Ok(serde_json::json!({"id": id, "kept": keep}))
            }
            ("POST", ["task", id, "restore"]) => {
                p.restore_task(id)?;
                Ok(serde_json::json!({"id": id, "restored": true}))
            }
            ("POST", ["task", id, "cancel"]) => {
                p.cancel(id)?;
                Ok(serde_json::to_value(p.get(id)?).unwrap_or(Value::Null))
            }
            ("POST", ["task", id, "pause"]) => {
                Ok(serde_json::to_value(p.pause_task(id)?).unwrap_or(Value::Null))
            }
            ("POST", ["task", id, "resume"]) => {
                Ok(serde_json::to_value(p.resume_task(id)?).unwrap_or(Value::Null))
            }
            ("POST", ["task", id, "retry"]) => {
                Ok(serde_json::to_value(p.retry_task(id)?).unwrap_or(Value::Null))
            }
            ("POST", ["backfill"]) => {
                let req: BackfillReq =
                    serde_json::from_value(value_arg(body).clone()).map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                Ok(serde_json::to_value(p.submit_backfill(req)?).unwrap_or(Value::Null))
            }
            ("POST", ["download"]) => {
                let req: DownloadReq =
                    serde_json::from_value(value_arg(body).clone()).map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                Ok(serde_json::to_value(p.submit_download(req)?).unwrap_or(Value::Null))
            }
            ("POST", ["download", id, "retry"]) => {
                Ok(serde_json::to_value(p.retry_download(id)?).unwrap_or(Value::Null))
            }
            ("POST", ["download", id, "pause"]) => {
                Ok(serde_json::to_value(p.pause_download(id)?).unwrap_or(Value::Null))
            }
            ("POST", ["download", id, "resume"]) => {
                Ok(serde_json::to_value(p.resume_download(id)?).unwrap_or(Value::Null))
            }
            ("POST", ["download", id, "cancel"]) => {
                Ok(serde_json::to_value(p.cancel_download(id)?).unwrap_or(Value::Null))
            }
            ("DELETE", ["download", id]) => {
                let trash = query.get("file").and_then(Value::as_str) != Some("keep");
                p.delete_download(id, trash)?;
                Ok(serde_json::json!({"id": id, "deleted": true}))
            }
            ("POST", ["download", id, "pin"]) => {
                let pinned = value_arg(body)
                    .get("pinned")
                    .and_then(Value::as_bool)
                    .unwrap_or(true);
                Ok(serde_json::to_value(p.pin_download(id, pinned)?).unwrap_or(Value::Null))
            }
            ("POST", ["plans"]) => {
                let req: PlanReq =
                    serde_json::from_value(value_arg(body).clone()).map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                Ok(serde_json::to_value(p.add_plan(req)?).unwrap_or(Value::Null))
            }
            ("PATCH", ["plan", id]) => {
                let req: PlanReq =
                    serde_json::from_value(value_arg(body).clone()).map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                Ok(serde_json::to_value(p.update_plan(id, req)?).unwrap_or(Value::Null))
            }
            ("DELETE", ["plan", id]) => {
                p.del_plan(id)?;
                Ok(serde_json::json!({"id": id, "deleted": true}))
            }
            ("POST", ["plan", id, "pause"]) => {
                Ok(serde_json::to_value(p.set_plan_enabled(id, false)?).unwrap_or(Value::Null))
            }
            ("POST", ["plan", id, "resume"]) => {
                Ok(serde_json::to_value(p.set_plan_enabled(id, true)?).unwrap_or(Value::Null))
            }
            ("POST", ["record", id, "recheck"]) => {
                Ok(serde_json::to_value(p.recheck_record(id)?).unwrap_or(Value::Null))
            }
            ("DELETE", ["record", id]) => {
                p.delete_record(id)?;
                Ok(serde_json::json!({"id": id, "deleted": true}))
            }
            ("DELETE", ["asset", id]) => {
                let trash = query.get("file").and_then(Value::as_str) != Some("keep");
                p.delete_asset(id, trash)?;
                Ok(serde_json::json!({"id": id, "deleted": true}))
            }
            ("POST", ["recording", id, "stop"]) => {
                p.stop_recording(id)?;
                Ok(serde_json::json!({"id": id, "stopping": true}))
            }
            ("POST", ["recheck"]) => {
                let req: RecheckReq =
                    serde_json::from_value(value_arg(body).clone()).map_err(|e| PipelineErr {
                        status: 400,
                        code: "invalid_body".into(),
                        message: e.to_string(),
                    })?;
                Ok(serde_json::to_value(p.recheck(req)?).unwrap_or(Value::Null))
            }
            _ => Err(PipelineErr {
                status: 405,
                code: "method".into(),
                message: format!("{method} {path}"),
            }),
        };
    }
    match parts.as_slice() {
        ["task", id] => serde_json::to_value(p.get(id)?).map_err(|e| PipelineErr {
            status: 500,
            code: "json".into(),
            message: e.to_string(),
        }),
        ["plans"] => Ok(serde_json::json!({"items": p.list_plans()?})),
        ["plan", id] => Ok(serde_json::to_value(p.get_plan(id)?).unwrap_or(Value::Null)),
        ["records"] => {
            let state = query.get("state").and_then(Value::as_str);
            let kind = query.get("kind").and_then(Value::as_str);
            let q = query.get("q").and_then(Value::as_str);
            let from = query
                .get("from")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok());
            let to = query
                .get("to")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok());
            let cursor = query.get("cursor").and_then(Value::as_str);
            let limit = query
                .get("limit")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok())
                .unwrap_or(100);
            let items = p.list_records(state, kind, q, from, to, cursor, limit)?;
            let next = items.last().map(|r| format!("{}|{}", r.created_at, r.id));
            let full = items.len() == limit.clamp(1, 100);
            Ok(
                serde_json::json!({"items": items, "meta": {"next": if full { next } else { None }}}),
            )
        }
        ["record-summary"] => {
            let state = query.get("state").and_then(Value::as_str);
            let kind = query.get("kind").and_then(Value::as_str);
            let q = query.get("q").and_then(Value::as_str);
            let from = query
                .get("from")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok());
            let to = query
                .get("to")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok());
            serde_json::to_value(p.record_summary(state, kind, q, from, to)?).map_err(|e| {
                PipelineErr {
                    status: 500,
                    code: "json".into(),
                    message: e.to_string(),
                }
            })
        }
        ["record", id] => Ok(serde_json::to_value(p.get_record(id)?).unwrap_or(Value::Null)),
        ["record", id, "comments"] => {
            let cursor = query.get("cursor").and_then(Value::as_str);
            let level = query
                .get("level")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok());
            let parent = query.get("parent").and_then(Value::as_str);
            let q = query.get("q").and_then(Value::as_str);
            let limit = query
                .get("limit")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok())
                .unwrap_or(100);
            let page = p.list_record_comments(id, cursor, level, parent, q, limit)?;
            Ok(serde_json::json!({
                "items": page.items,
                "meta": {"next": page.next, "total": page.total}
            }))
        }
        ["assets"] => {
            let filter = asset_filter(query);
            let hide_cover = query.get("hide_cover").and_then(Value::as_str) == Some("1");
            let cursor = query.get("cursor").and_then(Value::as_str);
            let limit = query
                .get("limit")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok())
                .unwrap_or(100);
            let page = if query.get("direct").and_then(Value::as_str) == Some("1") {
                p.list_assets_direct(&filter, hide_cover, cursor, limit)?
            } else {
                p.list_assets(&filter, hide_cover, cursor, limit)?
            };
            Ok(serde_json::json!({"items": page.items, "meta": {"next": page.next}}))
        }
        ["library"] => {
            let filter = library_filter(query);
            let cursor = query.get("cursor").and_then(Value::as_str);
            let limit = query
                .get("limit")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok())
                .unwrap_or(100);
            let page = p.list_library(&filter, cursor, limit)?;
            Ok(serde_json::json!({"items": page.items, "meta": {"next": page.next}}))
        }
        ["library-summary"] => {
            let filter = library_filter(query);
            Ok(serde_json::to_value(p.library_summary(&filter)?).unwrap_or(Value::Null))
        }
        ["asset-summary"] => {
            let filter = asset_filter(query);
            let direct = filter.direct.as_deref() == Some("1");
            let count = if direct {
                p.asset_summary_direct(&filter)?
            } else {
                p.asset_summary(&filter)?
            };
            Ok(serde_json::json!({"count": count}))
        }
        ["assets", "dirs"] => Ok(serde_json::json!(
            p.asset_dirs(query.get("dir").and_then(Value::as_str))?
        )),
        ["asset", id] => Ok(serde_json::to_value(p.get_asset(id)?).unwrap_or(Value::Null)),
        ["asset", id, "images"] => Ok(serde_json::json!({"items": p.asset_images(id)?})),
        ["recordings"] => Ok(serde_json::json!({"items": p.list_recordings()?})),
        ["download", id] => Ok(serde_json::to_value(p.get_download(id)?).unwrap_or(Value::Null)),
        ["downloads"] => {
            let limit = query
                .get("limit")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok())
                .unwrap_or(200);
            Ok(serde_json::json!({"items": p.list_downloads(limit)?}))
        }
        ["stats"] => Ok(serde_json::to_value(p.stats()?).unwrap_or(Value::Null)),
        ["dashboard"] => Ok(serde_json::to_value(p.dashboard()?).unwrap_or(Value::Null)),
        ["tasks"] => {
            let scope = query.get("scope").and_then(Value::as_str);
            let filter = TaskFilter {
                kind: query
                    .get("kind")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                q: query.get("q").and_then(Value::as_str).map(str::to_string),
                source: query
                    .get("source")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                state: query
                    .get("state")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                batch_id: query
                    .get("batch")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                plan_id: query
                    .get("plan")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                kept: query.get("kept").and_then(Value::as_str).map(|v| v == "1"),
            };
            let cursor = query.get("cursor").and_then(Value::as_str);
            let limit = query
                .get("limit")
                .and_then(Value::as_str)
                .and_then(|x| x.parse().ok())
                .unwrap_or(50);
            let items = p.list(scope, Some(&filter), cursor, limit)?;
            let next = items.last().map(|t| format!("{}|{}", t.created_at, t.id));
            let full = items.len() == limit.clamp(1, 200);
            Ok(
                serde_json::json!({"items": items, "meta": {"next": if full { next } else { None }}}),
            )
        }
        ["task-summary"] => {
            let filter = TaskFilter {
                kind: query
                    .get("kind")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                q: query.get("q").and_then(Value::as_str).map(str::to_string),
                source: query
                    .get("source")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                state: query
                    .get("state")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                batch_id: query
                    .get("batch")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                plan_id: query
                    .get("plan")
                    .and_then(Value::as_str)
                    .map(str::to_string),
                kept: query.get("kept").and_then(Value::as_str).map(|v| v == "1"),
            };
            Ok(serde_json::to_value(p.task_summary(Some(&filter))?).unwrap_or(Value::Null))
        }
        _ => Err(PipelineErr {
            status: 404,
            code: "not_found".into(),
            message: path.into(),
        }),
    }
}

fn target_err(e: dycore::Error) -> ApiErr {
    ApiErr {
        status: e.status,
        message: e.message,
        kind: e.kind,
    }
}

fn target_resolve(api: &Api, target: &str) -> Result<Value, ApiErr> {
    let mut q = Map::new();
    q.insert("target".into(), Value::String(target.into()));
    let parsed = api
        .call("/resolve", &Value::Object(q))
        .map_err(target_err)?;
    let kind = parsed.get("type").and_then(Value::as_str).unwrap_or("");
    let id = parsed.get("id").and_then(Value::as_str).unwrap_or("");
    if id.is_empty() {
        return Err(ApiErr {
            kind: None,
            status: 400,
            message: "没有解析出目标".into(),
        });
    }
    let (card, detail) = match kind {
        "user" => {
            let mut q = Map::new();
            let user = api
                .call(&format!("/user/{id}"), &Value::Object(q.clone()))
                .map_err(target_err)?;
            q.clear();
            let uid = user
                .get("raw")
                .and_then(|v| v.get("sec_uid"))
                .and_then(Value::as_str)
                .filter(|v| !v.is_empty())
                .or_else(|| user.get("uid").and_then(Value::as_str))
                .unwrap_or(id);
            let card = serde_json::json!({
                "type": "user",
                "target_key": format!("user:{uid}"),
                "title": user.get("nickname").and_then(Value::as_str).unwrap_or(""),
                "subtitle": user.get("unique_id").and_then(Value::as_str).unwrap_or(""),
                "image": user.get("avatar").and_then(Value::as_str).unwrap_or(""),
                "uid": uid,
            });
            (card, user)
        }
        "video" => {
            let q = Map::new();
            let item = api
                .call(&format!("/video/{id}"), &Value::Object(q))
                .map_err(target_err)?;
            let author = item.get("author").cloned().unwrap_or(Value::Null);
            let card = serde_json::json!({
                "type": "video",
                "target_key": format!("video:{id}"),
                "title": item.get("desc").and_then(Value::as_str).unwrap_or(""),
                "subtitle": author.get("nickname").and_then(Value::as_str).unwrap_or(""),
                "image": item.get("cover").and_then(Value::as_str).unwrap_or(""),
                "aweme_id": id,
                "kind": item.get("kind").and_then(Value::as_str).unwrap_or("video"),
                "author": author,
            });
            (card, item)
        }
        "room" => {
            let mut q = Map::new();
            q.insert("target".into(), Value::String(format!("room:{id}")));
            let room = api
                .call("/live/resolve", &Value::Object(q))
                .map_err(target_err)?;
            let owner = room.get("owner").cloned().unwrap_or(Value::Null);
            let card = serde_json::json!({
                "type": "live",
                "target_key": format!("room:{id}"),
                "title": room.get("title").and_then(Value::as_str).unwrap_or("直播"),
                "subtitle": owner.get("nickname").and_then(Value::as_str).unwrap_or(""),
                "image": room.get("cover").and_then(Value::as_str).unwrap_or(""),
                "room_id": id,
                "author": owner,
            });
            (card, room)
        }
        _ => {
            return Err(ApiErr {
                kind: None,
                status: 400,
                message: format!("不支持的目标类型：{kind}"),
            })
        }
    };
    Ok(serde_json::json!({"item": card, "raw": detail}))
}

fn target_search(api: &Api, query: &str, kinds: &str) -> Result<Value, ApiErr> {
    let mut items = Vec::new();
    let want_user = kinds.is_empty() || kinds.split(',').any(|v| v.trim() == "user");
    let want_video = kinds.is_empty() || kinds.split(',').any(|v| v.trim() == "video");
    if want_user {
        let mut q = Map::new();
        q.insert("target".into(), Value::String(query.into()));
        q.insert("cursor".into(), Value::String(String::new()));
        q.insert("search_id".into(), Value::String(String::new()));
        let out = api
            .call("/search/users", &Value::Object(q))
            .map_err(target_err)?;
        for user in out
            .get("items")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .take(8)
        {
            let uid = user
                .get("sec_uid")
                .and_then(Value::as_str)
                .filter(|v| !v.is_empty())
                .or_else(|| user.get("uid").and_then(Value::as_str))
                .unwrap_or("");
            if uid.is_empty() {
                continue;
            }
            items.push(serde_json::json!({
                "type": "user",
                "target_key": format!("user:{uid}"),
                "title": user.get("nickname").and_then(Value::as_str).unwrap_or(""),
                "subtitle": user.get("unique_id").and_then(Value::as_str).unwrap_or(""),
                "image": user.get("avatar").and_then(Value::as_str).unwrap_or(""),
                "uid": uid,
            }));
        }
    }
    if want_video {
        let mut q = Map::new();
        q.insert("target".into(), Value::String(query.into()));
        q.insert("cursor".into(), Value::String(String::new()));
        q.insert("search_id".into(), Value::String(String::new()));
        q.insert("search_request_id".into(), Value::String(String::new()));
        let out = api.call("/search", &Value::Object(q)).map_err(target_err)?;
        for item in out
            .get("items")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .take(8)
        {
            let id = item.get("aweme_id").and_then(Value::as_str).unwrap_or("");
            if id.is_empty() {
                continue;
            }
            let author = item.get("author").cloned().unwrap_or(Value::Null);
            items.push(serde_json::json!({
                "type": "video",
                "target_key": format!("video:{id}"),
                "title": item.get("desc").and_then(Value::as_str).unwrap_or(""),
                "subtitle": author.get("nickname").and_then(Value::as_str).unwrap_or(""),
                "image": item.get("cover").and_then(Value::as_str).unwrap_or(""),
                "aweme_id": id,
                "kind": item.get("kind").and_then(Value::as_str).unwrap_or("video"),
                "author": author,
            }));
        }
    }
    Ok(serde_json::json!({"items": items}))
}

/// 当前签名实现与两边成败计数。
#[tauri::command]
pub fn sign_status(state: tauri::State<'_, Backend>) -> Value {
    serde_json::json!({"ok": true, "sign": state.api.sign_info().json()})
}

#[tauri::command]
pub fn identity_status(state: tauri::State<'_, Backend>) -> Value {
    serde_json::json!({"ok": true, "pool": state.api.identity_status()})
}

#[tauri::command]
pub fn identity_set_target(state: tauri::State<'_, Backend>, target: usize) -> Value {
    state.api.identity_set_target(target);
    serde_json::json!({"ok": true, "pool": state.api.identity_status()})
}

#[tauri::command]
pub fn identity_pause(state: tauri::State<'_, Backend>, paused: bool) -> Value {
    state.api.identity_pause(paused);
    serde_json::json!({"ok": true, "pool": state.api.identity_status()})
}

#[tauri::command]
pub fn identity_prepare(state: tauri::State<'_, Backend>) -> Value {
    state.api.identity_prepare();
    serde_json::json!({"ok": true, "pool": state.api.identity_status()})
}

#[tauri::command]
pub fn identity_clear(state: tauri::State<'_, Backend>) -> Value {
    state.api.identity_clear();
    serde_json::json!({"ok": true, "pool": state.api.identity_status()})
}

/// 应用出口代理列表与开关；持久化由前端设置一并写入。
#[tauri::command]
pub fn proxy_set(
    state: tauri::State<'_, Backend>,
    list: String,
    enabled: bool,
    cdn: bool,
) -> Value {
    state.api.set_proxies(&list, enabled);
    state.api.set_cdn_via_proxy(cdn);
    crate::net::set_cdn_proxy(state.api.cdn_proxy());
    let (count, on) = state.api.proxy_status();
    serde_json::json!({"ok": true, "count": count, "enabled": on, "cdn": cdn})
}

#[tauri::command]
pub fn proxy_get(state: tauri::State<'_, Backend>) -> Value {
    let (list, enabled, cdn) = state.api.proxy_config();
    crate::logging::info(
        "proxy",
        format!(
            "读取：{} 条 enabled={enabled} arc={:p}",
            list.lines().count(),
            Arc::as_ptr(&state.api)
        ),
    );
    serde_json::json!({"ok": true, "list": list, "enabled": enabled, "cdn": cdn})
}

#[tauri::command]
pub fn proxy_status(state: tauri::State<'_, Backend>) -> Value {
    let (count, on) = state.api.proxy_status();
    crate::logging::info("proxy", format!("状态查询：{count} 条 · {on}"));
    serde_json::json!({"ok": true, "count": count, "enabled": on, "cdn": state.api.cdn_proxy().is_some()})
}

/// 逐条实测代理，返回可用数与逐条结果；不改变已生效配置。
#[tauri::command]
pub async fn proxy_test(list: String) -> Value {
    let rows = tauri::async_runtime::spawn_blocking(move || Api::proxy_test(&list))
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|(url, ok, ms)| serde_json::json!({"url": url, "ok": ok, "ms": ms}))
        .collect::<Vec<_>>();
    let ok = rows.iter().filter(|r| r["ok"] == true).count();
    serde_json::json!({"ok": true, "usable": ok, "total": rows.len(), "rows": rows})
}

/// 运行时切换签名实现；持久化由前端设置一并写入。
#[tauri::command]
pub fn set_sign_backend(
    state: tauri::State<'_, Backend>,
    mode: String,
) -> std::result::Result<Value, String> {
    let backend =
        dycore::signer::Backend::parse(&mode).ok_or_else(|| format!("未知签名实现：{mode}"))?;
    state.api.set_sign_backend(backend);
    crate::logging::info("sign", format!("签名实现切换为 {}", backend.label()));
    Ok(serde_json::json!({"ok": true, "sign": state.api.sign_info().json()}))
}

#[tauri::command]
pub async fn api_call(
    state: tauri::State<'_, Backend>,
    _base_url: String,
    _token: String,
    path: String,
    query: std::collections::HashMap<String, String>,
    method: Option<String>,
    body: Option<Value>,
) -> Result<Value, ApiErr> {
    dispatch(
        Backend {
            api: state.api.clone(),
            pipeline: state.pipeline.clone(),
        },
        path,
        query,
        method,
        body,
    )
    .await
}

/// 与前端无关的分发核心；浏览器开发服务直接复用。
pub async fn dispatch(
    backend: Backend,
    path: String,
    query: std::collections::HashMap<String, String>,
    method: Option<String>,
    body: Option<Value>,
) -> Result<Value, ApiErr> {
    let p = path.clone();
    let started = std::time::Instant::now();
    let qsum = {
        let mut items: Vec<String> = query
            .iter()
            .map(|(k, v)| format!("{k}={}", v.chars().take(80).collect::<String>()))
            .collect();
        items.sort();
        let mut s = items.join("&");
        s.truncate(400);
        s
    };
    crate::logging::info("api", format!("→ {path} {qsum}"));
    let method = method.unwrap_or_else(|| "GET".into()).to_ascii_uppercase();
    let query_keys = {
        let mut keys: Vec<&str> = query.keys().map(String::as_str).collect();
        keys.sort_unstable();
        keys.join(",")
    };
    crate::logging::debug(
        "api",
        format!(
            "请求 method={method} path={path} query={query_keys} body={}B",
            body.as_ref()
                .and_then(|v| serde_json::to_vec(v).ok())
                .map(|v| v.len())
                .unwrap_or(0)
        ),
    );
    let mut q = Map::new();
    for (k, v) in query {
        q.insert(k, Value::String(v));
    }

    if path.trim_matches('/') == "disk_info" {
        let dir = q.get("dir").and_then(Value::as_str).unwrap_or("~/Videos");
        let (total, free) = crate::paths::capacity(&crate::paths::expand(dir));
        return Ok(serde_json::json!({"total": total, "free": free}));
    }
    if path.trim_matches('/') == "targets/resolve" {
        let api = backend.api.clone();
        let target = q
            .get("target")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim()
            .to_string();
        if target.is_empty() {
            return Err(ApiErr {
                kind: None,
                status: 400,
                message: "缺目标".into(),
            });
        }
        return match tauri::async_runtime::spawn_blocking(move || target_resolve(&api, &target))
            .await
        {
            Ok(Ok(v)) => Ok(v),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(ApiErr {
                kind: None,
                status: 0,
                message: e.to_string(),
            }),
        };
    }
    if path.trim_matches('/') == "targets/search" {
        let api = backend.api.clone();
        let query = q
            .get("q")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim()
            .to_string();
        let kinds = q
            .get("kinds")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        if query.is_empty() {
            return Ok(serde_json::json!({"items": []}));
        }
        return match tauri::async_runtime::spawn_blocking(move || {
            target_search(&api, &query, &kinds)
        })
        .await
        {
            Ok(Ok(v)) => Ok(v),
            Ok(Err(e)) => Err(e),
            Err(e) => Err(ApiErr {
                kind: None,
                status: 0,
                message: e.to_string(),
            }),
        };
    }

    let internal = matches!(
        path.trim_matches('/').split('/').next(),
        Some(
            "task"
                | "tasks"
                | "task-summary"
                | "backfill"
                | "recheck"
                | "download"
                | "downloads"
                | "stats"
                | "dashboard"
                | "plan"
                | "plans"
                | "record"
                | "records"
                | "record-summary"
                | "asset"
                | "assets"
                | "asset-summary"
                | "library"
                | "library-summary"
                | "recording"
                | "recordings"
        )
    );
    if internal {
        let pipe = backend.pipeline.clone();
        let p = path.clone();
        return match tauri::async_runtime::spawn_blocking(move || {
            pipeline_call(&pipe, &method, &p, &q, &body)
        })
        .await
        {
            Ok(Ok(v)) => Ok(v),
            Ok(Err(e)) => Err(e.into()),
            Err(e) => Err(ApiErr {
                kind: None,
                status: 0,
                message: e.to_string(),
            }),
        };
    }

    let api = backend.api.clone();
    let out = tauri::async_runtime::spawn_blocking(move || api.call(&p, &Value::Object(q)))
        .await
        .map_err(|e| ApiErr {
            kind: None,
            status: 0,
            message: e.to_string(),
        })?;
    match out {
        Ok(v) => {
            crate::logging::info(
                "api",
                format!("← {path} ok {}ms", started.elapsed().as_millis()),
            );
            crate::logging::debug(
                "api",
                format!(
                    "响应 method={method} path={path} status=200 {}B {}ms",
                    serde_json::to_vec(&v).map(|v| v.len()).unwrap_or(0),
                    started.elapsed().as_millis()
                ),
            );
            Ok(v)
        }
        Err(e) => {
            crate::logging::debug(
                "api",
                format!(
                    "响应 method={method} path={path} status={} kind={:?} {}ms",
                    e.status,
                    e.kind,
                    started.elapsed().as_millis()
                ),
            );
            crate::logging::error(
                "api",
                format!(
                    "← {path} {}ms status={} {}",
                    started.elapsed().as_millis(),
                    e.status,
                    e.message
                ),
            );
            Err(ApiErr {
                status: e.status,
                message: e.message,
                kind: e.kind,
            })
        }
    }
}
