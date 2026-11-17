import { $, closeModal, dur, empty, errorBox, esc, modal, nfmt, skeletonCards, toast } from "../lib/dom";
import { loadMore, type LoadMore } from "../lib/infinite";
import { mix, mixVideos, series, seriesEpisodes, seriesWall } from "../lib/api";
import { remember } from "../items";
import { go, render, type Ctx, type View } from "../router";

const view: View = {
  title: "短剧",
  cacheable: true,
  async render(ctx) {
    if (ctx.arg) return detail(ctx.arg);
    return `
    <section class="view active">
      <div class="pagehead between">
        <div><h1 class="h1">短剧</h1><div class="sub" id="swallSub">按播放发现 · 付费段单独标记</div></div>
        <div class="row">
          <button class="btn sm ghost" data-act="series-search"><svg class="i" style="width:14px;height:14px"><use href="#i-search"/></svg>搜索短剧</button>
          <button class="btn sm ghost" data-act="series-refresh"><svg class="i" style="width:14px;height:14px"><use href="#i-retry"/></svg>刷新</button>
        </div>
      </div>
      <div id="swall" class="wall">${skeletonCards(10)}</div>
      <div class="hintline" id="swallHint"></div>
    </section>`;
  },
  mount(ctx) {
    if (ctx.arg) {
      const groupQ = `?series=${encodeURIComponent(ctx.arg)}`;
      ctx.content.addEventListener("keydown", (e) => {
        const ep = (e.target as HTMLElement).closest<HTMLElement>(".ep[data-open]");
        if (ep && e.target === ep && (e.key === "Enter" || e.key === " ")) { e.preventDefault(); ep.click(); }
      });
      ctx.content.addEventListener("click", async (e) => {
        const t = (e.target as HTMLElement).closest<HTMLElement>("[data-open],[data-act]");
        if (!t) return;
        if (t.dataset.open) { location.hash = `#/stage/${t.dataset.open}${groupQ}`; return; }
        if (t.dataset.act === "all") {
          const btn = t as HTMLButtonElement;
          btn.disabled = true;
          btn.textContent = "正在读取分集…";
          const ids: string[] = [];
          let cursor = "";
          try {
            for (let page = 0; page < 20; page++) {
              const r = await seriesEpisodes(ctx.arg!, cursor);
              ids.push(...r.items.map((x) => x.id));
              if (!r.hasMore || !r.cursor || r.cursor === cursor) break;
              cursor = r.cursor;
            }
          } catch (err) {
            toast(`分集读取失败：${String((err as Error).message || err)}`, "err");
          } finally {
            btn.disabled = false;
            btn.textContent = "采集…";
          }
          if (!ids.length) { toast("这一季还没有分集", "err"); return; }
          const name = (document.querySelector<HTMLElement>(".pagehead .h1")?.textContent || "本剧").replace(/^短剧\s*·\s*/, "");
          pickRange(name, ids.length, (picked) => {
            const chosen = picked.map((n) => `video:${ids[n - 1]}`).filter((x) => x !== "video:undefined");
            document.dispatchEvent(new CustomEvent("ask-collect", { detail: chosen.join(",") }));
          });
        }
      });
      const eps = $("#eps", ctx.content);
      const end = $("#epsEnd", ctx.content);
      if (!eps || !end) return;
      end.innerHTML = "";
      const ctl = loadMore<any>({
        host: end,
        root: ctx.content,
        key: (e) => String(e.id ?? ""),
        load: async (cursor) => {
          const r = await seriesEpisodes(ctx.arg, cursor);
          return { items: r.items, hasMore: r.hasMore, cursor: r.cursor };
        },
        append: (items) => {
          const base = eps.querySelectorAll(".ep[data-open]").length;
          if (base === 0) eps.innerHTML = "";
          eps.insertAdjacentHTML("beforeend", items.map((e, i) => `
            <div class="ep" data-open="${esc(e.id)}" role="link" tabindex="0" onclick="location.hash='#/stage/${esc(e.id)}${groupQ}'"><img src="${e.cover ?? ""}" alt=""><span>第 ${base + i + 1} 集</span>${e.is_paid ? `<span class="lock"><svg class="i"><use href="#i-lock"/></svg></span>` : ""}</div>`).join(""));
        },
      });
      (ctx.content as any).__seriesEps = ctl;
      ctl.start();
      return;
    }
    const wall = $("#swall", ctx.content)!;
    const hint = $("#swallHint", ctx.content)!;
    const sub = $("#swallSub", ctx.content);
    hint.innerHTML = "";
    ctx.content.querySelector('[data-act="series-refresh"]')?.addEventListener("click", () => void render(true));
    ctx.content.querySelector('[data-act="series-search"]')?.addEventListener("click", () => go("/search?tab=series"));
    const ctl = loadMore<any>({
      host: hint,
      root: ctx.content,
      key: (s) => String(s.id ?? ""),
      load: async (cursor) => {
        const r = await seriesWall(cursor);
        return { items: r.items, hasMore: r.hasMore, cursor: r.cursor };
      },
      append: (items) => {
        if (wall.querySelector(".skel")) wall.innerHTML = "";
        wall.insertAdjacentHTML("beforeend", items.map((s) => `
          <a class="card seriescard" href="#/series/${esc(s.id)}" data-open-series="${esc(s.id)}">
            <div class="thumb">${s.cover ? `<img src="${esc(s.cover)}" loading="lazy">` : `<div class="nocover"><svg class="i"><use href="#i-video"/></svg></div>`}
              <span class="dur">${Number(s.episodes) || 0} 集</span>${s.charge ? `<span class="badge paid">付费</span>` : ""}
              ${s.raw?.status?.status_desc ? `<span class="badge stat">${esc(s.raw.status.status_desc)}</span>` : ""}</div>
            <div class="meta"><div class="desc">${esc(s.name)}</div><div class="by"><span>${esc(s.raw?.author?.nickname ?? "短剧")}</span>${s.raw?.stats?.play_vv ? `<span class="likes"><svg class="i"><use href="#i-play"/></svg>${esc(nfmt(s.raw.stats.play_vv))}</span>` : ""}</div></div>
          </a>`).join(""));
        const n = wall.querySelectorAll(".card").length;
        if (sub) sub.textContent = `${n} 部短剧 · 继续下滑加载更多`;
      },
      onEmpty: () => { if (sub) sub.textContent = "没有更多短剧了"; },
      onError: () => { if (sub) sub.textContent = "加载失败，点下方重试"; },
    });
    (ctx.content as any).__seriesLoad = ctl;
    ctl.start();
  },
  activate(ctx: Ctx) {
    if (ctx.arg) ((ctx.content as any).__seriesEps as LoadMore | undefined)?.resume();
    else ((ctx.content as any).__seriesLoad as LoadMore | undefined)?.resume();
  },
  onLeave(ctx) {
    ((ctx?.content as any)?.__seriesEps as LoadMore | undefined)?.pause();
    ((ctx?.content as any)?.__seriesLoad as LoadMore | undefined)?.pause();
  },
};

