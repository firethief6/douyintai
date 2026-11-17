import type { Item } from "./lib/api";

const cache = new Map<string, Item>();

export function remember(items: Item[]): void {
  for (const it of items) cache.set(it.id, it);
  if (cache.size > 600) {
    const keep = [...cache.keys()].slice(-400);
    for (const k of cache.keys()) if (!keep.includes(k)) cache.delete(k);
  }
}
export const itemById = (id: string): Item | undefined => cache.get(id);
export const all = (): Item[] => [...cache.values()];
