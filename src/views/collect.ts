import { $, closeModal, debounce, empty, esc, modal, toast } from "../lib/dom";
import { COLLECT, TASK } from "../lib/copy";
import {
  ackTask, cancelDownload, cancelTask, deleteDownload, getStats, getTaskSummary, keepTask,
  listDownloads, listPlans, listRecords, listTasks, pauseDownload, pauseTask,
  recheck, resumeDownload, resumeTask, retryDownload, retryTask, submitBatch, taskAction,
  type CaptureRecord, type Download, type Task, type TaskAction, type TaskFilter, type TaskScope, type TaskSummary,
} from "../lib/tasks";
import { contentIcon } from "../lib/content";
import { go } from "../router";
import type { View } from "../router";
import { commentLabel, commentsIncomplete } from "../lib/record-meta";
import { RECORD_META } from "../lib/copy";

const ACTIVE = new Set(["queued", "running", "paused"]);
const CANCELABLE = new Set(["queued", "running", "paused"]);
const RETRYABLE = new Set(["failed", "dead", "canceled"]);
const dlBytes = (n: number | null | undefined) => !n ? "0 B" : n >= 1 << 30 ? `${(n / (1 << 30)).toFixed(1)} GB` : n >= 1 << 20 ? `${(n / (1 << 20)).toFixed(1)} MB` : n >= 1024 ? `${Math.round(n / 1024)} KB` : `${n} B`;
const SCOPES: TaskScope[] = ["active", "pending", "recent", "history", "deleted"];
type Scope = typeof SCOPES[number];
const when = (n: number | null) => n ? new Date(n * 1000).toLocaleString() : "—";
const isOrphan = (t: Task) => t.id.startsWith("orphan:");
const isActive = (t: Task) => ACTIVE.has(t.state);
const titleOf = (t: Task) => String(t.params?.display_title || t.target_key || t.endpoint);
const subOf = (t: Task) => String(t.params?.display_subtitle || "");
const downloadOf = (t: Task) => typeof t.params?.download_id === "string" ? t.params.download_id : isOrphan(t) ? t.id.slice(7) : "";
const taskIdOf = (t: Task) => isOrphan(t) ? "" : t.id;
const stateText = (t: Task) => t.state === "canceled" || (t.state === "dead" && t.error_code === "cancelled")
  ? TASK.cancelledState
  : t.state === "queued" && t.retry_at
  ? `退避 ${Math.max(0, t.retry_at - Math.floor(Date.now() / 1000))}s`
  : TASK.states[t.state] || t.state;
const DL_STATE_TEXT: Record<string, string> = { queued: "排队中", running: "传输中", paused: "已暂停", done: "已完成", failed: "失败", gone: "已失效", canceled: "已取消" };
const stateClass = (state: string) => state === "running" ? "run" : state === "paused" ? "pause" : state === "done" ? "done" : state === "canceled" ? "cancel" : "fail";
const ASSET_TEXT: Record<string, string> = { video: "视频", cover: "封面", gallery: "图集", music: "原声", record: "直播录制" };
const kindText = (t: Task) => ASSET_TEXT[String(t.params?.asset_kind || "")] || TASK.kinds[t.kind] || t.kind;
const iconOf = (t: Task) => contentIcon(t.kind, String(t.params?.asset_kind || ""));
const kindMatches = (t: Task, kind: string) => !kind || (kind === "json" ? ["collection", "recheck", "backfill"].includes(t.kind) : kind === "download" ? !!downloadOf(t) : kind === "live" ? String(t.params?.asset_kind || "") === "record" : kind === "other" ? !["video", "cover", "gallery", "music", "record"].includes(String(t.params?.asset_kind || "")) && !["collection", "recheck", "backfill"].includes(t.kind) : String(t.params?.asset_kind || "") === kind);
const scopeCount = (s: TaskSummary, scope: Scope) => scope === "active" ? s.active : scope === "pending" ? s.pending : scope === "recent" ? s.recent : scope === "history" ? s.history : s.deleted;
const commentResult = (t: Task) => {
  const n = Number(t.result_summary?.comments_collected);
  if (!Number.isFinite(n)) return "";
  const target = Number(t.result_summary?.comments_target);
  return `评论 ${n}${target ? `/${target}` : ""}`;
};

