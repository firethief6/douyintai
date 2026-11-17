import { $, closeModal, empty, esc, modal, toast } from "../lib/dom";
import { addPlan, deletePlan, listPlans, pausePlan, resumePlan, updatePlan, type Plan } from "../lib/tasks";
import { go, type View } from "../router";

const when = (n: number) => new Date(n * 1000).toLocaleString();
const INTERVALS: Record<Plan["kind"], Array<[number, string]>> = {
  author_new: [[300, "5 分钟"], [600, "10 分钟"], [1800, "30 分钟"], [3600, "60 分钟"]],
  live_auto: [[30, "30 秒"], [60, "60 秒"], [120, "120 秒"]],
};

function editPlan(p?: Plan): void {
  const kind = p?.kind || "author_new";
  const scrim = modal(`<div class="dh"><h3>${p ? "编辑计划" : "新建计划"}</h3><button class="iconbtn" data-close><svg class="i"><use href="#i-close"/></svg></button></div>
    <div class="db">
      ${p ? "" : `<div class="fieldrow"><label>类型</label><select class="select" id="pkind"><option value="author_new">追新作品</option><option value="live_auto">开播自动录制</option></select></div><div class="fieldrow"><label>目标</label><input class="input mono" id="ptarget" placeholder="user:sec_uid 或 room:房间号"></div>`}
      <div class="fieldrow"><label>间隔</label><select class="select" id="pint"></select></div>
      <div id="plive" hidden><div class="fieldrow"><label>最长录制</label><select class="select" id="pmax"><option value="1800">30 分钟</option><option value="3600">1 小时</option><option value="7200">2 小时</option><option value="14400" selected>4 小时</option><option value="28800">8 小时</option><option value="0">不限</option></select></div><div class="fieldrow"><label>最大文件</label><select class="select" id="pbytes"><option value="2147483648">2 GB</option><option value="5368709120">5 GB</option><option value="21474836480" selected>20 GB</option><option value="53687091200">50 GB</option><option value="0">不限</option></select></div><div class="fieldrow"><label>磁盘保留</label><input class="input mono" id="pretain" type="number" min="5" max="100" value="20"> GB</div></div>
    </div><div class="df"><button class="btn sm ghost" data-close>取消</button><button class="btn sm primary" data-save>保存</button></div>`);
  const kindSel = $("#pkind", scrim) as HTMLSelectElement | null;
  const intSel = $("#pint", scrim) as HTMLSelectElement;
  const live = $("#plive", scrim)!;
  const paintInt = () => {
    const k = (kindSel?.value || kind) as Plan["kind"];
    live.hidden = k !== "live_auto";
    intSel.innerHTML = INTERVALS[k].map(([v, n]) => `<option value="${v}" ${p?.interval_sec === v || (!p && v === (k === "live_auto" ? 60 : 600)) ? "selected" : ""}>${n}</option>`).join("");
  };
  kindSel?.addEventListener("change", paintInt);
  paintInt();
  scrim.addEventListener("click", async (e) => {
    const el = (e.target as HTMLElement).closest<HTMLElement>("[data-close],[data-save]");
    if (!el) return;
    if (el.hasAttribute("data-close")) { closeModal(); return; }
    const k = (kindSel?.value || kind) as Plan["kind"];
    const target = p?.target_key || ($("#ptarget", scrim) as HTMLInputElement).value.trim();
    if (!target) { toast("填目标", "err"); return; }
    const interval = Number(intSel.value);
    const params = k === "live_auto" ? { max_secs: Number(($("#pmax", scrim) as HTMLSelectElement).value), max_bytes: Number(($("#pbytes", scrim) as HTMLSelectElement).value), retain_bytes: (Number(($("#pretain", scrim) as HTMLInputElement).value) || 20) * 1024 ** 3 } : { ...(p?.params || {}) };
    try {
      if (p) await updatePlan(p.id, { interval_sec: interval, params });
      else await addPlan(k, target, interval, params);
      closeModal(); toast("计划已保存"); go("/plans"); location.reload();
    } catch (err) { toast(String(err), "err"); }
  });
}

const view: View = {
  title: "定时计划",
  render: () => `<section class="view active"><div class="pagehead between"><div><h1 class="h1">定时计划</h1><div class="sub">只追作者新作品，或直播开播自动录制</div></div><button class="btn sm primary" data-new>新建计划</button></div><div id="plist"></div></section>`,
  mount(ctx) {
    const box = $("#plist", ctx.content)!;
    const load = async () => {
      let rows: Plan[] = [];
      try { rows = await listPlans(); } catch (e) { box.innerHTML = empty("读取失败", String(e)); return; }
      box.innerHTML = rows.length ? rows.map((p) => `<div class="planrow ${p.enabled ? "" : "off"}"><div class="picon"><svg class="i"><use href="#i-${p.kind === "live_auto" ? "live" : "user"}"/></svg></div><div class="grow"><b>${esc(String(p.params.title || p.target_key))}</b><span>${p.kind === "live_auto" ? `开播自动录制 · 每 ${p.interval_sec} 秒` : `追新作品 · 每 ${p.interval_sec / 60} 分钟`} · 下次 ${when(p.next_due_at)}</span><span class="${p.last_error ? "err" : "dim"}">${esc(p.last_error || (p.enabled ? "运行中" : "已暂停"))}${p.fail_streak ? ` · 连续失败 ${p.fail_streak}` : ""}</span></div><div class="ops">${p.last_task_id ? `<button class="btn sm ghost" data-task="${esc(p.last_task_id)}">最近任务</button>` : ""}<button class="btn sm ghost" data-edit="${esc(p.id)}">编辑</button>${p.enabled ? `<button class="btn sm ghost" data-pause="${esc(p.id)}">暂停</button>` : `<button class="btn sm ghost" data-resume="${esc(p.id)}">恢复</button>`}<button class="btn sm ghost" data-del="${esc(p.id)}">删除</button></div></div>`).join("") : empty("没有计划", "可从作者主页或直播间快捷创建。");
    };
    ctx.content.addEventListener("click", async (e) => {
      const el = (e.target as HTMLElement).closest<HTMLElement>("[data-new],[data-edit],[data-pause],[data-resume],[data-del],[data-task]");
      if (!el) return;
      if (el.hasAttribute("data-new")) { editPlan(); return; }
      if (el.dataset.task) { go(`/task/${encodeURIComponent(el.dataset.task)}?from=plans`); return; }
      if (el.dataset.del) {
        if (!window.confirm("删除这个计划？已创建的任务不受影响。")) return;
        try { await deletePlan(el.dataset.del); await load(); } catch (err) { toast(String(err), "err"); }
        return;
      }
      try {
        if (el.dataset.pause) { await pausePlan(el.dataset.pause); await load(); return; }
        if (el.dataset.resume) { await resumePlan(el.dataset.resume); await load(); return; }
        if (el.dataset.edit) {
          const rows = await listPlans();
          const p = rows.find((x) => x.id === el.dataset.edit);
          if (p) editPlan(p);
        }
      } catch (err) { toast(String(err), "err"); }
    });
    void load();
  },
};
export default view;

export async function quickPlan(kind: Plan["kind"], target: string, title = ""): Promise<void> {
  const params = kind === "live_auto" ? { title, max_secs: 14400, max_bytes: 21474836480, retain_bytes: 21474836480 } : { title };
  try {
    await addPlan(kind, target, kind === "live_auto" ? 60 : 600, params);
    toast("计划已创建");
  } catch (e) {
    toast(String(e), "err");
  }
}
