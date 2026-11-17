import { $, empty, esc, toast } from "../lib/dom";
import { COLLECT, TASK } from "../lib/copy";
import { getDashboard, recheck, type Dashboard, type DayPoint, type KindPoint, type OutcomePoint } from "../lib/tasks";
import { contentIcon } from "../lib/content";
import { go } from "../router";
import type { View } from "../router";

const bytes = (n: number) => n >= 1 << 30 ? `${(n / (1 << 30)).toFixed(1)} GB` : n >= 1 << 20 ? `${(n / (1 << 20)).toFixed(1)} MB` : n >= 1024 ? `${Math.round(n / 1024)} KB` : `${n} B`;
const dayLabel = (day: string) => day.slice(5).replace("-", "/");
const rate = (ok: number, all: number) => all ? `${Math.round(ok / all * 100)}%` : "—";
const OUTCOME: Record<string, string> = { ok: "成功", gone: "失效", risk: "风控", network: "网络", param: "参数" };
const OUTCOME_COLOR: Record<string, string> = { ok: "var(--green)", gone: "var(--text-3)", risk: "var(--amber)", network: "var(--red)", param: "var(--focus)" };

function barChart(series: DayPoint[]): string {
  const max = Math.max(1, ...series.map((d) => d.submitted));
  return `<div class="chartbars">${series.map((d) => {
    const ok = d.succeeded / max * 100, bad = d.failed / max * 100;
    return `<div class="barcol" title="${esc(d.day)}：提交 ${d.submitted}，成功 ${d.succeeded}，失败 ${d.failed}">
      <div class="stack"><i class="bad" style="height:${bad}%"></i><i class="ok" style="height:${ok}%"></i></div>
      <span>${esc(dayLabel(d.day))}</span>
    </div>`;
  }).join("")}</div>`;
}

function kindBars(kinds: KindPoint[]): string {
  const rows = kinds.filter((k) => k.active + k.done + k.failed > 0).sort((a, b) => (b.active + b.failed) - (a.active + a.failed));
  const max = Math.max(1, ...rows.map((k) => k.active + k.done + k.failed));
  if (!rows.length) return empty("还没有任务", "新建采集后会显示各类内容分布。");
  return `<div class="kindbars">${rows.map((k) => `<div class="kindrow"><span class="kico"><svg class="i"><use href="#${contentIcon(k.kind)}"/></svg></span><b>${esc(TASK.kinds[k.kind] || k.kind)}</b><div class="kbar"><i style="width:${(k.active + k.done + k.failed) / max * 100}%"></i></div><span class="mono">${k.active} 活动 · ${k.done} 完成 · ${k.failed} 失败</span></div>`).join("")}</div>`;
}

function donut(outcomes: OutcomePoint[]): string {
  const total = outcomes.reduce((n, o) => n + o.count, 0);
  if (!total) return empty("暂无结果", "完成采集后这里会显示结果构成。");
  let at = 0;
  const parts = outcomes.map((o) => {
    const from = at, to = at + o.count / total * 100;
    at = to;
    return `${OUTCOME_COLOR[o.outcome] || "var(--text-3)"} ${from}% ${to}%`;
  });
  return `<div class="donutwrap"><div class="donut" style="background:conic-gradient(${parts.join(",")})"><span><b>${total}</b><small>近 14 天</small></span></div><div class="legend">${outcomes.map((o) => `<div><i style="background:${OUTCOME_COLOR[o.outcome] || "var(--text-3)"}"></i><span>${esc(OUTCOME[o.outcome] || o.outcome)}</span><b>${o.count}</b></div>`).join("")}</div></div>`;
}

