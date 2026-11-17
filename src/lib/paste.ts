import { invoke } from "@tauri-apps/api/core";
import { mixVideos, resolve, seriesEpisodes } from "./api";
import { toast } from "./dom";
import { isTauri } from "./store";

const LINK = /douyin\.com|iesdouyin\.com|^(video|note|user|uid|sec|room|series|mix|collection|challenge):/i;
const TARGET = new RegExp(`${LINK.source}|^\\d{4,}$`, "i");
const FIND = /(?:https?:\/\/[^\s"'<>，,；;]*(?:douyin\.com|iesdouyin\.com)[^\s"'<>，,；;]*)|(?:(?:video|note|user|uid|sec|room|series|mix|collection|challenge):[^\s，,；;]+)|(?<!\d)\d{15,20}(?!\d)/gi;

/** 带链接的输入直接解析，不把整段分享口令当关键词。 */
export const looksLikeLink = (text: string) => LINK.test(text.trim());

/** 分享口令、短链、各类型链接、前缀或裸 ID。 */
export const looksLikeTarget = (text: string) => TARGET.test(text.trim());

/** 文本里出现的全部目标；去重保序。 */
export function extractTargets(text: string): string[] {
  return [...new Set((text.match(FIND) ?? []).map((x) => x.trim()).filter(Boolean))];
}

/** 合集/短剧逐页展开为作品 ID。 */
async function expandGroup(kind: "series" | "mix", id: string, ids: string[]): Promise<void> {
  const pager = kind === "series" ? seriesEpisodes : mixVideos;
  let cursor = "";
  const seen = new Set<string>();
  for (let p = 0; p < 200; p++) {
    const prev = cursor;
    const page = await pager(id, cursor);
    for (const it of page.items) {
      if (it.id && !seen.has(it.id)) { seen.add(it.id); ids.push(it.id); }
    }
    cursor = page.cursor;
    if (!page.hasMore || !page.items.length || cursor === prev) break;
  }
}

/** 从一段文本取出作品 ID；短链、合集与短剧逐个解析。 */
export async function collectVideoIds(text: string): Promise<string[]> {
  const ids: string[] = [];
  for (const target of extractTargets(text)) {
    const direct = target.match(/(?:^|:)(?:video|note):?(\d{15,})$/i)?.[1];
    if (direct) { ids.push(direct); continue; }
    try {
      const r = await resolve(target);
      if (r?.type === "series" || r?.type === "mix") {
        await expandGroup(r.type, String(r.id), ids);
        continue;
      }
      const id = r?.aweme_id ?? (r?.type === "video" ? r?.id : r?.items?.[0]?.aweme_id);
      if (id) ids.push(String(id));
    } catch { /* 单个失败不阻断 */ }
  }
  return [...new Set(ids)];
}

export async function readClipboardText(): Promise<string> {
  if (isTauri()) return invoke<string>("clipboard_text");
  return navigator.clipboard.readText();
}

/** 解析目标并跳转；返回是否成功。 */
export async function openTarget(text: string): Promise<boolean> {
  const raw = text.trim();
  let id = raw.match(/(?:video|note|slides)\/(\d{15,})/)?.[1];
  if (!id) {
    try {
      const r = await resolve(raw);
      if (r?.type === "user") { location.hash = `#/user/${encodeURIComponent(r.id)}`; return true; }
      if (r?.type === "room") { location.hash = `#/live/${encodeURIComponent(r.id)}`; return true; }
      if (r?.type === "series") { location.hash = `#/series/${encodeURIComponent(r.id)}`; return true; }
      if (r?.type === "mix") { location.hash = `#/mix/${encodeURIComponent(r.id)}`; return true; }
      if (r?.type === "challenge") { location.hash = `#/topic/${encodeURIComponent(r.id)}`; return true; }
      if (r?.type === "keyword") { location.hash = `#/search?tab=video&q=${encodeURIComponent(r.id)}`; return true; }
      id = r?.aweme_id ?? r?.items?.[0]?.aweme_id;
    } catch (e) {
      toast("解析失败：" + String((e as Error).message), "err");
      return false;
    }
  }
  if (!id) { toast("没认出链接里的内容", "err"); return false; }
  location.hash = `#/stage/${id}`;
  return true;
}

/** 多条目标 → 批量下载确认框；单条沿用跳转。 */
export async function openTargets(text: string): Promise<boolean> {
  const targets = extractTargets(text);
  if (targets.length <= 1) return openTarget(targets[0] ?? text);
  const ids = await collectVideoIds(text);
  if (!ids.length) return openTarget(targets[0]);
  document.dispatchEvent(new CustomEvent("ask-collect", { detail: ids.map((x) => `video:${x}`).join(",") }));
  return true;
}
