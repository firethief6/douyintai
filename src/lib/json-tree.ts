import { $$, esc } from "./dom";

const isObject = (v: unknown): v is Record<string, unknown> | unknown[] => typeof v === "object" && v !== null;

function scalar(v: unknown): { cls: string; text: string; copy: string } {
  if (v === null) return { cls: "null", text: "null", copy: "null" };
  if (typeof v === "string") return { cls: "str", text: v, copy: v };
  if (typeof v === "boolean") return { cls: "bool", text: String(v), copy: String(v) };
  if (typeof v === "number") return { cls: "num", text: String(v), copy: String(v) };
  return { cls: "str", text: String(v), copy: String(v) };
}

function node(key: string, value: unknown, depth: number): string {
  const k = key === "" ? "" : `<span class="jkey">${esc(key)}</span>`;
  if (isObject(value)) {
    const list = Array.isArray(value);
    const entries = list ? value.map((v, i) => [String(i), v] as const) : Object.entries(value);
    if (!entries.length) return `<div class="jleaf" data-jrow data-jtext="${esc(key)}"><span class="jlabel">${k}<span class="jtype">${list ? "[]" : "{}"}</span></span><span class="jval">${list ? "空数组" : "空对象"}</span></div>`;
    return `<details class="jnode" data-jrow data-jtext="${esc(key)}" ${depth < 2 ? "open" : ""}><summary><span class="jlabel">${k}<span class="jtype">${list ? `数组 · ${entries.length}` : `对象 · ${entries.length}`}</span></span></summary><div class="jchildren">${entries.map(([k2, v2]) => node(k2, v2, depth + 1)).join("")}</div></details>`;
  }
  const s = scalar(value);
  return `<div class="jleaf" data-jrow data-jtext="${esc(`${key} ${s.text}`)}"><span class="jlabel">${k}</span><span class="jval ${s.cls}">${esc(s.text)}</span><button class="jcopy" data-jcopy="${esc(s.copy)}" title="复制值" aria-label="复制值"><svg class="i"><use href="#i-copy"/></svg></button></div>`;
}

export function jsonTree(value: unknown): string {
  return `<div class="jsontree">${node("", value, 0)}</div>`;
}

/** 按字段名或值过滤 JSON 树，自动展开命中节点的祖先。 */
export function filterJsonTree(root: HTMLElement, query: string): number {
  const q = query.trim().toLowerCase();
  const rows = $$<HTMLElement>("[data-jrow]", root);
  rows.forEach((el) => { el.hidden = false; });
  if (!q) return rows.length;
  let hits = 0;
  rows.forEach((el) => {
    const hit = (el.dataset.jtext || "").toLowerCase().includes(q);
    el.hidden = !hit;
    if (hit) {
      hits++;
      el.querySelectorAll<HTMLElement>("[data-jrow]").forEach((child) => { child.hidden = false; });
      let parent = el.parentElement?.closest<HTMLDetailsElement>("details");
      while (parent) {
        parent.hidden = false;
        parent.open = true;
        parent = parent.parentElement?.closest<HTMLDetailsElement>("details") ?? null;
      }
    }
  });
  return hits;
}
