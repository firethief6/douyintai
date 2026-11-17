const ASSET_ICONS: Record<string, string> = {
  video: "i-video", cover: "i-image", image: "i-image", gallery: "i-images",
  music: "i-music", record: "i-live", live: "i-live", json: "i-json", gif: "i-gif",
};

const TASK_ICONS: Record<string, string> = {
  collection: "i-json", recheck: "i-json", backfill: "i-history", record: "i-live", download: "i-download",
};

export function contentIcon(kind: string, assetKind = ""): string {
  return ASSET_ICONS[assetKind] || ASSET_ICONS[kind] || TASK_ICONS[kind] || "i-database";
}

export function contentKindIcon(assetKind: string): string {
  return ASSET_ICONS[assetKind] || "i-database";
}
