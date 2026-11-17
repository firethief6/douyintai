import { invoke } from "@tauri-apps/api/core";
import { isTauri, settings } from "./store";
import { log } from "./log";
import { toast } from "./dom";

export class ApiError extends Error {
  constructor(public status: number, message: string, public kind?: string) { super(message); }
}

/** 新设备被服务端风控：通常过一两天自动恢复。 */
export const isWarmingError = (e: unknown): boolean =>
  e instanceof ApiError
  && (e.message.includes("-99999") || e.message.includes("风控期") || e.message.includes("身份池"));

/** 签名失败只提示一次，附带跳到签名设置的入口。 */
let lastSignHint = 0;
function hintSign(e: ApiError): void {
  if (e.kind !== "sign_failed") return;
  const now = Date.now();
  if (now - lastSignHint < 5000) return;
  lastSignHint = now;
  toast(e.message, "err", {
    label: "去切换",
    run: () => { location.hash = "#/settings?tab=conn"; },
  });
}

type Q = Record<string, string | number | boolean | undefined>;

/** 响应可能是多份 JSON 拼接（服务端偶发）：逐份解析，优先 status_code=0 且数据非空的那份。 */
function parseDocs(text: string): unknown {
  const docs: any[] = [];
  let depth = 0, start = -1, inStr = false, esc = false;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (inStr) {
      if (esc) esc = false;
      else if (c === "\\") esc = true;
      else if (c === '"') inStr = false;
      continue;
    }
    if (c === '"') { inStr = true; continue; }
    if (c === "{" || c === "[") { if (depth++ === 0) start = i; continue; }
    if (c === "}" || c === "]") {
      if (depth > 0 && --depth === 0 && start >= 0) {
        try { docs.push(JSON.parse(text.slice(start, i + 1))); } catch { /* 跳过坏段 */ }
        start = -1;
      }
    }
  }
  const rich = (v: any) => ["comments", "aweme_list", "business_data", "challenge_list", "user_list"]
    .some((k) => Array.isArray(v?.[k]) ? v[k].length : v?.[k] && typeof v[k] === "object" ? Object.keys(v[k]).length : false);
  return docs.find((d) => (d?.status_code ?? 0) === 0 && rich(d)) ?? docs[0] ?? text;
}

const normalize = (v: unknown): unknown => {
  if (typeof v !== "string") return v;
  const t = v.trim();
  return t.startsWith("{") || t.startsWith("[") ? parseDocs(t) : v;
};

