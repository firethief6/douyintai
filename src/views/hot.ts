import { $, copy, errorBox, esc, loading, nfmt, skeleton } from "../lib/dom";
import { hot } from "../lib/api";
import type { View } from "../router";

const view: View = {
  cacheable: true,
  title: "热榜",
  render: () => `
    <section class="view active">
      <div class="pagehead between">
        <div><h1 class="h1">热榜</h1><div class="sub">抖音热搜 · 只读快照 · 点词转搜索</div></div>
        <button class="btn sm" data-act="reload"><svg class="i" style="width:14px;height:14px"><use href="#i-retry"/></svg>刷新</button>
      </div>
      <div id="hotlist">${skeleton(1).replace('class="wall"', "")}</div>
    </section>`,
  mount(ctx) {
    const box = $("#hotlist", ctx.content)!;
    let busy = false;
    const load = async () => {
      if (busy) return;
      busy = true;
      box.innerHTML = loading("正在获取热榜…");
      try {
        const rows = await hot();
        box.innerHTML = rows.map((h, i) => `
          <a class="hotrow" href="#/search?tab=video&q=${encodeURIComponent(h.word)}" data-w="${esc(h.word)}" title="点击搜索；右键复制">
            <span class="rk ${i < 3 ? "top" : ""}">${h.pos || i + 1}</span>
            <div><div class="kw">${esc(h.word)}</div><div class="mm">热度 ${esc(nfmt(h.value))}</div></div>
            <svg class="i hot-go"><use href="#i-fwd"/></svg>
          </a>`).join("");
      } catch (e) { box.innerHTML = errorBox(e); }
      finally { busy = false; }
    };
    (ctx.content as any).__hotLoad = load;
    ctx.content.addEventListener("click", (e) => {
      const t = (e.target as HTMLElement).closest<HTMLElement>("[data-act]");
      if (!t) return;
      if (t.dataset.act === "reload" || t.dataset.act === "retry") { e.stopPropagation(); void load(); }
    });
    ctx.content.addEventListener("contextmenu", (e) => {
      const row = (e.target as HTMLElement).closest<HTMLElement>(".hotrow[data-w]");
      if (!row) return;
      e.preventDefault();
      void copy(row.dataset.w!);
    });
    load();
  },
  activate(ctx) {
    const load = (ctx.content as any).__hotLoad as (() => Promise<void>) | undefined;
    if (load && !ctx.content.querySelector(".hotrow")) void load();
  },
};
export default view;
