import { $, empty, esc, toast } from "../lib/dom";
import { backendTail, clearBackend, exportDiagnostics, log, logPath, onLog, revealPath, setLogLevel, type Line } from "../lib/log";
import { isTauri, saveSettings, settings, type LogLevel } from "../lib/store";
import type { View } from "../router";

const show = (ls: Line[]) => ls.length
  ? ls.map((l) => `<div class="logline ${l.level}"><span class="mono">${new Date(l.at).toLocaleTimeString("zh-CN")}</span><span class="mono dim">[${esc(l.scope)}]</span>${esc(l.text)}</div>`).join("")
  : empty("还没有日志");

const view: View = {
  title: "运行日志",
  render: () => {
    const lv = settings().logLevel;
    return `<section class="view active"><div class="pagehead"><h1 class="h1">运行日志</h1><div class="sub">请求、播放、下载与错误；写入本地文件并轮转。排查问题时开 debug，复现后导出诊断包</div></div>
    <div class="row" style="margin-bottom:12px">${(["debug", "info", "warn", "error"] as const).map((v) => `<button class="btn sm ${lv === v ? "primary" : ""}" data-level="${v}" aria-pressed="${lv === v}">${v}</button>`).join("")}<span class="topsep"></span><button class="btn sm" data-act="copy">复制全部</button><button class="btn sm ghost" data-act="clear">清空</button><button class="btn sm ghost" data-act="open">打开日志文件</button><button class="btn sm primary" data-act="export">导出诊断包</button></div>
    <div id="logbox" class="logbox"></div></section>`;
  },
  async mount(ctx) {
    const box = $("#logbox", ctx.content)!;
    let cur: Line[] = [];
    const paint = (ls: Line[]) => { cur = ls; box.innerHTML = show(ls); };
    const off = onLog(paint);
    const tail = await backendTail(1000);
    if (ctx.signal.aborted) { off(); return; }
    if (tail.length) paint(tail);
    log("info", "打开运行日志", "ui");
    ctx.content.addEventListener("click", async (e) => {
      const el = (e.target as HTMLElement).closest<HTMLElement>("[data-act],[data-level]");
      if (!el) return;
      const a = el.dataset.act;
      if (el.dataset.level) {
        const lv = el.dataset.level as LogLevel;
        saveSettings({ logLevel: lv });
        try { await setLogLevel(lv); toast(lv === "debug" ? "已开启 debug，复现后导出诊断包" : "日志级别已保存"); }
        catch (err) { toast(`设置失败：${String((err as Error).message)}`); }
        document.querySelectorAll<HTMLElement>("[data-level]").forEach((b) => { b.classList.toggle("primary", b.dataset.level === lv); b.setAttribute("aria-pressed", String(b.dataset.level === lv)); });
        return;
      }
      if (a === "copy") await navigator.clipboard.writeText(cur.map((l) => `${new Date(l.at).toISOString()} [${l.scope}] ${l.text}`).join("\n"));
      if (a === "clear") { await clearBackend(); paint([]); toast("日志已清空"); }
      if (a === "open" && isTauri()) { const p = await logPath(); if (p) await revealPath(p); }
      if (a === "export") {
        try { const p = await exportDiagnostics(); toast(`诊断包已导出：${p}`); await revealPath(p); }
        catch (err) { toast(`导出失败：${String((err as Error).message)}`); }
      }
    });
    (ctx.content as any).__off = off;
  },
  onLeave() { (document.getElementById("content") as any)?.__off?.(); },
};
export default view;
