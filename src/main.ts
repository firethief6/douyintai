import "./styles.css";
import { $, bytes, closeModal, esc, modal, toast } from "./lib/dom";
import { applyTheme, hydrateStore, isTauri, saveSettings, settings } from "./lib/store";
import { initDownloads, enqueue, subscribe as subscribeDownloads, type ResolvedDownload } from "./lib/downloads";
import { initLog, log } from "./lib/log";
import { call, downloadUrl, gears, health, music, searchSug, signStatus, video, type Gear } from "./lib/api";
import { hydrateRuns } from "./lib/collect";
import { collectVideoIds, looksLikeTarget, openTarget, openTargets, readClipboardText } from "./lib/paste";
import { itemById, remember } from "./items";
import { filterIds } from "./lib/filter";
import { back, currentRoute, forward, register, render, go } from "./router";
import { refreshCounts, setStatus, shellHTML } from "./shell";
import { STATUS } from "./lib/copy";
import { applyPlatform } from "./lib/platform";

import feed from "./views/feed";
import stage from "./views/stage";
import search from "./views/search";
import user from "./views/user";
import hot from "./views/hot";
import topic from "./views/topic";
import series, { mixView } from "./views/series";
import live from "./views/live";
import library, { favoritesView, historyView } from "./views/library";
import settingsView from "./views/settings";
import connect from "./views/connect";
import gallery from "./views/gallery";
import collect from "./views/collect";
import tasks from "./views/tasks";
import task from "./views/task";
import plans from "./views/plans";
import download from "./views/download";
import { recordView, recordsView } from "./views/records";
import logView from "./views/log";
import proxyView from "./views/proxy";

applyPlatform();
const app = document.querySelector<HTMLElement>("#app")!;
await Promise.all([hydrateStore(), hydrateRuns()]);
app.outerHTML = shellHTML();
applyTheme();
refreshCounts();

register("feed", feed);
register("stage", stage);
register("search", search);
register("user", user);
register("hot", hot);
register("topic", topic);
register("series", series);
register("mix", mixView);
register("live", live);
register("library", library);
register("settings", settingsView);
register("proxy", proxyView);
register("connect", connect);
register("gallery", gallery);
register("collect", collect);
register("tasks", tasks);
register("task", task);
register("plans", plans);
register("records", recordsView);
register("favorites", favoritesView);
register("history", historyView);
register("record", recordView);
register("download", download);
register("log", logView);

window.addEventListener("hashchange", () => { void render(); });
document.addEventListener("lib-change", refreshCounts);
document.addEventListener("collect-change", refreshCounts);
subscribeDownloads((ts) => {
  const active = ts.filter((t) => t.state === "run" || t.state === "queue" || t.state === "pause");
  const count = document.getElementById("cntDl");
  if (count) { count.textContent = String(active.length); count.hidden = active.length === 0; }
  const pill = document.getElementById("qpill");
  if (!pill) return;
  pill.hidden = active.length === 0;
  if (!active.length) return;
  const running = active.filter((t) => t.state === "run");
  const speed = running.reduce((n, t) => n + (t.speed || 0), 0);
  const state = !running.length ? (active.some((t) => t.state === "queue") ? "排队中" : "已暂停") : speed > 0 ? `${bytes(speed)}/s` : "连接中";
  const text = STATUS.queue(active.length, state);
  pill.innerHTML = `${running.length ? '<span class="ring" aria-hidden="true"></span>' : ""}${esc(text)}`;
  pill.setAttribute("aria-label", `${STATUS.queueTitle}：${text}`);
});

