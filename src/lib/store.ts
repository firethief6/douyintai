import { invoke } from "@tauri-apps/api/core";

export const isTauri = (): boolean => "__TAURI_INTERNALS__" in window;

export type LogLevel = "debug" | "info" | "warn" | "error";

export interface Settings {
  baseUrl: string;
  token: string;
  downloadDir: string;
  naming: string;
  concurrency: number;
  skipExisting: boolean;
  theme: "dark" | "light";
  firstRun: boolean;
  danmaku: boolean;
  danmakuOpacity: number;
  danmakuSize: number;
  signBackend: "native" | "emu";
  identityPoolSize: number;
  proxyEnabled: boolean;
  proxyList: string;
  proxyCdn: boolean;
  logLevel: LogLevel;
}

const DEFAULTS: Settings = {
  baseUrl: "http://127.0.0.1:8765",
  token: "",
  downloadDir: "~/Videos/抖音台",
  naming: "{author}/{date}_{desc}_{id}",
  concurrency: 3,
  skipExisting: true,
  theme: "dark",
  firstRun: true,
  danmaku: true,
  danmakuOpacity: 0.9,
  danmakuSize: 16,
  signBackend: "native",
  identityPoolSize: 64,
  proxyEnabled: false,
  proxyList: "",
  proxyCdn: false,
  logLevel: "info",
};

let cache: Settings | null = null;

export function settings(): Settings {
  if (!cache) {
    try { cache = { ...DEFAULTS, ...JSON.parse(localStorage.getItem("dy.settings") || "{}") }; }
    catch { cache = { ...DEFAULTS }; }
  }
  return cache!;
}

export function saveSettings(patch: Partial<Settings>): Settings {
  cache = { ...settings(), ...patch };
  const json = JSON.stringify(cache);
  localStorage.setItem("dy.settings", json);
  if (isTauri()) void invoke("db_set_settings", { json }).catch(() => {});
  return cache;
}

export function applyTheme(): void {
  const theme = settings().theme;
  document.documentElement.dataset.theme = theme;
  document.querySelector("#themeBtn use")?.setAttribute("href", theme === "dark" ? "#i-sun" : "#i-moon");
}

export interface LibItem {
  id: string;
  kind: "video" | "gallery" | "live" | "series";
  title: string;
  author: string;
  cover: string;
  path?: string;
  size?: number;
  at: number;
  favorite?: boolean;
}

type LibKind = "favorites" | "history" | "downloaded";

const LIB_KEY = "dy.library";

function libAll(): Record<string, LibItem[]> {
  try { return JSON.parse(localStorage.getItem(LIB_KEY) || "{}"); } catch { return {}; }
}

function saveLib(all: Record<string, LibItem[]>): void {
  localStorage.setItem(LIB_KEY, JSON.stringify(all));
}

export function lib(kind: LibKind, q = ""): LibItem[] {
  const all = libAll();
  let rows = all[kind] ?? [];
  if (q) rows = rows.filter((r) => (r.title + r.author).toLowerCase().includes(q.toLowerCase()));
  return [...rows].sort((a, b) => b.at - a.at);
}

export function libPut(kind: LibKind, item: LibItem): void {
  const all = libAll();
  const rows = (all[kind] ?? []).filter((r) => r.id !== item.id);
  const row = { ...item, at: Date.now() };
  rows.unshift(row);
  all[kind] = rows.slice(0, 500);
  saveLib(all);
  if (isTauri()) void invoke("db_put_row", { bucket: `lib:${kind}`, id: row.id, json: JSON.stringify(row), ord: row.at }).catch(() => {});
  document.dispatchEvent(new CustomEvent("lib-change"));
}

export function libDel(kind: LibKind, id: string): void {
  const all = libAll();
  all[kind] = (all[kind] ?? []).filter((r) => r.id !== id);
  saveLib(all);
  if (isTauri()) void invoke("db_del_row", { bucket: `lib:${kind}`, id }).catch(() => {});
  document.dispatchEvent(new CustomEvent("lib-change"));
}

export async function hydrateStore(): Promise<void> {
  if (!isTauri()) return;
  const raw = await invoke<string | null>("db_get_settings").catch(() => null);
  if (raw) {
    try { cache = { ...DEFAULTS, ...JSON.parse(raw) }; localStorage.setItem("dy.settings", raw); } catch { /* 坏记录忽略 */ }
  } else if (cache) {
    void invoke("db_set_settings", { json: JSON.stringify(cache) }).catch(() => {});
  }
  const all = libAll();
  for (const kind of ["favorites", "history", "downloaded"] as const) {
    const rows = await invoke<string[]>("db_load_rows", { bucket: `lib:${kind}` }).catch(() => []);
    if (!rows.length) {
      for (const r of all[kind] ?? []) {
        void invoke("db_put_row", { bucket: `lib:${kind}`, id: r.id, json: JSON.stringify(r), ord: r.at }).catch(() => {});
      }
      continue;
    }
    all[kind] = rows.flatMap((json) => { try { return [JSON.parse(json) as LibItem]; } catch { return []; } });
  }
  saveLib(all);
}

export function isFavorite(id: string): boolean {
  return lib("favorites").some((r) => r.id === id);
}

export function toggleFavorite(item: LibItem): boolean {
  if (isFavorite(item.id)) { libDel("favorites", item.id); return false; }
  libPut("favorites", item); return true;
}

export const invokeOr = async <T>(cmd: string, args?: Record<string, unknown>, fallback?: () => T | Promise<T>): Promise<T> => {
  if (isTauri()) return invoke<T>(cmd, args);
  if (!fallback) throw new Error("需要桌面端");
  return fallback();
};
