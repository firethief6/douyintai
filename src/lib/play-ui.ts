import { PLAYER_QUALITY } from "./copy";

export interface PlayUi {
  veil(text: string): void;
  veilOff(): void;
  locked(): boolean;
  reset(): void;
  playing(on: boolean): void;
  progress(at: number, total: number): void;
  qualityLabel(text: string): void;
  qualityOptions(html: string): void;
}

/** 播放器可见状态只有一个写入口，切集与加载逻辑不再直接碰 DOM。 */
export function createPlayUi(root: HTMLElement, player: HTMLElement | null, track: HTMLElement | null): PlayUi {
  const veil = root.querySelector<HTMLElement>("#veil");
  const time = root.querySelector<HTMLElement>("#ptime");
  const label = root.querySelector<HTMLElement>("#quality-label");
  const options = root.querySelector<HTMLElement>("#quality-options");
  const toggles = [...root.querySelectorAll<HTMLElement>('[data-act="toggle"]')];
  let lock = false;
  return {
    veil(text) { if (veil) { veil.textContent = text; veil.hidden = false; } },
    veilOff() { if (veil) veil.hidden = true; },
    locked: () => lock,
    reset() {
      lock = true;
      if (track) track.style.width = "0%";
      if (time) time.textContent = "0:00";
      if (label) label.textContent = PLAYER_QUALITY.loading;
      if (options) options.innerHTML = "";
      player?.classList.remove("playing");
      for (const b of toggles) {
        b.innerHTML = `<svg class="i"><use href="#i-play"/></svg>`;
        b.setAttribute("aria-label", "播放");
        b.classList.remove("pausing");
      }
      this.veil(PLAYER_QUALITY.loadingVideo);
    },
    playing(on) {
      if (lock) return;
      player?.classList.toggle("playing", on);
      for (const b of toggles) {
        b.innerHTML = `<svg class="i"><use href="#i-${on ? "pause" : "play"}"/></svg>`;
        b.setAttribute("aria-label", on ? "暂停" : "播放");
        b.classList.toggle("pausing", on);
      }
    },
    progress(at, total) {
      if (lock) return;
      if (track && total) track.style.width = `${(at / total) * 100}%`;
      if (time) time.textContent = `${Math.floor(at / 60)}:${String(Math.floor(at % 60)).padStart(2, "0")}`;
    },
    qualityLabel(text) { if (label) label.textContent = text; },
    qualityOptions(html) { if (options) options.innerHTML = html; },
  };
}