/* 顶栏搜索：搜索页复用同一个输入框，避免页内再放一条重复搜索栏。 */
const searchForm = $("#topSearch") as HTMLFormElement;
const topInput = $("#topSearchInput") as HTMLInputElement;
const submitSearch = () => {
  const q = topInput.value.trim();
  if (!q) return;
  if (looksLikeTarget(q)) { void openTarget(q); return; }
  if (currentRoute()?.name === "search") {
    window.dispatchEvent(new CustomEvent("top-search", { detail: q }));
    return;
  }
  go(`/search?tab=video&q=${encodeURIComponent(q)}`);
};
searchForm.addEventListener("submit", (e) => { e.preventDefault(); submitSearch(); });
let searchInputTimer = 0;
let suggestSeq = 0;
const suggestBox = $("#suggest")!;
const closeSuggest = () => { searchForm.classList.remove("open"); suggestBox.innerHTML = ""; };
const loadSuggest = async () => {
  const q = topInput.value.trim();
  const my = ++suggestSeq;
  if (!q) { closeSuggest(); return; }
  try {
    const words = await searchSug(q);
    if (my !== suggestSeq || !topInput.value.trim()) return;
    if (!words.length) { closeSuggest(); return; }
    suggestBox.innerHTML = `<div class="head">搜索联想</div>${words.map((w) => `<button type="button" data-sug="${esc(w)}"><svg class="i" style="width:14px;height:14px"><use href="#i-search"/></svg><span>${esc(w)}</span></button>`).join("")}`;
    searchForm.classList.add("open");
  } catch { closeSuggest(); }
};
topInput.addEventListener("input", () => {
  window.clearTimeout(searchInputTimer);
  searchInputTimer = window.setTimeout(() => void loadSuggest(), 220);
  if (currentRoute()?.name !== "search") return;
  window.setTimeout(() => {
    if (currentRoute()?.name === "search") window.dispatchEvent(new CustomEvent("top-search-input", { detail: topInput.value.trim() }));
  }, 280);
});
topInput.addEventListener("focus", () => { if (topInput.value.trim()) void loadSuggest(); });
suggestBox.addEventListener("mousedown", (e) => {
  const b = (e.target as HTMLElement).closest<HTMLElement>("[data-sug]");
  if (!b) return;
  e.preventDefault();
  topInput.value = b.dataset.sug || "";
  closeSuggest();
  submitSearch();
});
document.addEventListener("click", (e) => {
  if (!searchForm.contains(e.target as Node)) closeSuggest();
});
topInput.addEventListener("keydown", (e) => {
  if (e.key === "Escape") { closeSuggest(); topInput.blur(); }
  if (e.key === "ArrowDown") {
    const first = suggestBox.querySelector<HTMLElement>("[data-sug]");
    if (first) { e.preventDefault(); first.focus(); }
  }
});
suggestBox.addEventListener("keydown", (e) => {
  const b = (e.target as HTMLElement).closest<HTMLElement>("[data-sug]");
  if (!b) return;
  if (e.key === "ArrowDown") {
    const next = b.nextElementSibling as HTMLElement | null;
    if (next?.dataset.sug) { e.preventDefault(); next.focus(); }
  }
  if (e.key === "ArrowUp") {
    const prev = b.previousElementSibling as HTMLElement | null;
    if (prev?.dataset.sug) { e.preventDefault(); prev.focus(); }
    else { e.preventDefault(); topInput.focus(); }
  }
});

/* 粘贴解析 */
document.addEventListener("paste", (e) => {
  const text = e.clipboardData?.getData("text")?.trim();
  if (text && looksLikeTarget(text)) void openTargets(text);
});

async function pasteClipboard(): Promise<void> {
  try {
    const text = await readClipboardText();
    if (!looksLikeTarget(text)) { toast("剪贴板里没有抖音链接", "err"); return; }
    await openTargets(text);
  } catch (e) {
    const msg = String((e as Error)?.message ?? e);
    log("err", `读剪贴板失败：${msg}`, "paste");
    toast(`读剪贴板失败：${msg}`, "err");
  }
}

/* 下载确认 */
const galleryUrls = (item: any): string[] =>
  ((item?.raw?.images as any[]) ?? []).flatMap((x) => x?.download_url_list?.[0] ?? x?.url_list?.[0] ?? []).filter(Boolean);
const galleryLives = (item: any): string[] =>
  ((item?.raw?.images as any[]) ?? []).flatMap((x) => (x?.video?.play_addr?.url_list ?? []).slice(0, 1)).filter(Boolean);

const dayStart = (v: string): number => v ? Math.floor(new Date(`${v}T00:00:00`).getTime() / 1000) : 0;
const dayEnd = (v: string): number => v ? Math.floor(new Date(`${v}T23:59:59`).getTime() / 1000) : 0;

