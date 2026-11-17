export interface LoadPage<T> {
  items: T[];
  hasMore: boolean;
  cursor: string;
}

type State = "idle" | "loading" | "done" | "error";

export interface LoadMore {
  start(): void;
  pause(): void;
  resume(): void;
  retry(): void;
  stop(): void;
  state(): State;
}

interface Options<T> {
  host: HTMLElement;
  root: Element | null;
  load: (cursor: string) => Promise<LoadPage<T>>;
  append: (items: T[]) => void;
  key?: (item: T) => string;
  onEmpty?: () => void;
  onError?: (error: unknown) => void;
  margin?: number;
}

const sleep = (ms: number) => new Promise((resolve) => window.setTimeout(resolve, ms));

/**
 * 哨兵滚到预载区就取下一页，一次只取一页。
 * 加载完成且哨兵仍在预载区时继续补齐，避免短页需要再次滚动。
 */
export function loadMore<T>(opts: Options<T>): LoadMore {
  const margin = opts.margin ?? 700;
  const box = document.createElement("div");
  box.className = "loadmore";
  box.innerHTML = '<div class="loadmore-msg" role="status"></div>';
  const msg = box.querySelector<HTMLElement>(".loadmore-msg")!;
  opts.host.append(box);

  let cursor = "";
  let state: State = "idle";
  let active = false;
  let generation = 0;
  let stalls = 0;
  let chain = 0;
  const seen = new Set<string>();

  const trigger = () => {
    chain = 0;
    void next();
  };

  const io = new IntersectionObserver((entries) => {
    if (entries.some((entry) => entry.isIntersecting)) trigger();
  }, { root: opts.root, rootMargin: `${margin}px 0px` });

  const onScroll = () => {
    if (active && state === "idle" && near()) trigger();
  };
  opts.root?.addEventListener("scroll", onScroll, { passive: true });

  const near = () => {
    if (!box.isConnected) return false;
    const view = opts.root?.getBoundingClientRect();
    const bottom = view ? view.bottom : innerHeight;
    return box.getBoundingClientRect().top < bottom + margin;
  };

  const paint = () => {
    msg.className = `loadmore-msg ${state}`;
    if (state === "loading") {
      msg.innerHTML = '<span class="spinner"></span><span>正在加载更多…</span>';
    } else if (state === "done") {
      msg.textContent = "已经到底了";
    } else if (state === "error") {
      msg.innerHTML = '<span>加载失败</span><button class="btn sm ghost" data-retry>重试</button>';
    } else {
      msg.textContent = "";
    }
  };

  const finish = () => {
    state = "done";
    paint();
    io.unobserve(box);
  };

  const next = async (): Promise<void> => {
    if (!active || state === "loading" || state === "done" || state === "error") return;
    const mine = generation;
    const from = cursor;
    state = "loading";
    paint();
    try {
      const page = await opts.load(from);
      if (mine !== generation) return;
      const fresh = opts.key
        ? page.items.filter((item) => {
          const k = opts.key!(item);
          if (!k) return true;
          if (seen.has(k)) return false;
          seen.add(k);
          return true;
        })
        : page.items;
      if (fresh.length) {
        stalls = 0;
        opts.append(fresh);
      } else {
        stalls++;
      }
      cursor = page.cursor || cursor;

      if (!page.hasMore || stalls >= 3) {
        if (!fresh.length) opts.onEmpty?.();
        finish();
        return;
      }
      state = "idle";
      paint();
      if (fresh.length) {
        chain++;
        if (near() && chain < 2) queueMicrotask(() => void next());
      } else {
        await sleep(400);
        if (mine === generation && active && near()) void next();
      }
    } catch (error) {
      if (mine !== generation) return;
      state = "error";
      paint();
      opts.onError?.(error);
    }
  };

  msg.addEventListener("click", (event) => {
    if (!(event.target as HTMLElement).closest("[data-retry]")) return;
    event.stopPropagation();
    if (state !== "error") return;
    state = "idle";
    paint();
    void next();
  });

  return {
    start() {
      if (active) return;
      active = true;
      generation++;
      state = "idle";
      paint();
      io.observe(box);
      trigger();
    },
    pause() {
      active = false;
      io.unobserve(box);
    },
    resume() {
      if (active || state === "done") return;
      active = true;
      io.observe(box);
      if (near()) trigger();
    },
    retry() {
      if (state !== "error") return;
      state = "idle";
      paint();
      active = true;
      io.observe(box);
      trigger();
    },
    stop() {
      active = false;
      generation++;
      io.disconnect();
      opts.root?.removeEventListener("scroll", onScroll);
      const parent = box.parentNode;
      if (parent) parent.removeChild(box);
    },
    state: () => state,
  };
}
