import Hls from "hls.js";
import { $, empty, esc, nfmt, toast } from "../lib/dom";
import { isWarmingError, liveFeed, liveMessages, liveResolve, mediaSrc, searchUsers, userLive, type Room } from "../lib/api";
import { invokeOr, isTauri } from "../lib/store";
import { loadMore, type LoadMore } from "../lib/infinite";
import { listPlans, listRecordings, stopRecording, submitDownload, type Plan, type Recording } from "../lib/tasks";
import { go, replace, type Ctx, type View } from "../router";
import { log } from "../lib/log";


const sz = (n: number) => n > 1 << 30 ? `${(n / (1 << 30)).toFixed(2)} GB` : `${(n / (1 << 20)).toFixed(1)} MB`;
const roomIdOf = (v: string) => v.match(/(?:live\.douyin\.com\/)?(\d{6,})/)?.[1] ?? "";

/** WebKitGTK 自带 HLS 解复用会卡在首片；退化为逐片拉 TS 直接喂 video。 */
async function playSegments(v: HTMLVideoElement, m3u8: string, hint: HTMLElement, current: () => boolean, played: Set<string>): Promise<void> {
  while (current()) {
    const base = new URL(m3u8);
    const text = await (await fetch(m3u8, { cache: "no-store" })).text();
    const urls = text.split("\n").map((x) => x.trim()).filter((x) => x && !x.startsWith("#"))
      .map((x) => new URL(x, base).href);
    const seg = urls.find((u) => !played.has(u));
    if (!seg) { await new Promise((r) => setTimeout(r, 1500)); continue; }
    played.add(seg);
    hint.textContent = "LIVE · 分片";
    v.src = seg;
    await new Promise<void>((resolve) => {
      const done = () => { v.removeEventListener("ended", done); v.removeEventListener("error", done); resolve(); };
      v.addEventListener("ended", done, { once: true });
      v.addEventListener("error", done, { once: true });
      v.play().catch(() => {});
      setTimeout(done, 6000);
    });
  }
}

const roomRow = (r: Room) => `<button class="room" type="button" data-room="${esc(r.id)}" data-title="${esc(r.title)}" data-nick="${esc(r.nick)}" data-hls="${esc(r.hls)}">
  <img src="${esc(r.cover)}" alt=""><div><b>${esc(r.title)}</b><span>${esc(r.nick)} · ${esc(nfmt(r.users))} 人在线</span></div></button>`;

const userRow = (u: { uid: string; numericUid: string; nickname: string; avatar: string; followers: number }) => `<button class="room userroom" type="button" data-user="${esc(u.numericUid)}" data-sec="${esc(u.uid)}" data-nick="${esc(u.nickname)}">
  <img src="${esc(u.avatar)}" alt=""><div><b>${esc(u.nickname)}</b><span>${esc(nfmt(u.followers))} 粉丝 · <i class="lstate">检测中</i></span></div><em class="ltag" hidden></em></button>`;