document.addEventListener("ask-download", async (e) => {
  const d = (e as CustomEvent<unknown>).detail;
  const raw = typeof d === "string" ? d : (d as { id?: string })?.id;
  if (!raw) return;
  let allIds = [...new Set(raw.split(",").map((x) => x.trim()).filter(Boolean))];
  // 接口来的目标已带类型前缀；短剧/合集在这里展开为分集。
  if (allIds.some((x) => x.includes(":") || /^https?:|douyin\.com/.test(x))) {
    const expanded = await collectVideoIds(allIds.join(",")).catch(() => []);
    if (expanded.length) allIds = expanded.map((x) => `video:${x}`);
  }
  const missing = allIds.map((x) => x.replace(/^video:/, "")).filter((one) => !itemById(one));
  if (missing.length) {
    const items = await Promise.all(missing.map(async (one) => await video(one).catch(() => undefined)));
    remember(items.filter((it) => it !== undefined));
  }
  let ids = allIds.map((x) => x.replace(/^video:/, ""));
  const id = ids[0];
  const batch = ids.length > 1;
  const it = (!batch && typeof d === "object" && d ? d : itemById(id)) as ReturnType<typeof itemById>;
  const resolved = new Map<string, ResolvedDownload>();
  const scrim = modal(`
    <div class="dh"><h3>加入下载队列</h3><button class="iconbtn" data-close><svg class="i"><use href="#i-close"/></svg></button></div>
    <div class="db">
      ${batch ? `<div class="preview"><b id="dlcount">${ids.length} 条作品</b><span>${esc(ids.slice(0, 3).join(" · "))}${ids.length > 3 ? " …" : ""}</span></div>
        <div class="frow"><span>类型</span><select class="input" id="dlkind" style="width:150px"><option value="all">全部</option><option value="video">仅视频</option><option value="gallery">仅图集</option></select></div>
        <div class="frow"><span>发布日期</span><div class="row"><input class="input" type="date" id="dlfrom"><span>至</span><input class="input" type="date" id="dlto"></div></div>
        <div class="frow"><span>最多条数</span><input class="input mono" id="dlnum" type="number" min="0" step="1" value="0" style="width:110px" placeholder="0=不限"></div>
        <div class="frow"><span>单文件上限</span><input class="input mono" id="dlmb" type="number" min="0" step="1" value="0" style="width:110px" placeholder="0=不限"> MB</div>`
      : it ? `<div class="preview">${it.cover ? `<img src="${esc(it.cover)}">` : ""}<div><b>${esc(it.desc)}</b><span>${esc(it.author.nickname)} · ${esc(it.id)}</span></div></div>` : `<div class="preview"><b>${esc(id)}</b></div>`}
      ${!batch && it?.kind !== "gallery" ? `<div class="frow"><span>画质</span><select class="input" id="dlgear" style="width:220px"><option value="">最高可用</option></select></div>` : ""}
      <div class="frow"><span>预计大小</span><b id="dlsize">计算中…</b></div>
      <div class="frow"><span>保存到</span><b class="mono" style="font-size:11px">${esc(settings().downloadDir)}</b></div>
      ${!batch && it?.kind !== "gallery" ? `<div class="frow"><span>同时保存</span><label class="row" style="gap:12px"><label><input type="checkbox" id="dlmusic"> 原声</label><span class="dim">封面默认随视频下载</span></label></div>` : ""}
      <div class="frow"><span>完成后</span><b>记入资料库${settings().skipExisting ? " · 已下载跳过" : ""}</b></div>
    </div>
    <div class="df"><button class="btn sm ghost" data-close>取消</button><button class="btn sm primary" data-act="ok" disabled>准备中…</button></div>`);
  const okBtn = scrim.querySelector<HTMLButtonElement>('[data-act="ok"]')!;
  const size = $("#dlsize", scrim)!;
  let done = 0, totalBytes = 0, bad = 0, skipped = 0, galleries = 0, galleryImages = 0, liveCount = 0;
  let gear = "";
  const gearSel = $("#dlgear", scrim) as HTMLSelectElement | null;
  const musicBox = $("#dlmusic", scrim) as HTMLInputElement | null;

  if (gearSel) {
    void gears(id).then((list: Gear[]) => {
      for (const g of list) {
        const label = `${g.height ? `${g.height}P` : g.quality || g.name}${g.width ? ` · ${g.width}×${g.height}` : ""}${g.bit_rate ? ` · ${(g.bit_rate / 1_000_000).toFixed(1)} Mbps` : ""}${g.size ? ` · ${bytes(g.size)}` : ""}${g.h264 ? "" : " · H.265"}`;
        const o = document.createElement("option");
        o.value = g.name || g.quality;
        o.textContent = label;
        gearSel.append(o);
      }
      if (!list.length) gearSel.options[0].textContent = "最高可用（接口未给档位）";
    }).catch((err) => { log("err", `档位读取失败：${String((err as Error).message || err)}`, "dl"); gearSel.options[0].textContent = "最高可用（档位读取失败）"; });
    gearSel.addEventListener("change", () => { gear = gearSel.value; recalc(); });
  }

  const paintEstimate = () => {
    const parts = [`已确认 ${done}/${ids.length}`];
    if (galleries) parts.push(`图集 ${galleries} 个 · ${galleryImages} 张`);
    if (liveCount) parts.push(`实况 ${liveCount} 段`);
    if (totalBytes > 0) parts.push(`视频约 ${bytes(totalBytes)}`);
    else if (done === ids.length && !galleries) parts.push("接口未返回大小");
    if (skipped) parts.push(`跳过 ${skipped}`);
    if (bad) parts.push(`失败 ${bad}`);
    size.textContent = parts.join(" · ");
    if (done === ids.length) { okBtn.disabled = false; okBtn.textContent = galleries === ids.length ? "打包图集" : "开始下载"; }
  };

  const maxMb = () => Math.max(0, Number(($("#dlmb", scrim) as HTMLInputElement | null)?.value ?? 0) || 0);
  const tally = () => {
    const cap = maxMb() * 1024 * 1024;
    totalBytes = 0;
    skipped = 0;
    for (const r of resolved.values()) {
      if (cap > 0 && (r.expect_bytes ?? 0) > cap) skipped++;
      else totalBytes += r.expect_bytes ?? 0;
    }
  };
  let generation = 0;
  const worker = async (my: number, list: string[], box: { i: number }) => {
    while (box.i < list.length) {
      const one = list[box.i++];
      const item = itemById(one);
      const urls = galleryUrls(item);
      if (item?.kind === "gallery" && urls.length) {
        const lives = galleryLives(item).length;
        log("info", `图集 ${one} ${urls.length} 图 ${lives} 实况`, "dl");
        if (my === generation) {
          galleries++;
          galleryImages += urls.length;
          liveCount += lives;
          done++;
        }
      } else {
        try {
          const r = await downloadUrl(one, gear);
          if (my === generation) {
            resolved.set(one, r);
            done++;
            tally();
          }
        } catch (err) { if (my === generation) { log("err", `预估失败 ${one}：${String((err as Error).message || err)}`, "dl"); bad++; done++; } }
      }
      if (my === generation) paintEstimate();
    }
  };
  const startWorkers = () => {
    const my = ++generation;
    const list = ids.slice();
    const box = { i: 0 };
    const n = Math.min(4, list.length);
    void Promise.all(Array.from({ length: n }, () => worker(my, list, box)));
  };
  const recalc = () => {
    // 换档位后重新估算已解析的单条。
    if (batch) return;
    resolved.clear();
    totalBytes = 0; bad = 0; skipped = 0; done = 0;
    startWorkers();
  };
  if (gearSel) void gears(id).catch(() => [] as Gear[]).finally(startWorkers);
  else startWorkers();

  const applyFilters = () => {
    if (!batch) return true;
    const kind = ($("#dlkind", scrim) as HTMLSelectElement).value as "all" | "video" | "gallery";
    const from = dayStart(($("#dlfrom", scrim) as HTMLInputElement).value);
    const to = dayEnd(($("#dlto", scrim) as HTMLInputElement).value);
    const limit = Math.max(0, Number(($("#dlnum", scrim) as HTMLInputElement).value) || 0);
    ids = filterIds(allIds.map((x) => x.replace(/^video:/, "")), itemById, { kind, from, to, limit });
    ($("#dlcount", scrim)!).textContent = `${ids.length} 条作品`;
    if (!ids.length) { okBtn.disabled = true; size.textContent = "筛选后没有作品"; return false; }
    return true;
  };

  scrim.addEventListener("click", async (ev) => {
    const t = (ev.target as HTMLElement).closest<HTMLElement>("[data-close],[data-act]");
    if (!t) return;
    if (t.hasAttribute("data-close")) { closeModal(); return; }
    if (t.dataset.act === "ok" && !okBtn.disabled) {
      closeModal();
      let ok = 0, failed = 0, over = 0;
      for (const one of ids) {
        const item = (one === id && it) || itemById(one) || { id: one, desc: one, author: { nickname: "未知" }, kind: "video" as const };
        const urls = galleryUrls(item.id === one ? item : itemById(one));
        const lives = galleryLives(item.id === one ? item : itemById(one));
        let counted = false;
        try {
          if (item.kind === "gallery" && urls.length) {
            if (isTauri()) {
              const { submitDownload } = await import("./lib/tasks");
              for (const [i, url] of urls.entries()) {
                await submitDownload(`video:${one}`, { asset_kind: "gallery", target_key: `video:${one}:gallery:${i + 1}`, url, title: item.desc, author: item.author.nickname, cover: urls[0], ext: "jpg" });
              }
              for (const [i, live] of lives.entries()) {
                await submitDownload(`video:${one}`, { asset_kind: "video", target_key: `video:${one}:live:${i + 1}`, url: live, title: item.desc, author: item.author.nickname, cover: urls[0], ext: "mp4" });
              }
              toast(`图集已入队：${urls.length} 张${lives.length ? ` · 实况 ${lives.length} 段` : ""}`);
            } else {
              urls.forEach((url) => window.open(url, "_blank"));
            }
          } else {
            const r = resolved.get(one);
            const cap = maxMb() * 1024 * 1024;
            if (!r) { over++; continue; }
            if (cap > 0 && (r.expect_bytes ?? 0) > cap) { over++; continue; }
            const task = await enqueue(item, { resolved: r, gear });
            if (task?.err) skipped++; else ok++;
            counted = true;
            if (!batch) {
              const { submitDownload } = await import("./lib/tasks");
              const save = async (info: { url: string; ext?: string; title?: string; expect_bytes?: number }, suffix: "music" | "cover") => {
                await submitDownload(`video:${one}`, {
                  asset_kind: suffix, target_key: `video:${one}:${suffix}`, url: info.url,
                  title: info.title || item.desc, author: item.author.nickname,
                  ext: info.ext || (suffix === "music" ? "mp3" : "jpg"), expected_bytes: info.expect_bytes,
                });
              };
              if (musicBox?.checked) await music(one).then((m) => save(m, "music")).catch(() => toast("原声获取失败", "err"));
            }
          }
          if (!counted) ok++;
        } catch { failed++; }
      }
      toast(
        [ok ? `完成 ${ok} 条` : "", skipped ? `跳过 ${skipped} 条` : "", over ? `超限 ${over} 条` : "", failed ? `失败 ${failed} 条` : ""].filter(Boolean).join("，") || "没有可下载项",
        failed || over ? "err" : "ok",
      );
      if (ids.some((one) => itemById(one)?.kind !== "gallery")) go("/collect?kind=download");
    }
  });

  if (batch) {
    for (const sel of ["#dlkind", "#dlfrom", "#dlto", "#dlnum"] as const) {
      $("#"+sel.slice(1), scrim)?.addEventListener("change", applyFilters);
    }
    $("#dlmb", scrim)?.addEventListener("input", () => { tally(); paintEstimate(); });
    applyFilters();
  }
});

