import { bindCards, card } from "../components";
import { $, copy, empty, errorBox, esc, nfmt, skeleton, toast } from "../lib/dom";
import { user, userMixes, userPosts, userSeries, type User } from "../lib/api";
import { loadMore, type LoadMore } from "../lib/infinite";
import { remember } from "../items";
import type { Ctx, View } from "../router";

const profileHTML = (u: User) => `<div class="profile">
  <div class="cover" ${u.raw?.cover ? `style="background-image:url('${esc(u.raw.cover)}')"` : ""}></div>
  <div class="pbody"><img src="${esc(u.avatar)}" alt="">
    <div class="ptext"><div class="name">${esc(u.nickname)}</div><div class="bio">${esc(u.signature || "无简介")}</div>
      <div class="stats"><span>粉丝 <b>${esc(nfmt(u.followers))}</b></span>${u.account ? `<span class="uidline">抖音号：${esc(u.account)}<button class="copy-id" data-act="copy-id" title="复制抖音号" aria-label="复制抖音号"><svg class="i"><use href="#i-copy"/></svg></button></span>` : ""}</div></div>
    <div class="row"><button class="btn sm primary" data-act="collect-author">采集作者</button><button class="btn sm icon" data-act="open-share" title="复制主页链接" aria-label="复制主页链接"><svg class="i"><use href="#i-copy"/></svg></button></div>
  </div>
</div>`;