async function transport(path: string, query: Q, method = "GET", body?: unknown): Promise<unknown> {
  const started = performance.now();
  const { baseUrl, token } = settings();
  const qs = new URLSearchParams();
  for (const [k, v] of Object.entries(query)) if (v !== undefined && v !== "") qs.set(k, String(v));
  const url = `${baseUrl.replace(/\/$/, "")}${path}${qs.size ? "?" + qs : ""}`;

  log("debug", `→ ${method} ${path}?${[...qs].map(([k, v]) => `${k}=${v.slice(0, 60)}`).join("&")}`, "api-ui");
  window.dispatchEvent(new CustomEvent("api-start", { detail: path }));
  try {
    let out: unknown;
    if (isTauri()) {
      try {
        out = normalize(await invoke("api_call", { baseUrl, token, path, query: Object.fromEntries(qs), method, body }));
      } catch (e) {
        const err = e as { status?: number; message?: string; kind?: string } | string;
        const ae = typeof err === "object" && err?.status
          ? new ApiError(err.status, err.message || `HTTP ${err.status}`, err.kind)
          : new ApiError(0, String(typeof err === "string" ? err : err?.message || e));
        hintSign(ae);
        throw ae;
      }
    } else {
      const res = await fetch(url, { method, headers: { ...(token ? { Authorization: `Bearer ${token}` } : {}), ...(body === undefined ? {} : { "Content-Type": "application/json" }) }, body: body === undefined ? undefined : JSON.stringify(body) });
      const text = await res.text();
      if (!res.ok) {
        let kind: string | undefined;
        let message = text.slice(0, 400);
        try {
          const j = JSON.parse(text) as { message?: string; kind?: string };
          kind = j.kind;
          if (j.message) message = j.message;
        } catch { /* 非 JSON 错误体 */ }
        const ae = new ApiError(res.status, message, kind);
        hintSign(ae);
        throw ae;
      }
      out = normalize(text);
    }
    const ms = Math.round(performance.now() - started);
    log("debug", `← ${method} ${path} ok ${ms}ms`, "api-ui");
    if (ms > 800) log("warn", `${path} 慢 ${ms}ms`, "api-ui");
    window.dispatchEvent(new CustomEvent("api-ok", { detail: path }));
    return out;
  } catch (e) {
    const status = e instanceof ApiError ? e.status : 0;
    const msg = e instanceof ApiError ? `${e.status} ${e.message}` : String((e as Error)?.message || e);
    log("debug", `← ${method} ${path} fail status=${status} ${Math.round(performance.now() - started)}ms`, "api-ui");
    log("err", `${path} 失败 ${Math.round(performance.now() - started)}ms ${msg}`, "api-ui");
    window.dispatchEvent(new CustomEvent("api-fail", { detail: { path, status, message: msg } }));
    throw e;
  } finally {
    window.dispatchEvent(new CustomEvent("api-end", { detail: path }));
  }
}

export const call = <T>(path: string, query: Q = {}) => transport(path, query) as Promise<T>;
export const post = <T>(path: string, body: unknown = {}, query: Q = {}) => transport(path, query, "POST", body) as Promise<T>;
export const del = <T>(path: string, query: Q = {}) => transport(path, query, "DELETE", {}) as Promise<T>;
export const patch = <T>(path: string, body: unknown = {}) => transport(path, {}, "PATCH", body) as Promise<T>;

interface Page<T> { items: T[]; meta: { has_more?: boolean; cursor?: string | null; search_id?: string | null; search_request_id?: string | null } }

/* ---------- 归一化 ---------- */
export interface Author { uid: string; account: string; nickname: string; avatar: string }
export interface Item {
  id: string; desc: string; cover: string; durationMs: number; createTime: number;
  kind: "video" | "gallery" | "live"; author: Author; comments: number; likes: number;
  shares: number; play: number; shareUrl: string; raw: Record<string, unknown>;
}

const str = (v: unknown): string => (typeof v === "string" ? v : "");
const num = (v: unknown): number => (typeof v === "number" ? v : 0);
const firstUrl = (v: unknown): string => Array.isArray(v) ? str(v[0]) : str(v);

export function normItem(x: any): Item {
  const raw = x?.raw ?? {};
  const video = raw.video ?? {};
  const images = Array.isArray(raw.images) ? raw.images : [];
  // 图集接口给的是 heic，WebKit 不解码；优先带 webp 的下载直链。
  const imgUrl = (i: any): string => str(i?.download_url_list?.[0]) || str(i?.url_list?.[0]);
  const author = x?.author ?? raw.author ?? {};
  return {
    id: str(x?.aweme_id ?? raw.aweme_id),
    desc: str(x?.desc ?? raw.desc) || "无标题",
    cover: imgUrl(images[0]) || str(x?.cover) || str(video.cover) || firstUrl(video.cover?.url_list),
    durationMs: num(x?.duration_ms) || num(video.duration) || (video.width && video.height ? 0 : 0),
    createTime: num(x?.create_time ?? raw.create_time),
    kind: x?.kind === "gallery" || images.length ? "gallery" : "video",
    author: {
      // 主页接口只认 sec_uid，作者对象可能只有数字 uid。
      uid: str(author.sec_uid) || str(author.uid),
      account: str(author.unique_id) || str(author.short_id),
      nickname: str(author.nickname) || "未知作者",
      avatar: str(author.avatar) || firstUrl(author.avatar_thumb?.url_list),
    },
    comments: num(x?.statistics?.comment_count),
    likes: num(x?.statistics?.digg_count),
    shares: num(x?.statistics?.share_count),
    play: num(x?.statistics?.play_count),
    shareUrl: str(x?.share_url ?? raw.share_url),
    raw: { ...raw, images },
  };
}

