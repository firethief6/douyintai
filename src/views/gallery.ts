import { $$, errorBox, esc, toast } from "../lib/dom";
import { video } from "../lib/api";
import { itemById, remember } from "../items";
import { invokeOr, isTauri, settings } from "../lib/store";
import type { View } from "../router";
import { submitDownload } from "../lib/tasks";

const view: View = {
  title: "图集",
  async render(ctx) {
    let it = itemById(ctx.arg);
    if (!it || !Array.isArray((it.raw as any)?.images)) { try { it = await video(ctx.arg); remember([it]); } catch (e) { return `<section class="view active">${errorBox(e)}</section>`; } }
    const cells = ((it.raw.images as any[]) ?? []).map((x) => ({
      img: x?.download_url_list?.[0] ?? x?.url_list?.[0] ?? "",
      live: x?.video?.play_addr?.url_list?.[0] ?? "",
    })).filter((x) => x.img);
    const lives = cells.filter((x) => x.live).length;
    return `
    <section class="view active">
      <div class="pagehead between"><div><h1 class="h1">图集 · ${cells.length} 图${lives ? ` · ${lives} 段实况` : ""}</h1><div class="sub">${esc(it.desc)}</div></div>
        <div class="row"><button class="btn sm" data-act="zip">打包 ZIP</button>${lives ? `<button class="btn sm" data-act="dlive">采集实况</button>` : ""}<button class="btn sm primary" data-act="dlsel">采集所选</button></div></div>
      <div class="gwall${cells.length === 1 ? " single" : cells.length === 2 ? " double" : ""}" id="gwall">${cells.map((x, i) => `<div class="gimg" role="checkbox" aria-checked="false" tabindex="0" data-url="${esc(x.img)}" data-live="${esc(x.live)}" data-name="${i + 1}.jpg"><img src="${esc(x.img)}" loading="lazy">${x.live ? `<span class="badge img" style="left:8px;right:auto">实况</span>` : ""}<span class="num">${String(i + 1).padStart(2, "0")} / ${String(cells.length).padStart(2, "0")}</span><span class="tick"><svg class="i"><use href="#i-check"/></svg></span></div>`).join("")}</div>
    </section>`;
  },
  mount(ctx) {
    ctx.content.addEventListener("keydown", (e) => {
      const cell = (e.target as HTMLElement).closest<HTMLElement>(".gimg[data-url]");
      if (cell && e.target === cell && (e.key === "Enter" || e.key === " ")) { e.preventDefault(); cell.click(); }
    });
    ctx.content.addEventListener("click", async (e) => {
      const t = (e.target as HTMLElement).closest<HTMLElement>("[data-url],[data-act]");
      if (!t) return;
      if (t.dataset.url) {
        t.classList.toggle("picked");
        t.setAttribute("aria-checked", String(t.classList.contains("picked")));
        return;
      }
      if (t.dataset.act === "zip") {
        if (!isTauri()) { toast("浏览器开发模式下不打包", "err"); return; }
        const picked = $$(".gimg.picked", ctx.content);
        const urls = (picked.length ? picked : $$(".gimg", ctx.content)).map((x) => x.dataset.url!);
        const btn = t as HTMLButtonElement;
        btn.disabled = true; btn.textContent = "打包中…";
        try {
          const path = await invokeOr<string>("gallery_zip", {
            urls, dir: `${settings().downloadDir}/图集/${ctx.arg}`, name: ctx.arg,
          });
          toast(`已打包 ${urls.length} 张`);
          invokeOr("reveal", { path }).catch(() => {});
        } catch (e) { toast(`打包失败：${String((e as Error).message || e)}`, "err"); }
        btn.disabled = false; btn.textContent = "打包 ZIP";
        return;
      }
      if (t.dataset.act === "dlsel" || t.dataset.act === "dlive") {
        const all = $$(".gimg", ctx.content);
        const picked = (all.some((x) => x.classList.contains("picked")) ? all.filter((x) => x.classList.contains("picked")) : all)
          .map((x) => ({ url: x.dataset.url!, live: x.dataset.live!, name: x.dataset.name! }));
        const onlyLive = t.dataset.act === "dlive";
        if (onlyLive && !picked.some((x) => x.live)) { toast("所选没有实况", "err"); return; }
        if (!onlyLive && !picked.length) { toast("先点图勾选", "err"); return; }
        let n = 0;
        for (const [i, p] of picked.entries()) {
          if (!onlyLive) {
            if (isTauri()) {
              await submitDownload(`video:${ctx.arg}`, { asset_kind: "gallery", target_key: `video:${ctx.arg}:gallery:${i + 1}`, url: p.url, title: p.name.replace(".jpg", ""), cover: p.url, ext: "jpg" });
            } else window.open(p.url, "_blank");
            n++;
          }
          if (p.live) {
            if (isTauri()) {
              await submitDownload(`video:${ctx.arg}`, { asset_kind: "video", target_key: `video:${ctx.arg}:live:${i + 1}`, url: p.live, title: p.name.replace(".jpg", " 实况"), cover: p.url, ext: "mp4" });
            } else window.open(p.live, "_blank");
            n++;
          }
        }
        toast(`已加入 ${n} 项`);
      }
    });
  },
};
export default view;
