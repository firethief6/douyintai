import { $, $$, errorBox } from "./lib/dom";
import { log } from "./lib/log";

export interface Ctx {
  arg: string;
  query: URLSearchParams;
  content: HTMLElement;
  signal: AbortSignal;
}

export interface View {
  title?: string;
  /** 列表类页面保留 DOM 与滚动位置，返回时无需重新请求。 */
  cacheable?: boolean;
  render(ctx: Ctx): string | Promise<string>;
  mount?(ctx: Ctx): void | Promise<void>;
  activate?(ctx: Ctx): void;
  inspector?(ctx: Ctx): string | Promise<string>;
  inspectorMount?(ctx: Ctx): void | Promise<void>;
  onLeave?(ctx?: Ctx): void;
}

const ROOTS = new Set(["feed", "hot", "live", "series", "search", "tasks", "library", "collect", "task", "records", "record", "favorites", "history", "plans", "settings", "log", "connect"]);

const views: Record<string, View> = {};
let current: { name: string; arg: string } | null = null;
let currentKey = "";
let leaving: { view: View; ctx: Ctx } | null = null;
let inflight: AbortController | null = null;
let snapSeq = 0;

interface Snapshot {
  key: string;
  node: HTMLElement;
  scrollTop: number;
  at: number;
  seq: number;
}

const snapshots = new Map<string, Snapshot>();
const SNAPSHOT_LIMIT = 12;
const SNAPSHOT_TTL = 10 * 60 * 1000;

export function register(name: string, v: View): void { views[name] = v; }

export function go(path: string, replace = false): void {
  const to = "#" + (path.startsWith("/") ? path : "/" + path);
  if (location.hash === to) return;
  if (replace) { replacing = true; location.replace(to); } else location.hash = to;
}

/** 原地替换查询参数时同步缓存键，避免返回后命中旧键。 */
export function replace(path: string): void {
  const to = "#" + (path.startsWith("/") ? path : "/" + path);
  if (location.hash !== to) history.replaceState(null, "", to);
  currentKey = to;
}

const trail: string[] = [];
let trailIdx = -1;
let jumping: "back" | "fwd" | null = null;
let replacing = false;

function noteKey(key: string): void {
  if (replacing) {
    replacing = false;
    if (trailIdx >= 0) trail[trailIdx] = key;
    else { trail.push(key); trailIdx = 0; }
    return;
  }
  if (jumping === "back") {
    jumping = null;
    trailIdx = Math.max(0, trailIdx - 1);
    return;
  }
  if (jumping === "fwd") {
    jumping = null;
    trailIdx = Math.min(trail.length - 1, trailIdx + 1);
    return;
  }
  if (trail[trailIdx] === key) return;
  trail.splice(trailIdx + 1);
  trail.push(key);
  if (trail.length > 60) trail.shift();
  trailIdx = trail.length - 1;
}

/** 原生式返回：有本机导航栈就后退，没有就落在兜底页。 */
export function back(fallback = "/feed"): void {
  const here = route();
  if (here.name === "live" && here.arg) { go("/live"); return; }
  if (trailIdx > 0) {
    jumping = "back";
    history.back();
  } else {
    go(fallback);
  }
}

export function forward(): void {
  if (trailIdx < trail.length - 1) {
    jumping = "fwd";
    history.forward();
  }
}

export const canBack = (): boolean => trailIdx > 0;

