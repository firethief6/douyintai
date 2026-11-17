import { $, closeModal, esc, modal, toast } from "./dom";
import { mediaSrc } from "./api";

export interface PreviewItem {
  id: string;
  path: string;
  kind: string;
  title?: string | null;
  target_key?: string | null;
  cover?: string | null;
  missing?: boolean;
}

const cache = new Map<string, string>();
const MAX_CACHE = 80;
async function source(path: string): Promise<string> {
  const hit = cache.get(path);
  if (hit) return hit;
  const src = await mediaSrc(path).catch(() => path);
  cache.set(path, src);
  if (cache.size > MAX_CACHE) cache.delete(cache.keys().next().value as string);
  return src;
}

/** 统一的视频、图片、音频预览器。方向键切换，图片支持缩放与拖动。 */
export async function preview(items: PreviewItem[], index: number): Promise<void> {
  if (!items.length) return;
  let i = Math.max(0, Math.min(index, items.length - 1));
  let scale = 1, x = 0, y = 0, drag = false, ox = 0, oy = 0;
  const scrim = modal(`<div class="lightbox"><div class="lbhead"><b id="lbtitle"></b><span class="dim" id="lbnum"></span><button class="iconbtn" data-close aria-label="关闭"><svg class="i"><use href="#i-close"/></svg></button></div><div class="lbbody"><button class="lbnav prev" data-nav="-1" aria-label="上一个">‹</button><div class="lbview" id="lbview"></div><button class="lbnav next" data-nav="1" aria-label="下一个">›</button></div><div class="lbfoot"><button class="btn sm ghost" data-zoom="-1">缩小</button><b id="lbzoom">100%</b><button class="btn sm ghost" data-zoom="1">放大</button><button class="btn sm ghost" data-reset>复位</button><button class="btn sm ghost" data-reveal>目录</button></div></div>`, { width: 980 });
  const box = $("#lbview", scrim)!;
  let video: HTMLVideoElement | null = null;
  let paintSeq = 0;

  const preload = async (at: number) => {
    const a = items[at];
    if (!a || a.missing) return;
    const src = await source(a.path).catch(() => "");
    if (!src) return;
    if (a.kind === "video" || a.kind === "record" || a.kind === "music") {
      const v = document.createElement("video");
      v.preload = "metadata";
      v.src = src;
    } else {
      const img = new Image();
      img.src = src;
    }
  };

  const paint = async () => {
    const mine = ++paintSeq;
    const a = items[i];
    if (!a) { closeModal(); return; }
    $("#lbtitle", scrim)!.textContent = a.title || a.target_key || a.path.split("/").at(-1) || "预览";
    $("#lbnum", scrim)!.textContent = `${i + 1} / ${items.length}`;
    scale = 1; x = y = 0;
    $("#lbzoom", scrim)!.textContent = "100%";
    if (a.missing) {
      video = null;
      box.innerHTML = `<div class="missingbig"><b>文件已移动或删除</b><span class="mono">${esc(a.path)}</span></div>`;
      return;
    }
    box.innerHTML = `<div class="lbloading"><span class="spinner"></span>读取中…</div>`;
    const src = await source(a.path);
    if (mine !== paintSeq) return;
    const a2 = items[i];
    if (!a2) return;
    if (a2.kind === "video" || a2.kind === "record" || a2.kind === "music") {
      box.innerHTML = `<video controls autoplay playsinline preload="auto" src="${esc(src)}"></video><div class="lbhint" hidden></div>`;
      video = $("video", box);
      if (!video) return;
      if (a2.kind === "record") {
        const hint = $(".lbhint", box) as HTMLElement;
        const show = (text: string) => { hint.textContent = text; hint.hidden = false; };
        video.addEventListener("loadedmetadata", () => {
          if (!Number.isFinite(video!.duration) || video!.duration < 0.1) show("这份录制没有分片（空录制），目录已创建但内容为空。");
        }, { once: true });
        video.addEventListener("error", () => show("录制清单无法播放，可打开目录检查分片。"), { once: true });
        window.setTimeout(() => { if (video && video.readyState < 1) show("等待录制清单…"); }, 3000);
      }
      const key = `dy.asset.progress.${a2.id}`;
      const at = Number(localStorage.getItem(key) || 0);
      video.addEventListener("loadedmetadata", () => { if (at > 0 && Number.isFinite(video?.duration)) video!.currentTime = Math.min(at, video!.duration - 1); }, { once: true });
      let last = 0;
      video.addEventListener("timeupdate", () => { if (video && Date.now() - last > 1500) { last = Date.now(); localStorage.setItem(key, String(video.currentTime)); } });
    } else {
      video = null;
      box.innerHTML = `<img draggable="false" src="${esc(src)}" alt=""><div class="lbhint" hidden>图片无法读取，可能已移动。</div>`;
      $("img", box)?.addEventListener("error", () => { const h = $(".lbhint", box) as HTMLElement | null; if (h) h.hidden = false; }, { once: true });
    }
    void preload((i + 1) % items.length);
    void preload((i - 1 + items.length) % items.length);
  };

  scrim.addEventListener("click", async (e) => {
    const el = (e.target as HTMLElement).closest<HTMLElement>("[data-close],[data-nav],[data-zoom],[data-reset],[data-reveal]");
    if (!el) return;
    if (el.hasAttribute("data-close")) { closeModal(); return; }
    if (el.dataset.nav) { i = (i + Number(el.dataset.nav) + items.length) % items.length; await paint(); return; }
    if (el.dataset.zoom) { scale = Math.max(0.25, Math.min(5, scale + Number(el.dataset.zoom) * 0.25)); const img = $<HTMLImageElement>("img", box); if (img) img.style.transform = `translate(${x}px,${y}px) scale(${scale})`; $("#lbzoom", scrim)!.textContent = `${Math.round(scale * 100)}%`; return; }
    if (el.hasAttribute("data-reset")) { scale = 1; x = y = 0; const img = $<HTMLImageElement>("img", box); if (img) img.style.transform = ""; $("#lbzoom", scrim)!.textContent = "100%"; return; }
    if (el.hasAttribute("data-reveal")) { await import("./store").then(({ invokeOr }) => invokeOr("reveal", { path: items[i].path })).catch(() => toast("打开失败", "err")); }
  });
  box.addEventListener("wheel", (e) => { if (!video) { e.preventDefault(); scale = Math.max(0.25, Math.min(5, scale + (e.deltaY < 0 ? 0.2 : -0.2))); const img = $<HTMLImageElement>("img", box); if (img) img.style.transform = `translate(${x}px,${y}px) scale(${scale})`; $("#lbzoom", scrim)!.textContent = `${Math.round(scale * 100)}%`; } }, { passive: false });
  box.addEventListener("pointerdown", (e) => { if (video) return; drag = true; ox = e.clientX - x; oy = e.clientY - y; box.setPointerCapture(e.pointerId); });
  box.addEventListener("pointermove", (e) => { if (!drag || video) return; x = e.clientX - ox; y = e.clientY - oy; const img = $<HTMLImageElement>("img", box); if (img) img.style.transform = `translate(${x}px,${y}px) scale(${scale})`; });
  box.addEventListener("pointerup", () => { drag = false; });
  const key = (e: KeyboardEvent) => {
    if (e.key === "Escape") { closeModal(); return; }
    if (e.key === "ArrowLeft") { i = (i - 1 + items.length) % items.length; void paint(); }
    if (e.key === "ArrowRight") { i = (i + 1) % items.length; void paint(); }
  };
  document.addEventListener("keydown", key, { signal: (scrim as unknown as { __signal: AbortSignal }).__signal });
  await paint();
}