/* 采集 */
document.addEventListener("ask-collect", async (e) => {
  const d = (e as CustomEvent<unknown>).detail;
  const raw = typeof d === "string" ? d : (d as { id?: string })?.id;
  if (!raw) return;
  // 支持 "id1,id2,…" 批量：串行跑，失败不中断。
  const ids = [...new Set(raw.split(",").map((x) => x.trim()).filter(Boolean))];
  if (isTauri()) {
    const { openTaskCreator } = await import("./lib/task-create");
    if (ids.length === 1) {
      const target = ids[0];
      const it = target.startsWith("video:") ? itemById(target.slice(6)) : undefined;
      openTaskCreator({
        target,
        context: target.startsWith("user:") ? "user" : target.startsWith("video:") ? "video" : "global",
        seed: it ? { type: "video", target_key: target, title: it.desc, subtitle: it.author.nickname, image: it.cover, aweme_id: it.id, kind: it.kind } : undefined,
      });
    } else {
      openTaskCreator({ targets: ids, context: "page" });
    }
    return;
  }
  const { runOne } = await import("./lib/collect");
  if (ids.length === 1) {
    toast("采集开始…");
    const r = await runOne(ids[0]);
    toast(r.state === "done" ? `采集完成 · ${r.comments ?? 0} 条评论` : r.state === "stopped" ? `已停止 · ${r.comments ?? 0} 条评论` : `采集失败：${r.err}`, r.state === "done" || r.state === "stopped" ? "ok" : "err");
    return;
  }
  toast(`批量采集 ${ids.length} 条，已入任务队列`);
  let ok = 0, bad = 0;
  for (const [i, id] of ids.entries()) {
    toast(`采集 ${i + 1}/${ids.length}…`);
    const r = await runOne(id).catch((err) => ({ state: "fail" as const, rows: 0, err: String(err) }));
    if (r.state === "done") ok++; else if (r.state === "stopped") ok++; else bad++;
  }
  toast(`批量结束：成功 ${ok}，失败 ${bad}`, bad ? "err" : "ok");
});

