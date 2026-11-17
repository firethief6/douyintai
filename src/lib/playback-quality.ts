export interface PlaybackGear {
  name: string;
  quality: string;
  width: number;
  height: number;
  bit_rate: number;
  size: number;
  h264: boolean;
}

export type QualityLevel = "ultra" | "high" | "medium" | "low";

export interface QualityChoice { level: QualityLevel; gear: PlaybackGear }

const ORDER: QualityLevel[] = ["ultra", "high", "medium", "low"];

/** 按短边分档：1080 高、720 中、360/540 低、超过 1080 极高。 */
export function levelOf(gear: Pick<PlaybackGear, "width" | "height">): QualityLevel {
  const short = Math.min(gear.width, gear.height);
  if (short > 1080) return "ultra";
  if (short >= 1000) return "high";
  if (short >= 700) return "medium";
  return "low";
}

/** 每档只留最优资源，按极高→低的固定顺序输出。 */
export function qualityChoices(gears: PlaybackGear[]): QualityChoice[] {
  const sorted = [...gears]
    .filter(g => g.name && g.width > 0 && g.height > 0)
    .sort((a, b) => b.width * b.height - a.width * a.height || b.bit_rate - a.bit_rate || Number(b.h264) - Number(a.h264));
  const best = new Map<QualityLevel, PlaybackGear>();
  const seen = new Set<string>();
  for (const g of sorted) {
    const level = levelOf(g);
    const sameSize = `${level}:${Math.min(g.width, g.height)}`;
    if (seen.has(sameSize)) continue;
    seen.add(sameSize);
    if (!best.has(level)) best.set(level, g);
  }
  return ORDER.flatMap(level => {
    const gear = best.get(level);
    return gear ? [{ level, gear }] : [];
  });
}

const capabilities = new Map<string, Promise<boolean>>();

/** 每次应用启动重新检测；扩展的安装/卸载需要重启 WebView。 */
export function supportsGear(gear: PlaybackGear): Promise<boolean> {
  const key = `${gear.h264}:${gear.width}x${gear.height}:${gear.bit_rate}`;
  const old = capabilities.get(key);
  if (old) return old;
  const result = (async () => {
    const v = document.createElement("video");
    const codecs = gear.h264 ? ["avc1.640028"] : ["hvc1.1.6.L153.B0", "hev1.1.6.L153.B0"];
    for (const codec of codecs) {
      const contentType = `video/mp4; codecs="${codec}"`;
      if (!v.canPlayType(contentType)) continue;
      if (!navigator.mediaCapabilities?.decodingInfo) return true;
      try {
        const info = await navigator.mediaCapabilities.decodingInfo({
          type: "file",
          video: { contentType, width: gear.width, height: gear.height, bitrate: Math.max(1, gear.bit_rate), framerate: 30 },
        });
        if (info.supported) return true;
      } catch {
        // 较旧的 WebView 不接受该配置时使用 canPlayType，实际视频帧检查再兜底。
        return true;
      }
    }
    return false;
  })();
  capabilities.set(key, result);
  return result;
}
