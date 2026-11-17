import { $, ago, closeModal, copy, errorBox, esc, modal, nfmt, skeleton, toast } from "../lib/dom";
import { danmaku, isWarmingError, resolve, type Gear, type Item } from "../lib/api";
import { itemById } from "../items";
import { toggleFavorite, isFavorite, libPut, saveSettings, settings } from "../lib/store";
import { log } from "../lib/log";
import { back, go as goRoute, type View } from "../router";
import { loadMore, type LoadMore } from "../lib/infinite";
import { loadComments, loadSupportedGears, loadVideo } from "../lib/stage-cache";
import { qualityChoices, type QualityChoice } from "../lib/playback-quality";
import { fetchPlaySource, gearId, issueOf } from "../lib/play-source";
import { queueFor } from "../lib/play-queue";
import { recordsHistory } from "../lib/play-list";
import { createPlayUi } from "../lib/play-ui";
import { PLAYER_QUALITY, STAGE } from "../lib/copy";
import { groupFromRaw, knownVideoRatio } from "../lib/stage-nav";

/** 详情里的短剧上下文：集号、总集数、免费清单、付费标记、剧名。 */
function seriesMeta(raw: unknown) {
  const r = (raw ?? {}) as any;
  const index = Math.max(0, Number(r.series_play_info?.series_aweme_index) || 0);
  let total = 0;
  let free: number[] = [];
  let freeTo = 0;
  try {
    const extra = JSON.parse(String(r.series_info?.extra || "{}"));
    total = Math.max(0, Number(extra.total_episode) || 0);
    freeTo = Math.max(0, Number(extra.free_to_paid_config?.free_to_paid_count) || 0);
    free = (extra.free_to_paid_config?.free_to_paid_episodes ?? []).map(Number).filter((n: number) => n > 0);
  } catch { /* 旧详情没有剧集扩展 */ }
  const name = String(r.series_info?.series_name || "").replace(/^短剧\s*·\s*/, "");
  const charged = !!r.series_info?.is_charge_series &&
    (free.length ? !free.includes(index) : freeTo > 0 && index > freeTo);
  return { index, total, free, charged, name };
}

/** 跨页面共享：一次滚动手势只切一条，避免新页面重复响应剩余动量。 */
let wheelGate = 0;
let wheelSettle = 0;


