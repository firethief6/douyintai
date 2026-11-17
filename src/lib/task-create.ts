import { closeModal, esc, modal, toast } from "./dom";
import { TASK } from "./copy";
import { looksLikeTarget } from "./paste";
import { resolveTarget, searchTargets, type TargetCard } from "./targets";
import { addPlan, backfill, submitBatch, submitOne } from "./tasks";
import { downloadUrl, gears, type Gear } from "./api";
import { settings } from "./store";
import { go } from "../router";

type Scope = "item" | "page" | "all" | "follow";
type Context = "global" | "video" | "user" | "page";

export interface TaskCreateOptions {
  target?: string;
  targets?: string[];
  title?: string;
  context?: Context;
  pageTargets?: string[];
  seed?: TargetCard;
  recordId?: string;
}

const bytes = (n: number): string => {
  if (!n) return "未知";
  const units = ["B", "KB", "MB", "GB"];
  let v = n;
  let i = 0;
  while (v >= 1024 && i < units.length - 1) { v /= 1024; i++; }
  return `${v.toFixed(v >= 10 || i === 0 ? 0 : 1)} ${units[i]}`;
};

export function openTaskCreator(opts: TaskCreateOptions = {}): void {
  const fixedBatch = !!(opts.targets?.length && !opts.target);
  const ctx: Context = fixedBatch ? "page" : (opts.context || (opts.target ? "video" : "global"));
  const locked = fixedBatch || (!!opts.target && ctx !== "global");
  const pageTargets = [...new Set(opts.pageTargets ?? [])].filter(Boolean);
  let resolved: TargetCard | null = null;
  let scope: Scope = fixedBatch ? "page" : ctx === "video" ? "item" : ctx === "user" ? (pageTargets.length ? "page" : "all") : "item";
  let seq = 0;
  let timer = 0;
  let gear = "";

  const title = opts.title || (ctx === "video" ? "采集此作品" : ctx === "user" ? "采集作者" : fixedBatch ? `采集选中内容（${opts.targets!.length} 条）` : "新建采集");
  const scrim = modal(`
    <div class="dh"><h3>${esc(title)}</h3><button class="iconbtn" data-close><svg class="i"><use href="#i-close"/></svg></button></div>
    <div class="db">
      ${locked ? "" : `<div class="fieldrow"><label>${TASK.create.target}</label><input class="input" id="tctarget" autofocus value="${esc(opts.target || "")}" placeholder="${TASK.create.placeholder}"></div>`}
      <div id="tcstatus" class="hint dim"></div>
      <div id="tcresults"></div>
      <div id="tccard"></div>
      <div id="tcform" hidden>
        <div id="tcscope" hidden>
          <div class="sectionhead"><b>采集范围</b></div>
          <div class="segrow" id="tcscopes"></div>
          <div id="tcscopeopt"></div>
        </div>
        <div class="sectionhead"><b>采集内容</b></div>
        <div id="tcopts"></div>
        <div class="collectnote">保存到 <span class="mono">${esc(settings().downloadDir)}</span> · 完成后记入资料库</div>
      </div>
    </div>
    <div class="df"><button class="btn sm ghost" data-close>取消</button><button class="btn sm primary" data-act="submit" disabled>${ctx === "video" ? "采集此作品" : "创建采集任务"}</button></div>`, { width: 640 });

  const status = document.getElementById("tcstatus")!;
  const results = document.getElementById("tcresults")!;
  const card = document.getElementById("tccard")!;
  const form = document.getElementById("tcform")!;
  const scopeBox = document.getElementById("tcscope")!;
  const scopes = document.getElementById("tcscopes")!;
  const scopeOpt = document.getElementById("tcscopeopt")!;
  const optsBox = document.getElementById("tcopts")!;
  const submit = scrim.querySelector<HTMLButtonElement>('[data-act="submit"]')!;
  const input = document.getElementById("tctarget") as HTMLInputElement | null;

  const submitText = () => {
    if (!resolved) return fixedBatch ? `采集 ${opts.targets!.length} 条` : "创建采集任务";
    if (scope === "follow") return "创建跟踪计划";
    if (scope === "all") return "采集全部作品";
    if (scope === "page" && ctx === "user" && pageTargets.length) return `采集本页 ${pageTargets.length} 条`;
    if (resolved.type === "video") return "采集此作品";
    if (resolved.type === "user") return "采集最近一页";
    return "创建采集任务";
  };

  const invalidate = () => {
    resolved = null;
    card.innerHTML = "";
    form.hidden = true;
    submit.disabled = true;
  };

  const paintScopes = () => {
    if (!resolved || resolved.type !== "user") {
      if (resolved?.type === "video" || resolved?.type === "live") { scope = resolved.type === "video" ? "item" : "item"; scopeBox.hidden = true; }
      return;
    }
    const choices: Array<[Scope, string]> = [];
    if (pageTargets.length) choices.push(["page", `本页 ${pageTargets.length} 条`]);
    else if (ctx === "global") choices.push(["page", "最近一页"]);
    choices.push(["all", "全部作品"], ["follow", "持续跟踪新作品"]);
    if (!choices.some(([v]) => v === scope)) scope = choices[0][0];
    scopeBox.hidden = false;
    scopes.innerHTML = choices.map(([v, n]) => `<button class="chip ${v === scope ? "active" : ""}" data-scope="${v}">${n}</button>`).join("");
    paintScopeOpt();
    submit.textContent = submitText();
  };

  const paintScopeOpt = () => {
    if (scope === "all") {
      scopeOpt.innerHTML = `<div class="frow"><span>回溯上限</span><select class="select" id="tcpages" style="width:120px"><option value="10">10 页</option><option value="20">20 页</option><option value="50" selected>50 页</option></select></div>`;
    } else if (scope === "follow") {
      scopeOpt.innerHTML = `<div class="frow"><span>检查间隔</span><select class="select" id="tcint" style="width:130px"><option value="300">5 分钟</option><option value="600" selected>10 分钟</option><option value="1800">30 分钟</option><option value="3600">60 分钟</option></select></div>
        <label class="frow"><span>先补采已有作品</span><input type="checkbox" id="tcbackfill" checked></label>
        <div class="hint dim">首次先回溯最多 10 页，之后只补新作品。</div>`;
    } else {
      scopeOpt.innerHTML = "";
    }
  };

  const paintOptions = (kind: string) => {
    if (kind === "live") {
      optsBox.innerHTML = `<div class="notice">直播先登记采集；自动录制参数在计划页配置。</div>`;
    } else if (kind === "gallery") {
      optsBox.innerHTML = `
        <label class="fieldrow"><label><input type="checkbox" id="tcg" checked> ${TASK.create.gallery}</label><span class="dim">封面随图集默认保存</span></label>
        <label class="fieldrow"><label><input type="checkbox" id="tcc"> ${TASK.create.comments}</label></label>
        ${commentBox()}`;
    } else if (kind === "mixed") {
      optsBox.innerHTML = `
        <label class="fieldrow"><label><input type="checkbox" id="tcv" checked> ${TASK.create.video}</label></label>
        <label class="fieldrow"><label><input type="checkbox" id="tcg" checked> ${TASK.create.gallery}</label><span class="dim">封面默认保存</span></label>
        <label class="fieldrow"><label><input type="checkbox" id="tca"> ${TASK.create.audio}</label></label>
        <label class="fieldrow"><label><input type="checkbox" id="tcc"> ${TASK.create.comments}</label></label>
        ${commentBox()}`;
    } else {
      optsBox.innerHTML = `
        <label class="fieldrow"><label><input type="checkbox" id="tcv" checked> ${TASK.create.video}</label><span class="dim">封面随视频默认保存</span></label>
        <label class="fieldrow"><label><input type="checkbox" id="tca"> ${TASK.create.audio}</label></label>
        <label class="fieldrow"><label><input type="checkbox" id="tcc"> ${TASK.create.comments}</label></label>
        ${commentBox()}
        <div id="tcgear" hidden><div class="fieldrow"><label>画质</label><select class="select" id="tcg"><option value="">最高可用</option></select></div><div class="collectnote" id="tcsize">按最高可用画质保存</div></div>`;
    }
    const comments = document.getElementById("tcc") as HTMLInputElement | null;
    comments?.addEventListener("change", () => {
      const box = document.getElementById("tccomment");
      if (box) box.hidden = !comments.checked;
    });
  };

  const commentBox = () => `<div id="tccomment" hidden>
      <div class="fieldrow"><label>${TASK.create.level}</label><select class="select" id="tcl"><option value="1">仅一级</option><option value="2">一级 + 回复</option></select></div>
      <div class="fieldrow"><label>${TASK.create.max}</label><input class="input mono" id="tcmax" type="number" min="0" placeholder="${TASK.create.maxHint}"></div>
      <div class="fieldrow"><label>${TASK.create.replies}</label><input class="input mono" id="tcr" type="number" min="0" placeholder="${TASK.create.all}"></div>
    </div>`;

  const loadGears = (item: TargetCard) => {
    const box = document.getElementById("tcgear");
    const sel = document.getElementById("tcg") as HTMLSelectElement | null;
    if (!box || !sel || !item.aweme_id) return;
    box.hidden = false;
    void gears(item.aweme_id).then((list: Gear[]) => {
      for (const g of list) {
        const o = document.createElement("option");
        o.value = g.name || g.quality;
        o.textContent = `${g.height ? `${g.height}P` : g.quality || g.name}${g.size ? ` · ${bytes(g.size)}` : ""}${g.h264 ? "" : " · H.265"}`;
        sel.append(o);
      }
      if (!list.length) sel.options[0].textContent = "最高可用（接口未给档位）";
      sel.addEventListener("change", () => {
        gear = sel.value;
        const size = document.getElementById("tcsize");
        const g = list.find((x) => (x.name || x.quality) === gear);
        if (size) size.textContent = g?.size ? `预计 ${bytes(g.size)}` : "预计大小未知";
      });
    }).catch(() => { sel.options[0].textContent = "最高可用（档位读取失败）"; });
  };

  const choose = (item: TargetCard) => {
    resolved = item;
    results.innerHTML = "";
    status.textContent = "";
    if (input) input.value = item.target_key;
    const image = item.image ? `<img src="${esc(item.image)}" alt="">` : "";
    card.innerHTML = `<div class="preview">${image}<div><b>${esc(item.title || item.target_key)}</b><span>${esc(item.subtitle || item.target_key)}</span></div></div>`;
    form.hidden = false;
    submit.disabled = false;
    const kind = item.type === "video" ? (item.kind || "video") : item.type;
    paintOptions(kind);
    paintScopes();
    if (kind === "video") loadGears(item);
    submit.textContent = submitText();
  };

  const resolveNow = async (value: string) => {
    const my = ++seq;
    status.textContent = TASK.create.resolving;
    try {
      const item = await resolveTarget(value);
      if (my !== seq) return;
      choose(item);
    } catch (err) {
      if (my !== seq) return;
      status.textContent = `${TASK.create.badTarget}：${String((err as Error).message || err)}`;
    }
  };

  const searchNow = async (value: string) => {
    const my = ++seq;
    status.textContent = "搜索中…";
    try {
      const items = await searchTargets(value);
      if (my !== seq) return;
      status.textContent = items.length ? "" : TASK.create.searchEmpty;
      results.innerHTML = items.map((item, i) => `
        <button class="preview" data-pick="${i}" type="button" style="width:100%;text-align:left">
          <img src="${esc(item.image)}" alt="">
          <span class="grow"><b>${esc(item.title || item.target_key)}</b><span class="dim">${esc(item.type === "user" ? "用户" : "作品")} · ${esc(item.subtitle || item.target_key)}</span></span>
        </button>`).join("");
      results.querySelectorAll<HTMLElement>("[data-pick]").forEach((el) => {
        el.addEventListener("click", () => { const item = items[Number(el.dataset.pick)]; if (item) choose(item); });
      });
    } catch (err) {
      if (my !== seq) return;
      status.textContent = `搜索失败：${String((err as Error).message || err)}`;
    }
  };

  if (input) {
    input.addEventListener("input", () => {
      invalidate();
      window.clearTimeout(timer);
      const value = input.value.trim();
      results.innerHTML = "";
      if (!value) { status.textContent = ""; return; }
      timer = window.setTimeout(() => {
        if (looksLikeTarget(value)) void resolveNow(value);
        else void searchNow(value);
      }, 300);
    });
    input.addEventListener("keydown", (e) => {
      if (e.key !== "Enter") return;
      e.preventDefault();
      const value = input.value.trim();
      if (value) void resolveNow(value);
    });
  }

  scopes.addEventListener("click", (e) => {
    const el = (e.target as HTMLElement).closest<HTMLElement>("[data-scope]");
    if (!el?.dataset.scope) return;
    scope = el.dataset.scope as Scope;
    scopes.querySelectorAll<HTMLElement>("[data-scope]").forEach((b) => b.classList.toggle("active", b === el));
    paintScopeOpt();
    submit.textContent = submitText();
  });

  const assetParams = () => {
    const n = (id: string) => Math.max(0, Math.floor(Number((document.getElementById(id) as HTMLInputElement | null)?.value) || 0));
    const comments = !!(document.getElementById("tcc") as HTMLInputElement | null)?.checked;
    const video = !!(document.getElementById("tcv") as HTMLInputElement | null)?.checked;
    const gallery = !!(document.getElementById("tcg") as HTMLInputElement | null)?.checked;
    const audio = !!(document.getElementById("tca") as HTMLInputElement | null)?.checked;
    return {
      save_video: video,
      save_gallery: gallery,
      save_cover: true,
      save_music: audio,
      comments,
      level: Number((document.getElementById("tcl") as HTMLSelectElement | null)?.value) || 1,
      max_comments: n("tcmax"),
      max_replies: n("tcr"),
      target_type: resolved?.type === "user" ? "user" : "video",
      display_title: resolved?.title || "",
      display_subtitle: resolved?.subtitle || "",
      display_image: resolved?.image || "",
      ...(opts.recordId ? { record_id: opts.recordId } : {}),
    };
  };

  scrim.addEventListener("click", async (e) => {
    const el = (e.target as HTMLElement).closest<HTMLElement>("[data-close],[data-act]");
    if (!el) return;
    if (el.hasAttribute("data-close")) { closeModal(); return; }
    if (el.dataset.act !== "submit") return;
    submit.disabled = true;
    try {
      if (fixedBatch) {
        const targets = opts.targets!;
        const r = await submitBatch(targets.map((target) => ({ target, params: assetParams() })));
        if (r.rejected) throw new Error(`${r.rejected} 条未受理`);
        closeModal();
        toast(`已创建 ${r.accepted} 条采集任务`);
        go("/collect");
        return;
      }
      if (!resolved) throw new Error("还没有选择目标");
      const target = resolved.target_key;
      const params = assetParams();
      if (resolved.type === "user") {
        if (scope === "page") {
          if (pageTargets.length) {
            const r = await submitBatch(pageTargets.map((t) => ({ target: t, params })));
            if (r.rejected) throw new Error(`${r.rejected} 条未受理`);
            toast(`已创建 ${r.accepted} 条采集任务`);
          } else {
            const r = await submitOne(target, params);
            if (!r.ok) throw new Error(r.message || "提交失败");
            toast("已创建采集任务");
          }
          go("/collect");
          return;
        }
        if (scope === "all") {
          const pages = Number((document.getElementById("tcpages") as HTMLSelectElement | null)?.value) || 50;
          const r = await backfill(target, pages, params);
          if (!r.ok) throw new Error(r.message || "提交失败");
          closeModal();
          toast(`已开始采集全部作品（最多 ${pages} 页）`);
          go("/collect");
          return;
        }
        const interval = Number((document.getElementById("tcint") as HTMLSelectElement | null)?.value) || 600;
        const initial = !!(document.getElementById("tcbackfill") as HTMLInputElement | null)?.checked;
        if (initial) {
          const r = await backfill(target, 10, params);
          if (!r.ok) throw new Error(r.message || "补采提交失败");
        }
        await addPlan("author_new", target, interval, { ...params, title: resolved.title || resolved.subtitle });
        closeModal();
        toast(initial ? "跟踪计划已创建，并开始补采已有作品" : "跟踪计划已创建");
        go("/plans");
        return;
      }
      if (resolved.type === "video") {
        if (params.save_video && gear && resolved.aweme_id) {
          const info = await downloadUrl(resolved.aweme_id, gear);
          Object.assign(params, { video_url: info.url, video_id: resolved.aweme_id, video_ext: info.ext || "mp4" });
        }
        const r = await submitOne(target, params);
        if (!r.ok) throw new Error(r.message || "提交失败");
      } else {
        const r = await submitOne(target, params);
        if (!r.ok) throw new Error(r.message || "提交失败");
      }
      closeModal();
      toast(TASK.create.created);
      go("/collect");
    } catch (err) {
      submit.disabled = false;
      toast(`创建失败：${String((err as Error).message || err)}`, "err");
    }
  });

  if (fixedBatch) {
    card.innerHTML = `<div class="preview"><div><b>已选 ${opts.targets!.length} 条内容</b><span>范围固定为当前选中项</span></div></div>`;
    form.hidden = false;
    submit.disabled = false;
    paintOptions("mixed");
    scopeBox.hidden = true;
    submit.textContent = `采集 ${opts.targets!.length} 条`;
  } else if (opts.target && ctx !== "global") {
    if (input) input.value = opts.target;
    if (opts.seed) choose(opts.seed);
    else void resolveNow(opts.target);
  } else {
    window.setTimeout(() => input?.focus(), 30);
  }
  scrim.addEventListener("remove", () => window.clearTimeout(timer));
}
