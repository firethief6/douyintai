export type Platform = "macos" | "windows" | "linux";

export function detectPlatform(): Platform {
  const ua = navigator.userAgent.toLowerCase();
  if (ua.includes("macintosh") || ua.includes("mac os")) return "macos";
  if (ua.includes("windows")) return "windows";
  return "linux";
}

export function applyPlatform(): Platform {
  const platform = detectPlatform();
  document.documentElement.dataset.platform = platform;
  return platform;
}

export const isMac = (): boolean => document.documentElement.dataset.platform === "macos";
export const isWindows = (): boolean => document.documentElement.dataset.platform === "windows";
