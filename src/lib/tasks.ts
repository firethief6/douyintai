import { call as get, del, patch, post } from "./api";

export type TaskState = "queued" | "running" | "paused" | "done" | "failed" | "dead" | "canceled";
export interface Task {
  id: string; kind: string; endpoint: string; params: Record<string, unknown>;
  target_key: string | null; state: TaskState; attempts: number; max_attempts: number;
  batch_id: string | null; outcome: string | null; error_code: string | null;
  created_at: number; started_at: number | null; finished_at: number | null;
  request_id: string | null; identity: string | null; upstream_url: string | null; retry_at: number | null; output_path: string | null;
  source: string | null; progress: number; bytes: number; cover?: string | null;
  acknowledged_at: number | null; archived_at: number | null; deleted_at: number | null;
  keep_until: number | null; trigger: string; plan_id: string | null;
  result_summary: Record<string, unknown> | null;
}

export type TaskScope = "active" | "pending" | "recent" | "history" | "deleted";
export interface TaskFilter { kind?: string; q?: string; source?: string; state?: string; batch?: string; plan?: string; kept?: string }
export interface TaskPage { items: Task[]; meta: { next: string | null } }
export interface TaskSummary { active: number; pending: number; recent: number; history: number; deleted: number; kept: number }
export interface TaskActionResult { count: number; ids: string[] }
export type TaskAction = "pause" | "cancel" | "retry" | "ack" | "keep" | "unkeep" | "delete" | "restore" | "purge";

export interface SubmitResult {
  index: number; ok: boolean; id?: string | null; state?: string | null;
  error_code?: string | null; message?: string | null; deduped: boolean;
}

export const submitOne = (target: string, params: Record<string, unknown> = {}) =>
  post<SubmitResult>("/task", { target, params });
export const submitBatch = (items: unknown[]) => post<{ results: SubmitResult[]; accepted: number; rejected: number; deduped: number }>("/tasks", { items });
export const getTask = (id: string) => get<Task>(`/task/${encodeURIComponent(id)}`);
const pageQuery = (scope: TaskScope, filter: TaskFilter = {}, cursor?: string) => {
  const q: Record<string, string> = { scope };
  for (const [k, v] of Object.entries(filter)) if (v) q[k] = v;
  if (cursor) q.cursor = cursor;
  return q;
};
export const listTasks = (scope: TaskScope = "active", filter: TaskFilter = {}, cursor?: string) =>
  get<TaskPage>("/tasks", pageQuery(scope, filter, cursor));
export const getTaskSummary = (filter: TaskFilter = {}) => {
  const q: Record<string, string> = {};
  for (const [k, v] of Object.entries(filter)) if (v) q[k] = v;
  return get<TaskSummary>("/task-summary", q);
};
export const taskAction = (action: TaskAction, opts: { ids?: string[]; scope?: TaskScope; filter?: TaskFilter; file?: "keep" | "trash" } = {}) =>
  post<TaskActionResult>("/tasks/action", { action, ids: opts.ids, scope: opts.scope, filter: opts.filter, file: opts.file });
export const ackTask = (id: string) => post<unknown>(`/task/${encodeURIComponent(id)}/ack`);
export const keepTask = (id: string, keep = true) => post<unknown>(`/task/${encodeURIComponent(id)}/keep`, { keep });
export const restoreTask = (id: string) => post<unknown>(`/task/${encodeURIComponent(id)}/restore`);
export const cancelTask = (id: string) => post<Task>(`/task/${encodeURIComponent(id)}/cancel`);
export const deleteTask = (id: string, file: "trash" | "keep" = "keep") =>
  del<unknown>(`/task/${encodeURIComponent(id)}`, { file });
export const pauseTask = (id: string) => post<Task>(`/task/${encodeURIComponent(id)}/pause`);
export const resumeTask = (id: string) => post<Task>(`/task/${encodeURIComponent(id)}/resume`);
export const retryTask = (id: string) => post<Task>(`/task/${encodeURIComponent(id)}/retry`);

export interface Stats { day: string; submitted: number; completed: number; succeeded: number; failed: number; gone: number; avg_duration_ms: number; queue_depth: number; disk_bytes: number; pool_available: number; breakers: number }
export const getStats = () => get<Stats>("/stats");

export interface DayPoint { day: string; submitted: number; succeeded: number; failed: number; bytes: number }
export interface KindPoint { kind: string; active: number; done: number; failed: number }
export interface OutcomePoint { outcome: string; count: number }
export interface Dashboard extends Stats {
  series: DayPoint[];
  kinds: KindPoint[];
  outcomes: OutcomePoint[];
}
export const getDashboard = () => get<Dashboard>("/dashboard");

