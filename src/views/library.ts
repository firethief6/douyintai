import { $, $$, bytes, closeModal, empty, esc, modal, nfmt, toast } from "../lib/dom";
import { ago } from "../lib/dom";
import { getAsset, getAssetImages, getLibrarySummary, libraryAction, listLibrary, type LibraryItem, type LibraryQuery } from "../lib/tasks";
import { isTauri, lib, libDel } from "../lib/store";
import { registerPlayList } from "../lib/play-list";
import { mediaSrc } from "../lib/api";
import { preview, type PreviewItem } from "../lib/preview";
import { contentKindIcon } from "../lib/content";
import { go, replace, type View } from "../router";
import { commentLabel, commentsIncomplete, METRIC_LABELS } from "../lib/record-meta";
import { LIBRARY, RECORD_META } from "../lib/copy";

type Layout = "grid" | "list";
const KINDS = LIBRARY.kinds;
const FILTERS: Array<[string, string]> = [["", KINDS.all], ["video", KINDS.video], ["image", KINDS.image], ["comment", KINDS.comment], ["other", KINDS.other]];
const PAGE_SIZES = [10, 20, 50];
const rawId = (item: LibraryItem) => item.id.slice(item.id.indexOf(":") + 1);
const itemTitle = (item: LibraryItem) => {
  if (item.title) return item.title;
  const key = item.target_key || "";
  if (key.startsWith("video:")) return `作品 ID ${key.slice(6)}`;
  if (key.startsWith("room:")) return `直播间 ${key.slice(5)}`;
  return key || LIBRARY.untitled;
};
const asRecord = (item: LibraryItem) => item as unknown as Parameters<typeof commentLabel>[0];

function pickDelete(): Promise<"trash" | "keep" | null> {
  return new Promise((resolve) => {
    const scrim = modal(`<div class="dh"><h3>${LIBRARY.delete}</h3><button class="iconbtn" data-close><svg class="i"><use href="#i-close"/></svg></button></div>
      <div class="db"><p>${LIBRARY.deleteHint}</p></div>
      <div class="df"><button class="btn sm ghost" data-close>${LIBRARY.cancel}</button><button class="btn sm" data-f="keep">${LIBRARY.keep}</button><button class="btn sm danger" data-f="trash">${LIBRARY.trash}</button></div>`);
    scrim.addEventListener("click", (e) => {
      const el = (e.target as HTMLElement).closest<HTMLElement>("[data-f],[data-close]");
      if (!el) return;
      closeModal();
      resolve((el.dataset.f as "trash" | "keep") || null);
    });
  });
}

