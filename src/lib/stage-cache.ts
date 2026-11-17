import { comments, gears, mediaSrc, mixVideos, playUrl, seriesEpisodes, video, type Gear, type Item } from "./api";
import type { StageGroup } from "./stage-nav";
import { remember } from "../items";
import { supportsGear } from "./playback-quality";

interface Entry<T> { value: T; at: number; ttl: number }

class TtlLru<T> {
  private rows = new Map<string, Entry<T>>();
  constructor(private readonly limit: number) {}

  get(key: string): T | undefined {
    const row = this.rows.get(key);
    if (!row) return undefined;
    if (Date.now() - row.at > row.ttl) { this.rows.delete(key); return undefined; }
    this.rows.delete(key);
    this.rows.set(key, row);
    return row.value;
  }

  set(key: string, value: T, ttl: number): void {
    this.rows.delete(key);
    this.rows.set(key, { value, at: Date.now(), ttl });
    while (this.rows.size > this.limit) {
      const oldest = this.rows.keys().next().value as string | undefined;
      if (!oldest) break;
      this.rows.delete(oldest);
    }
  }
}

const videos = new TtlLru<Item>(80);
const plays = new TtlLru<{ url: string; src: string; keyErr?: string; enc: boolean }>(200);
const commentPages = new TtlLru<{ items: any[]; hasMore: boolean; cursor: number }>(240);
const videoFlights = new Map<string, Promise<Item>>();
const playFlights = new Map<string, Promise<{ url: string; src: string; keyErr?: string; enc: boolean }>>();
const gearFlights = new Map<string, Promise<Gear[]>>();
const supportedGears = new TtlLru<Gear[]>(80);
const commentFlights = new Map<string, Promise<{ items: any[]; hasMore: boolean; cursor: number }>>();
const groupIds = new TtlLru<string[]>(40);
const groupFlights = new Map<string, Promise<string[]>>();

const single = <T>(flights: Map<string, Promise<T>>, key: string, run: () => Promise<T>): Promise<T> => {
  const old = flights.get(key);
  if (old) return old;
  const next = run().finally(() => flights.delete(key));
  flights.set(key, next);
  return next;
};

/** 作品详情短时复用，避免前进后退重复请求。 */
export function loadVideo(id: string): Promise<Item> {
  const hit = videos.get(id);
  if (hit) return Promise.resolve(hit);
  return single(videoFlights, id, async () => {
    const item = await video(id);
    videos.set(id, item, 5 * 60_000);
    remember([item]);
    return item;
  });
}

/** 播放地址有时效，缓存时间短于地址通常的有效期。 */
export function loadPlay(id: string, gear?: Gear): Promise<{ url: string; src: string; keyErr?: string; enc: boolean }> {
  const key = `${id}:${gear?.name ?? "default"}:${gear?.h264 ?? true}`;
  const hit = plays.get(key);
  if (hit) return Promise.resolve(hit);
  return single(playFlights, key, async () => {
    const { url, key: mediaKey, key_err } = await playUrl(id, gear);
    const src = await mediaSrc(url, mediaKey, key_err);
    const value = { url, src, keyErr: key_err, enc: !!mediaKey };
    plays.set(key, value, 8 * 60_000);
    return value;
  });
}

export function loadSupportedGears(id: string): Promise<Gear[]> {
  const hit = supportedGears.get(id);
  if (hit) return Promise.resolve(hit);
  return single(gearFlights, id, async () => {
    const list = await gears(id);
    const supported = await Promise.all(list.map(g => supportsGear(g)));
    const result = list.filter((_, i) => supported[i]);
    supportedGears.set(id, result, 5 * 60_000);
    return result;
  });
}

export function loadComments(id: string, cursor = 0, count = 20, seen: string[] = []) {
  const key = `${id}:${cursor}:${count}:${cursor === 0 && !seen.length ? "" : seen.slice(-200).join(",")}`;
  const hit = commentPages.get(key);
  if (hit) return Promise.resolve(hit);
  return single(commentFlights, key, async () => {
    const page = await comments(id, cursor, count, seen);
    commentPages.set(key, page, 2 * 60_000);
    return page;
  });
}

/** 按服务端顺序取全集/合集的作品 ID；分页上限与页面采集一致。 */
export function loadGroupIds(group: StageGroup): Promise<string[]> {
  const key = `${group.kind}:${group.id}`;
  const hit = groupIds.get(key);
  if (hit) return Promise.resolve(hit);
  return single(groupFlights, key, async () => {
    const ids: string[] = [];
    let cursor = "";
    for (let page = 0; page < 20; page++) {
      const r = group.kind === "series"
        ? await seriesEpisodes(group.id, cursor)
        : await mixVideos(group.id, cursor);
      ids.push(...r.items.map((x) => x.id).filter(Boolean));
      if (!r.hasMore || !r.cursor || r.cursor === cursor) break;
      cursor = r.cursor;
    }
    const unique = [...new Set(ids)];
    groupIds.set(key, unique, 5 * 60_000);
    return unique;
  });
}

/** 预取相邻条目的支持档位和首屏评论。 */
export function prefetchStage(ids: string[]): void {
  for (const id of [...new Set(ids)].slice(0, 2)) {
    // 相邻条目仅预取档位；最高画质在用户进入时加载，避免额外占用带宽。
    void loadSupportedGears(id).catch(() => {});
    void loadComments(id, 0, 20).catch(() => {});
  }
}