export interface Plan {
  id: string; kind: "author_new" | "live_auto"; target_key: string;
  interval_sec: number; params: Record<string, unknown>; enabled: boolean;
  next_due_at: number; fail_streak: number; last_task_id: string | null;
  last_error: string | null; created_at: number;
}
export const listPlans = async () => (await get<{ items: Plan[] }>("/plans")).items;
export const addPlan = (kind: Plan["kind"], target: string, interval_sec: number, params: Record<string, unknown> = {}) =>
  post<Plan>("/plans", { kind, target, interval_sec, params });
export const updatePlan = (id: string, patchValue: Partial<Pick<Plan, "interval_sec" | "params" | "enabled">>) =>
  patch<Plan>(`/plan/${encodeURIComponent(id)}`, patchValue);
export const deletePlan = (id: string) => del<unknown>(`/plan/${encodeURIComponent(id)}`);
export const pausePlan = (id: string) => post<Plan>(`/plan/${encodeURIComponent(id)}/pause`);
export const resumePlan = (id: string) => post<Plan>(`/plan/${encodeURIComponent(id)}/resume`);

export const recheck = (limit = 20) => post<{ accepted: number }>("/recheck", { limit });
export const backfill = (target: string, pages: number, params: Record<string, unknown> = {}) =>
  post<SubmitResult>("/backfill", { target, pages, params });

export interface Download {
  id: string; task_id: string | null; target_key: string; asset_key: string;
  asset_kind: "video" | "cover" | "music" | "gallery" | "record";
  url: string; path: string | null; partial_path: string | null;
  bytes: number | null; expected_bytes: number | null; progress: number; speed: number;
  state: "queued" | "running" | "paused" | "done" | "failed" | "gone" | "canceled";
  pinned: boolean; attempts: number; created_at: number; finished_at: number | null;
  error: string | null; title: string | null; author: string | null;
  cover: string | null; record_id: string | null; asset_id: string | null;
  task_deleted_at: number | null;
}
export const listDownloads = async () => (await get<{ items: Download[] }>("/downloads")).items;
export const getDownload = (id: string) => get<Download>(`/download/${encodeURIComponent(id)}`);
export const retryDownload = (id: string) => post<Download>(`/download/${encodeURIComponent(id)}/retry`);
export const pauseDownload = (id: string) => post<Download>(`/download/${encodeURIComponent(id)}/pause`);
export const resumeDownload = (id: string) => post<Download>(`/download/${encodeURIComponent(id)}/resume`);
export const cancelDownload = (id: string) => post<Download>(`/download/${encodeURIComponent(id)}/cancel`);
export const deleteDownload = (id: string, file: "trash" | "keep") =>
  del<unknown>(`/download/${encodeURIComponent(id)}`, { file });
export const pinDownload = (id: string, pinned: boolean) => post(`/download/${encodeURIComponent(id)}/pin`, { pinned });
export interface DownloadMeta {
  target_key?: string; asset_kind?: Download["asset_kind"]; title?: string; author?: string;
  cover?: string; ext?: string; expected_bytes?: number; record_id?: string; parent_id?: string;
  url?: string; pinned?: boolean;
}
export const submitDownload = (target: string, meta: DownloadMeta = {}) =>
  post<Download>("/download", { target, ...meta });

/** 视频默认附带封面：既写入下载行供列表显示，也单独下载封面文件。 */
export async function submitVideoWithCover(target: string, meta: DownloadMeta & { cover?: string }): Promise<Download> {
  const out = await submitDownload(target, { ...meta, asset_kind: "video" });
  if (meta.cover) {
    await submitDownload(target, {
      asset_kind: "cover", target_key: `${target}:cover`, url: meta.cover,
      title: meta.title, author: meta.author, cover: meta.cover, ext: "jpg", pinned: meta.pinned,
    }).catch(() => undefined);
  }
  return out;
}

export interface RecordComment {
  cid: string; parent_cid: string | null; level: number; author_key: string | null;
  author_name: string | null; content: string; likes: number | null;
  ip_label: string | null; created_at: number | null; raw: Record<string, unknown>;
}
export interface CaptureRecord {
  id: string; target_key: string; task_id: string | null; title: string | null;
  author_key: string | null; author_name: string | null; cover: string | null;
  thumb_path: string | null; metrics: Record<string, unknown>;
  metrics_history: Array<{ at: number; metrics: Record<string, unknown> }>;
  state: string; created_at: number; last_checked: number | null;
  comment_count: number; asset_count: number; comments_requested: boolean;
  comment_target: number; comment_outcome: string | null; output_path: string | null;
}
export interface RecordQuery { state?: string; kind?: string; q?: string; from?: number; to?: number; cursor?: string; limit?: number; [key: string]: string | number | undefined }
export interface RecordSummary { count: number; live: number; deleted: number }
export const listRecords = (q: RecordQuery = {}) =>
  get<{ items: CaptureRecord[]; meta: { next: string | null } }>("/records", q);
