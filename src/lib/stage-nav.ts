export type StageGroup = { kind: "series" | "mix"; id: string };

export type StepReason = "ok" | "first" | "last" | "missing";
export interface StepResult { id: string | null; index: number; reason: StepReason }

/** 播放页查询串里的分组上下文；短剧优先于合集。 */
export function groupOf(query: URLSearchParams): StageGroup | null {
  for (const kind of ["series", "mix"] as const) {
    const id = (query.get(kind) ?? "").trim();
    if (id) return { kind, id };
  }
  return null;
}

export function groupQuery(group: StageGroup): string {
  return `?${group.kind}=${encodeURIComponent(group.id)}`;
}

/** 详情原始数据自带的短剧归属，用于没有分组参数的深链。 */
export function groupFromRaw(raw: unknown): StageGroup | null {
  const native = (raw as any)?.native_series?.series_id;
  if (native !== undefined && native !== null && String(native).trim()) return { kind: "series", id: String(native).trim() };
  const info = (raw as any)?.series_info;
  const series = info?.series_id;
  if (series !== undefined && series !== null && String(series).trim()) {
    return { kind: Number(info?.content_sub_type) === 3 ? "mix" : "series", id: String(series).trim() };
  }
  const mixed = (raw as any)?.mix_info?.mix_id;
  if (mixed !== undefined && mixed !== null && String(mixed).trim()) return { kind: "mix", id: String(mixed).trim() };
  return null;
}

/** 在服务端给出的顺序里前后移动，不循环。 */
export function stepIn(ids: string[], current: string, step: -1 | 1): StepResult {
  const index = ids.indexOf(current);
  if (index < 0) return { id: null, index: -1, reason: "missing" };
  const at = index + step;
  if (at < 0) return { id: null, index, reason: "first" };
  if (at >= ids.length) return { id: null, index, reason: "last" };
  return { id: ids[at] ?? null, index, reason: "ok" };
}

const size = (value: unknown): number => {
  const n = Number(value);
  return Number.isFinite(n) && n > 0 ? n : 0;
};

const ratioOf = (box: any): number => {
  const width = size(box?.width);
  const height = size(box?.height);
  return width && height ? width / height : 0;
};

/** 从详情里取已知画幅；没有可用尺寸时返回 0。 */
export function knownVideoRatio(raw: unknown): number {
  const video = (raw as any)?.video;
  const direct = ratioOf(video);
  if (direct) return direct;
  const variants = Array.isArray(video?.bit_rate) ? video.bit_rate : [];
  for (const variant of variants) {
    const ratio = ratioOf(variant?.play_addr);
    if (ratio) return ratio;
  }
  return 0;
}

/** 从详情里取已知画幅；没有可用尺寸时按竖屏处理。 */
export function videoRatio(raw: unknown): number {
  return knownVideoRatio(raw) || 9 / 16;
}
