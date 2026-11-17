import { comments, mix, mixVideos, replies, resolve, series, seriesEpisodes, video } from "./api";
import { invokeOr, isTauri } from "./store";
import { log } from "./log";

export type CollectState = "run" | "stopping" | "stopped" | "done" | "fail";
export type CollectPhase = "resolve" | "video" | "comments" | "replies" | "write" | "";

export interface Run {
  id: string; target: string; kind: string; state: CollectState;
  rows: number; path: string; err: string; at: number;
  /** 运行进度；旧记录缺省时按 0 处理。 */
  comments?: number;
  replies?: number;
  page?: number;
  episode?: number;
  episodes?: number;
  limit?: number;
  phase?: CollectPhase;
  updated?: number;
}

const KEY = "dy.collect";
/** 评论/回复固定页大小，不向任务暴露。上游超过 50 会发生游标跳号。 */
export const COMMENT_PAGE_SIZE = 50;
/** 单次采集的翻页上限，防止游标异常导致死循环。 */
const MAX_PAGES = 2000;

/** 全局评论采集默认值；任务参数只在本次 runOne 内覆盖。0 表示不限额。 */
interface CollectLimits { level: 1 | 2; maxComments: number; maxRepliesPerComment: number }
const GLOBAL_LIMITS: CollectLimits = { level: 1, maxComments: 0, maxRepliesPerComment: 0 };
const CANCELLED = new Set<string>();
/** 运行中被删除的任务：不再回写，避免节流进度把行重新塞回列表。 */
const DROPPED = new Set<string>();
const LAST_PROGRESS = new Map<string, number>();

export function collectLimits(): CollectLimits { return { ...GLOBAL_LIMITS }; }
export function setCollectLimits(next: Partial<CollectLimits>): void {
  if (next.level !== undefined) GLOBAL_LIMITS.level = next.level === 2 ? 2 : 1;
  if (next.maxComments !== undefined) GLOBAL_LIMITS.maxComments = Math.max(0, Math.floor(next.maxComments) || 0);
  if (next.maxRepliesPerComment !== undefined) GLOBAL_LIMITS.maxRepliesPerComment = Math.max(0, Math.floor(next.maxRepliesPerComment) || 0);
}
function limitsOf(opts: CollectOptions): CollectLimits {
  return {
    level: opts.level === 1 || opts.level === 2 ? opts.level : GLOBAL_LIMITS.level,
    maxComments: opts.maxComments === undefined ? GLOBAL_LIMITS.maxComments : Math.max(0, Math.floor(opts.maxComments) || 0),
    maxRepliesPerComment: opts.maxRepliesPerComment === undefined ? GLOBAL_LIMITS.maxRepliesPerComment : Math.max(0, Math.floor(opts.maxRepliesPerComment) || 0),
  };
}
const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));
/** 单页失败重试 3 次；全部评论翻页多，429/空包下不整条中断。 */
async function pageRetry<T>(id: string, fn: () => Promise<T>): Promise<T> {
  let last: unknown;
  for (let i = 0; i < 3; i++) {
    checkStop(id);
    try { return await fn(); } catch (e) { last = e; await sleep(600 * (i + 1)); }
  }
  throw last;
}

export function runs(): Run[] {
  try { return JSON.parse(localStorage.getItem(KEY) || "[]"); } catch { return []; }
}
function writeRuns(list: Run[]): void {
  localStorage.setItem(KEY, JSON.stringify(list.slice(0, 100)));
}
function save(list: Run[]): void {
  writeRuns(list);
  document.dispatchEvent(new CustomEvent("collect-change"));
}
function persist(r: Run): void {
  if (isTauri()) void invokeOr("db_put_row", { bucket: "collect", id: r.id, json: JSON.stringify(r), ord: r.at }).catch(() => {});
}
export function putRun(r: Run): void {
  if (DROPPED.has(r.id)) return;
  save([r, ...runs().filter((x) => x.id !== r.id)]);
  persist(r);
}
/** 页级进度：默认只更新当前行并节流落盘，不触发整页重绘。 */
export function progressRun(r: Run): void {
  if (DROPPED.has(r.id)) return;
  const next = { ...r, updated: Date.now() };
  writeRuns([next, ...runs().filter((x) => x.id !== r.id)]);
  document.dispatchEvent(new CustomEvent<Run>("collect-progress", { detail: next }));
  const now = Date.now();
  if (now - (LAST_PROGRESS.get(r.id) ?? 0) >= 1500) {
    LAST_PROGRESS.set(r.id, now);
    persist(next);
  }
}
export function stopRun(id: string): void {
  CANCELLED.add(id);
  const r = runs().find((x) => x.id === id);
  if (r && r.state === "run") putRun({ ...r, state: "stopping", phase: "", updated: Date.now() });
}
function checkStop(id: string): void {
  if (CANCELLED.has(id)) throw new Error("__COLLECT_STOPPED__");
}
export function dropRun(id: string): void {
  // 运行中的任务删除即取消：进度和收尾都不再回写，行保持消失。
  if (runs().some((x) => x.id === id && (x.state === "run" || x.state === "stopping"))) {
    DROPPED.add(id); CANCELLED.add(id);
  }
  LAST_PROGRESS.delete(id);
  save(runs().filter((x) => x.id !== id));
  if (isTauri()) void invokeOr("db_del_row", { bucket: "collect", id }).catch(() => {});
}

