import { nfmt } from "./dom";
import { RECORD_META } from "./copy";
import type { CaptureRecord } from "./tasks";

export const METRIC_LABELS: Record<string, string> = {
  digg_count: "点赞", comment_count: "评论", collect_count: "收藏", share_count: "分享",
  play_count: "播放", recommend_count: "推荐", admire_count: "赞赏", forward_count: "转发",
  download_count: "下载", exposure_count: "曝光", live_watch_count: "直播观看",
  lose_count: "流失", lose_comment_count: "评论流失", whatsapp_share_count: "WhatsApp 分享",
  aweme_id: "作品 ID",
};

export const workCommentTotal = (r: CaptureRecord): number =>
  Math.max(0, Number(r.metrics?.comment_count) || 0);

const totalComments = workCommentTotal;

export const commentLabel = (r: CaptureRecord): string => {
  const total = workCommentTotal(r);
  const work = total ? RECORD_META.work(nfmt(total)) : "";
  if (!r.comments_requested) return `${RECORD_META.notRequested}${work}`;
  const partial = commentsIncomplete(r);
  if (r.comment_target > 0) {
    return `${RECORD_META.withTarget(nfmt(r.comment_count), nfmt(r.comment_target), partial)}${work}`;
  }
  return `${RECORD_META.collected(nfmt(r.comment_count), partial)}${work}`;
};

export const commentsIncomplete = (r: CaptureRecord): boolean => {
  const underTarget = r.comment_target > 0 && r.comment_count < r.comment_target;
  const workRemaining = totalComments(r) === 0 || r.comment_count < totalComments(r);
  return r.comments_requested && (r.comment_outcome === "partial" || (underTarget && workRemaining));
};
