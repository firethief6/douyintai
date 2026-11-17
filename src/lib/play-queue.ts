import { all } from "../items";
import { advance, originOf, queueQuery, registeredList, type PlayOrigin } from "./play-list";
import { prefetchPlay } from "./play-source";
import { groupOf, type StageGroup, type StepResult } from "./stage-nav";
import { lib } from "./store";

const FROM: PlayOrigin[] = ["feed", "search", "history", "favorites"];

export type { PlayOrigin } from "./play-list";
export { registerPlayList } from "./play-list";

export interface PlayQueue {
  origin: PlayOrigin;
  group: StageGroup | null;
  query(): string;
  ids(): Promise<string[]>;
  pickIds(): Promise<string[]>;
  step(current: string, dir: -1 | 1): Promise<StepResult>;
  prefetch(current: string): void;
}

const videoIds = (rows: Array<{ id: string; kind: string }>): string[] =>
  rows.filter((r) => r.kind === "video" && r.id).map((r) => r.id);

async function groupIds(group: StageGroup): Promise<string[]> {
  const { loadGroupIds } = await import("./stage-cache");
  return loadGroupIds(group);
}

async function idsOf(origin: PlayOrigin, group: StageGroup | null): Promise<string[]> {
  if (origin === "series" || origin === "mix") return group ? groupIds(group) : [];
  if (origin === "history" || origin === "favorites") return registeredList(origin) ?? videoIds(lib(origin));
  return registeredList(origin) ?? videoIds(all());
}

/** 由查询串构造队列；短剧/合集按服务端顺序不循环，其余列表循环。 */
export function queueFor(query: URLSearchParams, fallback?: StageGroup | null): PlayQueue {
  // 分组只描述当前条目属于哪部剧，决定标题和选集；切条顺序由显式来源优先。
  const group = groupOf(query) ?? fallback ?? null;
  const from = originOf(query, false);
  const origin = from === "other" ? group?.kind ?? "other" : from;
  const wrap = origin !== "series" && origin !== "mix";
  return {
    origin,
    group,
    query: () => FROM.includes(origin)
      ? queueQuery(origin)
      : group
        ? queueQuery(group.kind, group.kind === "series" ? group.id : undefined, group.kind === "mix" ? group.id : undefined)
        : "",
    ids: () => idsOf(origin, group),
    pickIds: () => group ? groupIds(group) : idsOf(origin, group),
    step: async (current, dir) => {
      const hit = advance(await idsOf(origin, group), current, dir, wrap);
      // 从列表进入后又用分组选集跳到列表外时，退回到分组内的相邻集。
      if (hit.reason === "missing" && group) return advance(await groupIds(group), current, dir, false);
      return hit;
    },
    prefetch(current) {
      void this.step(current, 1).then((r) => { if (r.id && r.id !== current) prefetchPlay(r.id); }).catch(() => {});
    },
  };
}