export async function hydrateRuns(): Promise<void> {
  if (!isTauri()) return;
  const rows = await invokeOr<string[]>("db_load_rows", { bucket: "collect" }).catch(() => []);
  if (!rows.length) {
    for (const r of runs()) {
      void invokeOr("db_put_row", { bucket: "collect", id: r.id, json: JSON.stringify(r), ord: r.at }).catch(() => {});
    }
    return;
  }
  const parsed = rows.flatMap((json) => { try { return [JSON.parse(json) as Run]; } catch { return []; } });
  // 上次进程退出时仍在跑的任务已无 Promise 驱动，落成“已停止”，避免永久显示采集中。
  for (const r of parsed) {
    if (r.state !== "run" && r.state !== "stopping") continue;
    r.state = "stopped"; r.phase = ""; r.updated = Date.now(); r.err = "";
    void invokeOr("db_put_row", { bucket: "collect", id: r.id, json: JSON.stringify(r), ord: r.at }).catch(() => {});
  }
  localStorage.setItem(KEY, JSON.stringify(parsed.slice(0, 100)));
  document.dispatchEvent(new CustomEvent("collect-change"));
}

export interface CollectOptions {
  comments?: boolean;
  /** 1 = 仅一级评论；2 = 一级 + 二级回复。 */
  level?: 1 | 2;
  /** 整个任务的一级评论上限；0 = 全部。 */
  maxComments?: number;
  /** 每条一级评论的回复上限；0 = 全部。 */
  maxRepliesPerComment?: number;
  raw?: boolean;
}