const view: View = {
  title: "直播",
  cacheable: true,
  render: () => `
    <section class="view active">
      <div class="pagehead between">
        <div><h1 class="h1">直播</h1><div class="sub" id="liveSub">正在获取直播房间…</div></div>
        <div class="row liveops">
          <form class="lsearch" id="liveSearch" role="search">
            <svg class="i"><use href="#i-search"/></svg>
            <input id="liveSearchInput" placeholder="搜主播、昵称或房间号" autocomplete="off" aria-label="搜索主播或房间号">
            <button class="btn sm ghost" type="submit">搜索</button>
          </form>
          <button class="btn sm ghost" id="liveRefresh" title="换一批"><svg class="i" style="width:14px;height:14px"><use href="#i-retry"/></svg>刷新</button>
        </div>
      </div>
      <div class="livegrid">
        <div class="rooms" id="rooms"><div id="roomList">${Array.from({ length: 6 }, () => `<div class="room room-skel"><span class="skel"></span><div><i class="skel"></i><i class="skel short"></i></div></div>`).join("")}</div><div class="hintline" id="liveEnd"></div></div>
        <div class="livebody">
          <div class="player liveplayer" id="lplayer">
            <video id="lvideo" playsinline muted autoplay referrerpolicy="no-referrer"></video>
            <div class="shade"></div>
            <div class="livehint" id="lhint">选一个房间</div>
            <button class="liveaudio off" data-audio title="声音" aria-label="声音开关"><svg class="i"><use href="#i-volume"/></svg></button>
          </div>
          <div class="liveinfo" id="linfo"><div class="dim">左侧是正在直播的房间，选中即播。也可以搜索主播昵称或粘贴直播间号。</div></div>
          <div class="reclist" id="reclist"></div>
        </div>
        <div class="chat" id="chat"><div class="dim" style="padding:12px">弹幕与消息</div></div>
      </div>
    </section>`,
  mount(ctx) {
    const rooms = $("#rooms", ctx.content)!;
    const list = $("#roomList", ctx.content)!;
    const end = $("#liveEnd", ctx.content)!;
    const sub = $("#liveSub", ctx.content);
    const hint = $("#lhint", ctx.content)!;
    const info = $("#linfo", ctx.content)!;
    const chat = $("#chat", ctx.content)!;
    const v = $("#lvideo", ctx.content) as HTMLVideoElement;
    const input = $("#liveSearchInput", ctx.content) as HTMLInputElement;
    let current: { id: string; raw: string; title: string; nick: string } | null = null;
    let hls: Hls | null = null;
    let poll: number | null = null;
    let segRun = 0;
    let statusTimer: number | null = null;
    let retryTimer: number | null = null;
    let recTimer: number | null = null;
    let sig = ctx.signal;
    const dead = () => sig.aborted;
    type RecPaint = () => Promise<void>;
    let onRec: RecPaint | null = null;
    let recSeq = 0;
    const genBox = { n: 0 };
    let ctl: LoadMore | null = null;
    let mode: "feed" | "search" = "feed";
    let query = "";
    let activeRoom = "";
    let autoTimer: number | null = null;
    const probes = new Set<string>();
    const liveCache = new Map<string, { at: number; room: Room | null }>();
    const clearStatusTimer = () => {
      if (statusTimer !== null) window.clearInterval(statusTimer);
      statusTimer = null;
    };
    const clearRetryTimer = () => {
      if (retryTimer !== null) window.clearTimeout(retryTimer);
      retryTimer = null;
    };
    const stop = () => {
      clearStatusTimer();
      clearRetryTimer();
      hls?.destroy(); hls = null;
      if (poll !== null) { window.clearTimeout(poll); poll = null; }
      segRun++;
      v.pause(); v.removeAttribute("src"); v.load();
    };
    (ctx.content as any).__gen = genBox;
    (ctx.content as any).__liveStop = stop;
    v.addEventListener("loadedmetadata", () => log("info", `直播元数据 ${v.duration} ${v.videoWidth}x${v.videoHeight}`, "live"));
    v.addEventListener("canplay", () => log("info", "直播可播放", "live"));
    v.addEventListener("playing", () => log("info", `直播开始 @${v.currentTime.toFixed(2)}`, "live"));
    v.addEventListener("waiting", () => log("warn", `直播缓冲 @${v.currentTime.toFixed(2)} ready=${v.readyState}`, "live"));
    v.addEventListener("stalled", () => log("warn", "直播网络停滞", "live"));
    v.addEventListener("error", () => log("err", `直播错误 code=${v.error?.code} ${v.error?.message ?? ""}`, "live"));

    /** 拉流失败（清单 404/空/解析失败）时换一次地址重试，CDN 边缘节点会轮换。 */
    const begin = async (room: { id: string; title: string; nick: string; hls: string }, mine: number, attempt: number) => {
      try {
        const r = await liveResolve(room.id);
        if (mine !== genBox.n || dead()) { log("info", `${room.id} 丢弃过期拉流`, "live"); return; }
        const raw = r.hls || room.hls;
        if (!raw) throw new Error("没有可用拉流地址");
        current = { id: room.id, raw, title: room.title, nick: room.nick };
        paintRec();
        // 清单与分片都走回环代理：CDN 不带 CORS 头，直连 hls.js 会被拦。
        const src = await mediaSrc(raw);
        if (mine !== genBox.n || dead()) return;
        void fetch(src, { cache: "no-store" }).then((r) => r.text()).then((t) => log("info", `清单 ${t.length}B\n${t}`)).catch(() => {});
        // 统一走 hls.js/MSE（WebView2、Chromium 只能这样）；不支持 MSE 的 WebKit 才用原生 HLS。
        // 清单窗口只有 2 个分片，WKWebView 原生 HLS 每次都会从头播并反复重载；统一用 hls.js 追边。
        log("info", `HLS supported=${Hls.isSupported()} MSE=${typeof MediaSource !== "undefined"} try=${attempt + 1}`, "live");
        if (Hls.isSupported()) {
          // 标准 HLS（非 LL-HLS）：留在直播边缘前 4 个分片，留足缓冲，避免 1s 分片抖动导致反复卡顿。
          hls = new Hls({
            lowLatencyMode: false,
            liveSyncDurationCount: 1,
            liveMaxLatencyDurationCount: 3,
            maxBufferLength: 8,
            maxMaxBufferLength: 12,
            backBufferLength: 30,
          });
          hls.loadSource(src); hls.attachMedia(v);
          hls.on(Hls.Events.MANIFEST_PARSED, () => v.play().catch(() => {}));
          hls.on(Hls.Events.ERROR, (_e, d) => {
            log(d.fatal ? "err" : "warn", `${room.id} HLS ${d.type}/${d.details}`, "live");
            if (!d.fatal) return;
            if (attempt < 1 && mine === genBox.n && !dead()) {
              hls?.destroy(); hls = null;
              hint.textContent = "拉流失败，换地址重试…";
              retryTimer = window.setTimeout(() => { if (mine === genBox.n && !dead()) void begin(room, mine, attempt + 1); }, 800);
            } else hint.textContent = "拉流失败：" + d.details;
          });
        } else { v.src = src; }
        v.play().catch(() => {});
        hint.textContent = "LIVE";
        const run = segRun;
        setTimeout(() => {
          // 已拿到元数据就继续等 CDN 缓冲；完全没有动静才退化为逐片拉取。
          if (run !== segRun || v.currentTime > 0.3 || v.readyState >= 1) return;
          hint.textContent = "LIVE · 切换分片";
          hls?.destroy(); hls = null;
          playSegments(v, src, hint, () => run === segRun && !dead(), new Set()).catch((e) => { hint.textContent = "拉流失败：" + String(e); });
        }, 8000);
        clearStatusTimer();
        statusTimer = window.setInterval(() => {
          if (mine !== genBox.n || dead()) { clearStatusTimer(); return; }
          if (v.error) hint.textContent = `播放错误 ${v.error.code}`;
          else if (v.currentTime > 0.2) { hint.textContent = "LIVE"; clearStatusTimer(); }
          else hint.textContent = "LIVE · 缓冲中";
        }, 1000);
      } catch (e) { hint.textContent = "拉流失败：" + String((e as Error).message); log("err", `${room.id} 拉流失败：${String((e as Error).message)}`, "live"); }
    };

    const start = async (room: { id: string; title: string; nick: string; hls: string }) => {
      const mine = ++genBox.n;
      stop();
      v.muted = false;
      $("[data-audio]", ctx.content)?.classList.remove("off");
      activeRoom = room.id;
      document.body.classList.add("is-sub");
      const backBtn = $<HTMLButtonElement>('[data-act="back"]');
      if (backBtn) backBtn.disabled = false;
      replace("/live");
      list.querySelectorAll<HTMLElement>(".room").forEach((x) => x.classList.toggle("active", x.dataset.room === room.id));
      log("info", `${room.id} 打开直播`, "live");
      hint.textContent = "拉流中…";
      info.innerHTML = `<div class="between"><div><b>${esc(room.title)}</b><div class="dim">${esc(room.nick)} · ${esc(room.id)}</div></div><div class="row"><button class="btn sm ghost" data-plan="auto">开播自动录制</button><button class="btn sm" data-rec="toggle">录制</button></div></div>`;
      chat.innerHTML = `<div class="dim" style="padding:12px">连接弹幕…</div>`;
      void begin(room, mine, 0);

      try {
        let cursor = "", ext = "";
        const tick = async () => {
          if (mine !== genBox.n || dead()) return;
          try {
            const d = await liveMessages(room.id, cursor, ext);
            if (mine !== genBox.n || dead()) return;
            cursor = d.cursor ?? ""; ext = d.internal_ext ?? "";
            if (d.items?.length) {
              chat.insertAdjacentHTML("beforeend", d.items.map((msg) => {
                const nick = msg.user?.nickname || "系统";
                const text = msg.content ?? "";
                return `<p><b>${esc(nick)}</b>：${esc(text)}</p>`;
              }).join(""));
              while (chat.children.length > 200) chat.firstElementChild?.remove();
            }
            chat.scrollTop = chat.scrollHeight;
            poll = window.setTimeout(tick, Math.max(800, d.interval_ms ?? 1200));
          } catch (e) {
            if (mine !== genBox.n || dead()) return;
            log("warn", `${room.id} 弹幕失败，稍后重试：${String((e as Error).message)}`, "live");
            poll = window.setTimeout(tick, 2000);
          }
        };
        chat.innerHTML = "";
        void tick();
      } catch (e) { log("warn", `${room.id} 弹幕失败：${String((e as Error).message)}`, "live"); }
    };

    /* ---------- 房间列表 / 搜索 ---------- */
    const clearList = () => {
      ctl?.stop();
      ctl = null;
      list.innerHTML = "";
      end.innerHTML = "";
    };

    const startFeed = () => {
      clearList();
      mode = "feed";
      query = "";
      input.value = "";
      if (sub) sub.textContent = "正在获取直播房间…";
      list.innerHTML = `<div class="room room-skel"><span class="skel"></span><div><i class="skel"></i><i class="skel short"></i></div></div>`.repeat(12);
      ctl = loadMore<Room>({
        host: end,
        root: rooms,
        key: (r) => r.id,
        load: async (cursor) => liveFeed(cursor),
        append: (items) => {
          if (list.querySelector(".room-skel")) list.innerHTML = "";
          list.insertAdjacentHTML("beforeend", items.map(roomRow).join(""));
          if (sub) sub.textContent = `正在直播 · 已加载 ${list.querySelectorAll(".room[data-room]").length} 个 · 继续下滑加载更多`;
          list.querySelectorAll<HTMLElement>(`[data-room="${activeRoom}"]`).forEach((x) => x.classList.add("active"));
        },
        onError: (e) => {
          if (isWarmingError(e)) {
            if (sub) sub.textContent = "账号还在准备或风控期，请稍后重试";
            if (list.querySelector(".room-skel")) {
              list.innerHTML = `<div class="dim" style="padding:16px;font-size:12px">账号还在准备或风控期，一般 1–2 天后恢复。<button class="btn sm ghost" data-live-retry style="margin-left:8px">重试</button></div>`;
            }
          } else if (sub) sub.textContent = "房间加载失败，点下方重试";
        },
      });
      ctl.start();
    };

    /** 批量探测主播是否在播，限 4 并发并缓存 60 秒，结果写回行内。 */
    const probe = async (rows: HTMLElement[]) => {
      const queue = rows.filter((r) => r.dataset.live === undefined && r.dataset.user && !probes.has(r.dataset.user));
      let i = 0;
      const one = async () => {
        while (i < queue.length) {
          const row = queue[i++];
          const uid = row.dataset.user!;
          probes.add(uid);
          try {
            const cached = liveCache.get(uid);
            let room = cached && Date.now() - cached.at < 60_000 ? cached.room : null;
            if (!cached || Date.now() - cached.at >= 60_000) {
              room = await userLive(uid);
              liveCache.set(uid, { at: Date.now(), room });
            }
            if (!room) throw new Error("未开播");
            if (!row.isConnected) continue;
            row.dataset.live = "1";
            row.dataset.room = room.id;
            row.dataset.title = room.title || row.dataset.nick || "直播";
            row.dataset.hls = room.hls;
            const tag = row.querySelector<HTMLElement>(".ltag");
            if (tag) { tag.hidden = false; tag.textContent = "直播中"; }
            const state = row.querySelector<HTMLElement>(".lstate");
            if (state) state.textContent = `${nfmt(room.users)} 人在线`;
            // 在播的主播排到前面。
            const firstOff = [...list.children].find((x) => (x as HTMLElement).dataset.live === "0");
            if (firstOff && firstOff !== row) list.insertBefore(row, firstOff);
          } catch {
            liveCache.set(uid, { at: Date.now(), room: null });
            if (!row.isConnected) continue;
            row.dataset.live = "0";
            const state = row.querySelector<HTMLElement>(".lstate");
            if (state) state.textContent = "当前未开播";
          }
        }
      };
      await Promise.all(Array.from({ length: 4 }, one));
    };

    const startSearch = (q: string) => {
      clearList();
      mode = "search";
      query = q;
      let searchId = "";
      if (sub) sub.textContent = `正在搜索“${q}”…`;
      ctl = loadMore<any>({
        host: end,
        root: rooms,
        key: (u) => u.uid,
        load: async (cursor) => {
          const p = await searchUsers(q, cursor, searchId);
          searchId = p.searchId || searchId;
          return { items: p.items, hasMore: p.hasMore, cursor: p.cursor };
        },
        append: (items) => {
          if (!items.length) return;
          if (list.querySelector(".empty") || list.querySelector(".room-skel")) list.innerHTML = "";
          const html = items.map(userRow).join("");
          list.insertAdjacentHTML("beforeend", html);
          const rows = [...list.querySelectorAll<HTMLElement>(".userroom:not([data-live])")];
          if (sub) sub.textContent = `“${q}” 的主播 · 在播的会排在前面`;
          void probe(rows);
        },
        onEmpty: () => {
          if (!list.querySelector(".userroom")) list.innerHTML = empty("没有找到主播", "换个昵称，或直接粘贴直播间号");
          else if (sub) sub.textContent = `“${q}” 的主播都已列出`;
        },
      });
      ctl.start();
    };

    const runSearch = async () => {
      const raw = input.value.trim();
      if (!raw) { startFeed(); return; }
      const rid = roomIdOf(raw);
      if (rid && (raw.replace(/\D/g, "") === rid || raw.includes("live.douyin.com"))) {
        // 纯房间号或直播间链接：直接进房。
        try {
          const room = await liveResolve(rid);
          start({ id: room.id, title: room.title, nick: room.nick, hls: room.hls });
          if (sub) sub.textContent = `正在播放：${room.title}`;
          return;
        } catch (e) {
          toast(`直播间打开失败：${String((e as Error).message || e)}`, "err");
        }
      }
      startSearch(raw);
    };

    const enterUser = async (row: HTMLElement) => {
      const uid = row.dataset.user;
      if (!uid) return;
      const sec = row.dataset.sec || uid;
      if (row.dataset.live === "1" && row.dataset.room) {
        start({ id: row.dataset.room, title: row.dataset.title || row.dataset.nick || "直播", nick: row.dataset.nick || "", hls: row.dataset.hls || "" });
        return;
      }
      if (row.dataset.live === "0") { go(`/user/${encodeURIComponent(sec)}`); return; }
      const tag = row.querySelector<HTMLElement>(".ltag");
      if (tag) { tag.hidden = false; tag.textContent = "查询中"; }
      try {
        const room = await userLive(uid);
        start({ id: room.id, title: room.title, nick: room.nick, hls: room.hls });
      } catch (e) {
        if (tag) { tag.textContent = "未开播"; }
        toast(`${row.dataset.nick || "该主播"}当前未开播，已打开主页`, "err");
        go(`/user/${encodeURIComponent(sec)}`);
      }
    };

    ctx.content.addEventListener("click", (e) => {
      const audio = (e.target as HTMLElement).closest<HTMLElement>("[data-audio]");
      if (audio) {
        v.muted = !v.muted;
        audio.classList.toggle("off", v.muted);
        if (!v.muted) v.play().catch(() => {});
        return;
      }
      const retry = (e.target as HTMLElement).closest<HTMLElement>("[data-act=\"retry\"]");
      if (retry) { e.preventDefault(); ctl?.retry(); return; }
      const r = (e.target as HTMLElement).closest<HTMLElement>(".room[data-room]");
      if (r) { start({ id: r.dataset.room!, title: r.dataset.title!, nick: r.dataset.nick!, hls: r.dataset.hls! }); return; }
      const u = (e.target as HTMLElement).closest<HTMLElement>(".userroom[data-user]");
      if (u) void enterUser(u);
    });

    const onSearch = (e: Event) => { e.preventDefault(); void runSearch(); };
    $("#liveSearch", ctx.content)?.addEventListener("submit", onSearch);
    $("#liveRefresh", ctx.content)?.addEventListener("click", () => { if (mode === "search" && query) startSearch(query); else startFeed(); });
    list.addEventListener("click", (e) => { if ((e.target as HTMLElement).closest("[data-live-retry]")) startFeed(); });
    input.addEventListener("keydown", (e) => {
      if (e.key === "Escape") { input.value = ""; e.stopPropagation(); startFeed(); input.blur(); }
      if (e.key === "ArrowDown") { e.preventDefault(); list.querySelector<HTMLElement>(".room")?.focus(); }
    });
    list.addEventListener("keydown", (e) => {
      const row = (e.target as HTMLElement).closest<HTMLElement>(".room");
      if (!row) return;
      if (e.key === "ArrowDown" || e.key === "ArrowUp") {
        e.preventDefault();
        const all = [...list.querySelectorAll<HTMLElement>(".room")];
        const i = all.indexOf(row) + (e.key === "ArrowDown" ? 1 : -1);
        all[Math.max(0, Math.min(all.length - 1, i))]?.focus();
      }
    });

    /* 空闲时自动换一批，正在看某个房间时不动列表。 */
    const armAuto = () => {
      if (autoTimer !== null) window.clearInterval(autoTimer);
      autoTimer = window.setInterval(() => {
        if (dead() || mode !== "feed" || current || rooms.scrollTop > 24 || document.hidden) return;
        if (!list.querySelector(".room[data-room]")) return;
        void liveFeed("").then((p) => {
          if (mode !== "feed" || !p.items.length) return;
          const seen = new Set([...list.querySelectorAll<HTMLElement>("[data-room]")].map((x) => x.dataset.room));
          const fresh = p.items.filter((r) => !seen.has(r.id));
          if (fresh.length) list.insertAdjacentHTML("afterbegin", fresh.map(roomRow).join(""));
        }).catch(() => {});
      }, 45_000);
    };
    armAuto();
    (ctx.content as any).__liveAutoArm = armAuto;
    (ctx.content as any).__liveAutoStop = () => { if (autoTimer !== null) { window.clearInterval(autoTimer); autoTimer = null; } };

    const paintRec: RecPaint = async () => {
      const box = $("#reclist", ctx.content);
      if (!box || !isTauri() || document.hidden) return;
      const seq = ++recSeq;
      const [rows, plans] = await Promise.all([
        listRecordings().catch(() => [] as Recording[]),
        listPlans().catch(() => [] as Plan[]),
      ]);
      if (seq !== recSeq || dead()) return;
      const btn = $("[data-rec]", ctx.content) as HTMLButtonElement | null;
      if (btn && current) {
        const mine = rows.some((r) => r.state === "running" && r.room_id === current!.id);
        btn.textContent = mine ? "停止录制" : "录制";
        btn.classList.toggle("rec-on", mine);
      }
      const plan = $("[data-plan]", ctx.content) as HTMLButtonElement | null;
      if (plan && current) {
        const on = plans.some((p) => p.enabled && p.kind === "live_auto" && p.target_key === `room:${current!.id}`);
        plan.textContent = on ? "已设自动录制" : "开播自动录制";
        plan.disabled = on;
      }
      const stamp = (t: number) => {
        const d = new Date(t * 1000);
        return `${String(d.getHours()).padStart(2, "0")}:${String(d.getMinutes()).padStart(2, "0")}`;
      };
      const shown = rows.slice(0, 20);
      box.innerHTML = shown.length ? shown.map((r) => {
        const busy = r.state === "running" || r.state === "queued";
        const text = busy ? `录制中 · ${sz(r.bytes)}`
          : r.state === "done" ? (r.bytes > 0 ? `已结束 · ${sz(r.bytes)}` : "空录制 · 无分片")
            : `失败：${esc(r.error || "")}`;
        return `
        <div class="recrow">
          <span class="dot ${busy ? "on" : (r.state === "failed" || r.state === "fail") ? "fail" : "done"}"></span>
          <time class="dim">${stamp(r.started_at)}</time>
          <b>${esc(r.room_id || r.target_key)}</b>${r.plan_id ? '<span class="tagauto">自动</span>' : ""}
          <span class="dim">${text}</span>
          <button class="btn sm ghost" data-rev="${esc(r.dir)}">目录</button>
          ${busy ? "" : `<button class="btn sm ghost" data-rdel="${esc(r.dir)}">删除</button>`}
        </div>`;
      }).join("") + (rows.length > shown.length ? `<div class="dim" style="font-size:11px;padding:4px 0">仅显示最近 ${shown.length} 条，共 ${rows.length} 条</div>` : "")
        : `<div class="dim" style="font-size:11px;padding:4px 0">还没有录制</div>`;
    };
    onRec = paintRec;
    (ctx.content as any).__liveRec = onRec;
    document.addEventListener("rec-change", onRec);

    ctx.content.addEventListener("click", async (e) => {
      const t = (e.target as HTMLElement).closest<HTMLElement>("[data-rec],[data-rev],[data-rdel],[data-plan]");
      if (!t) return;
      if (t.hasAttribute("data-plan")) {
        if (!current) { toast("先选一个直播", "err"); return; }
        const { quickPlan } = await import("./plans");
        await quickPlan("live_auto", `room:${current.id}`, current.title);
        await paintRec();
        return;
      }
      if (t.dataset.rev) { invokeOr("reveal", { path: t.dataset.rev }).catch(() => {}); return; }
      if (t.dataset.rdel) {
        if (!window.confirm("删除这份录制及本地文件？")) return;
        await invokeOr("rec_delete", { path: t.dataset.rdel }).catch((err) => toast(`删除失败：${String((err as Error).message || err)}`, "err"));
        await paintRec();
        return;
      }
      if (!current) { toast("先选一个直播", "err"); return; }
      if (!isTauri()) { toast("浏览器开发模式下不能录制", "err"); return; }
      const rows = await listRecordings().catch(() => [] as Recording[]);
      const run = rows.find((r) => (r.state === "running" || r.state === "queued") && r.room_id === current!.id);
      if (run) { await stopRecording(run.id); toast("已停录"); paintRec(); return; }
      try {
        await submitDownload(`room:${current.id}`, { asset_kind: "record", url: current.raw, title: current.title, pinned: true, ext: "m3u8" });
        toast("录制已入队"); paintRec();
      } catch (err) { toast(`录制失败：${String((err as Error).message || err)}`, "err"); }
    });

    void paintRec();
    const armRec = () => {
      if (recTimer !== null) window.clearInterval(recTimer);
      recTimer = window.setInterval(() => void paintRec(), 3000);
    };
    armRec();
    (ctx.content as any).__liveRecArm = armRec;
    (ctx.content as any).__liveRecStop = () => { if (recTimer !== null) { window.clearInterval(recTimer); recTimer = null; } };
    (ctx.content as any).__liveResume = (next: AbortSignal) => {
      sig = next;
      armAuto();
      armRec();
      void paintRec();
      if (current) void start({ id: current.id, title: current.title, nick: current.nick, hls: current.raw });
    };

    if (ctx.arg) {
      void liveResolve(ctx.arg)
        .then((room) => {
          if (sub) sub.textContent = `正在播放：${room.title}`;
          return start({ id: room.id, title: room.title, nick: room.nick, hls: room.hls });
        })
        .catch((err) => {
          hint.textContent = `直播间打开失败：${String((err as Error).message || err)}`;
          toast(hint.textContent, "err");
        });
    }

    startFeed();
  },
  activate(ctx: Ctx) {
    const c = ctx.content as any;
    (c.__liveResume as ((next: AbortSignal) => void) | undefined)?.(ctx.signal);
  },
  onLeave(ctx) {
    const c = ctx?.content ?? document.getElementById("content");
    const g = (c as any)?.__gen;
    if (g) g.n++;
    (c as any)?.__liveAutoStop?.();
    const saved = (c as any)?.__liveStop as (() => void) | undefined;
    saved?.();
    (c as any)?.__liveRecStop?.();
  },
};

export default view;
