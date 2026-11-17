import { $, ago, copy, empty, esc, nfmt, toast } from "../lib/dom";
import { deleteRecord, getAssetImages, getRecord, listAssets, listRecordComments, listRecords, recheckRecord, type Asset, type CaptureRecord, type RecordComment } from "../lib/tasks";
import { mediaSrc } from "../lib/api";
import { isTauri } from "../lib/store";
import { preview } from "../lib/preview";
import { go, type View } from "../router";
import { RECORD_META } from "../lib/copy";
import { commentLabel, commentsIncomplete, METRIC_LABELS, workCommentTotal } from "../lib/record-meta";

type RecordRow = CaptureRecord & { displayThumb?: string };
const titleOf = (r: CaptureRecord): string => {
  if (r.title && !/^\d+$/.test(r.title)) return r.title;
  const key = r.target_key || "";
  if (key.startsWith("video:")) return `作品 ID ${key.slice(6)}`;
  if (/^\d+$/.test(key)) return `作品 ID ${key}`;
  if (key.startsWith("room:")) return `直播间 ${key.slice(5)}`;
  return key || "未命名记录";
};
const avatar = (c: RecordComment): string => {
  const user = c.raw.user as Record<string, unknown> | undefined;
  const thumb = user?.avatar_thumb as { url_list?: string[] } | undefined;
  return thumb?.url_list?.[0] || "";
};
const metrics = (m: Record<string, unknown>) => Object.entries(m)
  .filter(([k, v]) => k !== "aweme_id" && (k !== "digest" || String(v).trim() !== "") && !["", "0"].includes(String(v).trim()))
  .map(([k, v]) => `<span><b>${esc(METRIC_LABELS[k] || k)}</b>${esc(String(v))}</span>`).join("");
const KIND_LABEL: Record<string, string> = { video: "视频", gallery: "图片", image: "图片", music: "音频", record: "直播录制" };

function exportText(r: CaptureRecord, comments: RecordComment[], assets: Asset[]): string {
  const cover = assets.find((a) => a.kind === "cover") || null;
  return JSON.stringify({ record: r, comments, assets: assets.filter((a) => a.kind !== "cover"), cover }, null, 2);
}

function highlight(text: string, query: string): string {
  const safe = esc(text);
  if (!query.trim()) return safe;
  const needle = esc(query).replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  return safe.replace(new RegExp(needle, "gi"), (m) => `<mark>${m}</mark>`);
}

export const recordsView: View = {
  title: "采集记录",
  render: () => `<section class="view active">
    <div class="pagehead between"><div><h1 class="h1">采集记录</h1><div class="sub">作品失效后仍保留本地正文、指标与评论</div></div><button class="btn sm ghost" data-act="refresh">刷新</button></div>
    <div class="assetbar"><input class="input" id="rq" type="search" placeholder="搜索标题或作者"><select class="select" id="rstate"><option value="">全部状态</option><option value="live">有效</option><option value="deleted">已失效</option></select></div>
    <div id="rlist"></div><div id="rsentinel" class="sentinel"></div>
  </section>`,
  async mount(ctx) {
    let rows: RecordRow[] = [], next = "", loading = false, seq = 0;
    const box = $("#rlist", ctx.content)!;
    const load = async (reset = false) => {
      if (loading) return;
      const mine = ++seq;
      loading = true;
      if (reset) { rows = []; next = ""; }
      try {
        const p = await listRecords({
          cursor: next,
          state: ($("#rstate", ctx.content) as HTMLSelectElement).value,
          q: ($("#rq", ctx.content) as HTMLInputElement).value.trim(),
          limit: 60,
        });
        if (mine !== seq) return;
        for (const r of p.items) {
          const src = r.thumb_path ? await mediaSrc(r.thumb_path).catch(() => "") : "";
          rows.push({ ...r, displayThumb: src });
        }
        next = p.meta.next || "";
        box.innerHTML = rows.length ? `<div class="recordgrid">${rows.map((r) => `<article class="recordcard" data-id="${esc(r.id)}"><div class="thumb">${r.displayThumb ? `<img src="${esc(r.displayThumb)}">` : r.cover ? `<img src="${esc(r.cover)}" loading="lazy">` : `<svg class="i"><use href="#i-json"/></svg>`}<span class="statechip ${r.state}">${r.state === "deleted" ? "已失效" : "有效"}</span></div><div class="meta"><b>${esc(titleOf(r))}</b><span>${esc(r.author_name || "未知作者")} · ${ago(r.created_at)}</span><span class="${commentsIncomplete(r) ? "commentmeta partial" : "commentmeta"}">${esc(commentLabel(r))}${RECORD_META.assets(r.asset_count)}</span></div></article>`).join("")}</div>` : empty("没有采集记录", "调整筛选，或从作品页创建采集任务。");
      } catch (e) { box.innerHTML = empty("读取失败", String(e)); }
      finally { if (mine === seq) loading = false; }
    };
    const io = new IntersectionObserver((e) => { if (e.some((x) => x.isIntersecting) && next) void load(); }, { rootMargin: "300px" });
    io.observe($("#rsentinel", ctx.content)!);
    ctx.signal.addEventListener("abort", () => io.disconnect());
    ctx.content.addEventListener("click", (e) => {
      const el = (e.target as HTMLElement).closest<HTMLElement>("[data-act],[data-id]");
      if (el?.dataset.act === "refresh") void load(true);
      else if (el?.dataset.id) go(`/record/${encodeURIComponent(el.dataset.id)}?from=records`);
    });
    $("#rstate", ctx.content)!.addEventListener("change", () => void load(true));
    let timer = 0;
    $("#rq", ctx.content)!.addEventListener("input", () => { window.clearTimeout(timer); timer = window.setTimeout(() => void load(true), 280); });
    void load(true);
  },
};