export const getRecordSummary = (q: RecordQuery = {}) => get<RecordSummary>("/record-summary", q);
export const recordAction = (action: "delete", opts: { ids?: string[]; filter?: RecordQuery } = {}) =>
  post<{ count: number }>("/records/action", { action, ...opts });
export const getRecord = (id: string) => get<CaptureRecord>(`/record/${encodeURIComponent(id)}`);
export const listRecordComments = (id: string, cursor = "", q = "", limit = 100) =>
  get<{ items: RecordComment[]; meta: { next: string | null; total: number } }>(
    `/record/${encodeURIComponent(id)}/comments`, { cursor, q, limit });
export const recheckRecord = (id: string) => post<SubmitResult>(`/record/${encodeURIComponent(id)}/recheck`);
export const deleteRecord = (id: string) => del<unknown>(`/record/${encodeURIComponent(id)}`);

export interface Asset {
  id: string; target_key: string; kind: string; title: string | null; author: string | null;
  cover: string | null; path: string; dir: string; size: number | null; missing: boolean;
  task_id: string | null; record_id: string | null; created_at: number;
}
export interface AssetQuery { kind?: string; title?: string; author?: string; from?: number; to?: number; source?: string; record?: string; dir?: string; cursor?: string; limit?: number; direct?: string; hide_cover?: string; [key: string]: string | number | undefined }
export const listAssets = async (q: AssetQuery = {}) =>
  get<{ items: Asset[]; meta: { next: string | null } }>("/assets", q);
export const getAssetSummary = (q: AssetQuery = {}) => get<{ count: number }>("/asset-summary", q);
export const assetAction = (action: "delete", opts: { ids?: string[]; filter?: AssetQuery; file?: "keep" | "trash" } = {}) =>
  post<{ count: number }>("/assets/action", { action, ids: opts.ids, filter: opts.filter, file: opts.file });
export const getAsset = (id: string) => get<Asset>(`/asset/${encodeURIComponent(id)}`);
export const getAssetImages = async (id: string) =>
  (await get<{ items: string[] }>(`/asset/${encodeURIComponent(id)}/images`)).items;
export const deleteAsset = (id: string, file: "trash" | "keep") =>
  del<unknown>(`/asset/${encodeURIComponent(id)}`, { file });
export interface AssetDir { path: string; name: string; count: number; bytes: number }
export const listAssetDirs = (dir = "") =>
  get<{ root: string; items: AssetDir[] }>("/assets/dirs", dir ? { dir } : {});

export type LibraryKind = "video" | "image" | "comment" | "other";
export interface LibraryItem {
  id: string; source: "asset" | "record"; item_type: LibraryKind; asset_kind: string;
  title: string | null; author: string | null; target_key: string;
  cover: string | null; thumb_path: string | null; path: string | null;
  size: number | null; created_at: number; missing: boolean;
  record_state: string | null; metrics: Record<string, unknown>;
  comments_requested: boolean; comment_count: number; comment_target: number;
  comment_outcome: string | null; asset_count: number;
}
export interface LibraryQuery { kind?: string; q?: string; from?: number; to?: number; cursor?: string; limit?: number; [key: string]: string | number | undefined }
export interface LibrarySummary { total: number; video: number; image: number; comment: number; other: number }
export const listLibrary = (q: LibraryQuery = {}) =>
  get<{ items: LibraryItem[]; meta: { next: string | null } }>("/library", q);
export const getLibrarySummary = (q: LibraryQuery = {}) => get<LibrarySummary>("/library-summary", q);
export const libraryAction = (action: "delete", opts: { ids?: string[]; filter?: LibraryQuery; file?: "keep" | "trash" } = {}) =>
  post<{ count: number }>("/library/action", { action, ...opts });

export interface Recording { id: string; plan_id: string | null; target_key: string; room_id: string | null; dir: string; state: string; bytes: number; started_at: number; finished_at: number | null; error: string | null }
export const listRecordings = async () => (await get<{ items: Recording[] }>("/recordings")).items;
export const stopRecording = (id: string) => post(`/recording/${encodeURIComponent(id)}/stop`);
export type { ApiError as _ApiError } from "./api";
