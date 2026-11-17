export interface FilterItem { kind?: string; createTime?: number }

export interface DownloadFilter {
  kind: "all" | "video" | "gallery";
  from: number;
  to: number;
  limit: number;
}

/** 从原始清单派生筛选结果，不修改输入。 */
export function filterIds(
  ids: readonly string[],
  itemOf: (id: string) => FilterItem | undefined,
  f: DownloadFilter,
): string[] {
  const next = ids.filter((id) => {
    const item = itemOf(id);
    if (f.kind !== "all" && item && item.kind !== f.kind) return false;
    if (f.kind === "gallery" && !item) return false;
    if (f.from && item?.createTime && item.createTime < f.from) return false;
    if (f.to && item?.createTime && item.createTime > f.to) return false;
    return true;
  });
  return f.limit > 0 ? next.slice(0, f.limit) : next;
}
