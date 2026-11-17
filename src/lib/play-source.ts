import { PLAYER_QUALITY } from "./copy";
import { qualityChoices, type PlaybackGear, type QualityChoice } from "./playback-quality";
import { loadPlay, loadSupportedGears } from "./stage-cache";

/** 播放凭证问题的归类，界面据此选文案，不再各自判断字符串。 */
export type PlayIssue = "v2" | "key";

export const issueOf = (why?: string): PlayIssue | null => {
  if (!why) return null;
  return why.toLowerCase().includes("v2") ? "v2" : "key";
};

export const gearId = (g: Pick<PlaybackGear, "name" | "h264">): string => `${g.name}:${g.h264}`;

export function usableChoices(all: QualityChoice[], rejected: Set<string>): QualityChoice[] {
  return all.filter((c) => !rejected.has(gearId(c.gear)));
}

/** 优先沿用当前档；当前档不可用时退回最优可用档。 */
export function chooseGear(list: QualityChoice[], rejected: Set<string>, preferred?: PlaybackGear): PlaybackGear | undefined {
  if (preferred) {
    const keep = list.find((c) => c.gear.name === preferred.name && c.gear.h264 === preferred.h264 && !rejected.has(gearId(c.gear)));
    if (keep) return keep.gear;
  }
  return usableChoices(list, rejected)[0]?.gear;
}

export interface PlaySource {
  src: string;
  keyErr?: string;
  enc: boolean;
  gear: PlaybackGear;
}

/** 解析档位并取可播放地址；档位、可用性、密钥问题都收在这一层。 */
export async function fetchPlaySource(id: string, rejected: Set<string>, preferred?: PlaybackGear): Promise<PlaySource> {
  const list = qualityChoices(await loadSupportedGears(id));
  const gear = chooseGear(list, rejected, preferred);
  if (!gear) throw new Error(PLAYER_QUALITY.failed);
  const source = await loadPlay(id, gear);
  return { ...source, gear };
}

let prefetching = "";

/** 相邻条目预取：只跑一条，重复请求由缓存合并。 */
export function prefetchPlay(id: string): void {
  if (prefetching === id) return;
  prefetching = id;
  void loadSupportedGears(id)
    .then((list) => {
      const gear = qualityChoices(list)[0]?.gear;
      if (gear) return loadPlay(id, gear);
    })
    .catch(() => {})
    .finally(() => { if (prefetching === id) prefetching = ""; });
}