export interface PageResult<T> { items: T[]; hasMore: boolean; cursor: string }
export interface SearchPage extends PageResult<Item> { searchId: string; searchRequestId: string }

const normPage = (p: Page<any>): PageResult<Item> => ({
  items: (p.items ?? []).map(normItem).filter((i) => i.id),
  hasMore: !!p.meta?.has_more,
  cursor: String(p.meta?.cursor ?? ""),
});

/* ---------- 内容 ---------- */
export const feed = async (count = 12): Promise<{ items: Item[]; hasMore: boolean }> => {
  const p = await call<Page<any>>("/feed", { count });
  return normPage(p);
};

export const video = async (id: string): Promise<Item> => normItem(await call<any>(`/video/${id}`));
export interface Danmaku { id: string; text: string; offset_ms: number; color: string }
export const danmaku = async (id: string, cursor = 0): Promise<{ items: Danmaku[]; hasMore: boolean; cursor: number }> => {
  const p = await call<{ items: Danmaku[]; meta: { has_more?: boolean; cursor?: string | number } }>(`/video/${id}/danmaku`, { cursor });
  return { items: p.items ?? [], hasMore: !!p.meta?.has_more, cursor: Number(p.meta?.cursor ?? 0) };
};

export const playUrl = (id: string, gear?: Gear) =>
  call<{ kind: string; url: string; gear: string; width: number; height: number; bit_rate: number; key?: string; key_err?: string }>(
    `/video/${id}/play`, gear ? { gear: gear.name || gear.quality, hevc: gear.h264 ? 0 : 1 } : {});

/** CDN 会拒掉 WebView 的本地 Referer，走端内代理补头；加密集带 CENC 内容密钥。 */
export const mediaSrc = async (url: string, key?: string, keyErr?: string): Promise<string> => {
  if (!isTauri()) {
    const k = key ? `&k=${encodeURIComponent(key)}` : "";
    return `${settings().baseUrl.replace(/\/$/, "")}/media?u=${encodeURIComponent(url)}${k}`;
  }
  return invoke<string>("media_url", { url, key: key || null, keyErr: keyErr || null });
};
export interface Gear { name: string; quality: string; width: number; height: number; bit_rate: number; size: number; h264: boolean }
export const gears = async (id: string) =>
  ((await call<{ items: Gear[] }>(`/video/${id}/gears`)).items ?? []);
export const downloadUrl = (id: string, gear?: string) =>
  call<{ url: string; expect_bytes?: number; ext?: string; bit_rate?: number; width?: number; height?: number; gear?: string }>(
    `/video/${id}/download`, gear ? { gear } : {});
export const music = (id: string) =>
  call<{ url: string; expect_bytes?: number; ext?: string; title?: string; author?: string }>(`/video/${id}/music`);
export const coverUrl = (id: string) =>
  call<{ url: string; expect_bytes?: number; ext?: string }>(`/video/${id}/cover`);
export const comments = async (id: string, cursor = 0, count = 20, seen: string[] = []): Promise<{ items: any[]; hasMore: boolean; cursor: number }> => {
  const p = await call<Page<any>>(`/video/${id}/comments`, { cursor, count, seen: seen.join(",") });
  return {
    items: (p.items ?? []).map((c) => ({ ...c, raw: undefined })),
    hasMore: !!p.meta?.has_more,
    cursor: Number(p.meta?.cursor ?? cursor),
  };
};
export const replies = (id: string, cid: string, cursor = 0, count = 20) => call<Page<any>>(`/video/${id}/replies`, { comment: cid, cursor, count });

