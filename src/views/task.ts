import { $, esc, toast } from "../lib/dom";
import { TASK } from "../lib/copy";
import { cancelTask, deleteTask, getTask, pauseTask, restoreTask, resumeTask, retryTask, taskAction, type Task } from "../lib/tasks";
import { back, go } from "../router";
import { invokeOr } from "../lib/store";
import type { View } from "../router";

function outcomeClass(t: Task): string {
  if (t.outcome === "ok") return "ok";
  if (t.outcome === "partial") return "partial";
  if (t.outcome === "gone") return "gone";
  if (t.outcome === "risk") return "risk";
  if (t.outcome === "network") return "network";
  if (t.outcome === "param" || t.error_code === "param") return "param";
  return "";
}

const ASSET_KINDS: Record<string, string> = { video: "视频下载", cover: "封面下载", music: "原声下载", gallery: "图集下载", record: "直播录制" };
function kindText(t: Task): string {
  return ASSET_KINDS[String(t.params?.asset_kind || "")] || TASK.kinds[t.kind] || t.kind;
}

const emptyRecord = (t: Task) => t.params?.asset_kind === "record" && t.state === "done" && !(t.bytes > 0);

function stateText(t: Task): string {
  if (emptyRecord(t)) return "空录制";
  if (t.params?.asset_kind === "record" && t.state === "done") return "录制完成";
  if (t.params?.asset_kind === "record" && t.state === "failed") return "录制失败";
  return TASK.states[t.state] || TASK.cancelledState;
}

function targetText(t: Task): string {
  const key = t.target_key || String(t.params?.target || "");
  const record = key.match(/^(?:download:)?record:(?:room:)?(\d+)/);
  if (record) return `直播间 ${record[1]}`;
  const video = key.match(/^(?:download:)?video:(\d+)/);
  if (video) return `作品 ${video[1]}`;
  return key || "—";
}

function commentResult(t: Task): string {
  const collected = Number(t.result_summary?.comments_collected);
  const target = Number(t.result_summary?.comments_target);
  const total = Number(t.result_summary?.comments_total);
  if (!Number.isFinite(collected)) return "";
  return TASK.commentsResult(collected, target, total);
}

function detail(t: Task): string {
  const outcome = t.outcome ? `<span class="outcome ${outcomeClass(t)}">${esc(TASK.outcomes[t.outcome] || t.outcome)}</span>` : "";
  const retry = t.state === "queued" && t.retry_at
    ? `<div class="notice wait">${esc(TASK.retryIn(Math.max(0, t.retry_at - Math.floor(Date.now() / 1000))))}</div>` : "";
  const fail = t.error_code || t.request_id || t.identity || t.upstream_url || t.endpoint ? `
    <details class="failbox" ${t.state === "failed" || t.state === "dead" ? "open" : ""}>
      <summary>${TASK.diagnostics}</summary>
      <dl>
        <dt>${TASK.fields.error}</dt><dd>${esc(t.error_code || "—")} ${outcome}</dd>
        <dt>${TASK.fields.request}</dt><dd class="mono">${esc(t.request_id || "—")}</dd>
        <dt>${TASK.fields.identity}</dt><dd class="mono">${esc(t.identity || "—")}</dd>
        <dt>${TASK.fields.upstream}</dt><dd class="mono wrap">${esc(t.upstream_url || t.endpoint)}</dd>
      </dl>
    </details>` : "";
  const when = (n: number | null) => n ? new Date(n * 1000).toLocaleString() : "—";
  const downloadId = typeof t.params?.download_id === "string" ? t.params.download_id : "";
  const timeline = `<div class="timeline"><div><b>创建</b><span>${when(t.created_at)}</span></div>${t.started_at ? `<div><b>开始</b><span>${when(t.started_at)}</span></div>` : ""}${t.finished_at ? `<div><b>结束</b><span>${when(t.finished_at)}</span></div>` : ""}${t.retry_at ? `<div><b>计划重试</b><span>${when(t.retry_at)}</span></div>` : ""}</div>`;
  return `
    <section class="view active taskdetail">
      <div class="pagehead between"><div><h1 class="h1">${esc(String(t.params?.display_title || t.target_key || TASK.detailTitle))}</h1><div class="sub">${esc(kindText(t))} · <span class="mono">${esc(t.id)}</span></div></div>
        <div class="row">${downloadId ? `<button class="btn sm ghost" data-act="download">下载详情</button>` : ""}${t.output_path ? `<button class="btn sm ghost" data-act="reveal">${TASK.reveal}</button>` : ""}<button class="btn sm ghost" data-act="back">${TASK.back}</button>${t.state === "queued" || t.state === "running" ? `<button class="btn sm ghost" data-act="pause">暂停</button>` : ""}${t.state === "paused" ? `<button class="btn sm ghost" data-act="resume">继续</button>` : ""}${t.state === "dead" || t.state === "failed" || t.state === "canceled" ? `<button class="btn sm ghost" data-act="retry">重试</button>` : ""}${t.state === "queued" || t.state === "running" || t.state === "paused" ? `<button class="btn sm danger" data-act="cancel">${TASK.cancel}</button>` : ""}${t.deleted_at ? `<button class="btn sm ghost" data-act="restore">恢复</button><button class="btn sm danger" data-act="purge">永久删除</button>` : ["done", "failed", "dead", "canceled"].includes(t.state) ? `<button class="btn sm danger" data-act="delete">删除</button>` : ""}</div></div>
      ${retry}
      ${t.deleted_at ? `<div class="notice warn">记录已在回收区，${Math.max(0, 7 - Math.floor((Date.now() / 1000 - t.deleted_at) / 86400))} 天后永久删除。</div>` : ""}
      ${t.keep_until ? `<div class="notice">已保留，自动清理跳过。</div>` : ""}
      <div class="detailcard">
        <dl class="taskmeta">
          <dt>${TASK.fields.kind}</dt><dd>${esc(kindText(t))}</dd>
          <dt>${TASK.fields.state}</dt><dd>${esc(stateText(t))}${emptyRecord(t) ? `<span class="dim"> · 清单可播但无分片</span>` : ""}</dd>
          <dt>${TASK.fields.target}</dt><dd>${esc(targetText(t))}</dd>
          ${commentResult(t) ? `<dt>${TASK.fields.commentsResult}</dt><dd>${esc(commentResult(t))}</dd>` : ""}
          <dt>${TASK.fields.attempts}</dt><dd>${t.attempts} / ${t.max_attempts}</dd>
          ${t.output_path ? `<dt>产出</dt><dd class="mono wrap">${esc(t.output_path)}</dd>` : ""}
        </dl>
        ${fail}
        <div class="sectionhead"><b>时间线</b></div>${timeline}
      </div>
    </section>`;
}