/* 命令面板 */
const COMMANDS: Array<[string, string, () => void]> = [
  ["解析剪贴板链接", "Ctrl V", () => { void pasteClipboard(); }],
  ["去推荐", "Ctrl 1", () => go("/feed")],
  ["搜索", "/", () => { topInput.focus(); }],
  ["打开下载", "Ctrl 8", () => go("/collect?kind=download")],
  ["新建采集", "", () => { void import("./lib/task-create").then(({ openTaskCreator }) => openTaskCreator()); }],
  ["概览", "", () => go("/tasks")],
  ["采集", "", () => go("/collect")],
  ["设置", "Ctrl ,", () => go("/settings")],
  ["切换主题", "", () => { const t = settings().theme === "dark" ? "light" : "dark"; saveSettings({ theme: t }); applyTheme(); }],
  ["连接诊断", "", () => go("/settings?tab=diag")],
];

function palette(): void {
  const scrim = modal(`<div class="palette"><div class="pinput"><svg class="i"><use href="#i-search"/></svg><input placeholder="输入命令或页面名…" autofocus><kbd>Esc</kbd></div><div class="plist"></div></div>`, { width: 620 });
  const input = $<HTMLInputElement>("input", scrim)!;
  const list = $(".plist", scrim)!;
  let sel = 0;
  const paint = () => {
    const q = input.value.trim().toLowerCase();
    const rows = COMMANDS.filter(([n]) => n.toLowerCase().includes(q));
    sel = Math.min(sel, Math.max(rows.length - 1, 0));
    list.innerHTML = rows.map(([n, k], i) => `<div class="pitem ${i === sel ? "on" : ""}" data-i="${i}">${esc(n)}<span class="tag2">${esc(k)}</span></div>`).join("");
    list.dataset.len = String(rows.length);
    (list as any).__rows = rows;
  };
  paint();
  input.addEventListener("input", paint);
  input.addEventListener("keydown", (e) => {
    const rows = (list as any).__rows as typeof COMMANDS;
    if (e.key === "ArrowDown") { sel = Math.min(sel + 1, rows.length - 1); paint(); e.preventDefault(); }
    if (e.key === "ArrowUp") { sel = Math.max(sel - 1, 0); paint(); e.preventDefault(); }
    if (e.key === "Enter") { closeModal(); rows[sel]?.[2](); }
    if (e.key === "Escape") closeModal();
  });
  list.addEventListener("click", (e) => {
    const i = (e.target as HTMLElement).closest<HTMLElement>("[data-i]")?.dataset.i;
    const rows = (list as any).__rows as typeof COMMANDS;
    if (i != null) { closeModal(); rows[Number(i)]?.[2](); }
  });
}