const view: View = {
  cacheable: true,
  title: "用户",
  render(ctx) {
    if (!ctx.arg) return `<section class="view active">${empty("还没有打开的用户", "从搜索、评论或播放页的「作者」进入")}</section>`;
    return `
    <section class="view active">
      <div id="uprofile"><div class="profile"><div class="pbody">
        <span class="skel" style="width:72px;height:72px;border-radius:50%;flex:none"></span>
        <div class="ptext"><div class="skel" style="width:180px;height:20px"></div><div class="skel short" style="width:320px;height:13px;margin-top:10px"></div></div>
      </div></div></div>
      <div class="filters between">
        <div class="row" id="utabs" style="gap:6px" hidden><button class="chip" data-tab="posts" hidden>作品</button><button class="chip" data-tab="mixes" hidden>合集</button><button class="chip" data-tab="series" hidden>短剧</button></div>
      </div>
      <div id="ures">${skeleton(5)}</div>
      <div class="hintline" id="uend"></div>
    </section>`;
  },
  mount(ctx) {
    const res = $("#ures", ctx.content);
    if (!res) return;
    const profileBox = $("#uprofile", ctx.content)!;
    let profile: User | null = null;
    const loadProfile = async () => {
      profileBox.innerHTML = `<div class="profile"><div class="pbody">
        <span class="skel" style="width:72px;height:72px;border-radius:50%;flex:none"></span>
        <div class="ptext"><div class="skel" style="width:180px;height:20px"></div><div class="skel short" style="width:320px;height:13px;margin-top:10px"></div></div>
      </div></div>`;
      try {
        const u = await user(ctx.arg);
        if (ctx.signal.aborted) return;
        profile = u;
        profileBox.innerHTML = profileHTML(u);
        void probeTabs();
      } catch (e) {
        if (!ctx.signal.aborted) profileBox.innerHTML = errorBox(e, "user-profile-retry");
      }
    };
    const seed = new Map<string, any>();
    const probeTabs = async () => {
      const uid = ctx.arg;
      const blank = { items: [] as unknown[] };
      let failed = false;
      const grab = async (fn: () => Promise<{ items: unknown[] }>) => {
        try { return await fn(); } catch { failed = true; return blank; }
      };
      const [posts, mixes, series] = await Promise.all([
        grab(() => userPosts(uid, "")),
        grab(() => userMixes(uid, "")),
        grab(() => userSeries(uid, "")),
      ]);
      if (ctx.signal.aborted) return;
      seed.set("posts", posts).set("mixes", mixes).set("series", series);
      const counts: Record<string, number> = { posts: posts.items.length, mixes: mixes.items.length, series: series.items.length };
      const allTabs = $$("[data-tab]", ctx.content);
      const tabs = failed ? allTabs : allTabs.filter((b) => (counts[b.dataset.tab || ""] ?? 0) > 0);
      const row = $("#utabs", ctx.content);
      if (row) row.hidden = tabs.length === 0;
      for (const b of tabs) b.hidden = false;
      const first = tabs[0]?.dataset.tab;
      if (!first) { res.innerHTML = empty("没有公开内容", "这个作者还没有作品、合集或短剧"); return; }
      tabs[0].classList.add("active");
      load(first);
    };
    void loadProfile();
    const end = $("#uend", ctx.content)!;
    const signal = () => ((ctx.content as any).__userSignal as AbortSignal | undefined) ?? ctx.signal;
    (ctx.content as any).__userSignal = ctx.signal;
    let currentTab = "posts";
    let ctl: LoadMore | null = null;
    const load = (tab: string) => {
      currentTab = tab;
      ctl?.stop();
      res.innerHTML = skeleton(5);
      let wall: HTMLElement | null = null;
      ctl = loadMore<any>({
        host: end,
        root: ctx.content,
        key: (x) => String(x.id ?? x.uid ?? x.name ?? ""),
        load: async (cursor) => {
          if (!cursor && seed.has(tab)) {
            const first = seed.get(tab);
            seed.delete(tab);
            return first;
          }
          if (tab === "posts") return userPosts(ctx.arg, cursor);
          if (tab === "mixes") return userMixes(ctx.arg, cursor);
          return userSeries(ctx.arg, cursor);
        },
        append: (items) => {
          if (signal().aborted) return;
          if (!wall) {
            res.innerHTML = tab === "posts" ? '<div class="wall"></div>' : "<div></div>";
            wall = res.firstElementChild as HTMLElement;
          }
          if (tab === "posts") {
            remember(items);
            wall.insertAdjacentHTML("beforeend", items.map((i) => card(i)).join(""));
          } else if (tab === "mixes") {
            wall.insertAdjacentHTML("beforeend", items.map((m) => `<a class="task" data-id="${esc(m.id)}" data-kind="mix" href="#/mix/${esc(m.id)}" style="grid-template-columns:64px 1fr 100px"><img src="${esc(m.cover)}" style="width:64px;height:40px;border-radius:6px;object-fit:cover"><div><div class="tt">${esc(m.name)}</div><div class="tm">${m.count} 个作品</div></div><span class="btn sm">打开</span></a>`).join(""));
          } else {
            wall.insertAdjacentHTML("beforeend", items.map((x) => `<a class="task" data-id="${esc(x.id)}" data-kind="series" href="#/series/${esc(x.id)}" style="grid-template-columns:64px 1fr 100px"><img src="${esc(x.cover)}" style="width:64px;height:40px;border-radius:6px;object-fit:cover"><div><div class="tt">${esc(x.name)}</div><div class="tm">${x.episodes} 集 · ${x.charge ? "付费" : "免费"}</div></div><span class="btn sm">打开</span></a>`).join(""));
          }
        },
        onEmpty: () => {
          if (wall && wall.childElementCount) return;
          res.innerHTML = tab === "posts" ? empty("没有作品", "可能被隐藏或需要登录") : tab === "mixes" ? empty("没有合集") : empty("没有短剧");
        },
        onError: (e) => {
          if (wall && wall.childElementCount) return;
          res.innerHTML = errorBox(e, "user-retry");
        },
      });
      (ctx.content as any).__userCtl = ctl;
      ctl.start();
    };
    bindCards(res);
    (ctx.content as any).__userRetry = () => load(currentTab);
    $$("[data-tab]", ctx.content).forEach((b) => b.addEventListener("click", () => {
      $$("[data-tab]", ctx.content).forEach((x) => x.classList.toggle("active", x === b));
      load((b as HTMLElement).dataset.tab!);
    }));
    ctx.content.addEventListener("click", async (e) => {
      const act = (e.target as HTMLElement).closest<HTMLElement>("[data-act]")?.dataset.act;
      if (act === "user-retry") { load(currentTab); return; }
      if (act === "user-profile-retry") { void loadProfile(); return; }
      if (act === "copy-id") {
        const u = profile || await user(ctx.arg);
        if (u.account) copy(u.account);
        return;
      }
      if (act === "collect-author") {
        const u = profile || await user(ctx.arg);
        const uid = String(u.raw?.sec_uid || u.uid || "");
        if (!uid) { toast("没有作者 ID", "err"); return; }
        const pageTargets = [...res.querySelectorAll<HTMLElement>("[data-id]")]
          .map((x) => x.dataset.kind ? `${x.dataset.kind}:${x.dataset.id!}` : `video:${x.dataset.id!}`)
          .filter((x) => x !== "video:");
        const { openTaskCreator } = await import("../lib/task-create");
        openTaskCreator({
          target: `user:${uid}`, context: "user", pageTargets, title: "采集作者",
          seed: { type: "user", target_key: `user:${uid}`, title: u.nickname, subtitle: u.account, image: u.avatar, uid },
        });
        return;
      }
      if (act === "open-share") {
        const u = profile || await user(ctx.arg);
        const url = `https://www.douyin.com/user/${u.raw?.sec_uid || u.uid}`;
        await navigator.clipboard.writeText(url).then(() => toast("主页链接已复制")).catch(() => toast("复制失败", "err"));
        return;
      }
    });
  },
  activate(ctx: Ctx) {
    (ctx.content as any).__userSignal = ctx.signal;
    const ctl = (ctx.content as any).__userCtl as LoadMore | undefined;
    if (ctl) ctl.resume();
    const res = $("#ures", ctx.content);
    if (res && !res.querySelector(".card, .task, .errbox")) (ctx.content as any).__userRetry?.();
  },
  onLeave(ctx) {
    ((ctx?.content as any)?.__userCtl as LoadMore | undefined)?.pause();
  },
};
function $$(sel: string, root: ParentNode) { return [...root.querySelectorAll<HTMLElement>(sel)]; }
export default view;
