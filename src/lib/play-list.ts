import { stepIn, type StepResult } from "./stage-nav.ts";

/** 进入播放页时所在的列表，决定「上一条/下一条」和左栏高亮。 */
export type PlayOrigin = "feed" | "search" | "history" | "favorites" | "series" | "mix" | "other";

const FROM: PlayOrigin[] = ["feed", "search", "history", "favorites"];
const lists = new Map<PlayOrigin, string[]>();

/** 列表页按自己的显示顺序登记；播放页只读。 */
export function registerPlayList(origin: PlayOrigin, ids: string[]): void {
  lists.set(origin, [...new Set(ids.filter(Boolean))]);
}

export const registeredList = (origin: PlayOrigin): string[] | undefined => lists.get(origin);

/** 历史和收藏本身就是观看记录，重播时不再回写历史。 */
export const recordsHistory = (origin: PlayOrigin): boolean => origin !== "history" && origin !== "favorites";

/** 从查询串识别来源；分组参数优先于 from。 */
export function originOf(query: URLSearchParams, grouped: boolean): PlayOrigin {
  if (grouped) return query.has("series") ? "series" : "mix";
  const raw = (query.get("from") ?? "").trim() as PlayOrigin;
  return FROM.includes(raw) ? raw : "other";
}

/** 切条时要带上的上下文参数。 */
export function queueQuery(origin: PlayOrigin, series?: string, mix?: string): string {
  if (origin === "series" && series) return `?series=${encodeURIComponent(series)}`;
  if (origin === "mix" && mix) return `?mix=${encodeURIComponent(mix)}`;
  return FROM.includes(origin) && origin !== "other" ? `?from=${origin}` : "";
}

/** 纯推进：wrap 为真时首尾相接，否则到边界就停。 */
export function advance(ids: string[], current: string, dir: -1 | 1, wrap: boolean): StepResult {
  const hit = stepIn(ids, current, dir);
  if (!wrap || hit.reason === "ok" || hit.reason === "missing") return hit;
  if (hit.reason === "last") return { id: ids[0] ?? null, index: hit.index, reason: "ok" };
  return { id: ids[ids.length - 1] ?? null, index: hit.index, reason: "ok" };
}