export const recordView: View = {
  title: "采集记录详情",
  async render(ctx) {
    if (!ctx.arg) return empty("缺记录 ID");
    try {
      const r = await getRecord(ctx.arg);
      return `<section class="view active recorddetail">
        <div class="pagehead between"><div><h1 class="h1">${esc(titleOf(r))}</h1><div class="sub">${esc(r.author_name || "未知作者")} · ${ago(r.created_at)} · ${r.state === "deleted" ? "已失效" : "有效"} · <span class="${commentsIncomplete(r) ? "commentmeta partial" : "commentmeta"}">${esc(commentLabel(r))}</span></div></div><div class="row"><button class="btn sm ghost" data-act="rback">返回</button>${r.target_key.startsWith("video:") ? `<button class="btn sm ghost" data-act="open">原作品</button>` : ""}<button class="btn sm ghost" data-act="open-json" ${r.output_path ? "" : "disabled"}>打开 JSON</button><button class="btn sm ghost" data-act="copy">复制 JSON</button><button class="btn sm ghost" data-act="export">导出</button><button class="btn sm ghost" data-act="recheck">重新复查</button><button class="btn sm ghost" data-act="recollect">重新采集</button><button class="btn sm danger" data-act="delete">删除</button></div></div>
        <div class="metricrow">${metrics(r.metrics) || `<span>${r.target_key.startsWith("video:") ? "作品 ID " + esc(r.target_key.slice(6)) : "采集时无指标"}</span>`}</div>
        <div class="sectionhead"><b>复查时间线</b>${r.task_id ? `<button class="btn sm ghost" data-act="task">关联任务</button>` : ""}</div>
        <div class="timeline">${r.metrics_history.length ? r.metrics_history.map((h) => `<div><span>${ago(h.at)}</span><div class="metricrow">${metrics(h.metrics)}</div></div>`).join("") : `<div class="dim">尚无复查记录${r.last_checked ? ` · 最近复查 ${ago(r.last_checked)}` : ""}</div>`}</div>
        <div id="rcontent"><div id="rassets"></div>
          <section class="commentpanel"><div class="sectionhead"><b>评论</b><span class="dim" id="cstat"></span><span class="grow"></span></div>
            <div class="commenttools"><label class="searchmini"><svg class="i"><use href="#i-search"/></svg><input id="cq" type="search" placeholder="搜索评论内容或作者" aria-label="搜索评论"></label><button class="btn sm ghost" data-act="clear-csearch" hidden>清除</button></div>
            <div id="clist"></div><div id="cmore" class="loadmore"></div>
          </section>
        </div>
      </section>`;
    } catch (e) { return empty("记录读取失败", String(e)); }
  },
  async mount(ctx) {
    let comments: RecordComment[] = [], next = "", total = 0, loading = false, seq = 0, query = "";
    let assets: Asset[] = [];
    let previewable: Asset[] = [];
    const clist = $("#clist", ctx.content)!;
    const cmore = $("#cmore", ctx.content)!;
    const cstat = $("#cstat", ctx.content)!;
    const cq = $("#cq", ctx.content) as HTMLInputElement;
    const clearSearch = $("[data-act=clear-csearch]", ctx.content) as HTMLButtonElement;
    const record = await getRecord(ctx.arg);
    assets = (await listAssets({ record: ctx.arg, limit: 100 })).items;
    previewable = assets.filter((a) => a.kind !== "cover" && KIND_LABEL[a.kind]);

    const paintAssets = async () => {
      const host = $("#rassets", ctx.content)!;
      if (!previewable.length) { host.innerHTML = `<div class="sectionhead"><b>产出文件</b></div><div class="dim assetnone">没有可预览的产出文件，正文与评论仍保留在这里。</div>`; return; }
      const cards = await Promise.all(previewable.map(async (a, i) => {
        const src = a.cover || (a.kind !== "gallery" && !a.path.toLowerCase().endsWith(".zip") ? a.path : "");
        const thumb = src ? await mediaSrc(src).catch(() => "") : "";
        return `<button class="rasset" data-pv="${i}"><span class="rthumb">${thumb ? `<img src="${esc(thumb)}" loading="lazy" alt="">` : `<svg class="i"><use href="#i-image"/></svg>`}<em>${esc(KIND_LABEL[a.kind] || a.kind)}</em></span><b>${esc(a.title || a.path.split("/").at(-1) || a.kind)}</b></button>`;
      }));
      host.innerHTML = `<div class="sectionhead"><b>产出文件</b><span class="dim">${previewable.length} 项</span></div><div class="recordassets">${cards.join("")}</div>`;
    };

    const tree = () => {
      const top = comments.filter((c) => !c.parent_cid);
      const kids = (cid: string): RecordComment[] => comments.filter((c) => c.parent_cid === cid);
      const row = (c: RecordComment, child = false): string => {
        const replies = kids(c.cid);
        return `<div class="comment ${child ? "child" : ""}" data-cid="${esc(c.cid)}"><div class="chead">${avatar(c) ? `<img class="cavatar" src="${esc(avatar(c))}" loading="lazy" alt="">` : ""}<b>${highlight(c.author_name || "未知", query)}</b><span>${c.ip_label ? esc(c.ip_label) + " · " : ""}${c.created_at ? ago(c.created_at) : ""} · ${c.likes ?? 0} 赞${child ? ` · 第 ${c.level} 级` : ""}</span><button class="btn sm ghost" data-copy-c="${esc(c.cid)}">复制</button>${replies.length ? `<button class="btn sm ghost" data-fold="${esc(c.cid)}">收起 ${replies.length}</button>` : ""}</div><p>${highlight(c.content, query)}</p>${replies.length ? `<div class="children">${replies.map((x) => row(x, true)).join("")}</div>` : ""}</div>`;
      };
      return top.map((c) => row(c)).join("");
    };
    const flat = () => comments.map((c) => `<div class="comment" data-cid="${esc(c.cid)}"><div class="chead">${avatar(c) ? `<img class="cavatar" src="${esc(avatar(c))}" loading="lazy" alt="">` : ""}<b>${highlight(c.author_name || "未知", query)}</b><span>${c.created_at ? ago(c.created_at) : ""}${c.parent_cid ? " · 回复" : ""}</span><button class="btn sm ghost" data-copy-c="${esc(c.cid)}">复制</button></div><p>${highlight(c.content, query)}</p></div>`).join("");

    const paintComments = () => {
      const off = !record.comments_requested;
      const works = workCommentTotal(record);
      cstat.textContent = off
        ? `未采集评论${works ? ` · 作品 ${nfmt(works)} 条` : ""}`
        : query ? `匹配 ${total} 条 · 已载入 ${comments.length} 条` : `已采 ${total} 条${record.comment_target ? ` / 目标 ${record.comment_target}` : ""}${commentsIncomplete(record) ? " · 部分" : ""}`;
      clist.innerHTML = comments.length ? (query ? flat() : tree()) : empty(
        query ? "没有匹配的评论" : off ? "未采集评论" : "没有评论",
        query ? "换个关键词，可按内容或作者搜索。" : off ? RECORD_META.offHint(nfmt(works)) : "上游没有返回评论。",
        query ? `<button class="btn sm ghost" data-act="clear-csearch">清除搜索</button>` : "",
      );
      cmore.innerHTML = next ? `<button class="btn sm" data-more>加载更多${query ? `（已载入 ${comments.length}/${total}）` : ""}</button>` : "";
      clearSearch.hidden = !query;
    };

    const load = async (reset = false) => {
      if (loading && !reset) return;
      const mine = ++seq;
      loading = true;
      if (reset) { comments = []; next = ""; total = 0; clist.innerHTML = ""; }
      try {
        const p = await listRecordComments(ctx.arg, reset ? "" : next, query, 100);
        if (mine !== seq) return;
        if (reset) comments = p.items; else comments.push(...p.items);
        next = p.meta.next || "";
        total = p.meta.total ?? comments.length;
        paintComments();
      } catch (e) { toast(String(e), "err"); }
      finally { if (mine === seq) loading = false; }
    };

    let timer = 0;
    cq.addEventListener("input", () => {
      window.clearTimeout(timer);
      timer = window.setTimeout(() => { query = cq.value.trim(); void load(true); }, 260);
    });
    cq.addEventListener("keydown", (e) => { if (e.key === "Escape") { cq.value = ""; query = ""; void load(true); } });

    const json = {
      record,
      comments,
      assets: assets.filter((a) => a.kind !== "cover"),
      cover: assets.find((a) => a.kind === "cover") || null,
    };
    ctx.content.addEventListener("click", async (e) => {
      const el = (e.target as HTMLElement).closest<HTMLElement>("[data-act],[data-copy-c],[data-fold],[data-more],[data-pv]");
      if (!el) return;
      if (el.dataset.act === "rback") { go("/library?kind=comment"); return; }
      if (el.dataset.act === "open" && record.target_key.startsWith("video:")) { go(`/stage/${encodeURIComponent(record.target_key.slice(6))}`); return; }
      if (el.dataset.act === "task" && record.task_id) { go(`/task/${encodeURIComponent(record.task_id)}?from=record`); return; }
      if (el.dataset.act === "open-json" && record.output_path) {
        try {
          const { invokeOr } = await import("../lib/store");
          await invokeOr("open_path", { path: record.output_path });
        } catch (err) { toast(String(err), "err"); }
        return;
      }
      if (el.dataset.act === "copy") { await copy(JSON.stringify(json, null, 2), "已复制 JSON"); return; }
      if (el.dataset.act === "export") {
        const text = exportText(record, comments, assets);
        if (isTauri()) { const { save } = await import("@tauri-apps/plugin-dialog"); const path = await save({ defaultPath: "采集记录.json", filters: [{ name: "JSON", extensions: ["json"] }] }); if (path) { const { invokeOr } = await import("../lib/store"); await invokeOr("save_text", { path, text }); toast("已导出"); } }
        else { const a = document.createElement("a"); a.href = URL.createObjectURL(new Blob([text])); a.download = "采集记录.json"; a.click(); }
        return;
      }
      if (el.dataset.act === "recheck") { await recheckRecord(ctx.arg); toast("已提交复查"); return; }
      if (el.dataset.act === "recollect") { const { openTaskCreator } = await import("../lib/task-create"); openTaskCreator({ target: record.target_key, context: "video", title: "重新采集", recordId: ctx.arg }); return; }
      if (el.dataset.act === "delete") { if (!window.confirm("删除这条记录、评论与本地缓存？已下载资产保留。")) return; await deleteRecord(ctx.arg); toast("已删除"); go("/library?kind=comment"); return; }
      if (el.dataset.act === "clear-csearch") { cq.value = ""; query = ""; void load(true); return; }
      if (el.dataset.copyC) { const c = comments.find((x) => x.cid === el.dataset.copyC); if (c) await copy(JSON.stringify(c.raw, null, 2), "单条评论已复制"); return; }
      if (el.dataset.fold) { el.closest(".comment")?.querySelector(".children")?.classList.toggle("hidden"); return; }
      if (el.hasAttribute("data-more")) { await load(); return; }
      if (el.dataset.pv) {
        const a = previewable[Number(el.dataset.pv)];
        if (!a) return;
        if (a.kind === "gallery" && a.path.toLowerCase().endsWith(".zip")) {
          const paths = await getAssetImages(a.id).catch(() => [] as string[]);
          if (!paths.length) { toast("图集解压失败或没有图片", "err"); return; }
          await preview(paths.map((path, i) => ({ ...a, id: `${a.id}:${i}`, path, title: `${a.title || "图集"} ${i + 1}` })), 0);
          return;
        }
        await preview(previewable, Number(el.dataset.pv));
      }
    });
    await paintAssets();
    await load(true);
  },
};