document.addEventListener("keydown", (e) => {
  if (e.key === "Escape" && document.body.classList.contains("mini")) {
    e.preventDefault();
    e.stopImmediatePropagation();
    document.body.classList.remove("mini");
    if (isTauri()) import("@tauri-apps/api/core").then(({ invoke }) => invoke("mini", { on: false })).catch(() => {});
    return;
  }
  if (e.key === "Escape") {
    const hadModal = Boolean(document.querySelector(".scrim"));
    closeModal();
    if (!hadModal && document.body.classList.contains("is-sub")) { e.preventDefault(); back(); }
    return;
  }
  if (e.ctrlKey && e.key.toLowerCase() === "k") { e.preventDefault(); palette(); return; }
  if (document.activeElement?.tagName === "INPUT" || document.activeElement?.tagName === "TEXTAREA") return;
  if (e.key === "/") { e.preventDefault(); topInput.focus(); }
  if (e.ctrlKey && e.key === "1") { e.preventDefault(); go("/feed"); }
  if (e.ctrlKey && e.key === "8") { e.preventDefault(); go("/collect?kind=download"); }
  if (e.ctrlKey && e.key === ",") { e.preventDefault(); go("/settings"); }
  if ((e.metaKey || e.ctrlKey) && e.key === "\\") { e.preventDefault(); toggleRail(); }
  const nav = e.metaKey || e.ctrlKey;
  if (e.key === "[" || (nav && e.key === "ArrowLeft") || (e.altKey && e.key === "ArrowLeft")) { e.preventDefault(); back(); return; }
  if (e.key === "]" || (nav && e.key === "ArrowRight") || (e.altKey && e.key === "ArrowRight")) { e.preventDefault(); forward(); return; }
});

