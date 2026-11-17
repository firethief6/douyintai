import { esc, dur, nfmt } from "./lib/dom";
import type { Item } from "./lib/api";
import { isFavorite, libPut, toggleFavorite } from "./lib/store";
import { toast } from "./lib/dom";

export const card = (it: Item, from = ""): string => `
  <article class="card" data-id="${esc(it.id)}" data-open="${esc(it.id)}"${from ? ` data-from="${from}"` : ""} role="link" tabindex="0" title="${esc(it.desc)}">
    <div class="thumb">
      ${it.cover ? `<img src="${esc(it.cover)}" alt="" loading="lazy">` : `<div class="nocover"><svg class="i"><use href="#i-images"/></svg></div>`}
      ${it.kind === "gallery" ? `<span class="badge img">图集</span>` : ""}
      <span class="dur">${esc(dur(it.durationMs))}</span>
      <div class="acts">
        <button class="act" data-act="collect" title="采集本条" aria-label="采集本条"><svg class="i" aria-hidden="true"><use href="#i-database"/></svg></button>
        <button class="act ${isFavorite(it.id) ? "on" : ""}" data-act="fav" title="收藏" aria-label="收藏" aria-pressed="${isFavorite(it.id)}"><svg class="i" aria-hidden="true"><use href="#i-heart"/></svg></button>
      </div>
      <span class="tick"><svg class="i"><use href="#i-check"/></svg></span>
    </div>
    <div class="meta">
      <div class="desc">${esc(it.desc)}</div>
      <div class="by"${it.author.uid ? ` data-author="${esc(it.author.uid)}" title="打开作者主页"` : ""}><img src="${esc(it.author.avatar)}" alt=""><span>${esc(it.author.nickname)}</span><span class="likes"><svg class="i"><use href="#i-heart"/></svg>${esc(nfmt(it.likes))}</span></div>
    </div>
  </article>`;

export function bindCards(root: HTMLElement): void {
  root.addEventListener("keydown", (e) => {
    const el = (e.target as HTMLElement).closest<HTMLElement>(".card[data-open]");
    if (el && e.target === el && (e.key === "Enter" || e.key === " ")) { e.preventDefault(); el.click(); }
  });
  root.addEventListener("click", async (e) => {
    const t = (e.target as HTMLElement).closest<HTMLElement>("[data-act], [data-open]");
    if (!t) return;
    const cardEl = t.closest<HTMLElement>("[data-id]");
    const id = cardEl?.dataset.id;
    if (!id) return;
    const { itemById } = await import("./items");
    const it = itemById(id);

    if (t.dataset.act === "fav") {
      e.stopPropagation();
      if (!it) return;
      const on = toggleFavorite({ id: it.id, kind: it.kind, title: it.desc, author: it.author.nickname, cover: it.cover, at: Date.now() });
      t.classList.toggle("on", on);
      t.setAttribute("aria-pressed", String(on));
      t.setAttribute("aria-label", on ? "取消收藏" : "收藏");
      toast(on ? "已收藏" : "已取消收藏");
      return;
    }
    if (t.dataset.act === "collect") { e.stopPropagation(); document.dispatchEvent(new CustomEvent("ask-collect", { detail: `video:${id}` })); return; }
    const by = (e.target as HTMLElement).closest<HTMLElement>("[data-author]");
    if (by?.dataset.author) { e.stopPropagation(); location.hash = `#/user/${encodeURIComponent(by.dataset.author)}`; return; }
    if (t.dataset.open) {
      libPut("history", { id, kind: it?.kind ?? "video", title: it?.desc ?? id, author: it?.author.nickname ?? "", cover: it?.cover ?? "", at: Date.now() });
      const from = cardEl.dataset.from;
      location.hash = `#/stage/${id}${from ? `?from=${from}` : ""}`;
    }
  });
}