const view: View = {
  title: TASK.detailTitle,
  async render(ctx) {
    if (!ctx.arg) return `<div class="content"><div class="empty">${TASK.missing}</div></div>`;
    try { return detail(await getTask(ctx.arg)); }
    catch (e) { return `<div class="content"><div class="empty">${TASK.loadFail}：${esc(String(e))}</div></div>`; }
  },
  mount(ctx) {
    let timer = 0;
    const paint = async () => {
      const root = $("#content");
      if (!root) return;
      if (document.hidden) { timer = window.setTimeout(paint, 2000); return; }
      try {
        const t = await getTask(ctx.arg);
        if (ctx.signal.aborted) return;
        root.innerHTML = detail(t);
        if (t.state === "queued" || t.state === "running") timer = window.setTimeout(paint, 2000);
      } catch { /* 保留上一帧 */ }
    };
    document.addEventListener("click", (e) => {
      const el = (e.target as HTMLElement).closest<HTMLElement>("[data-act]");
      if (!el) return;
      const from = ctx.query.get("from");
      const fallback = from === "collect" ? "/collect" : from === "plans" ? "/plans" : from === "records" || from === "record" ? "/library?kind=comment" : from === "downloads" || from === "download" ? "/collect?kind=download" : "/tasks";
      if (el.dataset.act === "back") back(fallback);
      if (el.dataset.act === "reveal") void getTask(ctx.arg).then((t) => t.output_path && invokeOr("reveal", { path: t.output_path })).catch(() => {});
      if (el.dataset.act === "download") void getTask(ctx.arg).then((t) => { const id = t.params.download_id; if (id) go(`/download/${encodeURIComponent(String(id))}`); }).catch(() => {});
      if (el.dataset.act === "pause") void pauseTask(ctx.arg).then(() => { toast("已暂停"); void paint(); }).catch((err) => toast(String(err), "err"));
      if (el.dataset.act === "resume") void resumeTask(ctx.arg).then(() => { toast("已继续"); void paint(); }).catch((err) => toast(String(err), "err"));
      if (el.dataset.act === "retry") void retryTask(ctx.arg).then(() => { toast("已重新入队"); void paint(); }).catch((err) => toast(String(err), "err"));
      if (el.dataset.act === "cancel") void cancelTask(ctx.arg).then(() => { toast(TASK.canceled); void paint(); }).catch((err) => toast(String(err), "err"));
      if (el.dataset.act === "restore") void restoreTask(ctx.arg).then(() => { toast("已恢复"); void paint(); }).catch((err) => toast(String(err), "err"));
      if (el.dataset.act === "purge") {
        if (!window.confirm("永久删除这条任务记录？")) return;
        void taskAction("purge", { ids: [ctx.arg] }).then(() => { toast("已永久删除"); back(fallback); }).catch((err) => toast(String(err), "err"));
      }
      if (el.dataset.act === "delete") {
        const file = window.confirm("同时把产出文件移入回收站？\n确定：删除记录与文件；取消：仅删记录。");
        void deleteTask(ctx.arg, file ? "trash" : "keep").then(() => { toast("已删除"); back(fallback); }).catch((err) => toast(String(err), "err"));
      }
    }, { signal: ctx.signal });
    timer = window.setTimeout(paint, 2000);
    ctx.signal.addEventListener("abort", () => window.clearTimeout(timer));
  },
};
export default view;
