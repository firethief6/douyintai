import { $, bytes, esc, toast } from "../lib/dom";
import { cancelDownload, deleteDownload, getDownload, pauseDownload, resumeDownload, retryDownload, type Download } from "../lib/tasks";
import { invokeOr } from "../lib/store";
import { back, go, type View } from "../router";

const NAMES: Record<string, string> = { queued: "排队", running: "传输中", paused: "已暂停", done: "完成", failed: "失败", gone: "失效", canceled: "已取消" };
const when = (n: number | null) => n ? new Date(n * 1000).toLocaleString() : "—";
function html(d: Download): string {
  const empty = d.asset_kind === "record" && d.state === "done" && !(d.bytes && d.bytes > 0);
  return `<section class="view active"><div class="pagehead between"><div><h1 class="h1">${esc(d.title || d.target_key)}</h1><div class="sub">${esc(empty ? "空录制" : NAMES[d.state] || d.state)} · ${esc(d.author || "未知作者")}</div></div><div class="row"><button class="btn sm ghost" data-act="back">返回</button>${d.task_id ? `<button class="btn sm ghost" data-act="task">关联任务</button>` : ""}${d.path && d.state === "done" ? `<button class="btn sm ghost" data-act="reveal">目录</button>` : ""}${d.state === "queued" || d.state === "running" ? `<button class="btn sm ghost" data-act="pause">暂停</button>` : ""}${d.state === "paused" ? `<button class="btn sm ghost" data-act="resume">继续</button>` : ""}${d.state === "failed" || d.state === "gone" || d.state === "canceled" ? `<button class="btn sm ghost" data-act="retry">重试</button>` : ""}${["queued", "running", "paused"].includes(d.state) ? `<button class="btn sm danger" data-act="cancel">取消</button>` : `<button class="btn sm danger" data-act="delete">删除</button>`}</div></div>
    <div class="detailcard"><dl class="taskmeta"><dt>类型</dt><dd>${esc(d.asset_kind)}</dd><dt>状态</dt><dd>${esc(empty ? "空录制 · 无分片" : NAMES[d.state] || d.state)}</dd><dt>已传 / 预计</dt><dd>${d.bytes ? bytes(d.bytes) : "0 B"} / ${d.expected_bytes ? bytes(d.expected_bytes) : "—"}</dd><dt>速度</dt><dd>${d.speed ? `${bytes(d.speed)}/s` : "—"}</dd><dt>路径</dt><dd class="mono wrap">${esc(d.path || "—")}</dd><dt>临时文件</dt><dd class="mono wrap">${esc(d.partial_path || "—")}</dd><dt>错误</dt><dd>${esc(d.error || "—")}</dd></dl><div class="prog ${empty ? "empty" : ""}"><i style="width:${Math.round((d.progress || 0) * 100)}%"></i></div><div class="timeline"><div><b>创建</b><span>${when(d.created_at)}</span></div>${d.finished_at ? `<div><b>结束</b><span>${when(d.finished_at)}</span></div>` : ""}</div></div></section>`;
}
const view: View = {
  title: "下载详情",
  async render(ctx) { try { return html(await getDownload(ctx.arg)); } catch (e) { return `<div class="content"><div class="empty">${esc(String(e))}</div></div>`; } },
  mount(ctx) {
    let timer = 0;
    const paint = async () => { const root = $("#content"); if (!root) return; if (document.hidden) { timer = window.setTimeout(paint, 1500); return; } try { const d = await getDownload(ctx.arg); if (ctx.signal.aborted) return; root.innerHTML = html(d); if (["queued", "running", "paused"].includes(d.state)) timer = window.setTimeout(paint, 1500); } catch {} };
    document.addEventListener("click", async (e) => {
      const el = (e.target as HTMLElement).closest<HTMLElement>("[data-act]"); if (!el) return;
      try {
        if (el.dataset.act === "back") back("/collect?kind=download");
        if (el.dataset.act === "task") { const d = await getDownload(ctx.arg); if (d.task_id) go(`/task/${encodeURIComponent(d.task_id)}?from=collect`); }
        if (el.dataset.act === "reveal") { const d = await getDownload(ctx.arg); if (d.path) await invokeOr("reveal", { path: d.path }); }
        if (el.dataset.act === "pause") await pauseDownload(ctx.arg);
        if (el.dataset.act === "resume") await resumeDownload(ctx.arg);
        if (el.dataset.act === "retry") await retryDownload(ctx.arg);
        if (el.dataset.act === "cancel") { await cancelDownload(ctx.arg); toast("已取消"); }
        if (el.dataset.act === "delete") { const file = window.confirm("同时把文件移入回收站？确定删除文件与记录；取消仅删记录。"); await deleteDownload(ctx.arg, file ? "trash" : "keep"); back("/collect?kind=download"); }
        await paint();
      } catch (err) { toast(String(err), "err"); }
    }, { signal: ctx.signal });
    timer = window.setTimeout(paint, 1500);
    ctx.signal.addEventListener("abort", () => window.clearTimeout(timer));
  },
};
export default view;