export const hot = async () => {
  const p = await call<Page<any>>("/hot");
  return (p.items ?? []).map((h) => ({ word: str(h.word), value: num(h.hot_value), pos: num(h.position) }));
};

export const searchSug = async (target: string): Promise<string[]> => {
  if (!target.trim()) return [];
  const p = await call<Page<any>>("/search/sug", { target });
  return (p.items ?? []).map((x) => str(x.word)).filter(Boolean);
};

export const searchVideos = async (q: string, cursor = "", searchId = "", searchRequestId = ""): Promise<SearchPage> => {
  const p = await call<Page<any>>("/search", { target: q, cursor, search_id: searchId, search_request_id: searchRequestId });
  return {
    items: (p.items ?? []).map(normItem).filter((i) => i.id),
    hasMore: !!p.meta?.has_more,
    cursor: String(p.meta?.cursor ?? ""),
    searchId: String(p.meta?.search_id ?? searchId),
    searchRequestId: String(p.meta?.search_request_id ?? searchRequestId),
  };
};
export interface UserSearchPage extends PageResult<ReturnType<typeof normUser>> { searchId: string }
export const searchUsers = async (q: string, cursor = "", searchId = ""): Promise<UserSearchPage> => {
  const p = await call<Page<any>>("/search/users", { target: q, cursor, search_id: searchId });
  return {
    items: (p.items ?? []).map(normUser),
    hasMore: !!p.meta?.has_more,
    cursor: String(p.meta?.cursor ?? ""),
    searchId: String(p.meta?.search_id ?? searchId),
  };
};
const normUser = (u: any) => ({
  uid: str(u.sec_uid) || str(u.uid), numericUid: str(u.uid) || str(u.sec_uid), account: str(u.unique_id) || str(u.short_id),
  nickname: str(u.nickname), avatar: str(u.avatar), signature: str(u.signature), followers: num(u.follower_count), raw: u.raw,
});
const seriesInflight = new Map<string, Promise<PageResult<ReturnType<typeof normSeries>>>>();
export const searchSeries = (q: string): Promise<PageResult<ReturnType<typeof normSeries>>> => {
  const key = q.trim();
  const running = seriesInflight.get(key);
  if (running) return running;
  const run = call<Page<any>>("/search/series", { target: key })
    .then((p) => ({ items: (p.items ?? []).map(normSeries), hasMore: !!p.meta?.has_more, cursor: String(p.meta?.cursor ?? "") }))
    .finally(() => seriesInflight.delete(key));
  seriesInflight.set(key, run);
  return run;
};
export interface TopicSearchPage extends PageResult<ReturnType<typeof normTopic>> { searchId: string }
export const searchTopics = async (q: string, cursor = "", searchId = ""): Promise<TopicSearchPage> => {
  const p = await call<Page<any>>("/search/challenges", { target: q, cursor, search_id: searchId });
  return {
    items: (p.items ?? []).map(normTopic),
    hasMore: !!p.meta?.has_more,
    cursor: String(p.meta?.cursor ?? ""),
    searchId: String(p.meta?.search_id ?? searchId),
  };
};