function body(d: Dashboard): string {
  const success = d.succeeded + d.failed + d.gone;
  const todayBytes = d.series.at(-1)?.bytes || 0;
  return `
    <div class="dashkpis">
      <div class="dashkpi"><span>今日采集</span><b>${d.submitted}</b><small>完成率 ${rate(d.succeeded, success)}</small></div>
      <div class="dashkpi ok"><span>今日成功</span><b>${d.succeeded}</b><small>失效 ${d.gone}</small></div>
      <div class="dashkpi bad"><span>今日失败</span><b>${d.failed}</b><small>风控 / 网络 / 参数</small></div>
      <div class="dashkpi"><span>待处理队列</span><b>${d.queue_depth}</b><small>排队与执行中</small></div>
      <div class="dashkpi"><span>今日流量</span><b>${bytes(todayBytes)}</b><small>累计 ${bytes(d.disk_bytes)}</small></div>
      <div class="dashkpi ${d.pool_available ? "ok" : "bad"}"><span>可用身份</span><b>${d.pool_available}</b><small>熔断端点 ${d.breakers}</small></div>
    </div>
    <div class="dashgrid">
      <section class="dashcard span2"><div class="sectionhead"><b>14 天采集趋势</b><span class="dim">提交 / 成功 / 失败</span></div>${barChart(d.series)}</section>
      <section class="dashcard"><div class="sectionhead"><b>结果构成</b><span class="dim">近 14 天</span></div>${donut(d.outcomes)}</section>
      <section class="dashcard span2"><div class="sectionhead"><b>任务类型</b><span class="dim">当前累计</span></div>${kindBars(d.kinds)}</section>
      <section class="dashcard"><div class="sectionhead"><b>运行健康</b></div>
        <div class="health"><div><span>今日完成</span><b>${d.completed}</b></div><div><span>今日失效</span><b>${d.gone}</b></div><div><span>平均耗时</span><b>${d.avg_duration_ms ? `${Math.round(d.avg_duration_ms / 1000)} 秒` : "—"}</b></div><div><span>熔断端点</span><b>${d.breakers}</b></div><div><span>可用身份</span><b>${d.pool_available}</b></div><div><span>累计落盘</span><b>${bytes(d.disk_bytes)}</b></div></div>
        <div class="quickrow"><button class="btn sm" data-go="/collect">${COLLECT.title}</button><button class="btn sm" data-go="/collect?kind=download">下载队列</button><button class="btn sm" data-go="/plans">定时计划</button><button class="btn sm" data-go="/library">资料库</button></div>
      </section>
    </div>`;
}

const view: View = {
  title: "概览",
  render: () => `<section class="view active dashboard">
    <div class="pagehead between"><div><h1 class="h1">概览</h1><div class="sub">全局采集、下载、录制与运行健康</div></div>
      <div class="row"><button class="btn sm" data-act="recheck">${COLLECT.recheck}</button><button class="btn sm" data-act="refresh">刷新</button><button class="btn sm primary" data-act="new">${COLLECT.new}</button></div></div>
    <div id="dash"></div>
  </section>`,
  mount(ctx) {
    let alive = true;
    let timer = 0;
    let sig = "";
    const box = $("#dash", ctx.content)!;
    const paint = (d: Dashboard, force = false) => {
      const next = JSON.stringify(d);
      if (!force && next === sig) return;
      sig = next;
      box.innerHTML = body(d);
    };
    const load = async (force = false) => {
      window.clearTimeout(timer);
      if (document.hidden) { timer = window.setTimeout(load, 2000); return; }
      try {
        const d = await getDashboard();
        if (!alive) return;
        paint(d, force);
        timer = window.setTimeout(load, d.queue_depth ? 4000 : 10000);
      } catch {
        if (!alive) return;
        if (!sig) box.innerHTML = empty("概览读取失败", "检查后端连接后重试。", '<button class="btn sm" data-act="refresh">重试</button>');
        timer = window.setTimeout(load, 6000);
      }
    };
    ctx.content.addEventListener("click", async (e) => {
      const goEl = (e.target as HTMLElement).closest<HTMLElement>("[data-go]");
      if (goEl?.dataset.go) { go(goEl.dataset.go); return; }
      const el = (e.target as HTMLElement).closest<HTMLElement>("[data-act]");
      if (!el) return;
      try {
        if (el.dataset.act === "new") {
          const { openTaskCreator } = await import("../lib/task-create");
          openTaskCreator({ title: COLLECT.new });
          return;
        }
        if (el.dataset.act === "refresh") { el.setAttribute("disabled", ""); try { await load(true); } finally { el.removeAttribute("disabled"); } return; }
        if (el.dataset.act === "recheck") {
          el.setAttribute("disabled", "");
          try { const r = await recheck(20); toast(`已提交复查 ${r.accepted} 条`); go("/collect"); }
          finally { el.removeAttribute("disabled"); }
        }
      } catch (err) { toast(String(err), "err"); }
    });
    void load(true);
    ctx.signal.addEventListener("abort", () => { alive = false; window.clearTimeout(timer); });
  },
};
export default view;