function pickRange(title: string, total: number, done: (picked: number[]) => void): void {
  const scrim = modal(`<div class="dh"><h3>选择采集范围</h3><button class="iconbtn" data-close><svg class="i"><use href="#i-close"/></svg></button></div>
    <div class="db">
      <p class="hint">《${esc(title)}》共 ${total} 集，选择要采集的分集。</p>
      <div class="fieldrow"><label>范围</label><select class="select" id="srmode" style="width:150px"><option value="all">全部 ${total} 集</option><option value="range">指定区间</option><option value="last">最后若干集</option></select></div>
      <div class="frow" id="srrange" hidden><span>第</span><input class="input mono" id="srfrom" type="number" min="1" max="${total}" value="1" style="width:90px"><span>集 到第</span><input class="input mono" id="srto" type="number" min="1" max="${total}" value="${total}" style="width:90px"><span>集</span></div>
      <div class="frow" id="srlast" hidden><span>最后</span><input class="input mono" id="srn" type="number" min="1" max="${total}" value="${Math.min(10, total)}" style="width:90px"><span>集</span></div>
      <div class="hint dim">下一步再选择视频、音频或评论等采集内容。</div>
    </div>
    <div class="df"><button class="btn sm ghost" data-close>取消</button><button class="btn sm primary" data-ok>下一步</button></div>`, { width: 460 });
  const mode = $("#srmode", scrim) as HTMLSelectElement;
  const range = $("#srrange", scrim);
  const last = $("#srlast", scrim);
  mode.addEventListener("change", () => {
    range!.hidden = mode.value !== "range";
    last!.hidden = mode.value !== "last";
  });
  scrim.addEventListener("click", (e) => {
    if ((e.target as HTMLElement).closest("[data-close]")) { closeModal(); return; }
    if (!(e.target as HTMLElement).closest("[data-ok]")) return;
    let from = 1;
    let to = total;
    if (mode.value === "range") {
      from = Number(($("#srfrom", scrim) as HTMLInputElement).value);
      to = Number(($("#srto", scrim) as HTMLInputElement).value);
    } else if (mode.value === "last") {
      const n = Math.max(1, Math.floor(Number(($("#srn", scrim) as HTMLInputElement).value) || 0));
      from = Math.max(1, total - n + 1);
    }
    if (!Number.isFinite(from) || !Number.isFinite(to) || from < 1 || to > total || from > to) {
      toast(`请填写 1 到 ${total} 之间的区间`, "err");
      return;
    }
    closeModal();
    done(Array.from({ length: to - from + 1 }, (_, i) => from + i));
  });
}