const view: View = {
  title: COLLECT.title,
  render: () => `
    <section class="view active collectcenter">
      <div class="pagehead between"><div><h1 class="h1">${COLLECT.title}</h1><div class="sub">${COLLECT.sub}</div></div>
        <div class="row"><button class="btn sm" data-act="recheck">${COLLECT.recheck}</button><button class="btn sm" data-act="batch">${COLLECT.batch}</button><button class="btn sm primary" data-act="new">${COLLECT.new}</button></div></div>
      <div id="cstats" class="collectstats"></div>
      <div class="collectbody">
        <div class="collectmain">
          <div class="tasktools">
            <div class="row"><select class="select smselect" id="ckind" aria-label="内容类型"><option value="">${COLLECT.allKinds}</option><option value="download">下载</option><option value="video">视频</option><option value="live">直播</option><option value="gallery">图集</option><option value="json">JSON</option><option value="cover">封面</option><option value="music">原声</option><option value="other">其他</option></select>
              <select class="select smselect" id="csource" aria-label="任务来源"><option value="">全部来源</option><option value="user">用户</option><option value="plan">计划</option><option value="system">系统</option></select>
              <input class="input searchbox" id="csearch" placeholder="搜索标题、目标或错误" aria-label="搜索任务"><button class="btn sm" data-act="refresh">${COLLECT.refreshTask}</button><button class="chip" id="ckept" data-act="clear-kept" hidden>仅看保留 ×</button></div>
            <div class="dim taskhint">${COLLECT.retentionHint}</div>
          </div>
          <div class="dlbulk" id="dlbulk" hidden></div>
          <div class="ctaskhead"><span class="pick"><input type="checkbox" id="cselectpage" aria-label="选择本页"></span><span>${COLLECT.colState}</span><span>${COLLECT.colContent}</span><span class="col-time">${COLLECT.colTime}</span><span class="col-outcome">${COLLECT.colOutcome}</span><span class="ops">${COLLECT.colOps}</span></div>
          <div id="ctasks" role="list"></div>
          <div id="cmore" class="loadmore"></div>
        </div>
        <aside class="collectside">
          <section class="sidecard">
            <div class="sectionhead"><b>${COLLECT.records}</b><button class="btn sm" data-go="/library?kind=comment">${COLLECT.allRecords}</button></div>
            <div id="crecords"></div>
          </section>
          <section class="sidecard quicklinks">
            <button class="linkrow" data-go="/plans"><span>${COLLECT.plans}</span><b id="cplans">—</b></button>
            <button class="linkrow" data-go="/tasks"><span>${COLLECT.queue}</span><b>${COLLECT.viewQueue}</b></button>
          </section>
        </aside>
      </div>
    </section>`,
  mount(ctx) {
    let alive = true;
    let timer = 0;
    let generation = 0;
    let tasks: Task[] = [];
    let records: CaptureRecord[] = [];
    let recordsLoaded = false;
    let recordsFailed = false;
    let plans = 0;
    let loaded = false;
    let loading = false;
    let failed = false;
    let nextCursor: string | null = null;
    let downloading = new Map<string, Download>();
    let summary: TaskSummary = { active: 0, pending: 0, recent: 0, history: 0, deleted: 0, kept: 0 };
    const initialKind = ctx.query.get("kind") || "";
    let kindFilter = ["download", "video", "live", "gallery", "json", "cover", "music", "other"].includes(initialKind) ? initialKind : "";
    let sourceFilter = "";
    let queryText = "";
    let keptOnly = false;
    let scope: Scope = "active";
    const selected = new Set<string>();
    let selectAll = false;
    const expandedPlans = new Set<string>();
    const box = $("#ctasks", ctx.content)!;
    const moreBox = $("#cmore", ctx.content)!;
    const recBox = $("#crecords", ctx.content)!;
    const statsBox = $("#cstats", ctx.content)!;
    const bulkBox = $("#dlbulk", ctx.content)!;

    const filters = (): TaskFilter => ({ kind: kindFilter || undefined, source: sourceFilter || undefined, q: queryText.trim() || undefined, kept: keptOnly ? "1" : undefined });
    const pageTotal = () => tasks.length;

    const paintStats = () => {
      const cells: [string, number, Scope][] = [
        [COLLECT.stats.active, summary.active, "active"],
        [COLLECT.stats.pending, summary.pending, "pending"],
        [COLLECT.stats.done, summary.recent, "recent"],
        [COLLECT.stats.history, summary.history, "history"],
        [COLLECT.stats.deleted, summary.deleted, "deleted"],
        [COLLECT.stats.kept, summary.kept, "history"],
      ];
      statsBox.innerHTML = cells.map(([label, value, s], i) => `<button class="collectstat filterable${(i === 5 ? keptOnly : !keptOnly && s === scope) ? " active" : ""}" ${i === 5 ? 'data-act="kept"' : `data-scope="${s}"`}><span>${esc(label)}</span><b>${value}</b></button>`).join("");
      const kept = $("#ckept", ctx.content) as HTMLElement | null;
      if (kept) kept.hidden = !keptOnly;
    };

    const opHtml = (t: Task) => {
      const dl = downloadOf(t);
      const terminal = ["done", "failed", "dead", "canceled"].includes(t.state);
      const ops: string[] = [];
      if (scope === "deleted") {
        ops.push(`<button class="btn sm" data-act="restore" data-id="${esc(t.id)}">恢复</button>`);
        ops.push(`<button class="btn sm danger" data-act="purge" data-id="${esc(t.id)}">永久删除</button>`);
      } else {
        ops.push(`<button class="btn sm" data-act="${dl ? "download" : "detail"}" data-id="${esc(dl || t.id)}">${COLLECT.detail}</button>`);
        if (scope === "pending" && t.state === "dead") {
          ops.push(`<button class="btn sm" data-act="retry" data-id="${esc(t.id)}">${COLLECT.retry}</button>`);
          ops.push(`<button class="btn sm" data-act="ack" data-id="${esc(t.id)}">忽略</button>`);
        } else {
          if (isActive(t) && t.state !== "paused") ops.push(`<button class="btn sm" data-act="pause" data-id="${esc(t.id)}">${COLLECT.pause}</button>`);
          if (t.state === "paused") ops.push(`<button class="btn sm" data-act="resume" data-id="${esc(t.id)}">${COLLECT.resume}</button>`);
          if (CANCELABLE.has(t.state)) ops.push(`<button class="btn sm" data-act="cancel" data-id="${esc(t.id)}">${COLLECT.cancel}</button>`);
          if (RETRYABLE.has(t.state)) ops.push(`<button class="btn sm" data-act="retry" data-id="${esc(t.id)}">${COLLECT.retry}</button>`);
          if (terminal) ops.push(`<button class="btn sm" data-act="delete" data-id="${esc(t.id)}">${COLLECT.delete}</button>`);
        }
        if (t.keep_until) ops.push(`<button class="btn sm" data-act="unkeep" data-id="${esc(t.id)}">取消保留</button>`);
        else if (terminal) ops.push(`<button class="btn sm" data-act="keep" data-id="${esc(t.id)}">保留</button>`);
      }
      return ops.join("");
    };

    const rowHtml = (t: Task, nested = false) => {
      const dl = downloadOf(t);
      const d = downloading.get(dl);
      const state = d?.state || t.state;
      const image = String(t.cover || t.params?.display_image || "");
      const outcome = t.outcome ? TASK.outcomes[t.outcome] || t.outcome : "";
      const pct = d ? d.state === "done" ? 100 : Math.min(100, d.expected_bytes ? (d.bytes || 0) / d.expected_bytes * 100 : (d.progress || 0) * 100) : 0;
      const meter = d ? `${dlBytes(d.bytes)}${d.expected_bytes ? ` / ${dlBytes(d.expected_bytes)}` : ""}${d.speed ? ` · ${dlBytes(d.speed)}/s` : ""}` : "";
      return `<article class="ctask${nested ? " nested" : ""}" data-task="${esc(t.id)}" role="listitem">
        <span class="pick"><input type="checkbox" data-pick="${esc(t.id)}" aria-label="选择 ${esc(titleOf(t))}" ${selected.has(t.id) ? "checked" : ""}><span class="thumb">${image ? `<img src="${esc(image)}" loading="lazy" alt="">` : `<svg class="i"><use href="#${iconOf(t)}"/></svg>`}</span></span>
        <span class="state ${stateClass(state)}"><i></i><span class="state-text">${esc(d ? DL_STATE_TEXT[d.state] || d.state : stateText(t))}</span></span>
        <span class="grow"><b class="tt">${esc(titleOf(t))}</b><span class="tm">${esc([kindText(t), subOf(t), meter, commentResult(t), t.trigger === "plan" ? "计划创建" : ""].filter(Boolean).join(" · "))}</span>${d ? `<span class="dlbar"><i style="width:${pct}%"></i></span>` : ""}</span>
        <span class="time dim">${esc(when(t.created_at))}</span><span class="outcomecell">${outcome ? `<span class="outcome ${esc(t.outcome)}">${esc(outcome)}</span>` : '<span class="dim">—</span>'}</span>
        <span class="ops">${opHtml(t)}</span></article>`;
    };

    const groupedRows = (rows: Task[]) => {
      if (scope === "deleted" || scope === "pending" || scope === "active") return rows.map((t) => rowHtml(t));
      const out: string[] = [];
      const seen = new Set<string>();
      for (let i = 0; i < rows.length; i++) {
        const t = rows[i];
        if (t.trigger !== "plan" || !t.plan_id) { out.push(rowHtml(t)); continue; }
        if (seen.has(t.plan_id)) continue;
        seen.add(t.plan_id);
        const group = rows.filter((x) => x.plan_id === t.plan_id && x.trigger === "plan");
        const ok = group.filter((x) => x.state === "done").length;
        const bad = group.filter((x) => x.state === "dead").length;
        const open = expandedPlans.has(t.plan_id);
        out.push(`<article class="ctask plangroup" data-plan-group="${esc(t.plan_id)}">
          <span class="pick"><button class="iconbtn tiny" data-act="toggle-plan" data-id="${esc(t.plan_id)}" aria-label="${open ? "收起" : "展开"}">${open ? "▾" : "▸"}</button></span>
          <span class="state plan">计划</span>
          <span class="grow"><b class="tt">${esc(titleOf(t))}</b><span class="tm">本页 ${group.length} 次 · 成功 ${ok} · 失败 ${bad}</span></span>
          <span class="time dim">${esc(when(group[0]?.created_at || null))}</span><span class="outcomecell dim">计划执行</span>
          <span class="ops"><button class="btn sm" data-act="toggle-plan" data-id="${esc(t.plan_id)}">${open ? "收起" : "展开"}</button><button class="btn sm" data-act="delete-group" data-plan="${esc(t.plan_id)}">删除记录</button></span></article>`);
        if (open) out.push(...group.map((x) => rowHtml(x, true)));
      }
      return out;
    };

    const paintBulk = () => {
      const n = selectAll ? scopeCount(summary, scope) : selected.size;
      bulkBox.hidden = n === 0 && !selectAll;
      if (bulkBox.hidden) { bulkBox.innerHTML = ""; return; }
      const actions: TaskAction[] = scope === "pending" ? ["retry", "ack", "delete"] : scope === "deleted" ? ["restore", "purge"] : scope === "active" ? ["pause", "cancel", "delete"] : ["keep", "delete"];
      const label: Partial<Record<TaskAction, string>> = { retry: "重试所选", ack: "忽略所选", delete: "删除所选", restore: "恢复所选", purge: "永久删除所选", pause: "暂停所选", cancel: "取消所选", keep: "保留所选" };
      bulkBox.innerHTML = `<span class="bulksel">已选 <b>${n}</b> 条</span>${selectAll ? '<span class="dim">整个筛选结果</span>' : (nextCursor || selected.size < pageTotal()) ? `<button class="linkbtn" data-act="select-all-filter">选择当前筛选全部 ${scopeCount(summary, scope)} 条</button>` : ""}
        ${actions.map((a) => `<button class="btn sm${a === "delete" || a === "purge" || a === "cancel" ? " danger" : ""}" data-act="bulk" data-bulk="${a}">${label[a]}</button>`).join("")}
        <button class="btn sm ghost" data-act="clear-select">清除选择</button>`;
    };

    const paintList = () => {
      moreBox.innerHTML = "";
      if (!loaded) {
        box.dataset.mode = failed ? "error" : "loading";
        box.removeAttribute("role");
        box.innerHTML = failed
          ? empty(COLLECT.loadFail, "检查后端连接后重试。", `<button class="btn sm" data-act="refresh">${COLLECT.retryLoad}</button>`)
          : empty(COLLECT.loading, COLLECT.loadingHint);
        return;
      }
      if (!tasks.length) {
        box.dataset.mode = "empty";
        box.removeAttribute("role");
        box.innerHTML = empty(COLLECT.empty[scope], COLLECT.emptyHint[scope], scope === "active" || scope === "pending" ? `<button class="btn sm" data-act="new">${COLLECT.new}</button>` : "");
        return;
      }
      box.dataset.mode = "rows";
      box.setAttribute("role", "list");
      box.innerHTML = groupedRows(tasks).join("");
      if (nextCursor) moreBox.innerHTML = `<button class="btn sm" data-act="more" ${loading ? "disabled" : ""}>加载更多</button>`;
      const pageIds = tasks.map((t) => t.id);
      const all = pageIds.length > 0 && pageIds.every((id) => selected.has(id));
      const head = $("#cselectpage", ctx.content) as HTMLInputElement | null;
      if (head) head.checked = all;
    };

    const paint = () => {
      paintStats();
      paintBulk();
      paintList();
    };

    const paintRecords = () => {
      const latest = records.slice(0, 6);
      if (!latest.length) {
        recBox.innerHTML = !recordsLoaded ? empty(COLLECT.recordsLoading, "") : recordsFailed ? empty(COLLECT.recordsFail, "") : empty(COLLECT.recordsEmpty, COLLECT.recordsHint);
        return;
      }
      recBox.innerHTML = latest.map((r) => `
        <button class="recrow" data-record="${esc(r.id)}">
          <span class="thumb">${r.cover ? `<img src="${esc(r.cover)}" loading="lazy" alt="">` : `<svg class="i"><use href="#i-database"/></svg>`}</span>
          <span class="grow"><b>${esc(r.title || r.target_key)}</b><span>${esc(r.author_name || "未知作者")} · <span class="${commentsIncomplete(r) ? "commentmeta partial" : "commentmeta"}">${esc(commentLabel(r))}</span>${RECORD_META.assets(r.asset_count)}</span></span>
          <span class="statechip ${r.state === "deleted" ? "deleted" : ""}">${r.state === "deleted" ? COLLECT.invalid : COLLECT.valid}</span>
        </button>`).join("");
    };

    const schedule = () => {
      window.clearTimeout(timer);
      if (!["active", "pending", "recent"].includes(scope)) return;
      const wait = scope === "active" ? 2500 : scope === "pending" ? 4000 : 7000;
      timer = window.setTimeout(() => { void load("poll"); }, wait);
    };

    const orphanRows = (downloads: Download[], known: Set<string>) => downloads
      .filter((d) => !d.task_id && !d.task_deleted_at && !known.has(`orphan:${d.id}`))
      .map((d): Task => ({
        id: `orphan:${d.id}`, kind: "download", endpoint: "", params: { display_title: d.title, display_subtitle: d.author, asset_kind: d.asset_kind, download_id: d.id },
        target_key: d.target_key, state: d.state === "failed" ? "dead" : d.state === "gone" ? "done" : d.state,
        attempts: 0, max_attempts: 0, batch_id: null, outcome: d.state === "gone" ? "gone" : d.state === "done" ? "ok" : null, error_code: d.error,
        created_at: d.created_at, started_at: null, finished_at: d.finished_at, request_id: null, identity: null, upstream_url: null, retry_at: null, output_path: d.path,
        source: null, progress: d.progress, bytes: d.bytes || 0, cover: d.cover, acknowledged_at: null, archived_at: null,
        deleted_at: null, keep_until: null, trigger: "user", plan_id: null, result_summary: null,
      }))
      .filter((t) => kindMatches(t, kindFilter));

    const load = async (mode: "reset" | "poll" | "more") => {
      if (!alive || loading) return;
      window.clearTimeout(timer);
      if (document.hidden) { timer = window.setTimeout(() => void load("poll"), 2000); return; }
      const gen = ++generation;
      loading = true;
      if (mode === "reset") { loaded = false; tasks = []; nextCursor = null; failed = false; paint(); }
      try {
        const cursor = mode === "more" ? nextCursor || undefined : undefined;
        const [page, sum, downloads] = await Promise.all([
          listTasks(scope, filters(), cursor),
          getTaskSummary(filters()),
          listDownloads().catch(() => [] as Download[]),
        ]);
        if (!alive || gen !== generation) return;
        summary = sum;
        downloading = new Map(downloads.map((d) => [d.id, d]));
        const base = mode === "more" ? [...tasks, ...page.items] : page.items;
        const known = new Set(base.map((t) => t.id));
        tasks = ["active", "recent", "history"].includes(scope) ? [...base, ...orphanRows(downloads, known)] : base;
        nextCursor = page.meta.next;
        loaded = true;
      } catch {
        if (gen === generation) { failed = !tasks.length; loaded = !!tasks.length; }
      } finally {
        if (gen === generation) {
          loading = false;
          if (alive) { paint(); schedule(); }
        }
      }
    };

    const loadRecords = async () => {
      try {
        const page = await listRecords();
        records = page.items;
        recordsFailed = false;
        recordsLoaded = true;
      } catch { recordsFailed = true; recordsLoaded = true; }
      if (!alive) return;
      paintRecords();
    };

    const loadPlans = async () => {
      try { plans = (await listPlans()).length; } catch { plans = 0; }
      const el = $("#cplans", ctx.content);
      if (el) el.textContent = String(plans);
    };

    const setScope = (next: Scope) => {
      scope = next;
      keptOnly = false;
      selected.clear();
      selectAll = false;
      window.scrollTo({ top: 0 });
      void load("reset");
    };

    const resetAndReload = () => { void load("reset"); void loadRecords(); void loadPlans(); };

    const idsForAction = () => selectAll ? undefined : [...selected];
    const selectedTasks = () => selectAll ? tasks : tasks.filter((t) => selected.has(t.id));

    const undoDelete = (ids: string[]) => {
      if (!ids.length) return;
      const el = document.createElement("div");
      el.className = "toast action";
      el.innerHTML = `<span>已删除 ${ids.length} 条记录</span><button>撤销</button>`;
      let host = document.getElementById("toasts");
      if (!host) { host = document.createElement("div"); host.id = "toasts"; host.className = "toasts"; document.body.append(host); }
      host.append(el);
      requestAnimationFrame(() => el.classList.add("in"));
      const close = () => { el.classList.remove("in"); setTimeout(() => el.remove(), 200); };
      el.querySelector("button")!.addEventListener("click", async () => {
        try { await taskAction("restore", { ids }); toast("已恢复"); resetAndReload(); } catch (err) { toast(String(err), "err"); }
        close();
      });
      setTimeout(close, 8000);
    };

    const deleteFlow = (ids: string[] | undefined, count: number, mode: "delete" | "purge" = "delete") => {
      const scrim = modal(`
        <div class="dh"><h3>${mode === "purge" ? "永久删除" : "删除"} ${count} 条任务记录</h3><button class="iconbtn" data-close><svg class="i"><use href="#i-close"/></svg></button></div>
        <div class="db"><div>${mode === "purge" ? "记录立即移除，无法撤销。" : "记录进入回收区，7 天后自动永久删除。"}</div>
          <label class="checkrow"><input type="checkbox" id="dfile"> 同时把产出文件移入系统回收站</label>
          <div class="hint dim">采集记录、资产记录和任务历史相互独立；不勾选只删执行记录。</div></div>
        <div class="df"><button class="btn sm ghost" data-close>取消</button><button class="btn sm danger" data-act="confirm">${mode === "purge" ? "永久删除" : "删除"}</button></div>`);
      scrim.addEventListener("click", async (e) => {
        const el = (e.target as HTMLElement).closest<HTMLElement>("[data-close],[data-act]");
        if (!el) return;
        if (el.hasAttribute("data-close")) { closeModal(); return; }
        const file = ($("#dfile", scrim) as HTMLInputElement).checked ? "trash" : "keep";
        (el as HTMLButtonElement).disabled = true;
        try {
          const taskIds = ids?.filter((id) => !id.startsWith("orphan:"));
          const orphanIds = ids?.filter((id) => id.startsWith("orphan:")).map((id) => id.slice(7)) || [];
          const out = taskIds?.length ? await taskAction(mode, { ids: taskIds, file }) : { count: 0, ids: [] as string[] };
          for (const id of orphanIds) await deleteDownload(id, file).catch(() => undefined);
          closeModal();
          selected.clear();
          selectAll = false;
          if (mode === "delete") undoDelete(out.ids);
          else toast(`已永久删除 ${out.count + orphanIds.length} 条`);
          resetAndReload();
        } catch (err) { (el as HTMLButtonElement).disabled = false; toast(String(err), "err"); }
      });
    };

    const runOne = async (el: HTMLElement, fn: () => Promise<unknown>) => {
      el.setAttribute("disabled", "");
      try { await fn(); resetAndReload(); } finally { el.removeAttribute("disabled"); }
    };

    const actionOne = async (act: string, t: Task) => {
      const dl = downloadOf(t);
      const id = taskIdOf(t);
      if (!id && dl) {
        if (act === "pause") return pauseDownload(dl);
        if (act === "resume") return resumeDownload(dl);
        if (act === "cancel") return cancelDownload(dl);
        if (act === "retry") return retryDownload(dl);
        if (act === "delete") return deleteDownload(dl, "keep");
        return;
      }
      if (act === "detail") return;
      if (act === "download") return;
      if (act === "pause") return pauseTask(id);
      if (act === "resume") return resumeTask(id);
      if (act === "cancel") return cancelTask(id);
      if (act === "retry") return retryTask(id);
      if (act === "ack") return ackTask(id);
      if (act === "keep") return keepTask(id, true);
      if (act === "unkeep") return keepTask(id, false);
      if (act === "restore") return taskAction("restore", { ids: [id] });
      if (act === "purge") return taskAction("purge", { ids: [id], file: "keep" });
      if (act === "delete") return taskAction("delete", { ids: [id], file: "keep" });
    };

    const bulk = async (action: TaskAction) => {
      const ids = idsForAction();
      if (action === "delete" || action === "purge") {
        const count = selectAll ? scopeCount(summary, scope) : selected.size;
        if (!count) return;
        deleteFlow(ids, count, action);
        return;
      }
      try {
        const current = selectedTasks();
        const orphans = current.filter(isOrphan);
        if (orphans.length && !selectAll) {
          for (const t of orphans) await actionOne(action, t).catch(() => undefined);
        }
        const out = await taskAction(action, { ids, scope: selectAll ? scope : undefined, filter: selectAll ? filters() : undefined });
        toast(`已处理 ${out.count + orphans.length} 条`);
        selected.clear();
        selectAll = false;
        resetAndReload();
      } catch (err) { toast(String(err), "err"); }
    };

    const batchModal = () => {
      const scrim = modal(`
        <div class="dh"><h3>${TASK.batch.title}</h3><button class="iconbtn" data-close><svg class="i"><use href="#i-close"/></svg></button></div>
        <div class="db"><textarea class="input mono" id="btargets" rows="12" placeholder="一行一个目标"></textarea>
          <div class="hint dim">${esc(TASK.batch.hint)}</div><div id="brisk"></div></div>
        <div class="df"><button class="btn sm ghost" data-close>取消</button><button class="btn sm primary" data-act="submit">${TASK.batch.submit}</button></div>`);
      void (async () => {
        try {
          const s = await getStats();
          if (s.pool_available === 0) $("#brisk", scrim)!.innerHTML = `<div class="notice warn">${esc(TASK.riskWarn)}</div>`;
        } catch { /* 忽略 */ }
      })();
      scrim.addEventListener("click", async (e) => {
        const el = (e.target as HTMLElement).closest<HTMLElement>("[data-close],[data-act]");
        if (!el) return;
        if (el.hasAttribute("data-close")) { closeModal(); return; }
        const targets = ($("#btargets", scrim) as HTMLTextAreaElement).value.split(/\r?\n/).map((x) => x.trim()).filter(Boolean);
        if (!targets.length) { toast("没有条目", "err"); return; }
        (el as HTMLButtonElement).disabled = true;
        try {
          const out = await submitBatch(targets.map((target) => ({ target })));
          closeModal();
          toast(TASK.batch.accepted(out.accepted, out.rejected), out.rejected ? "err" : "ok");
          setScope("active");
        } catch (err) {
          (el as HTMLButtonElement).disabled = false;
          toast(String(err), "err");
        }
      });
    };

    const kindSelect = $("#ckind", ctx.content) as HTMLSelectElement;
    kindSelect.value = kindFilter;
    kindSelect.addEventListener("change", (e) => { kindFilter = (e.target as HTMLSelectElement).value; void load("reset"); });
    const sourceSelect = $("#csource", ctx.content) as HTMLSelectElement;
    sourceSelect.addEventListener("change", (e) => { sourceFilter = (e.target as HTMLSelectElement).value; void load("reset"); });
    const search = $("#csearch", ctx.content) as HTMLInputElement;
    search.addEventListener("input", debounce(() => { queryText = search.value; void load("reset"); }, 280));
    ctx.content.addEventListener("change", (e) => {
      const pick = (e.target as HTMLElement).closest<HTMLInputElement>("[data-pick]");
      if (!pick) return;
      selectAll = false;
      pick.checked ? selected.add(pick.dataset.pick!) : selected.delete(pick.dataset.pick!);
      paintBulk();
    });
    $("#cselectpage", ctx.content)!.addEventListener("change", (e) => {
      const on = (e.target as HTMLInputElement).checked;
      selectAll = false;
      for (const t of tasks) on ? selected.add(t.id) : selected.delete(t.id);
      paint();
    });

    ctx.content.addEventListener("click", async (e) => {
      const scopeEl = (e.target as HTMLElement).closest<HTMLElement>("[data-scope]");
      if (scopeEl?.dataset.scope) { setScope(scopeEl.dataset.scope as Scope); return; }
      const record = (e.target as HTMLElement).closest<HTMLElement>("[data-record]");
      if (record?.dataset.record) { go(`/record/${encodeURIComponent(record.dataset.record)}?from=collect`); return; }
      const nav = (e.target as HTMLElement).closest<HTMLElement>("[data-go]");
      if (nav?.dataset.go) { go(nav.dataset.go); return; }
      const el = (e.target as HTMLElement).closest<HTMLElement>("[data-act]");
      if (el) {
        try {
          const act = el.dataset.act!;
          if (act === "new") { const { openTaskCreator } = await import("../lib/task-create"); openTaskCreator({ title: COLLECT.new }); return; }
          if (act === "kept") { scope = "history"; keptOnly = true; selected.clear(); selectAll = false; void load("reset"); return; }
          if (act === "clear-kept") { keptOnly = false; void load("reset"); return; }
          if (act === "batch") { batchModal(); return; }
          if (act === "refresh") { resetAndReload(); return; }
          if (act === "recheck") { await runOne(el, async () => { const r = await recheck(20); toast(`已提交复查 ${r.accepted} 条`); }); return; }
          if (act === "more") { await load("more"); return; }
          if (act === "clear-select") { selected.clear(); selectAll = false; paint(); return; }
          if (act === "select-all-filter") { selectAll = true; selected.clear(); paint(); return; }
          if (act === "toggle-plan") { const id = el.dataset.id!; expandedPlans.has(id) ? expandedPlans.delete(id) : expandedPlans.add(id); paintList(); return; }
          if (act === "bare-delete") {
            const t = tasks.find((x) => x.id === el.dataset.id);
            const id = t && taskIdOf(t);
            if (id) deleteFlow([id], 1);
            return;
          }
          if (act === "delete-group") {
            const ids = tasks.filter((t) => t.plan_id === el.dataset.plan && taskIdOf(t)).map((t) => t.id);
            if (ids.length) deleteFlow(ids, ids.length);
            return;
          }
          if (act === "bulk") { await bulk(el.dataset.bulk as TaskAction); return; }
          if (act === "detail" || act === "download") { go(`/${act === "detail" ? "task" : "download"}/${encodeURIComponent(el.dataset.id!)}?from=collect`); return; }
          const t = tasks.find((x) => x.id === el.dataset.id);
          if (!t) return;
          if (act === "delete") { deleteFlow([t.id], 1); return; }
          if (act === "purge") { deleteFlow([t.id], 1, "purge"); return; }
          await runOne(el, () => actionOne(act, t));
        } catch (err) { toast(String(err), "err"); }
        return;
      }
      if ((e.target as HTMLElement).closest(".pick,.ops")) return;
      const row = (e.target as HTMLElement).closest<HTMLElement>("[data-task]");
      const t = tasks.find((x) => x.id === row?.dataset.task);
      if (t) { const dl = downloadOf(t); go(dl ? `/download/${encodeURIComponent(dl)}` : `/task/${encodeURIComponent(t.id)}?from=collect`); }
    });

    paint();
    paintRecords();
    paintStats();
    void load("reset");
    void loadRecords();
    void loadPlans();
    ctx.signal.addEventListener("abort", () => { alive = false; window.clearTimeout(timer); });
  },
};
export default view;
