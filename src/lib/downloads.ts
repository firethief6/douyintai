// 统一下载队列的轻量视图：状态全部来自 pipeline 的 downloads 表，不另存队列。
import { downloadUrl } from "./api";
import { toast } from "./dom";
import { isTauri } from "./store";
import {
  cancelDownload, deleteDownload, listDownloads, pauseDownload, resumeDownload,
  retryDownload, submitVideoWithCover, type Download,
} from "./tasks";

export interface Task {
  id: string; url: string; dest: string; name: string; title?: string; author?: string; state: string;
  done: number; total: number; speed: number; err: string;
}

const mem = new Map<string, Task>();
const raw = new Map<string, Download>();
const subs = new Set<(t: Task[]) => void>();
const rawSubs = new Set<(d: Download[]) => void>();
let timer = 0;

const asTask = (d: Download): Task => ({
  id: d.id, url: d.url, dest: d.path || "", name: d.title || d.asset_key,
  title: d.title || undefined, author: d.author || undefined,
  state: d.state === "running" ? "run" : d.state === "queued" ? "queue" : d.state === "paused" ? "pause" : d.state === "failed" ? "fail" : d.state,
  done: d.bytes || 0, total: d.expected_bytes || 0, speed: d.speed, err: d.error || "",
});

const publish = () => {
  subs.forEach((f) => f([...mem.values()]));
  rawSubs.forEach((f) => f([...raw.values()]));
};

export const subscribe = (f: (t: Task[]) => void): (() => void) => { subs.add(f); f([...mem.values()]); return () => subs.delete(f); };
export const subscribeRaw = (f: (d: Download[]) => void): (() => void) => { rawSubs.add(f); f([...raw.values()]); return () => rawSubs.delete(f); };
export const tasks = (): Task[] => [...mem.values()];

export async function initDownloads(): Promise<void> {
  if (timer) return;
  const tick = async () => {
    if (!isTauri() || document.hidden) return;
    try {
      const rows = await listDownloads();
      mem.clear();
      raw.clear();
      for (const d of rows) {
        mem.set(d.id, asTask(d));
        raw.set(d.id, d);
      }
      publish();
    } catch { /* 后端未就绪时下一轮再取 */ }
  };
  document.addEventListener("visibilitychange", () => { if (!document.hidden) void tick(); });
  await tick();
  timer = window.setInterval(tick, 2000);
}

export interface ResolvedDownload { url: string; expect_bytes?: number; ext?: string; bit_rate?: number; width?: number; height?: number; gear?: string }

export async function enqueue(
  it: { id: string; desc: string; cover?: string; author: { nickname: string }; kind: string },
  opts: { gear?: string; resolved?: ResolvedDownload } = {},
): Promise<Task | undefined> {
  const r = opts.resolved ?? await downloadUrl(it.id, opts.gear);
  const ext = r.ext || "mp4";
  if (!isTauri()) {
    window.open(r.url, "_blank");
    toast("浏览器预览：已打开直链，桌面端会直接落盘");
    return undefined;
  }
  const d = await submitVideoWithCover(`video:${it.id}`, {
    url: r.url, title: it.desc, author: it.author.nickname, cover: it.cover,
    ext, expected_bytes: r.expect_bytes,
  });
  const t = asTask(d);
  mem.set(t.id, t);
  publish();
  return t;
}

export async function signal(id: string, kind: "pause" | "resume" | "cancel"): Promise<void> {
  if (kind === "pause") await pauseDownload(id);
  else if (kind === "resume") {
    const t = mem.get(id);
    if (t?.state === "fail" || t?.state === "cancel") await retryDownload(id);
    else await resumeDownload(id);
  } else await cancelDownload(id);
}

export async function removeTask(id: string): Promise<void> {
  await deleteDownload(id, "keep");
  mem.delete(id);
  publish();
}