const view: View = {
  title: LIBRARY.title,
  render: (ctx) => {
    const legacy = ctx.query.get("tab") === "records" ? "comment" : "";
    const requested = ctx.query.get("kind") || legacy;
    const kind = FILTERS.some(([value]) => value === requested) ? requested : "";
    const layout = ctx.query.get("view") === "list" ? "list" : "grid";
    return `<section class="view active library">
      <div class="pagehead between"><div><h1 class="h1">${LIBRARY.title}</h1><div class="sub" id="lsum">${LIBRARY.sub}</div></div><div class="row"><button class="btn sm ghost" data-act="export">${LIBRARY.export}</button></div></div>
      <p class="libdesc">${LIBRARY.desc}</p>
      <div class="kindfilter" role="group" aria-label="${LIBRARY.kindsLabel}">${FILTERS.map(([value, label]) => `<button class="kindtab ${value === kind ? "active" : ""}" data-kind="${value}" aria-pressed="${value === kind}">${label}<em id="kind-${value || "all"}">0</em></button>`).join("")}</div>
      <div class="assetbar"><input class="input grow" id="lq" type="search" placeholder="${LIBRARY.search}" aria-label="${LIBRARY.search}"><select class="select" id="ltime" aria-label="${LIBRARY.time}"><option value="">${LIBRARY.timeAll}</option><option value="today">${LIBRARY.timeToday}</option><option value="7">${LIBRARY.time7}</option><option value="30">${LIBRARY.time30}</option><option value="older">${LIBRARY.timeOlder}</option></select><span class="grow"></span><button class="seg ${layout === "grid" ? "active" : ""}" data-view="grid">${LIBRARY.grid}</button><button class="seg ${layout === "list" ? "active" : ""}" data-view="list">${LIBRARY.list}</button></div>
      <div class="libhead"><label class="pickall"><input type="checkbox" id="lselectpage" aria-label="${LIBRARY.selectPage}"><span>${LIBRARY.selectPage}</span></label><span id="lcount" class="dim"></span><span class="grow"></span><div id="lbulk" class="libbulk" hidden></div></div>
      <div id="lres"></div><div id="lpager" class="pagerbar"></div>
    </section>`;
  },
  mount(ctx) {
    const legacy = ctx.query.get("tab") === "records" ? "comment" : "";
    const requested = ctx.query.get("kind") || legacy;
    let kind: string = FILTERS.some(([value]) => value === requested) ? requested : "";
    let viewMode: Layout = ctx.query.get("view") === "list" ? "list" : "grid";
    let rows: Array<LibraryItem & { displayThumb?: string }> = [];
    let page = 0;
    let pageSize = 20;
    let total = 0;
    let next = "";
    let loading = false;
    let seq = 0;
    let selectAll = false;
    const selected = new Set<string>();
    const cursors = [""];
    const res = $("#lres", ctx.content)!;
    const sum = $("#lsum", ctx.content)!;
    const pagerBox = $("#lpager", ctx.content)!;
    const bulkBox = $("#lbulk", ctx.content)!;
    const countEl = $("#lcount", ctx.content)!;
    const timer = { value: 0 };

    const dayStart = (offset = 0) => {
      const d = new Date();
      d.setHours(0, 0, 0, 0);
      d.setDate(d.getDate() + offset);
      return Math.floor(d.getTime() / 1000);
    };
    const range = (): { from?: number; to?: number } => {
      const v = ($("#ltime", ctx.content) as HTMLSelectElement | null)?.value || "";
      if (v === "today") return { from: dayStart(), to: dayStart(1) };
      if (v === "7") return { from: dayStart(-6), to: dayStart(1) };
      if (v === "30") return { from: dayStart(-29), to: dayStart(1) };
      if (v === "older") return { to: dayStart(-30) };
      return {};
    };
    const scopeFilters = (): LibraryQuery => ({
      q: ($("#lq", ctx.content) as HTMLInputElement | null)?.value.trim() || undefined,
      ...range(),
    });
    const filters = (): LibraryQuery => ({ ...scopeFilters(), kind: kind || undefined, limit: pageSize });
    const route = () => replace(`/library?kind=${kind}${viewMode !== "grid" ? `&view=${viewMode}` : ""}`);

    const paintKinds = (counts?: { total: number; video: number; image: number; comment: number; other: number }) => {
      $$<HTMLElement>("[data-kind]", ctx.content).forEach((b) => {
        const on = b.dataset.kind === kind;
        b.classList.toggle("active", on);
        b.setAttribute("aria-pressed", String(on));
      });
      if (counts) {
        const all = document.getElementById("kind-all");
        if (all) all.textContent = nfmt(counts.total);
        for (const key of ["video", "image", "comment", "other"] as const) {
          const el = document.getElementById(`kind-${key}`);
          if (el) el.textContent = nfmt(counts[key]);
        }
      }
    };

    const pagerHtml = () => {
      const pages = Math.max(1, Math.ceil(total / pageSize));
      return `<span>${LIBRARY.perPage}</span><select class="select" data-page-size aria-label="${LIBRARY.pageSize}">${PAGE_SIZES.map((n) => `<option value="${n}" ${n === pageSize ? "selected" : ""}>${n} ${LIBRARY.unit}</option>`).join("")}</select><span class="dim">${LIBRARY.page(page + 1, pages)}</span><button class="btn sm ghost" data-page-prev ${page === 0 || loading ? "disabled" : ""}>${LIBRARY.prev}</button><button class="btn sm" data-page-next ${!next || loading ? "disabled" : ""}>${LIBRARY.next}</button>`;
    };
    const paintPager = () => { pagerBox.innerHTML = pagerHtml(); };

    const paintHead = () => {
      countEl.textContent = total ? `${LIBRARY.total(total)} · ${LIBRARY.pageLocal(rows.length)}` : rows.length ? LIBRARY.pageLocal(rows.length) : "";
      const head = $("#lselectpage", ctx.content) as HTMLInputElement | null;
      if (head) head.checked = rows.length > 0 && rows.every((x) => selected.has(x.id));
    };
    const paintBulk = () => {
      const n = selectAll ? total : selected.size;
      bulkBox.hidden = n === 0;
      if (!n) { bulkBox.innerHTML = ""; return; }
      const canAll = !selectAll && total > selected.size;
      bulkBox.innerHTML = `<span class="bulksel">${LIBRARY.selected(n)}</span>${selectAll ? `<span class="dim">${LIBRARY.selectedScope}</span>` : canAll ? `<button class="linkbtn" data-act="select-all-filter">${LIBRARY.allFilter(total)}</button>` : ""}<button class="btn sm" data-act="del-keep">${LIBRARY.keep}</button><button class="btn sm danger" data-act="del-trash">${LIBRARY.trash}</button><button class="btn sm ghost" data-act="clear-select">${LIBRARY.clear}</button>`;
    };

    const cardActions = (item: LibraryItem) => item.source === "record"
      ? `<button class="act" data-open="${esc(item.id)}">${LIBRARY.summaryAction}</button><button class="act" data-rev="${esc(item.path || "")}" ${item.path ? "" : "disabled"}>${LIBRARY.dir}</button><button class="act" data-del="${esc(item.id)}" aria-label="${LIBRARY.delete}"><svg class="i"><use href="#i-trash"/></svg></button>`
      : `<button class="act" data-open="${esc(item.id)}">${LIBRARY.preview}</button><button class="act" data-rev="${esc(item.path || "")}">${LIBRARY.dir}</button><button class="act" data-del="${esc(item.id)}" aria-label="${LIBRARY.delete}"><svg class="i"><use href="#i-trash"/></svg></button>`;

    const itemCard = (item: LibraryItem & { displayThumb?: string }) => {
      const thumbSrc = item.displayThumb || "";
      const typeIcon = item.source === "record" ? "i-json" : contentKindIcon(item.asset_kind);
      const meta = item.source === "record"
        ? `<span class="${commentsIncomplete(asRecord(item)) ? "commentmeta partial" : "commentmeta"}">${esc(commentLabel(asRecord(item)))}${RECORD_META.assets(item.asset_count)}</span>`
        : `<span>${item.size ? bytes(item.size) : item.asset_kind === "record" ? "空录制" : ""}</span>`;
      const state = item.source === "record" ? `<span class="statechip ${item.record_state === "deleted" ? "deleted" : "live"}">${item.record_state === "deleted" ? LIBRARY.deleted : LIBRARY.valid}</span>` : "";
      const missing = item.missing ? `<span class="missingtag">${item.source === "record" ? LIBRARY.jsonMissing : LIBRARY.fileMissing}</span>` : `<span class="kindtag">${KINDS[item.item_type] || KINDS.other}</span>`;
      if (viewMode === "list") {
        return `<div class="assetrow ${item.missing ? "missing" : ""}"><label><input type="checkbox" data-pick="${esc(item.id)}" ${selected.has(item.id) ? "checked" : ""} aria-label="选择 ${esc(itemTitle(item))}"></label><div class="thumb">${thumbSrc ? `<img src="${esc(thumbSrc)}" loading="lazy">` : `<svg class="i"><use href="#${typeIcon}"/></svg>`}</div><div class="grow"><b title="${esc(itemTitle(item))}">${esc(itemTitle(item))}</b><span>${esc(item.author || LIBRARY.unknownAuthor)} · ${KINDS[item.item_type] || KINDS.other} · ${item.source === "record" ? LIBRARY.summary : LIBRARY.disk}</span>${item.source === "record" ? `<span class="${commentsIncomplete(asRecord(item)) ? "commentmeta partial" : "commentmeta"}">${esc(commentLabel(asRecord(item)))}${esc(RECORD_META.assets(item.asset_count))}</span>` : ""}${item.path ? `<span class="mono">${esc(item.path)}</span>` : ""}</div><span>${item.size ? bytes(item.size) : ""}</span>${cardActions(item)}</div>`;
      }
      return `<article class="assetcard ${item.missing ? "missing" : ""}" data-id="${esc(item.id)}"><label class="pick"><input type="checkbox" data-pick="${esc(item.id)}" ${selected.has(item.id) ? "checked" : ""} aria-label="选择 ${esc(itemTitle(item))}"></label><div class="thumb" data-open="${esc(item.id)}">${thumbSrc ? `<img src="${esc(thumbSrc)}" loading="lazy">` : `<svg class="i"><use href="#${typeIcon}"/></svg>`}${state}${missing}</div><div class="meta"><b title="${esc(itemTitle(item))}">${esc(itemTitle(item))}</b><span>${esc(item.author || LIBRARY.unknownAuthor)} · ${ago(item.created_at)}</span>${meta}</div><div class="acts">${cardActions(item)}</div></article>`;
    };

    const paintItems = () => {
      sum.textContent = total ? LIBRARY.total(total) : LIBRARY.sub;
      res.innerHTML = rows.length
        ? `<div class="${viewMode === "grid" ? "assetgrid" : "assetlist"}">${rows.map(itemCard).join("")}</div>`
        : empty(LIBRARY.empty[kind || "all"] || LIBRARY.empty.all, kind ? LIBRARY.emptyHint : LIBRARY.emptyFilterHint);
      paintHead();
      paintBulk();
      paintPager();
    };

    const load = async (reset = false) => {
      if (loading && !reset) return;
      const mine = ++seq;
      loading = true;
      paintPager();
      if (reset) {
        rows = [];
        next = "";
        total = 0;
        page = 0;
        cursors.length = 1;
        cursors[0] = "";
        selected.clear();
        selectAll = false;
        res.innerHTML = "";
        pagerBox.innerHTML = "";
        paintBulk();
      }
      let completed = false;
      try {
        const [p, counted] = await Promise.all([
          listLibrary({ ...filters(), cursor: cursors[page] || "" }),
          reset ? getLibrarySummary(scopeFilters()).catch(() => null) : Promise.resolve(null),
        ]);
        if (mine !== seq) return;
        if (counted) {
          total = kind ? Number((counted as unknown as Record<string, number>)[kind] || 0) : counted.total;
          paintKinds(counted);
        }
        const loaded: Array<LibraryItem & { displayThumb?: string }> = [];
        for (const item of p.items) {
          const source = item.thumb_path || item.cover || "";
          const displayThumb = source ? await mediaSrc(source).catch(() => "") : "";
          loaded.push({ ...item, displayThumb });
        }
        rows = loaded;
        next = p.meta.next || "";
        if (next) cursors[page + 1] = next;
        else cursors.length = page + 1;
        paintItems();
        completed = true;
      } catch (e) {
        res.innerHTML = empty(LIBRARY.loadingFail, String(e));
        pagerBox.innerHTML = "";
      } finally {
        if (mine === seq) {
          loading = false;
          if (completed) paintPager();
        }
      }
    };

    const assetPreview = (item: LibraryItem): PreviewItem | null => item.path
      ? { id: rawId(item), path: item.path, kind: item.asset_kind, title: item.title, target_key: item.target_key, missing: item.missing }
      : null;

    const openAsset = async (item: LibraryItem) => {
      let previews = rows.filter((x) => x.source === "asset").map(assetPreview).filter((x): x is PreviewItem => !!x);
      if (!previews.some((x) => x.id === rawId(item))) {
        const one = assetPreview(item);
        if (one) previews = [one];
      }
      const at = previews.findIndex((x) => x.id === rawId(item));
      if (item.asset_kind === "gallery" && item.path?.toLowerCase().endsWith(".zip")) {
        const paths = await getAssetImages(rawId(item)).catch(() => [] as string[]);
        if (!paths.length) { toast(LIBRARY.galleryFail, "err"); return; }
        await preview(paths.map((path, i) => ({ ...assetPreview(item)!, id: `${rawId(item)}:${i}`, path, title: `${item.title || "图集"} ${i + 1}` })), 0);
        return;
      }
      if (at >= 0) await preview(previews, at);
    };

    const metricHtml = (metrics: Record<string, unknown>) => {
      const entries = Object.entries(metrics)
        .filter(([k, v]) => (k === "digest" ? String(v).trim() !== "" : !["", "0"].includes(String(v).trim())))
        .slice(0, 8);
      return entries.length ? `<div class="metricrow">${entries.map(([k, v]) => `<span><b>${esc(METRIC_LABELS[k] || k)}</b>${esc(String(v))}</span>`).join("")}</div>` : "";
    };

    const recordPeek = (item: LibraryItem) => {
      const scrim = modal(`<div class="dh"><h3>${esc(itemTitle(item))}</h3><button class="iconbtn" data-close><svg class="i"><use href="#i-close"/></svg></button></div>
        <div class="db"><p class="dim">${esc(item.author || LIBRARY.unknownAuthor)} · ${esc(ago(item.created_at))} · ${item.record_state === "deleted" ? LIBRARY.workDeleted : LIBRARY.workLive}</p><p>${esc(commentLabel(asRecord(item)))}${esc(RECORD_META.assets(item.asset_count))}</p>${metricHtml(item.metrics)}</div>
        <div class="df"><button class="btn sm ghost" data-close>${LIBRARY.close}</button><button class="btn sm ghost" data-peek-rev ${item.path ? "" : "disabled"}>${LIBRARY.dir}</button><button class="btn sm ghost" data-peek-work ${item.target_key.startsWith("video:") ? "" : "disabled"}>${LIBRARY.work}</button><button class="btn sm ghost" data-peek-detail>${LIBRARY.comments}</button><button class="btn sm" data-peek-open ${item.path && !item.missing ? "" : "disabled"}>${LIBRARY.openJson}</button></div>`, { width: 620 });
      scrim.addEventListener("click", (e) => {
        const el = (e.target as HTMLElement).closest<HTMLElement>("[data-close],[data-peek-open],[data-peek-rev],[data-peek-work],[data-peek-detail]");
        if (!el) return;
        if (el.hasAttribute("data-close")) { closeModal(); return; }
        if (el.dataset.peekRev !== undefined && item.path) { void import("../lib/store").then(({ invokeOr }) => invokeOr("reveal", { path: item.path! })).catch(() => toast(LIBRARY.openFail, "err")); return; }
        if (el.dataset.peekWork !== undefined && item.target_key.startsWith("video:")) { closeModal(); go(`/stage/${encodeURIComponent(item.target_key.slice(6))}`); return; }
        if (el.dataset.peekOpen !== undefined && item.path) { closeModal(); void openJson(item.path); return; }
        if (el.dataset.peekDetail !== undefined) { closeModal(); go(`/record/${encodeURIComponent(rawId(item))}?from=library`); }
      });
    };

    const openJson = async (path: string) => {
      if (!isTauri()) { toast(LIBRARY.desktopOnly, "err"); return; }
      try {
        const { invokeOr } = await import("../lib/store");
        await invokeOr("open_path", { path });
      } catch (e) { toast(String(e), "err"); }
    };

    const openItem = (item: LibraryItem) => item.source === "record" ? recordPeek(item) : void openAsset(item);

    const deleteMany = (trash: boolean) => {
      const n = selectAll ? total : selected.size;
      if (!n) return;
      const scope = selectAll ? LIBRARY.filterScopeName : LIBRARY.selectedScopeName;
      const scrim = modal(`<div class="dh"><h3>${trash ? LIBRARY.deleteTrashTitle(n) : LIBRARY.deleteKeepTitle(n)}</h3><button class="iconbtn" data-close><svg class="i"><use href="#i-close"/></svg></button></div>
        <div class="db"><p>${trash ? LIBRARY.deleteTrashBody(scope, n) : LIBRARY.deleteKeepBody(scope, n)}</p></div>
        <div class="df"><button class="btn sm ghost" data-close>${LIBRARY.cancel}</button><button class="btn sm ${trash ? "danger" : ""}" data-act="confirm">${trash ? LIBRARY.confirmTrash : LIBRARY.keep}</button></div>`);
      scrim.addEventListener("click", async (e) => {
        const el = (e.target as HTMLElement).closest<HTMLElement>("[data-close],[data-act]");
        if (!el) return;
        if (el.hasAttribute("data-close")) { closeModal(); return; }
        (el as HTMLButtonElement).disabled = true;
        try {
          const out = await libraryAction("delete", { ids: selectAll ? undefined : [...selected], filter: selectAll ? filters() : undefined, file: trash ? "trash" : "keep" });
          closeModal();
          toast(LIBRARY.actionDone(out.count), "ok");
          await load(true);
        } catch (err) { (el as HTMLButtonElement).disabled = false; toast(String(err), "err"); }
      });
    };

    const del = async (id: string) => {
      const mode = await pickDelete();
      if (!mode) return;
      try {
        const out = await libraryAction("delete", { ids: [id], file: mode });
        toast(LIBRARY.actionDone(out.count), "ok");
        await load(true);
      } catch (e) { toast(LIBRARY.actionFail(String(e)), "err"); }
    };

    const exportRows = async () => {
      const checked = [...selected];
      const out = checked.length ? rows.filter((x) => checked.includes(x.id)) : rows;
      const text = JSON.stringify(out, null, 2);
      if (isTauri()) {
        const { save } = await import("@tauri-apps/plugin-dialog");
        const path = await save({ defaultPath: LIBRARY.exportName, filters: [{ name: "JSON", extensions: ["json"] }] });
        if (path) { const { invokeOr } = await import("../lib/store"); await invokeOr("save_text", { path, text }); toast(LIBRARY.exportDone); }
      } else {
        const a = document.createElement("a");
        a.href = URL.createObjectURL(new Blob([text], { type: "application/json" }));
        a.download = LIBRARY.exportName;
        a.click();
      }
    };

    const openDirectAsset = async (id: string) => {
      const hit = rows.find((x) => x.source === "asset" && rawId(x) === id);
      if (hit) { void openAsset(hit); return; }
      try {
        const a = await getAsset(id);
        const item: LibraryItem = {
          id: `asset:${a.id}`, source: "asset", item_type: a.kind === "gallery" ? "image" : a.kind === "video" ? "video" : "other",
          asset_kind: a.kind, title: a.title, author: a.author, target_key: a.target_key, cover: a.cover, thumb_path: null,
          path: a.path, size: a.size, created_at: a.created_at, missing: a.missing, record_state: null, metrics: {},
          comments_requested: false, comment_count: 0, comment_target: 0, comment_outcome: null, asset_count: 0,
        };
        void openAsset(item);
      } catch { toast(LIBRARY.assetGone, "err"); }
    };

    ctx.content.addEventListener("click", (e) => {
      const el = (e.target as HTMLElement).closest<HTMLElement>("[data-kind],[data-view],[data-open],[data-rev],[data-del],[data-act],[data-page-prev],[data-page-next],[data-lib-open],[data-lib-del]");
      if (!el) return;
      if (el.dataset.kind !== undefined) { kind = el.dataset.kind; page = 0; paintKinds(); route(); void load(true); return; }
      if (el.dataset.view) { viewMode = el.dataset.view as Layout; $$("[data-view]", ctx.content).forEach((b) => b.classList.toggle("active", b === el)); route(); paintItems(); return; }
      if (el.dataset.open) { const item = rows.find((x) => x.id === el.dataset.open); if (item) openItem(item); return; }
      if (el.dataset.rev) { if (el.dataset.rev) void import("../lib/store").then(({ invokeOr }) => invokeOr("reveal", { path: el.dataset.rev! })).catch(() => toast(LIBRARY.openFail, "err")); return; }
      if (el.dataset.del) { void del(el.dataset.del); return; }
      if (el.dataset.libOpen) { go(`/stage/${encodeURIComponent(el.dataset.libOpen)}`); return; }
      if (el.dataset.libDel) { libDel("favorites", el.dataset.libDel); return; }
      if (el.dataset.act === "export") { void exportRows(); return; }
      if (el.dataset.act === "select-all-filter") { selectAll = true; selected.clear(); paintBulk(); paintHead(); return; }
      if (el.dataset.act === "clear-select") { selected.clear(); selectAll = false; paintBulk(); paintHead(); return; }
      if (el.dataset.act === "del-keep") { deleteMany(false); return; }
      if (el.dataset.act === "del-trash") { deleteMany(true); return; }
      if (el.dataset.pagePrev !== undefined) { if (!loading && page > 0) { page -= 1; void load(); } return; }
      if (el.dataset.pageNext !== undefined) { if (!loading && next) { page += 1; void load(); } }
    });

    ctx.content.addEventListener("change", (e) => {
      const pick = (e.target as HTMLElement).closest<HTMLInputElement>("[data-pick]");
      if (pick) { selectAll = false; pick.checked ? selected.add(pick.dataset.pick!) : selected.delete(pick.dataset.pick!); paintBulk(); paintHead(); return; }
      if ((e.target as HTMLElement).id === "lselectpage") {
        const on = (e.target as HTMLInputElement).checked;
        selectAll = false;
        for (const item of rows) on ? selected.add(item.id) : selected.delete(item.id);
        paintItems();
        return;
      }
      const size = (e.target as HTMLSelectElement).dataset.pageSize;
      if (size !== undefined) { pageSize = Number((e.target as HTMLSelectElement).value); void load(true); }
    });

    $("#ltime", ctx.content)?.addEventListener("change", () => { void load(true); });
    $("#lq", ctx.content)?.addEventListener("input", () => {
      window.clearTimeout(timer.value);
      timer.value = window.setTimeout(() => { void load(true); }, 280);
    });

    paintKinds();
    void load(true).then(() => {
      const direct = ctx.query.get("asset");
      if (direct) void openDirectAsset(direct);
    });
  },
};
export default view;

