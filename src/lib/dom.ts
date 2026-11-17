import { invoke } from "@tauri-apps/api/core";
import { isTauri } from "./store";

export const $ = <T extends Element = HTMLElement>(sel: string, root: ParentNode = document): T | null =>
  root.querySelector<T>(sel);
export const $$ = <T extends Element = HTMLElement>(sel: string, root: ParentNode = document): T[] =>
  [...root.querySelectorAll<T>(sel)];

export const esc = (s: unknown): string =>
  String(s ?? "").replace(/[&<>"']/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" }[c]!));

export const nfmt = (n: number | null | undefined): string => {
  if (n == null) return "—";
  if (n >= 1e8) return (n / 1e8).toFixed(1) + "亿";
  if (n >= 1e4) return (n / 1e4).toFixed(1) + "万";
  return String(n);
};

export const dur = (ms: number | null | undefined): string => {
  if (!ms) return "";
  const s = Math.round(ms / 1000);
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, "0")}`;
};

export const bytes = (n: number | null | undefined): string => {
  if (!n) return "—";
  const u = ["B", "KB", "MB", "GB"];
  let i = 0;
  let v = n;
  while (v >= 1024 && i < u.length - 1) { v /= 1024; i++; }
  return `${v.toFixed(i ? 1 : 0)} ${u[i]}`;
};

export const ago = (ts: number | null | undefined): string => {
  if (!ts) return "";
  const d = Math.floor(Date.now() / 1000) - ts;
  if (d < 60) return "刚刚";
  if (d < 3600) return `${Math.floor(d / 60)} 分钟前`;
  if (d < 86400) return `${Math.floor(d / 3600)} 小时前`;
  if (d < 86400 * 30) return `${Math.floor(d / 86400)} 天前`;
  return new Date(ts * 1000).toLocaleDateString("zh-CN");
};

export function toast(
  text: string,
  kind: "ok" | "err" = "ok",
  action?: { label: string; run: () => void },
): void {
  const el = document.createElement("div");
  el.className = "toast" + (kind === "err" ? " err" : "");
  el.setAttribute("role", kind === "err" ? "alert" : "status");
  const body = document.createElement("span");
  body.className = "toast-text";
  body.textContent = text;
  el.append(body);
  if (action) {
    const btn = document.createElement("button");
    btn.className = "btn sm";
    btn.textContent = action.label;
    btn.addEventListener("click", () => { action.run(); dismiss(); });
    el.append(btn);
    el.classList.add("action");
  }
  const close = document.createElement("button");
  close.className = "toast-x";
  close.setAttribute("aria-label", "关闭通知");
  close.innerHTML = `<svg class="i" aria-hidden="true"><use href="#i-close"/></svg>`;
  close.addEventListener("click", () => dismiss());
  el.append(close);

  let host = document.getElementById("toasts");
  if (!host) {
    host = document.createElement("div");
    host.id = "toasts";
    host.className = "toasts";
    host.setAttribute("aria-live", "polite");
    document.body.append(host);
  }
  while (host.children.length >= 3) host.firstElementChild?.remove();
  host.append(el);
  requestAnimationFrame(() => el.classList.add("in"));

  let left = kind === "err" ? 0 : action ? 8000 : 2600;
  let started = 0;
  let timer = 0;
  const run = () => {
    if (!left) return;
    started = Date.now();
    timer = window.setTimeout(dismiss, left);
  };
  const pause = () => { if (!left) return; window.clearTimeout(timer); left -= Date.now() - started; };
  const resume = () => { if (left > 0) run(); };
  function dismiss(): void {
    window.clearTimeout(timer);
    left = 0;
    el.classList.remove("in");
    window.setTimeout(() => el.remove(), 180);
  }
  el.addEventListener("pointerenter", pause);
  el.addEventListener("pointerleave", resume);
  el.addEventListener("focusin", pause);
  el.addEventListener("focusout", resume);
  run();
}

let scrim: HTMLElement | null = null;
let restoreFocus: HTMLElement | null = null;

export function modal(html: string, opts: { width?: number; onClose?: () => void } = {}): HTMLElement {
  closeModal();
  restoreFocus = document.activeElement instanceof HTMLElement ? document.activeElement : null;
  scrim = document.createElement("div");
  scrim.className = "scrim show";
  scrim.innerHTML = `<div class="dialog" style="width:${opts.width ?? 480}px" role="dialog" aria-modal="true">${html}</div>`;
  const dialog = scrim.firstElementChild as HTMLElement;
  const title = dialog.querySelector<HTMLElement>("h3");
  if (title) {
    title.id = "dialogTitle";
    dialog.setAttribute("aria-labelledby", "dialogTitle");
  }
  scrim.addEventListener("click", (e) => { if (e.target === scrim) closeModal(); });
  scrim.addEventListener("keydown", (e) => {
    if (e.key === "Escape") { e.preventDefault(); closeModal(); return; }
    if (e.key !== "Tab") return;
    const items = [...dialog.querySelectorAll<HTMLElement>('a[href], button:not([disabled]), input:not([disabled]):not([type="hidden"]), select:not([disabled]), textarea:not([disabled]), [tabindex="0"]')];
    if (!items.length) { e.preventDefault(); return; }
    const first = items[0];
    const last = items[items.length - 1];
    if (e.shiftKey && document.activeElement === first) { e.preventDefault(); last.focus(); }
    else if (!e.shiftKey && document.activeElement === last) { e.preventDefault(); first.focus(); }
  });
  document.getElementById("app")?.setAttribute("inert", "");
  document.body.append(scrim);
  requestAnimationFrame(() => {
    const first = dialog.querySelector<HTMLElement>('[autofocus], input:not([type="hidden"]), select, textarea, button, [tabindex="0"]');
    (first ?? dialog).focus();
  });
  return scrim;
}
export function closeModal(): void {
  const previous = restoreFocus;
  restoreFocus = null;
  scrim?.remove();
  scrim = null;
  document.getElementById("app")?.removeAttribute("inert");
  previous?.focus();
}

export function empty(title: string, hint = "", action = ""): string {
  return `<div class="empty"><div class="art"><svg class="i" style="width:26px;height:26px"><use href="#i-inbox"/></svg></div><b>${esc(title)}</b><div>${esc(hint)}</div>${action}</div>`;
}

export function skeletonCards(n = 10): string {
  return Array.from({ length: n }, () => `<div class="card"><div class="thumb skel"></div><div class="skel-line"></div><div class="skel-line short"></div></div>`).join("");
}

export function skeleton(n = 10): string {
  return `<div class="wall">${skeletonCards(n)}</div>`;
}

export function friendlyError(error: unknown): string {
  const raw = String((error as Error)?.message ?? error ?? "").trim();
  if (/failed to fetch|networkerror|load failed|network request failed|error sending request|dns error|connection (?:refused|reset)|err_network|err_internet|timed? ?out|timeout|超时/i.test(raw)) {
    return "网络连接失败，请检查网络后重试";
  }
  if (/^0\s|status.?0/i.test(raw)) return "网络连接失败，请检查网络后重试";
  if (/^(?:500|502|503|504)\b/i.test(raw) || /status_code["']?\s*:\s*(?:500|502|503|504)/i.test(raw)) return "服务暂不可用，请稍后重试";
  if (/^429\b/i.test(raw)) return "请求过于频繁，请稍后重试";
  if (/^401\b/i.test(raw)) return "访问凭据无效或已过期";
  return raw || "未知错误";
}

export function errorBox(error: unknown, retry = "retry"): string {
  return `<div class="errbox" role="alert"><svg class="i"><use href="#i-alert"/></svg><div><b>加载失败</b><div class="dim">${esc(friendlyError(error))}</div></div><button class="btn sm" data-act="${retry}">重试</button></div>`;
}

export function loading(text = "加载中…"): string {
  return `<div class="inline-loading" role="status" aria-live="polite"><span class="spinner"></span><span>${esc(text)}</span></div>`;
}

export async function copy(text: string, done = "已复制"): Promise<void> {
  try {
    if (isTauri()) await invoke("clipboard_write", { text });
    else await navigator.clipboard.writeText(text);
    toast(done);
  } catch { toast("复制失败", "err"); }
}

export function debounce<A extends unknown[]>(fn: (...a: A) => void, ms: number): (...a: A) => void {
  let t = 0;
  return (...a: A) => { clearTimeout(t); t = window.setTimeout(() => fn(...a), ms); };
}