export function route(): { name: string; arg: string; query: URLSearchParams } {
  const raw = location.hash.replace(/^#\/?/, "") || "feed";
  const [path, qs = ""] = raw.split("?");
  const [name, ...rest] = path.split("/");
  return { name: name || "feed", arg: decodeURIComponent(rest.join("/")), query: new URLSearchParams(qs) };
}

export const currentRoute = () => current;

const navMap: Record<string, string> = {
  feed: "feed", stage: "feed", hot: "hot", live: "live", series: "series", mix: "series",
  topic: "topic", search: "search", user: "user",
  library: "library", collect: "collect", tasks: "tasks", task: "tasks", records: "library", record: "library", favorites: "favorites", history: "history", download: "collect", plans: "plans", settings: "settings", log: "log", gallery: "feed",
};

let navSeq = 0;

function pruneSnapshots(): void {
  const cutoff = Date.now() - SNAPSHOT_TTL;
  for (const [key, snap] of snapshots) {
    if (snap.at < cutoff) snapshots.delete(key);
  }
  while (snapshots.size > SNAPSHOT_LIMIT) {
    const oldest = [...snapshots.values()].sort((a, b) => a.seq - b.seq)[0];
    if (!oldest) break;
    snapshots.delete(oldest.key);
  }
}

function freshContent(): HTMLElement {
  const el = document.createElement("div");
  el.className = "content";
  el.id = "content";
  return el;
}

const redirects: Record<string, string> = { downloads: "/collect?kind=download", records: "/library?kind=comment" };

export async function render(fresh = false): Promise<void> {
  const r = route();
  const target = redirects[r.name];
  if (target) {
    const extra = location.hash.split("?")[1];
    go(extra ? `${target}&${extra}` : target, true);
    return;
  }
  const key = location.hash || "#/feed";
  const view = views[r.name] ?? views.feed;
  const seq = ++navSeq;
  const started = performance.now();
  inflight?.abort();
  const ac = new AbortController();
  inflight = ac;

  try { leaving?.view.onLeave?.(leaving.ctx); }
  catch (e) { log("warn", `离开页面失败：${String(e)}`, "route"); }
  if (leaving && currentKey && leaving.view.cacheable) {
    snapshots.set(currentKey, {
      key: currentKey,
      node: leaving.ctx.content,
      scrollTop: leaving.ctx.content.scrollTop,
      at: Date.now(),
      seq: ++snapSeq,
    });
  }
  leaving = null;
  pruneSnapshots();

  const oldContent = $("#content")!;
  const snapshot = fresh ? undefined : snapshots.get(key);
  if (fresh) snapshots.delete(key);
  const content = snapshot?.node ?? freshContent();
  if (snapshot) snapshots.delete(key);
  oldContent.replaceWith(content);
  const oldInspector = $("#inspector");
  if (oldInspector) oldInspector.replaceWith(oldInspector.cloneNode(false));

  noteKey(key);
  currentKey = key;
  current = { name: r.name, arg: r.arg };
  log("info", `路由 → ${r.name}${r.arg ? "/" + r.arg.slice(0, 80) : ""}`, "route");

  document.body.classList.remove("routing");
  document.querySelector(".app")?.classList.toggle("no-chrome", r.name === "connect");
  document.querySelector(".app")?.classList.toggle("on-search", r.name === "search");
  const isSub = !ROOTS.has(r.name) || ((r.name === "series" || r.name === "live") && !!r.arg) || r.name === "search";
  document.body.classList.toggle("is-sub", isSub);
  const inSeries = r.name === "stage" && (r.query.has("series") || r.query.has("mix"));
  const from = r.name === "stage" ? (r.query.get("from") ?? "") : "";
  const navTarget = inSeries ? "series" : (from && navMap[from]) || navMap[r.name];
  $$(".navitem").forEach((n) => n.classList.toggle("active", n.dataset.nav === navTarget));
  const historyBack = $<HTMLButtonElement>('[data-act="back"]');
  if (historyBack) historyBack.disabled = !isSub;
  $("#main")?.classList.remove("with-inspector");
  const insp = $("#inspector");
  if (insp) insp.innerHTML = "";

  const ctx: Ctx = { arg: r.arg, query: r.query, content, signal: ac.signal };

  if (snapshot) {
    leaving = { view, ctx };
    document.title = `${view.title ?? "抖音台"} · 抖音台`;
    content.scrollTop = snapshot.scrollTop;
    try { view.activate?.(ctx); } catch (e) { log("warn", `恢复页面失败 ${r.name}: ${String(e)}`, "route"); }
    log("info", `恢复页面 ${r.name}`, "route");
  } else {
    let loadingTimer = window.setTimeout(() => {
      if (seq === navSeq && !ac.signal.aborted) {
        document.body.classList.add("routing");
        content.innerHTML = `<div class="view active"><div class="route-loading" role="status" aria-live="polite"><span class="spinner"></span><span>加载中…</span></div></div>`;
      }
    }, 140);
    let html: string;
    try {
      html = await view.render(ctx);
    } catch (e) {
      if (seq !== navSeq) return;
      html = `<section class="view active">${errorBox(e)}</section>`;
      log("err", `页面渲染失败 ${r.name}: ${String(e)}`, "route");
    } finally {
      window.clearTimeout(loadingTimer);
      loadingTimer = 0;
    }
    if (seq !== navSeq) return;
    if (ac.signal.aborted) return;
    content.innerHTML = html;
    leaving = { view, ctx };
    try {
      await view.mount?.(ctx);
    } catch (e) {
      if (!ac.signal.aborted) log("err", `挂载失败 ${r.name}: ${String(e)}`, "route");
    }
    if (seq !== navSeq || ac.signal.aborted) return;
    if (view.inspector && $("#inspector")) {
      try {
        const html = await view.inspector(ctx);
        if (seq !== navSeq || ac.signal.aborted) return;
        $("#main")?.classList.add("with-inspector");
        $("#inspector")!.innerHTML = html;
        await view.inspectorMount?.(ctx);
      } catch (e) { if (!ac.signal.aborted) log("warn", `检查器失败 ${r.name}: ${String(e)}`, "route"); }
    }
    if (seq !== navSeq || ac.signal.aborted) return;
    document.title = `${view.title ?? "抖音台"} · 抖音台`;
    log("info", `路由完成 ${r.name} ${Math.round(performance.now() - started)}ms`, "route");
  }
  document.body.classList.remove("routing");
}