async function detail(id: string): Promise<string> {
  const s = await series(id).catch((e) => ({ err: String(e) }));
  if ("err" in s) return `<section class="view active">${errorBox(s.err)}</section>`;
  return `
  <section class="view active">
    <div class="pagehead between">
      <div><h1 class="h1">${esc(s.name)}</h1><div class="sub">${s.episodes} 集 · ${s.charge ? "含付费集" : "免费"} · ${esc(s.intro ?? "")}</div></div>
      <div class="row"><button class="btn sm primary" data-act="all">采集…</button></div>
    </div>
    <div class="row" style="gap:16px;align-items:flex-start;margin-bottom:18px">
      <img src="${esc(s.cover)}" style="width:220px;height:124px;border-radius:10px;object-fit:cover">
      <div class="sub">按顺序连播；已下载的集自动跳过。</div>
    </div>
    <div class="eps" id="eps">${Array.from({ length: 6 }, () => '<div class="ep skel"></div>').join("")}</div>
    <div class="hintline" id="epsEnd"></div>
  </section>`;
}

export const mixView: View = {
  title: "合集",
  async render(ctx) {
    if (!ctx.arg) return `<section class="view active">${empty("没有合集", "从用户页进入")}</section>`;
    const m = await mix(ctx.arg);
    return `
    <section class="view active">
      <div class="pagehead between">
        <div><h1 class="h1">${esc(m.name || "合集")}</h1><div class="sub">${m.author ? `合集 · ${esc(m.author)} · ` : ""}${m.count || "若干"} 个作品</div></div>
        <div class="row"><button class="btn sm primary" data-act="all">采集全部</button></div>
      </div>
      <div class="row" style="gap:16px;margin-bottom:18px">${m.cover ? `<img src="${esc(m.cover)}" style="width:200px;height:112px;border-radius:8px;object-fit:cover">` : ""}
        <div class="sub">按序连播，重复作品自动跳过。</div></div>
      <div class="wall" id="mwall">${skeletonCards(6)}</div>
      <div class="hintline" id="mend"></div>
    </section>`;
  },
  mount(ctx) {
    const groupQ = ctx.arg ? `?mix=${encodeURIComponent(ctx.arg)}` : "";
    if (ctx.arg) {
      const wall = $("#mwall", ctx.content);
      const end = $("#mend", ctx.content);
      if (wall && end) {
        const ctl = loadMore<any>({
          host: end,
          root: ctx.content,
          key: (i) => String(i.id ?? ""),
          load: async (cursor) => {
            const r = await mixVideos(ctx.arg, cursor);
            return { items: r.items, hasMore: r.hasMore, cursor: r.cursor };
          },
          append: (items) => {
            if (wall.querySelector(".skel")) wall.innerHTML = "";
            remember(items);
            wall.insertAdjacentHTML("beforeend", items.map((i) => `<article class="card" data-open="${esc(i.id)}" role="link" tabindex="0"><div class="thumb"><img src="${esc(i.cover)}" loading="lazy"><span class="dur">${esc(dur(i.durationMs))}</span></div><div class="meta"><div class="desc">${esc(i.desc)}</div></div></article>`).join(""));
          },
        });
        (ctx.content as any).__mixLoad = ctl;
        ctl.start();
      }
    }
    ctx.content.addEventListener("keydown", (e) => {
      const card = (e.target as HTMLElement).closest<HTMLElement>(".card[data-open]");
      if (card && e.target === card && (e.key === "Enter" || e.key === " ")) { e.preventDefault(); card.click(); }
    });
    ctx.content.addEventListener("click", async (e) => {
      const t = (e.target as HTMLElement).closest<HTMLElement>("[data-open],[data-act]");
      if (!t) return;
      if (t.dataset.open) { location.hash = `#/stage/${t.dataset.open}${groupQ}`; return; }
      if (t.dataset.act === "all") {
        const btn = t as HTMLButtonElement;
        btn.disabled = true;
        btn.textContent = "正在读取全部…";
        const list: string[] = [];
        let cursor = "";
        try {
          for (let page = 0; page < 20; page++) {
            const r = await mixVideos(ctx.arg!, cursor);
            list.push(...r.items.map((x) => `video:${x.id}`));
            if (!r.hasMore || !r.cursor || r.cursor === cursor) break;
            cursor = r.cursor;
          }
        } catch (err) {
          toast(`作品读取失败：${String((err as Error).message || err)}`, "err");
        } finally {
          btn.disabled = false;
          btn.textContent = "采集全部";
        }
        if (!list.length) { toast("这个合集还没有作品", "err"); return; }
        document.dispatchEvent(new CustomEvent("ask-collect", { detail: list.join(",") }));
      }
    });
  },
  activate(ctx: Ctx) {
    ((ctx.content as any).__mixLoad as LoadMore | undefined)?.resume();
  },
  onLeave(ctx) {
    ((ctx?.content as any)?.__mixLoad as LoadMore | undefined)?.pause();
  },
};

export default view;
