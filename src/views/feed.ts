import { bindCards, card } from "../components";
import { skeletonCards, errorBox, $ } from "../lib/dom";
import { feed, type Item } from "../lib/api";
import { remember } from "../items";
import { registerPlayList } from "../lib/play-list";
import { log } from "../lib/log";
import type { Ctx, View } from "../router";

const INITIAL_TARGET = 18;
const MORE_TARGET = 12;
const PAGE_SIZE = 20;
const EXPECTED_PAGE = 6;
const MAX_PARALLEL = 3;
const MAX_ROUNDS = 4;
const MAX_EMPTY_ROUNDS = 2;
const PRELOAD_PX = 900;

const view: View = {
  title: "推荐",
  cacheable: true,
  render: () => `
    <section class="view active">
      <div class="pagehead between">
        <div><h1 class="h1">推荐</h1><div class="sub" id="feedSub">匿名推荐流 · 正在准备首屏</div></div>
        <button class="btn sm ghost" data-act="reload"><svg class="i" style="width:14px;height:14px"><use href="#i-retry"/></svg>换一批</button>
      </div>
      <div id="wall" class="wall"></div>
      <div id="feedEnd" class="feed-end"></div>
    </section>`,
  mount(ctx) {
    const wall = $("#wall", ctx.content)!;
    const end = $("#feedEnd", ctx.content)!;
    const sub = $("#feedSub", ctx.content)!;
    const signal = () => ((ctx.content as any).__feedSignal as AbortSignal | undefined) ?? ctx.signal;
    (ctx.content as any).__feedSignal = ctx.signal;

    let busy = false;
    let paused = false;
    let generation = 0;
    let seen = new Set<string>();
    let retryTarget = MORE_TARGET;
    const hasCards = () => !!wall.querySelector(".card[data-id]");

    /**
     * 接口每次只回约 6 条且没有游标。按目标并发补批，并用本次运行的 seen
     * 过滤重复项；换一批会创建新的集合，旧请求不会污染新一批。
     */
    const gather = async (target: number, runSeen: Set<string>): Promise<{ items: Item[]; failed: unknown[] }> => {
      const items: Item[] = [];
      const failed: unknown[] = [];
      // 取消后旧批次不能污染 seen；批次内仍按去重集过滤。
      const batchSeen = new Set(runSeen);
      let emptyRounds = 0;
      for (let round = 0; round < MAX_ROUNDS && items.length < target; round++) {
        const remain = target - items.length;
        const parallel = Math.min(MAX_PARALLEL, Math.max(1, Math.ceil(remain / EXPECTED_PAGE)));
        const results = await Promise.allSettled(Array.from({ length: parallel }, () => feed(PAGE_SIZE)));
        let fresh = 0;
        let succeeded = 0;
        for (const result of results) {
          if (result.status === "rejected") {
            failed.push(result.reason);
            continue;
          }
          succeeded++;
          for (const item of result.value.items) {
            if (batchSeen.has(item.id)) continue;
            batchSeen.add(item.id);
            items.push(item);
            fresh++;
          }
        }
        // 一轮全部失败时立即报错，已加载的卡片保持不动。
        if (!succeeded) break;
        emptyRounds = fresh ? 0 : emptyRounds + 1;
        if (emptyRounds >= MAX_EMPTY_ROUNDS) break;
      }
      return { items, failed };
    };

    const paint = (items: Item[]) => {
      wall.insertAdjacentHTML("beforeend", items.map((item) => card(item, "feed")).join(""));
      registerPlayList("feed", [...seen]);
      sub.textContent = `匿名推荐流 · 已加载 ${wall.querySelectorAll(".card[data-id]").length} 条`;
    };

    const fillIfNeeded = () => {
      requestAnimationFrame(() => {
        if (signal().aborted || busy || paused) return;
        if (end.getBoundingClientRect().top < innerHeight + PRELOAD_PX) void load(false, MORE_TARGET);
      });
    };

    const load = async (replace = false, target = MORE_TARGET) => {
      if (signal().aborted) return;
      if (!replace && (busy || paused)) return;

      // 换一批可以打断旧请求；旧请求回来时 generation 已过期，不会再绘制。
      const mine = ++generation;
      const runSeen = replace ? new Set<string>() : seen;
      if (replace) {
        seen = runSeen;
        paused = false;
        retryTarget = INITIAL_TARGET;
        wall.innerHTML = skeletonCards(INITIAL_TARGET);
        end.innerHTML = "";
        end.classList.remove("failed");
        sub.textContent = "匿名推荐流 · 正在准备首屏";
        ctx.content.scrollTo({ top: 0 });
      }
      busy = true;
      if (!replace && hasCards()) end.innerHTML = "<div class=\"inline-loading\" role=\"status\"><span class=\"spinner\"></span><span>正在加载更多…</span></div>";

      try {
        const result = await gather(target, runSeen);
        if (signal().aborted || mine !== generation) return;
        for (const item of result.items) seen.add(item.id);
        if (replace || !hasCards()) wall.innerHTML = "";
        if (result.items.length) {
          remember(result.items);
          paint(result.items);
          log("info", `推荐流补入 ${result.items.length} 条`, "feed");
        }

        if (!hasCards()) {
          retryTarget = INITIAL_TARGET;
          end.innerHTML = "";
          wall.innerHTML = errorBox(result.failed[0] ?? "推荐流暂时没有内容");
          sub.textContent = "匿名推荐流 · 加载失败";
          return;
        }

        if (!result.items.length) {
          retryTarget = MORE_TARGET;
          if (result.failed.length) {
            end.innerHTML = errorBox(result.failed[0]);
            end.classList.add("failed");
          } else {
            paused = true;
            end.innerHTML = `<div class="inline-loading" role="status"><span>暂时没有更多新内容</span><button class="btn sm ghost" data-act="retry">继续加载</button></div>`;
          }
          return;
        }

        end.innerHTML = "";
        end.classList.remove("failed");
        fillIfNeeded();
      } catch (error) {
        if (signal().aborted || mine !== generation) return;
        if (hasCards()) {
          retryTarget = MORE_TARGET;
          end.innerHTML = errorBox(error);
          end.classList.add("failed");
        } else {
          retryTarget = INITIAL_TARGET;
          wall.innerHTML = errorBox(error);
          sub.textContent = "匿名推荐流 · 加载失败";
        }
        log("err", `推荐流失败：${String((error as Error).message || error)}`, "feed");
      } finally {
        if (mine === generation) busy = false;
      }
    };

    const watch = () => {
      (ctx.content as any).__io?.disconnect();
      const observer = new IntersectionObserver((entries) => {
        if (entries.some((entry) => entry.isIntersecting)) void load(false, MORE_TARGET);
      }, { rootMargin: `${PRELOAD_PX}px` });
      observer.observe(end);
      (ctx.content as any).__io = observer;
    };

    (ctx.content as any).__feedLoad = load;
    (ctx.content as any).__feedWatch = watch;
    (ctx.content as any).__cancelFeed = () => {
      generation++;
      if (busy) end.innerHTML = "";
      busy = false;
    };

    bindCards(wall);
    ctx.content.addEventListener("click", (event) => {
      const target = (event.target as HTMLElement).closest<HTMLElement>("[data-act]");
      if (target?.dataset.act === "reload") {
        event.stopPropagation();
        void load(true, INITIAL_TARGET);
        return;
      }
      if (target?.dataset.act === "retry") {
        event.stopPropagation();
        paused = false;
        end.innerHTML = "";
        end.classList.remove("failed");
        void load(!hasCards(), retryTarget);
      }
    });

    watch();
    void load(true, INITIAL_TARGET);
  },
  activate(ctx: Ctx) {
    const content = ctx.content;
    (content as any).__feedSignal = ctx.signal;
    const load = (content as any).__feedLoad as ((replace?: boolean, target?: number) => Promise<void>) | undefined;
    const watch = (content as any).__feedWatch as (() => void) | undefined;
    (content as any).__feedResumeOff?.();
    (content as any).__io?.disconnect();
    if (!load || !watch) return;
    if (!content.querySelector(".card[data-id]")) {
      void load(true, INITIAL_TARGET);
      return;
    }

    // 恢复已有列表时先不观察底部。滚回底部的那一刻通常仍在视口内，
    // 立即观察会马上再请求一页，让人误以为整页重新加载。
    const resume = () => {
      (content as any).__feedResumeOff = undefined;
      content.removeEventListener("pointerdown", resume);
      content.removeEventListener("wheel", resume);
      content.removeEventListener("keydown", resume);
      content.removeEventListener("touchstart", resume);
      watch();
    };
    (content as any).__feedResumeOff = () => {
      content.removeEventListener("pointerdown", resume);
      content.removeEventListener("wheel", resume);
      content.removeEventListener("keydown", resume);
      content.removeEventListener("touchstart", resume);
    };
    content.addEventListener("pointerdown", resume);
    content.addEventListener("wheel", resume, { passive: true });
    content.addEventListener("keydown", resume);
    content.addEventListener("touchstart", resume, { passive: true });
  },
  onLeave(ctx) {
    (ctx?.content as any)?.__io?.disconnect();
    (ctx?.content as any)?.__feedResumeOff?.();
    if (ctx?.content) (ctx.content as any).__feedResumeOff = undefined;
    (ctx?.content as any)?.__cancelFeed?.();
  },
};

export default view;
