import { SPRITE } from "./icons";
import { brandMark } from "./brand";
import { settings, lib, isTauri } from "./lib/store";
import { $, esc } from "./lib/dom";
import { STATUS } from "./lib/copy";

const navItem = (route: string, icon: string, label: string, title = label, extra = ""): string => `
  <a class="navitem" data-nav="${route}" href="#/${route}" title="${title}" aria-label="${title}">
    <span class="navico"><svg class="i"><use href="#i-${icon}"/></svg>${icon === "history" || icon === "log" ? "" : `<svg class="i fill"><use href="#i-${icon}-fill"/></svg>`}</span>
    <span class="txt">${label}</span>${extra}
  </a>`;

export function shellHTML(): string {
  const s = settings();
  const backend = isTauri() ? STATUS.internalBackend : s.baseUrl.replace(/^https?:\/\//, "");
  return `
  ${SPRITE}
  <div class="app" id="app">
    <header class="window-titlebar" data-tauri-drag-region="deep">
      <div class="titlebar-product">${brandMark("mark sm")}<b>抖音台</b></div>
      <div class="titlebar-actions">
        <button class="iconbtn" data-act="back" title="返回（⌘[）" aria-label="返回"><svg class="i"><use href="#i-back"/></svg></button>
        <button class="iconbtn" data-act="theme" title="切换主题" aria-label="切换主题"><svg class="i"><use href="#i-sun"/></svg></button>
        <button class="iconbtn" data-act="mini" title="退出迷你窗（Esc）" aria-label="退出迷你窗"><svg class="i"><use href="#i-pip"/></svg></button>
      </div>
      <div class="winbtns"><button data-act="min" title="最小化" aria-label="最小化"><span class="winsym">—</span></button><button data-act="max" title="最大化" aria-label="最大化"><span class="winsym">□</span></button><button class="close" data-act="close" title="关闭" aria-label="关闭"><svg class="i"><use href="#i-close"/></svg></button></div>
    </header>
    <header class="topbar">
      <div class="top-left">
        <button class="iconbtn railbtn" id="railBtn" data-act="rail" title="收起侧栏（⌘\\）" aria-label="收起侧栏" aria-expanded="true"><svg class="i"><use href="#i-rail"/></svg></button>
        <a class="brand" href="#/feed" title="回到推荐">
          ${brandMark()}
          <div class="brand-text"><b>抖音台</b><span>Douyin Desk</span></div>
        </a>
        <button class="navback" data-act="back" title="返回（⌘[）" aria-label="返回"><svg class="i"><use href="#i-back"/></svg><span>返回</span></button>
      </div>
      <form class="searchbox" id="topSearch" role="search">
        <div class="field">
          <svg class="i" aria-hidden="true"><use href="#i-search"/></svg>
          <input id="topSearchInput" placeholder="搜索视频、用户、短剧、话题" autocomplete="off" aria-label="全局搜索">
          <kbd>/</kbd>
        </div>
        <div class="suggest" id="suggest"></div>
      </form>
      <div class="top-right">
        <button class="btn sm pastebtn" data-act="paste" title="解析剪贴板链接（⌘V）" aria-label="粘贴链接"><svg class="i"><use href="#i-paste"/></svg><span class="paste-label">粘贴链接</span></button>
        <span class="topsep" aria-hidden="true"></span>
        <button class="status" id="statusPill" data-act="conn" role="status" aria-live="polite" aria-label="服务状态：连接中" title="服务状态，点击打开连接设置"><span class="dot amber"></span><span id="statusText">连接中</span></button>
        <div class="topgroup">
          <button class="iconbtn" id="themeBtn" data-act="theme" title="切换主题" aria-label="切换主题"><svg class="i"><use href="#i-sun"/></svg></button>
          <button class="iconbtn" data-act="mini" title="迷你窗（Ctrl+Shift+D 显示/隐藏）" aria-label="切换迷你窗"><svg class="i"><use href="#i-pip"/></svg></button>
        </div>
      </div>
    </header>
    <div class="routingbar" aria-hidden="true"><i></i></div>
    <div class="offline-banner" id="offlineBanner" hidden>网络不可用，部分内容可能无法更新；恢复后可重试。</div>

    <aside class="rail" aria-label="主导航">
      <div class="group">
        <div class="label">发现</div>
        ${navItem("feed", "home", "推荐")}
        ${navItem("hot", "fire", "热榜")}
        ${navItem("live", "live", "直播")}
        ${navItem("series", "series", "短剧")}
        ${navItem("favorites", "heart", "收藏", "收藏", `<span class="count" id="cntFav">0</span>`)}
        ${navItem("history", "history", "历史")}
      </div>
      <div class="group">
        <div class="label">工作台</div>
        ${navItem("tasks", "tasks", "概览")}
        ${navItem("collect", "database", "采集", "采集中心", `<span class="count" id="cntDl">0</span>`)}
        ${navItem("plans", "clock", "计划", "定时计划")}
        ${navItem("library", "folder", "资料", "资料库")}
        ${navItem("log", "log", "日志", "运行日志")}
        ${navItem("proxy", "globe", "代理", "出口代理")}
        ${navItem("settings", "settings", "设置")}
      </div>
      <div class="spacer"></div>
    </aside>

    <div class="main" id="main">
      <div class="content" id="content"></div>
      <aside class="inspector" id="inspector"></aside>
    </div>

    <div class="toasts" id="toasts" aria-live="polite"></div>

    <footer class="footbar" aria-label="运行状态">
      <a class="qpill" id="qpill" href="#/collect?kind=download" title="${STATUS.queueTitle}" aria-label="${STATUS.queueTitle}" hidden><span class="ring" aria-hidden="true"></span><span>下载</span></a>
      <div class="right">
        <button class="footmeta" id="footInfo" data-act="conn" title="${STATUS.identityHelp}" aria-label="${STATUS.anonymous}，${esc(backend)}">
          <svg class="i" aria-hidden="true"><use href="#i-user"/></svg><span>${STATUS.anonymous}</span><span class="sep" aria-hidden="true">·</span><span class="backend">${esc(backend)}</span>
        </button>
        <span class="footdisk" id="footDisk" role="status" aria-label="磁盘余量未知"><svg class="i" aria-hidden="true"><use href="#i-server"/></svg><span>${STATUS.disk}</span><b>—</b><span class="bar" aria-hidden="true"><i></i></span></span>
      </div>
    </footer>
  </div>`;
}

export function refreshCounts(): void {
  const fav = lib("favorites").length;
  const f = $("#cntFav");
  if (f) { f.textContent = String(fav); f.hidden = fav === 0; }
  const d = $("#cntDl");
  if (d) d.hidden = (d.textContent || "0") === "0";
}

export function setStatus(ok: boolean, text: string, loading = false): void {
  const pill = $("#statusPill");
  if (!pill) return;
  pill.classList.toggle("loading", loading);
  pill.classList.toggle("ok", ok && !loading);
  pill.classList.toggle("bad", !ok && !loading);
  pill.innerHTML = `<span class="${loading ? "spinner" : "dot"}"></span><span id="statusText">${text}</span>`;
  pill.setAttribute("aria-label", `服务状态：${text}`);
  const banner = $("#offlineBanner");
  if (banner) banner.hidden = ok || (navigator.onLine && text !== "网络不可用");
}