function inspectorInfo(it: Item): string {
  return `
    <div class="author" data-act="user" role="link" tabindex="0" title="打开作者主页"><img src="${esc(it.author.avatar)}" alt=""><div><b>${esc(it.author.nickname)}</b><span class="uidline">抖音号：${esc(it.author.account || "—")}${it.author.account ? `<button class="copy-id" data-act="copy-account" title="复制抖音号" aria-label="复制抖音号"><svg class="i"><use href="#i-copy"/></svg></button>` : ""}</span></div></div>
    <div class="stats"><span><strong>${esc(nfmt(it.likes))}</strong><small>赞</small></span><span><strong>${esc(nfmt(it.comments))}</strong><small>评论</small></span><span><strong>${esc(nfmt(it.shares))}</strong><small>分享</small></span></div>
    <div class="tags">${((it.raw.challenge as any[]) ?? []).map((c: any) => `<span class="tag">#${esc(c.challenge_name ?? c)}</span>`).join("")}</div>
    <div class="actbar">
      <button class="${isFavorite(it.id) ? "hot" : ""}" data-act="fav"><svg class="i"><use href="#i-heart"/></svg>收藏</button>
      <button data-act="collect"><svg class="i"><use href="#i-database"/></svg>采集</button>
      <button data-act="copy-link" title="复制分享链接"><svg class="i"><use href="#i-share"/></svg>分享链接</button>
    </div>`;
}

const view: View = {
  title: "播放页",
  async render(ctx) {
    const id = ctx.arg;
    let it = itemById(id);
    const cachedImgs = (it?.raw as any)?.images as any[] | undefined;
    const needGalleryDetail = it?.kind === "gallery" && (!Array.isArray(cachedImgs) || !cachedImgs.some((x) => x?.video?.play_addr?.url_list?.length));
    // 详情只阻塞没有缓存条目的深链；推荐流里已有的条目先出画面，详情在后台补齐。
    if (!it || needGalleryDetail) {
      try { it = await loadVideo(id); if (ctx.signal.aborted) return ""; }
      catch (e) {
        const msg = String((e as Error).message || e);
        const r = await resolve(id).catch(() => null);
        if (r?.type === "series" || r?.type === "mix") {
          location.hash = `#/${r.type}/${encodeURIComponent(String(r.id))}`;
          return `<section class="view active">${skeleton(1)}</section>`;
        }
        if (!it) return `<section class="view active">${errorBox(msg)}</section>`;
      }
    } else if (!it.author.uid || !it.author.account) {
      void loadVideo(id).catch(() => {});
    }
    const imgs: string[] = it?.kind === "gallery" ? ((it.raw.images as any[]) ?? []).flatMap((x: any) => x?.download_url_list?.[0] ?? x?.url_list?.[0] ?? []) : [];
    const graph = groupFromRaw(it?.raw);
    const queue = queueFor(ctx.query, graph);
    const group = queue.group;
    const grouped = !!group && (queue.origin === "series" || queue.origin === "mix");
    const { index: epIndex, total: epTotal, charged, name: seriesName } = seriesMeta(it?.raw);
    return `
    <section class="view active">
      <div class="stagewrap">
        <div class="stagebg" id="stagebg" style="background-image:url('${esc(it.cover)}')"></div>
        <div class="stagetop">
          ${group && seriesName
            ? `<button class="badge series" data-act="series" title="${STAGE.openSeries}">${esc(seriesName)}</button>`
            : `<span class="badge src">${it.kind === "gallery" ? "图集" : "视频"}</span>`}
          ${grouped && epIndex ? `<span class="badge ep" id="epidx" data-index="${epIndex}"${epTotal ? ` data-total="${epTotal}"` : ""}>第 ${epIndex}${epTotal ? ` / ${epTotal}` : ""} 集</span>` : ""}
          ${charged ? `<span class="badge paid" title="平台付费集，应用内可直接播放">付费</span>` : ""}
          ${it.kind === "gallery" ? `<button class="btn sm ghost" onclick="location.hash='#/gallery/${esc(it.id)}'">看全部 ${(it.raw.images as any[])?.length ?? 0} 图</button>` : ""}
          <span class="dim" style="font-size:11px">${esc(ago(it.createTime))}</span>
          <button style="margin-left:auto" class="idcopy mono" data-act="copy-id" title="复制作品 ID" aria-label="复制作品 ID"><span>${esc(it.id)}</span><svg class="i"><use href="#i-copy"/></svg></button>
        </div>
        <div class="stagebody">
          ${imgs.length ? `<div class="stagegallery" id="stagegallery">
            <img src="${esc(imgs[0])}" alt="">
            <div class="ptitle">${esc(it.desc)}</div>
            <div class="gcount">共 ${imgs.length} 图</div>
          </div>` : `<div class="player" id="player">
            <video id="video" poster="${esc(it.cover)}" playsinline autoplay${grouped ? "" : " loop"} referrerpolicy="no-referrer"></video>
            <div class="shade"></div>
            <div class="ptitle">${esc(it.desc)}</div>
            <div class="danmaku" id="dmk"${settings().danmaku ? "" : " hidden"}></div>
            <div class="veil" id="veil" hidden></div>
            <div class="centerplay"><button data-act="toggle" aria-label="播放"><svg class="i"><use href="#i-play"/></svg></button></div>
            <div class="pc">
              <div class="track" data-act="seek"><i></i></div>
              <div class="line">
                <button data-act="toggle" aria-label="播放"><svg class="i"><use href="#i-play"/></svg></button>
                <button data-act="mute" aria-label="静音"><svg class="i"><use href="#i-volume"/></svg></button>
                <span class="time" id="ptime">0:00</span>
                <span class="grow"></span>
                <details class="quality-menu" id="quality-menu">
                  <summary aria-label="${PLAYER_QUALITY.label}" id="quality-label">${PLAYER_QUALITY.loading}</summary>
                  <div class="quality-options" id="quality-options" role="group" aria-label="${PLAYER_QUALITY.label}"></div>
                </details>
                <button class="mono" data-act="rate">1.0×</button>
                <button class="mono${settings().danmaku ? " on" : ""}" data-act="dmk" title="弹幕开关" aria-pressed="${settings().danmaku}">弹</button>
                <button data-act="full" aria-label="全屏"><svg class="i"><use href="#i-full"/></svg></button>
              </div>
            </div>
          </div>`}
          <div class="epbar">
            <button class="btn sm ghost" data-act="prev">${grouped ? STAGE.prev : STAGE.prevOne}</button>
            ${grouped ? `<button class="btn sm ghost" data-act="pick">${STAGE.pick}</button>` : ""}
            <button class="btn sm ghost" data-act="next">${grouped ? STAGE.next : STAGE.nextOne}</button>
          </div>
        </div>
      </div>
    </section>`;
  },
  async mount(ctx) {
    const id = ctx.arg;
    const it = itemById(id);
    const graph = groupFromRaw(it?.raw);
    const queue = queueFor(ctx.query, graph);
    const group = queue.group;
    const grouped = !!group && (queue.origin === "series" || queue.origin === "mix");
    const queueCtx = queue.query();
    if (it && recordsHistory(queue.origin)) libPut("history", { id: it.id, kind: it.kind, title: it.desc, author: it.author.nickname, cover: it.cover, at: Date.now() });
    const knownRatio = knownVideoRatio(it?.raw);
    const player = $("#player", ctx.content);
    const v = $("#video", ctx.content) as HTMLVideoElement | null;
    const track = $(".track i", ctx.content) as HTMLElement | null;
    const content = ctx.content;
    const ui = createPlayUi(content, player, track);
    let veilTimer = 0;
    const showVeil = (text: string) => { clearTimeout(veilTimer); ui.veil(text); };
    const hideVeil = () => { clearTimeout(veilTimer); ui.veilOff(); };
    let posterRatio = 0;
    const fitPlayer = () => {
      if (!player || !v || document.fullscreenElement === player) return;
      const ratio = v.videoWidth && v.videoHeight ? v.videoWidth / v.videoHeight : knownRatio || posterRatio || 16 / 9;
      if (!ratio) return;
      const box = player.parentElement;
      if (!box) return;
      const style = getComputedStyle(box);
      const availW = Math.max(1, box.clientWidth - parseFloat(style.paddingLeft) - parseFloat(style.paddingRight));
      const bar = ($(".epbar", box) as HTMLElement | null)?.offsetHeight ?? 0;
      const availH = Math.max(1, box.clientHeight - parseFloat(style.paddingTop) - parseFloat(style.paddingBottom) - bar - 12);
      const maxH = Math.min(availH, window.innerHeight * 0.72, 760);
      const w = Math.min(availW, maxH * ratio);
      const h = w / ratio;
      player.style.width = `${Math.round(w)}px`;
      player.style.height = `${Math.round(h)}px`;
      player.style.aspectRatio = String(ratio);
    };
    const syncToggle = () => ui.playing(!!v && !v.paused && !v.ended);
    content.classList.add("stage-scroll");
    const ac = new AbortController();
    const onAbort = () => ac.abort();
    if (ctx.signal.aborted) ac.abort(); else ctx.signal.addEventListener("abort", onAbort, { once: true });
    let tries = 0;
    let retryTimer = 0;
    let waitTimer = 0;
    let srcSeq = 0;
    let leaving = false;
    let lastSaved = 0;
    let available: Gear[] | null = null;
    let choices: QualityChoice[] = [];
    let activeGear: Gear | undefined;
    let frameTimer = 0;
    let switching = false;
    let desiredPlaying = true;
    let restore: { time: number; paused: boolean; rate: number } | null = null;
    let fullPass = 0;
    let keyNote = "";
    const rejected = new Set<string>();
    const gearKey = (g: Gear) => gearId(g);
    const refreshQuality = () => {
      const all = qualityChoices(available ?? []);
      choices = all.filter(c => !rejected.has(gearKey(c.gear)));
      const current = all.find(c => c.gear.name === activeGear?.name && c.gear.h264 === activeGear?.h264);
      const note = keyNote ? " · V2" : "";
      ui.qualityLabel(current ? `${PLAYER_QUALITY.levels[current.level]} · ${Math.min(current.gear.width, current.gear.height)}P${note}` : `${PLAYER_QUALITY.unavailable}${note}`);
      ui.qualityOptions(all.map(c => {
        const dead = rejected.has(gearKey(c.gear));
        return `<button type="button" data-act="quality" data-level="${c.level}" class="${dead ? "dead" : ""}" aria-pressed="${c === current}">${PLAYER_QUALITY.levels[c.level]} · ${Math.min(c.gear.width, c.gear.height)}P${dead ? ` · ${PLAYER_QUALITY.dead}` : ""}</button>`;
      }).join(""));
    };
    const alive = () => !ac.signal.aborted && !leaving;
    const resetUi = () => { clearTimeout(veilTimer); ui.reset(); };

    const say = (level: "info" | "warn" | "err", text: string) => log(level, `${id} ${text}`, "player");
    const progressKey = `dy.progress.${id}`;
    const saveProgress = (force = false) => {
      if (!v || !v.duration || !Number.isFinite(v.duration)) return;
      const now = Date.now();
      if (!force && now - lastSaved < 5000) return;
      lastSaved = now;
      const at = v.ended || v.duration - v.currentTime < 10 ? 0 : v.currentTime;
      try { localStorage.setItem(progressKey, String(at)); } catch { /* 存储已满 */ }
    };

    const stop = (why: string) => {
      leaving = true;
      clearTimeout(veilTimer);
      clearTimeout(retryTimer);
      clearTimeout(waitTimer);
      clearTimeout(frameTimer);
      ac.abort();
      if (v) {
        saveProgress(true);
        try { v.pause(); } catch { /* 已释放 */ }
        const t = v.currentTime;
        v.removeAttribute("src");
        v.load();
        say("info", `释放（${why}）@${t.toFixed(2)}s`);
      }
    };

    const load = async (requested?: Gear) => {
      if (!v || !alive()) return;
      const mine = ++srcSeq;
      clearTimeout(frameTimer);
      clearTimeout(retryTimer);
      try {
        switching = true;
        const fresh = !v.currentSrc || !!requested;
        if (fresh) {
          clearTimeout(veilTimer);
          veilTimer = window.setTimeout(() => { if (alive() && mine === srcSeq) showVeil(PLAYER_QUALITY.loadingVideo); }, 200);
        }
        if (!restore && v.currentSrc) restore = { time: v.currentTime, paused: !desiredPlaying, rate: v.playbackRate };
        if (!available) available = await loadSupportedGears(id);
        if (!alive() || mine !== srcSeq) return;
        refreshQuality();
        say("info", `取播放地址 第${tries + 1}次`);
        const source = await fetchPlaySource(id, rejected, requested);
        activeGear = source.gear;
        if (source.enc && fresh && !v.readyState) showVeil(PLAYER_QUALITY.decrypting);
        if (source.keyErr && source.keyErr !== keyNote) {
          keyNote = source.keyErr;
          toast(issueOf(source.keyErr) === "v2" ? PLAYER_QUALITY.v2 : PLAYER_QUALITY.keyFailed.replace("{why}", source.keyErr), "err");
          say("warn", `播放凭证不可用：${source.keyErr}`);
        }
        if (!alive() || mine !== srcSeq) { say("info", "丢弃过期播放地址"); return; }
        say("info", `设 src ${source.src.length}B`);
        v.src = source.src;
        v.autoplay = desiredPlaying;
        queue.prefetch(id);
        refreshQuality();
        schedWait();
        frameTimer = window.setTimeout(() => {
          if (!alive() || mine !== srcSeq || !v.readyState ||
            (v.videoWidth > 0 && (!desiredPlaying || !v.getVideoPlaybackQuality || v.getVideoPlaybackQuality().totalVideoFrames > 0))) return;
          rejectCurrent();
        }, 6000);
        if (desiredPlaying) await v.play().catch((e) => {
          if (!alive()) return;
          if ((e as DOMException)?.name === "AbortError") say("info", "play 被中断");
          else say("err", `play 失败：${(e as Error)?.message || e}`);
        });
      } catch (e) {
        if (!alive() || mine !== srcSeq) return;
        hideVeil();
        const msg = String((e as Error)?.message || e);
        say("err", `取播放地址失败：${msg}`);
        if (++tries <= 2) {
          retryTimer = window.setTimeout(() => void load(activeGear), 600 * tries);
        } else {
          toast("播放加载失败：" + msg, "err");
          player?.classList.remove("playing");
        }
      }
    };

    const rejectCurrent = () => {
      if (!activeGear || !alive()) return;
      const failed = activeGear;
      if (!restore && v) restore = { time: v.currentTime, paused: !desiredPlaying, rate: v.playbackRate };
      // 实际无画面时整类 HEVC 排除，避免在几个同编码档位之间反复尝试。
      for (const g of available ?? []) if (gearKey(g) === gearKey(failed) || (!failed.h264 && !g.h264)) rejected.add(gearKey(g));
      refreshQuality();
      let next = choices[0]?.gear;
      // V2 凭证拿不到 key，换档位也一样，不跑重试。
      if (!next && keyNote.toLowerCase().includes("v2")) {
        rejected.clear();
        refreshQuality();
        v?.pause();
        return;
      }
      if (!next && fullPass++ < 1) {
        rejected.clear();
        fullPass = 1;
        refreshQuality();
        next = activeGear;
        toast(PLAYER_QUALITY.retry);
      }
      if (next) { if (next !== failed) toast(PLAYER_QUALITY.fallback); void load(next); }
      else { v?.pause(); toast(PLAYER_QUALITY.failed, "err"); }
    };

    const dmk = $("#dmk", content);
    let startDmk = () => {};

    const schedWait = () => {
      clearTimeout(waitTimer);
      waitTimer = window.setTimeout(() => {
        if (!v || !alive() || v.readyState >= 3) return;
        say("warn", `加载停滞 ready=${v.readyState} network=${v.networkState} 重试`);
        if (tries++ < 2) void load(activeGear);
      }, 8000);
    };

    if (v) {
      if (it?.cover) {
        const poster = new Image();
        poster.decoding = "async";
        poster.onload = () => {
          if (!alive() || v.videoWidth || !poster.naturalWidth || !poster.naturalHeight) return;
          posterRatio = poster.naturalWidth / poster.naturalHeight;
          fitPlayer();
        };
        poster.src = it.cover;
      }
      v.addEventListener("loadedmetadata", () => {
        hideVeil();
        clearTimeout(waitTimer);
        const saved = Number(localStorage.getItem(progressKey) || 0);
        if (restore) {
          const state = restore;
          restore = null;
          if (Number.isFinite(v.duration)) v.currentTime = Math.max(0, Math.min(state.time, v.duration - 0.1));
          v.playbackRate = state.rate;
          if (state.paused) v.pause();
        } else if (saved > 5 && saved < v.duration - 10) v.currentTime = saved;
        switching = false;
        fitPlayer();
        say("info", `元数据 ${v.duration.toFixed(2)}s ${v.videoWidth}x${v.videoHeight}${saved > 5 ? ` 续播 ${saved.toFixed(1)}s` : ""}`);
      });
      v.addEventListener("canplay", () => { clearTimeout(waitTimer); say("info", `可播放 ready=${v.readyState}`); startDmk(); });
      v.addEventListener("play", () => { if (!switching) desiredPlaying = true; syncToggle(); });
      v.addEventListener("playing", () => { clearTimeout(waitTimer); hideVeil(); ui.playing(true); say("info", `开始播放 @${v.currentTime.toFixed(2)}`); startDmk(); });
      v.addEventListener("pause", () => { if (!switching) desiredPlaying = false; ui.playing(false); say("info", `暂停 @${v.currentTime.toFixed(2)}`); });
      v.addEventListener("waiting", () => { say("warn", `缓冲等待 @${v.currentTime.toFixed(2)}`); schedWait(); });
      v.addEventListener("stalled", () => { say("warn", "网络停滞"); schedWait(); });
      v.addEventListener("ended", () => { saveProgress(true); syncToggle(); say("info", "播放结束"); if (group) void act("next"); });
      v.addEventListener("ratechange", () => say("info", `倍速 ${v.playbackRate}`));
      v.addEventListener("seeked", () => say("info", `跳转到 ${v.currentTime.toFixed(2)}`));
      v.addEventListener("error", () => {
        if (!alive()) return;
        const err = v.error;
        say("err", `视频错误 code=${err?.code ?? "-"} ${err?.message || ""} ready=${v.readyState} src=${v.currentSrc.slice(0, 120)}`);
        if (err?.code === 3 || err?.code === 4) rejectCurrent();
        else if (tries++ < 2) { retryTimer = window.setTimeout(() => void load(activeGear), 700 * tries); }
      });

      // 弹幕：全量预取后按时间轴放，取不到就静默（不影响播放）。
      const pool: { offset_ms: number; text: string }[] = [];
      let shot = 0;
      const paint = (t: number, size: number, op: number) => {
        if (!dmk || !settings().danmaku || !alive()) return;
        for (const d of pool) {
          const at = d.offset_ms / 1000;
          if (at >= t && at < t + 0.35) {
            const el = document.createElement("span");
            el.textContent = d.text;
            el.style.top = `${(shot++ * 37) % 62}%`;
            el.style.fontSize = `${size}px`;
            el.style.opacity = String(op);
            dmk.appendChild(el);
            setTimeout(() => el.remove(), 9000);
          }
        }
      };
      let dmkStarted = false;
      startDmk = () => {
        if (dmkStarted || !alive()) return;
        dmkStarted = true;
        void (async () => {
        try {
          let cursor = 0;
          for (let i = 0; i < 20 && alive(); i++) {
            const p = await danmaku(id, cursor);
            if (!alive()) return;
            pool.push(...p.items.map((x) => ({ offset_ms: x.offset_ms, text: x.text })).filter((x) => x.text));
            if (!p.hasMore || !p.items.length) break;
            cursor = p.cursor;
          }
          say("info", `弹幕 ${pool.length} 条`);
        } catch (e) { say("warn", `弹幕失败：${String((e as Error).message || e)}`); }
        })();
      };
      v.addEventListener("timeupdate", () => {
        const s = settings();
        paint(v.currentTime, s.danmakuSize, s.danmakuOpacity);
        ui.progress(v.currentTime, v.duration);
        saveProgress();
      });
      fitPlayer();
      syncToggle();
      void load();
      window.setTimeout(() => {
        if (!alive()) return;
        queue.prefetch(id);
        if (!group) return;
        void queue.ids().then((ids) => {
          if (!alive()) return;
          const tag = $("#epidx", content);
          const index = Number(tag?.dataset.index || 0);
          const total = Number(tag?.dataset.total || 0) || ids.length;
          if (tag && index && total > 1) tag.textContent = `第 ${index} / ${total} 集`;
          const next = ids[index] ?? null;
          if (next && next !== id) void loadVideo(next).catch(() => {});
        }).catch(() => {});
      }, 400);
    }

    const go = (hash: string, why: string, swap = false) => { stop(why); if (swap) goRoute(hash, true); else location.hash = hash; };

    let stepping = false;

    const pickEpisode = async () => {
      if (!group) return;
      let ids: string[];
      try {
        ids = await queue.pickIds();
      } catch (x) {
        toast(`剧集列表读取失败：${String((x as Error).message || x)}`, "err");
        return;
      }
      if (!alive() || !ids.length) {
        if (alive()) toast("剧集列表为空", "err");
        return;
      }
      const free = group.kind === "series" ? seriesMeta(itemById(id)?.raw).free : [];
      const at = ids.indexOf(id);
      const scrim = modal(`<div class="dh"><h3>${STAGE.pick}${at >= 0 ? ` <span class="dim">第 ${at + 1} / ${ids.length} 集</span>` : ""}</h3><button class="iconbtn" data-close aria-label="关闭"><svg class="i"><use href="#i-close"/></svg></button></div>
        <div class="picks">${ids.map((x, i) => `<button class="pick${x === id ? " on" : ""}${free.length && !free.includes(i + 1) ? " paid" : ""}" data-pick="${esc(x)}">${i + 1}</button>`).join("")}</div>`, { width: 520 });
      scrim.querySelector<HTMLElement>(".pick.on")?.scrollIntoView({ block: "center" });
      scrim.addEventListener("click", (e) => {
        const el = (e.target as HTMLElement).closest<HTMLElement>("[data-close],[data-pick]");
        if (!el) return;
        if (el.hasAttribute("data-close")) { closeModal(); return; }
        const next = el.dataset.pick!;
        closeModal();
        if (next !== id) go(`/stage/${next}${queueCtx}`, "选集", true);
      });
    };

    const act = async (a: string, t?: HTMLElement, e?: MouseEvent) => {
      if (a === "toggle" && v) {
        desiredPlaying = v.paused;
        if (desiredPlaying) void v.play().catch((x) => say("err", `手动播放失败：${x.message || x}`));
        else v.pause();
      }
      if (a === "quality" && v) {
        const choice = qualityChoices(available ?? []).find(c => c.level === t?.dataset.level);
        $("#quality-menu", content)?.removeAttribute("open");
        const same = !!choice && !!activeGear && gearKey(choice.gear) === gearKey(activeGear);
        if (choice && (!same || rejected.has(gearKey(choice.gear)))) {
          rejected.delete(gearKey(choice.gear));
          fullPass = 0;
          refreshQuality();
          restore = { time: v.currentTime, paused: v.paused, rate: v.playbackRate };
          desiredPlaying = !v.paused;
          tries = 0;
          void load(choice.gear);
        }
      }
      if (a === "mute" && v) { v.muted = !v.muted; if (t) t.innerHTML = `<svg class="i"><use href="#i-${v.muted ? "mute" : "volume"}"/></svg>`; say("info", v.muted ? "静音" : "取消静音"); }
      if (a === "rate" && v) { const r = v.playbackRate === 1 ? 1.5 : v.playbackRate === 1.5 ? 2 : 1; v.playbackRate = r; if (t) t.textContent = r.toFixed(1) + "×"; }
      if (a === "full") {
        if (!player) return;
        try {
          if (document.fullscreenElement) await document.exitFullscreen();
          else await player.requestFullscreen();
        } catch (x) { say("err", `全屏切换失败：${String((x as Error).message || x)}`); toast("全屏切换失败", "err"); }
      }
      if (a === "series" && group) { go(`/${group.kind}/${group.id}`, "进详情页"); return; }
      if (a === "pick") { void pickEpisode(); return; }
      if (a === "seek" && t && e && v) { const rect = t.getBoundingClientRect(); if (v.duration) v.currentTime = ((e.clientX - rect.left) / rect.width) * v.duration; }
      if (a === "next" || a === "prev") {
        if (leaving || stepping) return;
        stepping = true;
        try {
          const hit = await queue.step(id, a === "next" ? 1 : -1);
          if (!alive()) return;
          if (hit.reason === "first" || hit.reason === "last" || hit.reason === "missing" || !hit.id) {
            hideVeil();
            toast(hit.reason === "first" ? (grouped ? "已是第一集" : "已是第一条")
              : hit.reason === "last" ? (grouped ? "已是最后一集" : "已是最后一条")
              : "当前条目不在列表里");
            return;
          }
          resetUi();
          if (hit.id !== id) go(`/stage/${hit.id}${queueCtx}`, "切条", true);
          else hideVeil();
        } finally { if (!leaving) stepping = false; }
      }
      if (a === "back-feed") { stop("返回"); back("/feed"); }
      if (a === "dmk") {
        const on = !settings().danmaku;
        saveSettings({ danmaku: on });
        dmk?.replaceChildren();
        dmk?.toggleAttribute("hidden", !on);
        t?.classList.toggle("on", on);
        t?.setAttribute("aria-pressed", String(on));
        if (on) startDmk();
        toast(on ? "弹幕开" : "弹幕关");
      }
      if (a === "fav" && it) {
        const on = toggleFavorite({ id: it.id, kind: it.kind, title: it.desc, author: it.author.nickname, cover: it.cover, at: Date.now() });
        document.querySelectorAll<HTMLElement>('[data-act="fav"]').forEach((b) => b.classList.toggle("hot", on));
        toast(on ? "已收藏" : "已取消收藏");
      }
      if (a === "collect") {
        const { openTaskCreator } = await import("../lib/task-create");
        openTaskCreator({
          target: `video:${id}`, context: "video", title: "采集此作品",
          seed: it ? { type: "video", target_key: `video:${id}`, title: it.desc, subtitle: it.author.nickname, image: it.cover, aweme_id: id, kind: it.kind } : undefined,
        });
      }
      if (a === "copy-link") {
        if (it?.shareUrl) copy(it.shareUrl, "分享链接已复制");
        else toast("该作品没有分享链接", "err");
      }
      if (a === "copy-id" && it) copy(it.id, "作品 ID 已复制");
      if (a === "copy-account" && it?.author.account) copy(it.author.account, "抖音号已复制");
      if (a === "user" && it) {
        if (it.author.uid) go(`/user/${encodeURIComponent(it.author.uid)}`, "打开作者");
        else toast("没有作者 id");
      }
      document.querySelectorAll<HTMLDetailsElement>("details.actmenu[open]").forEach((d) => d.removeAttribute("open"));
    };
    (content as any).__act = act;

    const onClick = (e: MouseEvent) => {
      const t = (e.target as HTMLElement).closest<HTMLElement>("[data-act]");
      if (t) void act(t.dataset.act!, t, e);
    };
    content.addEventListener("click", onClick);
    let wheelAcc = 0;
    const onWheel = (e: WheelEvent) => {
      if ((e.target as HTMLElement).closest(".inspector")) return;
      e.preventDefault();
      if (Date.now() < wheelGate) {
        window.clearTimeout(wheelSettle);
        wheelSettle = window.setTimeout(() => { wheelGate = 0; }, 140);
        return;
      }
      wheelAcc += e.deltaY;
      if (Math.abs(wheelAcc) < 70) return;
      const next = wheelAcc > 0;
      wheelAcc = 0;
      wheelGate = Date.now() + 2500;
      window.clearTimeout(wheelSettle);
      wheelSettle = window.setTimeout(() => { wheelGate = 0; }, 140);
      void act(next ? "next" : "prev");
    };
    content.addEventListener("wheel", onWheel, { passive: false });
    const fullBtn = content.querySelector<HTMLElement>('[data-act="full"]');
    const syncFull = () => {
      const on = document.fullscreenElement === player;
      if (fullBtn) {
        fullBtn.innerHTML = `<svg class="i"><use href="#i-${on ? "unfull" : "full"}"/></svg>`;
        fullBtn.setAttribute("aria-label", on ? "退出全屏" : "全屏");
      }
      if (!on) fitPlayer();
    };
    document.addEventListener("fullscreenchange", syncFull);
    (content as any).__fullSync = syncFull;
    const fit = () => fitPlayer();
    window.addEventListener("resize", fit);

    const onKey = (e: KeyboardEvent) => {
      if (document.querySelector(".scrim.show")) return;
      const el = document.activeElement as HTMLElement | null;
      if (el?.matches("input,textarea,select,[contenteditable='true']")) return;
      if (e.key === " " || e.code === "Space") {
        e.preventDefault();
        void act("toggle");
        return;
      }
      if (e.key === "Escape" && !document.fullscreenElement) stop("Esc");
      if (e.key === "ArrowDown") void act("next");
      if (e.key === "ArrowUp") void act("prev");
    };
    document.addEventListener("keydown", onKey);
    (content as any).__key = onKey;
    (content as any).__click = onClick;
    (content as any).__wheel = onWheel;
    (content as any).__stop = stop;
    (content as any).__fit = fit;
    (content as any).__abort = onAbort;
    say("info", `挂载 kind=${it?.kind ?? "?"}`);
  },
  async inspector(ctx) {
    const it = itemById(ctx.arg);
    return `
      <div class="insp-body">
        <div class="insp-sec" id="inspInfo">${it ? inspectorInfo(it) : skeleton(2)}</div>
        <div class="insp-comhead">评论 <b id="commentCount">${esc(nfmt(it?.comments ?? 0))}</b></div>
        <div class="insp-comments" id="cscroll">
          <div class="comments" id="clist">${skeleton(3)}</div>
          <div id="cend"></div>
        </div>
      </div>`;
  },
  async inspectorMount(ctx) {
    const id = ctx.arg;
    const box = document.getElementById("inspector")!;
    ((box as any).__commentCtl as LoadMore | undefined)?.stop();
    const info = $("#inspInfo", box);
    const detail = loadVideo(id);
    void detail.then((item) => {
      if (ctx.signal.aborted || !info) return;
      info.innerHTML = inspectorInfo(item);
      const count = $("#commentCount", box);
      if (count) count.textContent = nfmt(item.comments);
    }).catch((e) => {
      if (ctx.signal.aborted || !info) return;
      info.innerHTML = `<div class="dim" style="font-size:12px">作者资料读取失败：${esc(String((e as Error).message || e))}</div>`;
    });
    const seen = new Set<string>();
    const list = $("#clist", box)!;
    const end = $("#cend", box)!;
    const scroll = $("#cscroll", box)!;
    list.innerHTML = "";
    let painted = false;
    const ctl = loadMore<any>({
      host: end,
      root: scroll,
      key: (c) => String(c.cid ?? c.comment_id ?? ""),
      load: async (cursor) => {
        const r = await loadComments(id, Number(cursor || 0), 20, [...seen].slice(-200));
        for (const c of r.items) {
          const cid = String(c.cid ?? c.comment_id ?? "");
          if (cid) seen.add(cid);
        }
        return { items: r.items, hasMore: r.hasMore, cursor: String(r.cursor) };
      },
      append: (items) => {
        if (ctx.signal.aborted) return;
        if (!painted) { list.innerHTML = ""; painted = true; }
        list.insertAdjacentHTML("beforeend", items.map((c) => {
          const nickname = String(c.user?.nickname ?? "");
          const uid = String(c.user?.sec_uid || c.user?.uid || "");
          const href = uid ? `#/user/${encodeURIComponent(uid)}` : "#/user";
          const user = uid ? `<a href="${href}" title="打开 ${esc(nickname)} 的主页">` : "<span>";
          const userEnd = uid ? "</a>" : "</span>";
          return `
          <div class="crow"><a class="cavatar" href="${href}" title="打开主页"><img src="${esc(c.user?.avatar ?? "")}" alt=""></a>
            <div class="cbody"><div class="cname">${user}${esc(nickname)}${userEnd}</div><div class="ctext">${esc(c.text)}</div>
            <div class="cmeta"><span>${esc(ago(c.create_time))}</span><span>${esc(c.ip_label ?? "")}</span><span>♥ ${esc(nfmt(c.digg_count))}</span></div></div>
          </div>`;
        }).join(""));
      },
      onEmpty: () => { if (!painted) list.innerHTML = `<div class="dim" style="font-size:12px;padding:12px 0">还没有评论</div>`; },
      onError: (e) => {
        if (!painted && isWarmingError(e)) {
          list.innerHTML = `<div class="dim" style="font-size:12px;padding:12px 0">账号还在准备或风控期，请稍后重试。</div>`;
        }
        log("err", `${id} 评论失败：${String((e as Error).message)}`, "comments");
      },
    });
    (box as any).__commentCtl = ctl;
    const onClick = (e: MouseEvent) => {
      if (!(e.target as HTMLElement).closest(".actmenu")) {
        box.querySelectorAll<HTMLDetailsElement>("details.actmenu[open]").forEach((d) => d.removeAttribute("open"));
      }
      const t = (e.target as HTMLElement).closest<HTMLElement>("[data-act]");
      if (t) (document.getElementById("content") as any).__act?.(t.dataset.act!, t, e);
    };
    box.addEventListener("click", onClick);
    (box as any).__click = onClick;
    ctl.start();
  },
  onLeave(ctx) {
    const c = ctx?.content ?? (document.getElementById("content") as HTMLElement | null);
    const stop = (c as any)?.__stop;
    if (stop) stop("离开页面");
    const commentCtl = (document.getElementById("inspector") as any)?.__commentCtl as LoadMore | undefined;
    commentCtl?.pause();
    const k = (c as any)?.__key;
    if (k) document.removeEventListener("keydown", k);
    const click = (c as any)?.__click;
    if (click) c?.removeEventListener("click", click);
    const wheel = (c as any)?.__wheel;
    if (wheel) c?.removeEventListener("wheel", wheel);
    const fullSync = (c as any)?.__fullSync;
    if (fullSync) document.removeEventListener("fullscreenchange", fullSync);
    const fit = (c as any)?.__fit as (() => void) | undefined;
    if (fit) window.removeEventListener("resize", fit);
    c?.classList.remove("stage-scroll");
    const box = document.getElementById("inspector");
    const boxClick = (box as any)?.__click;
    if (box && boxClick) box.removeEventListener("click", boxClick);
    const ab = (c as any)?.__abort;
    if (ab) ctx?.signal.removeEventListener("abort", ab);
  },
};

export default view;
