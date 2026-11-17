import { bindCards, card } from "../components";
import { $, empty, errorBox, esc, nfmt, skeleton, toast } from "../lib/dom";
import { topic, topicVideos } from "../lib/api";
import { loadMore, type LoadMore } from "../lib/infinite";
import { remember } from "../items";
import type { Ctx, View } from "../router";

const view: View = {
  cacheable: true,
  title: "话题",
  async render(ctx) {
    const name = ctx.arg;
    if (!name) return `<section class="view active">${empty("还没有打开的话题", "从搜索或热榜进入")}</section>`;
    let t;
    try { t = await topic(name); } catch (e) { return `<section class="view active">${errorBox(String((e as Error).message))}</section>`; }
    return `
    <section class="view active">
      <div class="pagehead between">
        <div><h1 class="h1">#${esc(t.name || name)}</h1><div class="sub">${esc(nfmt(t.views))} 次播放 · <span id="tcount">正在加载…</span></div></div>
        <div class="row"><button class="btn sm ghost" data-act="copy">复制话题</button><button class="btn sm primary" data-act="collect">采集本页</button></div>
      </div>
      <div class="filters"><button class="chip active" data-sort="0">综合</button><button class="chip" data-sort="1">最热</button></div>
      <div id="tres">${skeleton(5)}</div>
      <div class="hintline" id="tend"></div>
    </section>`;
  },
  mount(ctx) {
    const res = $("#tres", ctx.content)!;
    const end = $("#tend", ctx.content)!;
    const signal = () => ((ctx.content as any).__topicSignal as AbortSignal | undefined) ?? ctx.signal;
    (ctx.content as any).__topicSignal = ctx.signal;
    let currentSort = 0;
    let ctl: LoadMore | null = null;
    const load = (sort = 0) => {
      currentSort = sort;
      ctl?.stop();
      res.innerHTML = skeleton(5);
      let wall: HTMLElement | null = null;
      ctl = loadMore<any>({
        host: end,
        root: ctx.content,
        key: (i) => String(i.id ?? ""),
        load: async (cursor) => topicVideos(ctx.arg, sort, cursor),
        append: (items) => {
          if (signal().aborted) return;
          if (!wall) {
            res.innerHTML = '<div class="wall"></div>';
            wall = res.querySelector(".wall")!;
          }
          remember(items);
          wall.insertAdjacentHTML("beforeend", items.map((i) => card(i)).join(""));
          const count = $("#tcount", ctx.content);
          if (count) count.textContent = `已加载 ${wall.childElementCount} 条视频`;
        },
        onEmpty: () => { if (!wall) res.innerHTML = empty("话题下没有视频"); },
        onError: () => { if (!wall) res.innerHTML = ""; },
      });
      (ctx.content as any).__topicCtl = ctl;
      (ctx.content as any).__topicRetry = () => load(currentSort);
      ctl.start();
    };
    bindCards(res);
    ctx.content.addEventListener("click", async (e) => {
      const t = (e.target as HTMLElement).closest<HTMLElement>("[data-sort],[data-act]");
      if (!t) return;
      if (t.dataset.sort) {
        document.querySelectorAll("[data-sort]").forEach((b) => b.classList.toggle("active", b === t));
        load(Number(t.dataset.sort));
      }
      if (t.dataset.act === "copy") await navigator.clipboard.writeText(`#${ctx.arg}`);
      if (t.dataset.act === "collect") {
        const ids = [...res.querySelectorAll<HTMLElement>("[data-id]")].map((x) => `video:${x.dataset.id!}`).filter((x) => x !== "video:");
        if (!ids.length) { toast("这一页没有视频", "err"); return; }
        document.dispatchEvent(new CustomEvent("ask-collect", { detail: ids.join(",") }));
      }
    });
    load(0);
  },
  activate(ctx: Ctx) {
    (ctx.content as any).__topicSignal = ctx.signal;
    ((ctx.content as any).__topicCtl as LoadMore | undefined)?.resume();
    const res = $("#tres", ctx.content);
    if (res && !res.querySelector(".card, .empty, .errbox")) (ctx.content as any).__topicRetry?.();
  },
  onLeave(ctx) {
    ((ctx?.content as any)?.__topicCtl as LoadMore | undefined)?.pause();
  },
};
export default view;