function toggleRail(force?: boolean): void {
  const app = document.getElementById("app");
  if (!app) return;
  const collapsed = force ?? !app.classList.contains("rail-collapsed");
  app.classList.toggle("rail-collapsed", collapsed);
  localStorage.setItem("dy.rail", collapsed ? "collapsed" : "open");
  document.getElementById("railBtn")?.setAttribute("aria-expanded", String(!collapsed));
  document.getElementById("railBtn")?.setAttribute("title", collapsed ? "展开侧栏（⌘\\）" : "收起侧栏（⌘\\）");
}
if (localStorage.getItem("dy.rail") === "collapsed") toggleRail(true);

/* 鼠标侧键：后退/前进，和原生应用一致。 */
window.addEventListener("mousedown", (e) => {
  if (e.button === 3) { e.preventDefault(); back(); }
  else if (e.button === 4) { e.preventDefault(); forward(); }
});

/* 外壳动作 */
document.addEventListener("click", async (e) => {
  const t = (e.target as HTMLElement).closest<HTMLElement>("[data-act]");
  if (!t) return;
  const a = t.dataset.act;
  if (a === "theme") { const th = settings().theme === "dark" ? "light" : "dark"; saveSettings({ theme: th }); applyTheme(); }
  if (a === "paste") { log("info", "点击粘贴", "paste"); await pasteClipboard(); }
  if (a === "back") back();
  if (a === "conn") go("/settings?tab=conn");
  if (a === "rail") toggleRail();
  if (a === "forward") forward();
  if (a === "retry") void render(true);
  if (a === "min" || a === "max" || a === "close") {
    if (isTauri()) { const { invoke } = await import("@tauri-apps/api/core"); invoke("win_action", { action: a }); }
  }
  if (a === "mini") {
    if (!isTauri()) { toast("迷你窗只在桌面端可用，Ctrl+Shift+D 全局显隐", "err"); return; }
    const on = !document.body.classList.contains("mini");
    document.body.classList.toggle("mini", on);
    const { invoke } = await import("@tauri-apps/api/core");
    invoke("mini", { on }).catch(() => {});
    toast(on ? "迷你窗：置顶，Esc 退出" : "已回到主窗");
  }
  if (a === "reload") location.reload();
});

/* 连接状态：只跟随健康检查与持续失败，不被每次请求改写。 */
let seriousFailures = 0;
let healthFails = 0;
let recoverTimer = 0;