function localListView(kind: "favorites" | "history", title: string): View {
  return {
    title,
    render: () => `<section class="view active">
      <div class="pagehead between"><div><h1 class="h1">${title}</h1><div class="sub" id="llsum"></div></div></div>
      <div id="llres"></div>
    </section>`,
    mount(ctx) {
      const res = $("#llres", ctx.content)!;
      const sum = $("#llsum", ctx.content)!;
      const paint = () => {
        const rows = lib(kind);
        registerPlayList(kind, rows.map((r) => r.id));
        sum.textContent = `${rows.length} 项`;
        res.innerHTML = rows.length
          ? `<div class="assetgrid">${rows.map((r) => `<article class="assetcard" data-id="${esc(r.id)}"><div class="thumb" data-open="${esc(r.id)}">${r.cover ? `<img src="${esc(r.cover)}" loading="lazy">` : `<svg class="i"><use href="#${contentKindIcon(r.kind)}"/></svg>`}<span class="kindtag">${esc(KINDS[r.kind] || r.kind)}</span></div><div class="meta"><b>${esc(r.title)}</b><span>${esc(r.author || "未知作者")} · ${esc(ago(r.at))}</span></div><div class="acts"><button class="act" data-del="${esc(r.id)}" aria-label="移除"><svg class="i"><use href="#i-trash"/></svg></button></div></article>`).join("")}</div>`
          : empty(kind === "favorites" ? "没有收藏" : "没有观看历史", "在作品卡片上收藏或打开作品。");
      };
      ctx.content.addEventListener("click", (e) => {
        const el = (e.target as HTMLElement).closest<HTMLElement>("[data-open],[data-del]");
        if (!el) return;
        if (el.dataset.open) { go(`/stage/${encodeURIComponent(el.dataset.open)}?from=${kind}`); return; }
        if (el.dataset.del) { libDel(kind, el.dataset.del); paint(); }
      });
      paint();
    },
  };
}

export const favoritesView = localListView("favorites", "收藏");
export const historyView = localListView("history", "历史");
