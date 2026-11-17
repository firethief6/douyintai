import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { isTauri, settings, type LogLevel } from "./store";

export interface Line { at: number; level: "debug" | "info" | "warn" | "err"; scope: string; text: string }

const RANK: Record<LogLevel, number> = { debug: 0, info: 1, warn: 2, error: 3 };
const buf: Line[] = [];
const subs = new Set<(l: Line[]) => void>();

const threshold = (): number => RANK[settings().logLevel] ?? RANK.info;
const allowed = (level: Line["level"]): boolean => RANK[level === "err" ? "error" : level] >= threshold();
const visible = (): Line[] => buf.filter((l) => allowed(l.level));
const publish = (): void => { const rows = visible(); subs.forEach((f) => f(rows)); };

export function log(level: Line["level"], text: string, scope = "ui"): void {
  if (!allowed(level)) return;
  const line: Line = { at: Date.now(), level, scope, text: String(text) };
  buf.unshift(line);
  if (buf.length > 1000) buf.pop();
  publish();
  if (isTauri()) {
    void invoke("ui_log", { level, scope, text: line.text }).catch(() => {});
  }
}
export const lines = (): Line[] => visible();
export const onLog = (f: (l: Line[]) => void): (() => void) => { subs.add(f); f(visible()); return () => subs.delete(f); };

let listening = false;
export async function initLog(): Promise<void> {
  if (listening || !isTauri()) return;
  listening = true;
  await listen<Line>("log://line", (e) => {
    const line = e.payload;
    if (!allowed(line.level)) return;
    if (buf.some((x) => x.scope === line.scope && x.text === line.text && Math.abs(x.at - line.at) < 1000)) return;
    buf.unshift(line);
    if (buf.length > 1000) buf.pop();
    publish();
  });
  const old = await invoke<Line[]>("log_tail", { limit: 1000 }).catch(() => []);
  if (old.length) {
    buf.splice(0, buf.length, ...old.slice(0, 1000));
    publish();
  }
}

export async function backendTail(limit = 1000): Promise<Line[]> {
  if (!isTauri()) return visible().slice(0, limit);
  return invoke<Line[]>("log_tail", { limit }).catch(() => []);
}
export async function clearBackend(): Promise<void> {
  if (isTauri()) await invoke("log_clear").catch(() => {});
  buf.splice(0, buf.length);
  publish();
}
export async function setLogLevel(level: LogLevel): Promise<void> {
  if (isTauri()) await invoke<string>("log_set_level", { levelName: level });
  const old = await backendTail(1000);
  buf.splice(0, buf.length, ...old);
  publish();
}
export async function exportDiagnostics(): Promise<string> {
  if (!isTauri()) return "";
  return invoke<string>("log_export");
}
export async function logPath(): Promise<string> {
  if (!isTauri()) return "";
  return invoke<string>("log_path").catch(() => "");
}
export async function revealPath(path: string): Promise<void> {
  if (isTauri() && path) await invoke("reveal", { path }).catch(() => {});
}
