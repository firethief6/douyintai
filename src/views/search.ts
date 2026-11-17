import { bindCards, card } from "../components";
import { $, $$, empty, esc, nfmt, skeleton } from "../lib/dom";
import { searchSeries, searchTopics, searchUsers, searchVideos } from "../lib/api";
import { looksLikeTarget, openTarget } from "../lib/paste";
import { loadMore, type LoadMore } from "../lib/infinite";
import { remember } from "../items";
import { registerPlayList } from "../lib/play-list";
import { replace, type Ctx, type View } from "../router";

const TABS = [["video", "视频"], ["user", "用户"], ["series", "短剧"], ["topic", "话题"]] as const;

const view: View = {
  title: "搜索",
  cacheable: true,
  render(ctx) {
    const q = ctx.query.get("q") ?? "";
    const tab = ctx.query.get("tab") ?? "video";
    return `
    <section class="view active">
      <div class="pagehead between">
        <div><h1 class="h1">搜索</h1><div class="sub" id="searchSub">${q ? `“${esc(q)}” 的结果` : "在顶部输入关键词，或直接粘贴抖音链接"}</div></div>
      </div>
      <div class="filters">${TABS.map(([k, n]) => `<button class="chip ${k === tab ? "active" : ""}" data-tab="${k}">${n}</button>`).join("")}</div>
      <div id="sres">${q ? skeleton(5) : empty("输入关键词开始", "视频、用户、短剧、话题分类检索")}</div>
      <div class="hintline" id="send"></div>
    </section>`;
  },
  mount(ctx) {
    const res = $("#sres", ctx.content)!;
    const end = $("#send", ctx.content)!;
    let tab = ctx.query.get("tab") ?? "video";
    let q = ctx.query.get("q") ?? "";
    let ctl: LoadMore | null = null;
    const topInput = $("#topSearchInput") as HTMLInputElement | null;
    const signal = () => ((ctx.content as any).__searchSignal as AbortSignal | undefined) ?? ctx.signal;
    (ctx.content as any).__searchSignal = ctx.signal;

    const paintSub = () => {
      const sub = $("#searchSub", ctx.content);
      if (sub) sub.textContent = q ? `“${q}” 的结果` : "在顶部输入关键词，或直接粘贴抖音链接";
    };
    const run = () => {
      ctl?.stop();
      if (!q) { res.innerHTML = empty("输入关键词开始", "视频、用户、短剧、话题分类检索"); paintSub(); return; }
      if (looksLikeTarget(q)) { void openTarget(q); return; }
      res.innerHTML = skeleton(5) + (tab === "series" ? '<div class="hintline">短剧检索会合并多个关键词，通常需要十几秒</div>' : "");
      paintSub();
      let wall: HTMLElement | null = null;
      let searchId = "";
      let searchRequestId = "";
      let userSearchId = "";
      let topicSearchId = "";
      const paintWall = () => {
        if (wall) return wall;
        res.innerHTML = tab === "video" || tab === "series" ? '<div class="wall"></div>' : "<div></div>";
        wall = res.firstElementChild as HTMLElement;
        return wall;
      };
      ctl = loadMore<any>({
        host: end,
        root: ctx.content,
        key: (x) => String(x.id ?? x.uid ?? x.name ?? ""),
        load: async (cursor) => {
          if (tab === "video") {
            const r = await searchVideos(q, cursor, searchId, searchRequestId);
            searchId = r.searchId;
            searchRequestId = r.searchRequestId;
            return r;
          }
          if (tab === "user") {
            const r = await searchUsers(q, cursor, userSearchId);
            userSearchId = r.searchId;
            return r;
          }
          if (tab === "topic") {
            const r = await searchTopics(q, cursor, topicSearchId);
            topicSearchId = r.searchId;
            return r;
          }
          return searchSeries(q);
        },
        append: (items) => {
          if (signal().aborted) return;
          const box = paintWall();
          if (tab === "video") {
            remember(items);
            box.insertAdjacentHTML("beforeend", items.map((i) => card(i, "search")).join(""));
            registerPlayList("search", [...box.querySelectorAll<HTMLElement>("[data-id]")].map((el) => el.dataset.id!).filter(Boolean));
          } else if (tab === "user") {
            box.insertAdjacentHTML("beforeend", items.map((u: any) => `
              <div class="task" style="grid-template-columns:48px 1fr 120px"><img src="${esc(u.avatar)}" style="width:48px;height:48px;border-radius:50%">
                <div><div class="tt">${esc(u.nickname)}</div><div class="tm">${u.signature ? `${esc(u.signature)} · ` : ""}${esc(nfmt(u.followers))} 粉丝</div></div>
                <div class="ops"><a class="btn sm" href="#/user/${esc(u.uid)}">主页</a><button class="btn sm ghost" data-create-user="${esc(u.uid)}" data-user-name="${esc(u.nickname)}" data-user-avatar="${esc(u.avatar)}" data-user-account="${esc(u.unique_id || u.short_id || "")}">采集</button></div></div>`).join(""));
          } else if (tab === "series") {
            box.insertAdjacentHTML("beforeend", items.map((x: any) => `<a class="card" href="#/series/${esc(x.id)}"><div class="thumb">${x.cover ? `<img src="${esc(x.cover)}">` : ""}<span class="dur">${x.episodes} 集</span></div><div class="meta"><div class="desc">${esc(x.name)}</div><div class="by"><span>${x.charge ? "付费" : "免费"}</span></div></div></a>`).join(""));
          } else {
            box.insertAdjacentHTML("beforeend", items.map((x: any) => `<div class="hotrow two"><div><div class="kw">#${esc(x.name)}</div><div class="mm">${esc(nfmt(x.views))} 次播放</div></div><div class="ops"><a class="btn sm ghost" href="#/topic/${encodeURIComponent(x.name)}">打开</a></div></div>`).join(""));
          }
        },
        onEmpty: () => {
          if (wall) return;
          res.innerHTML = tab === "video" ? empty("没有视频", "换个说法或看联想词")
            : tab === "user" ? empty("没有人", "试试抖音号或昵称全称")
              : tab === "series" ? empty("没有短剧", "试试剧名全称")
                : empty("没有话题", "换个关键词");
        },
        onError: (error) => {
          if (wall) return;
          const e = error as { status?: number; message?: string } | undefined;
          const msg = String(e?.message || error || "未知错误");
          if (e?.status === 429 && msg.includes("登录")) {
            res.innerHTML = empty("话题搜索需要登录", "匿名身份被抖音限流，点下方重试或换个关键词");
          } else {
            res.innerHTML = empty("加载失败", msg);
          }
        },
      });
      (ctx.content as any).__searchCtl = ctl;
      ctl.start();
    };

    const choose = (nextQ: string, nextTab = tab) => {
      q = nextQ;
      tab = nextTab;
      $$("[data-tab]", ctx.content).forEach((b) => b.classList.toggle("active", b.dataset.tab === tab));
      replace(`/search?tab=${tab}&q=${encodeURIComponent(q)}`);
      void run();
    };
    (ctx.content as any).__searchRun = run;
    bindCards(res);
    const onTopSearch = (e: Event) => choose(String((e as CustomEvent).detail ?? ""));
    const onTopInput = (e: Event) => {
      const next = String((e as CustomEvent).detail ?? "").trim();
      if (next === q) return;
      q = next;
      paintSub();
      replace(`/search?tab=${tab}&q=${encodeURIComponent(q)}`);
      void run();
    };

    ctx.content.addEventListener("click", (e) => {
      const t = (e.target as HTMLElement).closest<HTMLElement>("[data-tab],[data-create-user]");
      if (!t) return;
      if (t.dataset.tab) { choose(q, t.dataset.tab); return; }
      if (t.dataset.createUser) {
        void import("../lib/task-create").then(({ openTaskCreator }) => openTaskCreator({
          target: `user:${t.dataset.createUser}`, context: "user",
          seed: { type: "user", target_key: `user:${t.dataset.createUser}`, title: t.dataset.userName || "", subtitle: t.dataset.userAccount || "", image: t.dataset.userAvatar || "", uid: t.dataset.createUser },
        }));
      }
    });
    const bindTop = (focus = false) => {
      (ctx.content as any).__topOff?.();
      window.addEventListener("top-search", onTopSearch);
      window.addEventListener("top-search-input", onTopInput);
      (ctx.content as any).__topOff = () => {
        window.removeEventListener("top-search", onTopSearch);
        window.removeEventListener("top-search-input", onTopInput as EventListener);
      };
      if (topInput) { topInput.value = q; if (focus && !q) queueMicrotask(() => topInput.focus()); }
    };
    (ctx.content as any).__topRebind = bindTop;
    bindTop(true);
    void run();
  },
  activate(ctx: Ctx) {
    (ctx.content as any).__searchSignal = ctx.signal;
    const rebind = (ctx.content as any).__topRebind as ((focus?: boolean) => void) | undefined;
    rebind?.(false);
    ((ctx.content as any).__searchCtl as LoadMore | undefined)?.resume();
    const res = $("#sres", ctx.content);
    const run = (ctx.content as any).__searchRun as (() => void) | undefined;
    if (res && !res.querySelector(".card, .task, .hotrow, .empty") && ctx.query.get("q")) run?.();
  },
  onLeave(ctx) {
    ((ctx?.content as any)?.__searchCtl as LoadMore | undefined)?.pause();
    (ctx?.content as any)?.__topOff?.();
  },
};

export default view;