export interface User { uid: string; account: string; nickname: string; avatar: string; signature: string; followers: number; raw: any }
export const user = async (uid: string): Promise<User> => {
  const u = await call<any>(`/user/${uid}`);
  return { uid: str(u.uid) || str(u.sec_uid) || uid, account: str(u.unique_id) || str(u.short_id), nickname: str(u.nickname), avatar: str(u.avatar), signature: str(u.signature), followers: num(u.follower_count), raw: u };
};
export const userPosts = async (uid: string, cursor = "") => {
  const p = await call<Page<any>>(`/user/${uid}/posts`, { count: 20, cursor });
  return normPage(p);
};
export const userMixes = async (uid: string, cursor = ""): Promise<PageResult<Mix>> => {
  const p = await call<Page<any>>(`/user/${uid}/mixes`, { cursor });
  return { items: (p.items ?? []).map(normMix), hasMore: !!p.meta?.has_more, cursor: String(p.meta?.cursor ?? "") };
};
export const userSeries = async (uid: string, cursor = ""): Promise<PageResult<Series>> => {
  const p = await call<Page<any>>(`/user/${uid}/series`, { cursor });
  return { items: (p.items ?? []).map(normSeries), hasMore: !!p.meta?.has_more, cursor: String(p.meta?.cursor ?? "") };
};

export const resolve = (target: string) => call<any>("/resolve", { target, json: true });

export interface Topic { name: string; views: number; videos: number; desc: string; raw: any }
const normTopic = (t: any): Topic => ({
  name: str(t.challenge_name ?? t.name), views: num(t.view_count ?? t.views),
  videos: num(t.video_count), desc: str(t.desc), raw: t,
});
export const topic = async (name: string) => {
  const t = await call<any>(`/challenge/${encodeURIComponent(name)}`);
  return normTopic(t);
};
export const topicVideos = async (name: string, sort = 0, cursor = ""): Promise<PageResult<Item>> => {
  const p = await call<Page<any>>(`/challenge/${encodeURIComponent(name)}/videos`, { sort, cursor });
  return normPage(p);
};

/* ---------- 合集 / 短剧 / 直播 ---------- */
export interface Series { id: string; name: string; cover: string; episodes: number; charge: boolean; intro: string; raw: any }
const normSeries = (s: any): Series => ({
  id: str(s.series_id ?? s.series?.series_id), name: str(s.series_name ?? s.series?.series_name) || "未命名短剧",
  cover: str(s.cover ?? s.cover_url) || firstUrl(s.cover_url?.url_list), episodes: num(s.episode_count),
  charge: !!s.is_charge, intro: str(s.intro ?? s.description), raw: s.raw ?? s,
});
export const seriesWall = async (cursor = "", count = 10): Promise<PageResult<Series>> => {
  const p = await call<Page<any>>("/series/wall", { cursor, count });
  return { items: (p.items ?? []).map(normSeries), hasMore: !!p.meta?.has_more, cursor: String(p.meta?.cursor ?? "") };
};
export const series = async (id: string) => normSeries(await call<any>(`/series/${id}`));
export const seriesEpisodes = async (id: string, cursor = ""): Promise<PageResult<Item>> => {
  const p = await call<Page<any>>(`/series/${id}/episodes`, { cursor });
  return normPage(p);
};
export interface Mix { id: string; name: string; cover: string; count: number; author: string; raw: any }
const normMix = (m: any): Mix => ({
  id: str(m.mix_id ?? m.mix_info?.mix_id), name: str(m.mix_name ?? m.mix_info?.mix_name) || "合集",
  cover: str(m.cover) || firstUrl(m.cover_url?.url_list), count: num(m.aweme_count ?? m.statistics?.aweme_count),
  author: str(m.author?.nickname ?? m.mix_info?.author?.nickname), raw: m,
});
export const mix = async (id: string, seed?: Mix): Promise<Mix> => {
  const r = await call<any>(`/mix/${id}/videos`).catch(() => null);
  const meta = r?.meta?.mix_info ?? r?.meta?.mix ?? null;
  return meta ? normMix(meta) : (seed ?? { id, name: "合集", cover: "", count: 0, author: "", raw: {} });
};
export const mixVideos = async (id: string, cursor = ""): Promise<PageResult<Item>> => {
  const p = await call<Page<any>>(`/mix/${id}/videos`, { cursor });
  return normPage(p);
};