/** 健康检查连续失败两次才降级，避免后端忙碌时顶栏来回跳。 */
async function poll(): Promise<void> {
  try {
    await health();
    seriousFailures = 0;
    healthFails = 0;
    setStatus(true, "已连接");
  } catch {
    healthFails++;
    if (healthFails < 2 && navigator.onLine) return;
    setStatus(false, navigator.onLine ? "服务异常" : "网络不可用");
  }
}
setInterval(poll, 20000);

/** 启动自检失败时提示一次；运行期失败由请求层直接提示。 */
async function signSelfCheckHint(): Promise<void> {
  if (!isTauri()) return;
  try {
    const r = await signStatus();
    const last = r.sign.last;
    if (!last || last.backend !== r.sign.backend) return;
    if (Date.now() / 1000 - last.at > 180) return;
    toast(last.message, "err", {
      label: "去切换",
      run: () => { location.hash = "#/settings?tab=conn"; },
    });
  } catch { /* 状态读不到就不提示 */ }
}
window.addEventListener("offline", () => setStatus(false, "网络不可用"));

async function refreshDisk(): Promise<void> {
  try {
    const info = isTauri()
      ? await (await import("@tauri-apps/api/core")).invoke<{ total: number; free: number }>("disk_info", { dir: settings().downloadDir })
      : await call<{ total: number; free: number }>("/disk_info", { dir: settings().downloadDir });
    const disk = document.querySelector<HTMLElement>("#footDisk");
    const value = disk?.querySelector("b");
    if (!disk || !info?.free) return;
    value!.textContent = bytes(info.free);
    disk.setAttribute("aria-label", `磁盘可用 ${bytes(info.free)}`);
    const bar = disk.querySelector<HTMLElement>(".bar i");
    if (info.total) {
      const ratio = Math.max(0, Math.min(1, (info.total - info.free) / info.total));
      if (bar) {
        bar.style.width = `${(ratio * 100).toFixed(1)}%`;
        bar.style.background = ratio > 0.9 ? "var(--red)" : ratio > 0.75 ? "var(--amber)" : "var(--cyan)";
      }
      disk.title = STATUS.diskHelp(bytes(info.total), bytes(info.total - info.free));
    }
  } catch { /* 磁盘信息失败不挡主流程 */ }
}
setInterval(refreshDisk, 60_000);
refreshDisk();

/* 请求只影响故障计数；连续两枚严重失败才改顶栏，恢复滞后 1.5 秒。 */
window.addEventListener("api-ok", () => {
  seriousFailures = 0;
  if (recoverTimer) return;
  if (document.querySelector("#statusPill .dot.red")) {
    recoverTimer = window.setTimeout(() => {
      recoverTimer = 0;
      if (!seriousFailures && navigator.onLine) setStatus(true, "已连接");
    }, 1500);
  }
});
window.addEventListener("api-fail", (e) => {
  const status = (e as CustomEvent<{ status?: number }>).detail?.status ?? 0;
  if (status !== 0 && status < 500) return;
  seriousFailures++;
  if (seriousFailures < 2) return;
  setStatus(false, status === 0 ? "网络不可用" : "服务异常");
});

window.addEventListener("online", () => {
  void poll();
  const retry = document.querySelector<HTMLElement>("#content .errbox [data-act=\"retry\"]");
  if (retry) retry.click();
  else if (document.querySelector("#content .errbox")) void render(true);
});

window.addEventListener("unhandledrejection", (e) => {
  const r = e.reason as { stack?: string; message?: string } | undefined;
  log("err", `未处理的 Promise：${r?.stack || r?.message || String(e.reason)}`);
});
window.addEventListener("error", (e) => log("err", `${e.message} @${e.filename}:${e.lineno}:${e.colno}`));


void (async () => {
  await initLog();
  await initDownloads();
  refreshCounts();
  void refreshDisk();
  if (!settings().firstRun) { void poll(); void render(); void signSelfCheckHint(); return; }
  try { await health(); saveSettings({ firstRun: false }); void poll(); void render(); void signSelfCheckHint(); }
  catch { location.hash = "#/connect"; await render(); }
})();