/** 任务覆盖参数只对本次调用生效，不写入 Run，也不供重跑/定时复用。 */
export async function runOne(target: string, opts: CollectOptions = {}): Promise<Run> {
  const forced = target.match(/^(video|series|mix|collection):/i)?.[1]?.toLowerCase();
  let id = target.replace(/^(video|series|mix|collection):/i, "").trim();
  let group: "series" | "mix" | "" = forced === "series" ? "series" : forced === "mix" || forced === "collection" ? "mix" : "";
  const limits = limitsOf(opts);
  const run: Run = {
    id: crypto.randomUUID(), target: id, kind: "single", state: "run", rows: 0, path: "", err: "", at: Date.now(),
    comments: 0, replies: 0, page: 0, phase: "resolve", limit: limits.maxComments, updated: Date.now(),
  };
  putRun(run);
  let out: unknown[] = [];
  let path = `~/Documents/抖音台/数据/${id}/${id}.jsonl`;
  let commentRows = 0;
  let replyRows = 0;
  const report = () => progressRun({
    ...run, state: CANCELLED.has(run.id) ? "stopping" : "run",
    rows: out.length, comments: commentRows, replies: replyRows, limit: limits.maxComments,
  });
  const saveOutput = async (): Promise<string> => {
    const text = out.map((x) => JSON.stringify(x)).join("\n") + "\n";
    if (isTauri()) return invokeOr("save_text", { path, text });
    const a = document.createElement("a");
    a.href = URL.createObjectURL(new Blob([text], { type: "application/x-ndjson" }));
    a.download = `${id}.jsonl`; a.click();
    return path;
  };
  try {
    checkStop(run.id);
    if (!group && forced !== "video") {
      run.phase = "resolve"; report();
      const r = await resolve(target).catch(() => null);
      if (r?.type === "series") { group = "series"; id = String(r.id); }
      else if (r?.type === "mix") { group = "mix"; id = String(r.id); }
      else if (r?.type === "video") id = String(r.aweme_id ?? r.id);
      path = `~/Documents/抖音台/数据/${id}/${id}.jsonl`;
    }
    if (!group && !/\d{15,20}$/.test(id)) id = target.match(/\d{15,20}/)?.[0] ?? "";
    if (!id) throw new Error("认不出目标");

    const addComments = async (aid: string) => {
      if (opts.comments === false || (limits.maxComments > 0 && commentRows >= limits.maxComments)) return;
      checkStop(run.id);
      let cursor = 0;
      const seen = new Set<string>();
      // 评论：翻到 hasMore 为假、达到任务上限，或游标异常时停。
      for (let p = 0; p < MAX_PAGES; p++) {
        checkStop(run.id);
        if (limits.maxComments > 0 && commentRows >= limits.maxComments) break;
        run.phase = "comments"; run.page = p + 1; report();
        const prev = cursor;
        const page = await pageRetry(run.id, () => comments(aid, cursor, COMMENT_PAGE_SIZE, [...seen].slice(-200)));
        cursor = page.cursor;
        let fresh = 0;
        for (const c of page.items) {
          checkStop(run.id);
          if (limits.maxComments > 0 && commentRows >= limits.maxComments) break;
          const cid = String(c.cid ?? c.comment_id ?? "");
          if (cid && !seen.has(cid)) seen.add(cid); else continue;
          fresh++;
          commentRows++;
          out.push({ type: "comment", data: c });
          if (limits.level < 2 || (c.reply_comment_total ?? 0) <= 0) continue;
          let rc = 0;
          let threadReplies = 0;
          const seenReply = new Set<string>();
          run.phase = "replies"; report();
          for (let rp = 0; rp < MAX_PAGES && (limits.maxRepliesPerComment === 0 || threadReplies < limits.maxRepliesPerComment); rp++) {
            checkStop(run.id);
            const rPrev = rc;
            const r = await pageRetry(run.id, () => replies(aid, cid, rc, COMMENT_PAGE_SIZE)).catch(() => ({ items: [] as any[], meta: {} as any }));
            const items = r.items ?? [];
            for (const x of items) {
              checkStop(run.id);
              if (limits.maxRepliesPerComment > 0 && threadReplies >= limits.maxRepliesPerComment) break;
              const rid = String(x.cid ?? x.comment_id ?? "");
              if (rid && !seenReply.add(rid)) continue;
              threadReplies++;
              replyRows++;
              out.push({ type: "reply", data: x });
            }
            report();
            if (!r.meta?.has_more || !items.length) break;
            rc = Number(r.meta?.cursor ?? 0);
            if (!rc || rc === rPrev) break;
          }
          run.phase = "comments"; report();
        }
        const total = page.items.length;
        log("info", `采集评论 ${aid} 第 ${p + 1} 页 · 新增 ${fresh}/${total} · 累计 ${commentRows} 条`);
        report();
        if (!page.hasMore || !total || cursor === prev || !fresh || (limits.maxComments > 0 && commentRows >= limits.maxComments)) break;
      }
    };

    if (group) {
      run.episode = 0;
      const info = group === "series" ? await series(id) : await mix(id);
      out.push({ type: group, fetched_at: Date.now(), data: opts.raw === false ? { ...info, raw: undefined } : info });
      const pager = group === "series" ? seriesEpisodes : mixVideos;
      let cursor = "";
      const seenEp = new Set<string>();
      for (let p = 0; p < MAX_PAGES; p++) {
        checkStop(run.id);
        const prev = cursor;
        const page = await pageRetry(run.id, () => pager(id, cursor));
        for (const ep of page.items) {
          checkStop(run.id);
          if (!ep.id || !seenEp.add(ep.id)) continue;
          run.episode = seenEp.size;
          out.push({ type: "episode", data: ep });
          await addComments(ep.id);
          report();
          if (limits.maxComments > 0 && commentRows >= limits.maxComments) break;
        }
        cursor = page.cursor;
        if (!page.hasMore || !page.items.length || cursor === prev || (limits.maxComments > 0 && commentRows >= limits.maxComments)) break;
      }
    } else {
      run.phase = "video"; report();
      const detail = await video(id);
      out.push({ type: "video", fetched_at: Date.now(), data: opts.raw === false ? { ...detail, raw: undefined } : detail });
      await addComments(id);
    }
    checkStop(run.id);
    run.phase = "write"; report();
    path = await saveOutput();
    putRun({ ...run, state: "done", rows: out.length, comments: commentRows, replies: replyRows, path, phase: "", updated: Date.now() });
    log("info", `采集完成 ${id} · ${commentRows} 条评论 · ${replyRows} 条回复`);
    return { ...run, state: "done", rows: out.length, comments: commentRows, replies: replyRows, path, phase: "" };
  } catch (e) {
    if (CANCELLED.has(run.id)) {
      path = await saveOutput().catch(() => "");
      putRun({ ...run, state: "stopped", rows: out.length, comments: commentRows, replies: replyRows, path, phase: "", updated: Date.now() });
      log("info", `采集已停止 ${id} · ${commentRows} 条评论 · ${replyRows} 条回复`);
      return { ...run, state: "stopped", rows: out.length, comments: commentRows, replies: replyRows, path, phase: "" };
    }
    const err = String((e as Error).message || e);
    putRun({ ...run, state: "fail", rows: out.length, comments: commentRows, replies: replyRows, err, phase: "", updated: Date.now() });
    log("err", `采集失败 ${id}：${err}`);
    return { ...run, state: "fail", rows: out.length, comments: commentRows, replies: replyRows, err, phase: "" };
  } finally {
    CANCELLED.delete(run.id);
    LAST_PROGRESS.delete(run.id);
    DROPPED.delete(run.id);
  }
}