export interface Room { id: string; title: string; cover: string; nick: string; avatar: string; users: number; flv: string; hls: string; raw: any }
const normRoom = (r: any): Room => ({
  id: str(r.room_id), title: str(r.title) || "直播", cover: str(r.raw?.cover?.url_list?.[0]) || str(r.cover),
  nick: str(r.owner?.nickname), avatar: str(r.owner?.avatar), users: num(r.user_count),
  flv: str(r.flv), hls: str(r.hls), raw: r.raw,
});
export const liveFeed = async (cursor = ""): Promise<PageResult<Room>> => {
  // 上游一批只有 6 个，让后端合并成 18 个一页，减少滚动停顿。
  const p = await call<Page<any>>("/live/feed", { count: 18, cursor });
  return { items: (p.items ?? []).map(normRoom).filter((r) => r.id), hasMore: !!p.meta?.has_more, cursor: String(p.meta?.cursor ?? "") };
};
export const userLive = async (uid: string): Promise<Room> => {
  const r = await call<any>(`/user/${encodeURIComponent(uid)}/live`);
  return normRoom(r);
};
export const liveResolve = async (room: string) => {
  const r = await call<any>("/live/resolve", { target: room });
  return normRoom(r);
};
export const livePull = (room: string) => call<{ url?: string; out?: string }>(`/live/${room}/pull`, { seconds: 6, out: "" });
export const liveMessages = (room: string, cursor = "", internal_ext = "") =>
  call<{ items: { method: string; content: string; user: { nickname: string } }[]; cursor: string; internal_ext: string; interval_ms: number }>(
    `/live/${room}/messages`, { cursor, internal_ext });

/* ---------- 系统 ---------- */
export interface SignInfo {
  backend: "native" | "emu";
  env?: "native" | "emu" | null;
  native: { ok: number; err: number };
  emu: { ok: number; err: number };
  last: { backend: "native" | "emu"; at: number; message: string } | null;
}
export const signStatus = () => isTauri()
  ? invoke<{ ok: boolean; sign: SignInfo }>("sign_status")
  : call<{ ok: boolean; sign: SignInfo }>("/health");
export const setSignBackend = (backend: "native" | "emu") =>
  invoke<{ ok: boolean; sign: SignInfo }>("set_sign_backend", { mode: backend });
export interface IdentityPool {
  target: number; total: number; available: number; cooling: number; retired: number;
  dead: number; creating: number; paused: boolean; last_error: string;
}
export const identityStatus = () => invoke<{ ok: boolean; pool: IdentityPool }>("identity_status");
export const identitySetTarget = (target: number) => invoke<{ ok: boolean; pool: IdentityPool }>("identity_set_target", { target });
export const identityPause = (paused: boolean) => invoke<{ ok: boolean; pool: IdentityPool }>("identity_pause", { paused });
export const identityPrepare = () => invoke<{ ok: boolean; pool: IdentityPool }>("identity_prepare");
export const identityClear = () => invoke<{ ok: boolean; pool: IdentityPool }>("identity_clear");
export interface ProxyRow { url: string; ok: boolean; ms: number }
export interface ProxyConfig { ok: boolean; list: string; enabled: boolean; cdn: boolean }
export const proxyGet = () => invoke<ProxyConfig>("proxy_get");
export const proxyStatus = () => invoke<{ ok: boolean; count: number; enabled: boolean }>("proxy_status");
export const proxySet = (list: string, enabled: boolean, cdn: boolean) =>
  invoke<{ ok: boolean; count: number; enabled: boolean; cdn: boolean }>("proxy_set", { list, enabled, cdn });
export const proxyTest = (list: string) =>
  invoke<{ ok: boolean; usable: number; total: number; rows: ProxyRow[] }>("proxy_test", { list });
export const health = () => call<{ ok: boolean }>("/health");
export const probe = () => call<Record<string, unknown>>("/probe");
export const version = () => call<Record<string, unknown>>("/version");
export const caps = () => call<Array<{ name: string; route: string; help: string; login?: boolean }>>("/caps");
