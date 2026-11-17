/** 品牌母标：胶片回环 + 播放窗。随深海群青应用图标同步。 */
export const brandMark = (cls = "mark"): string => `<svg class="${cls}" viewBox="0 0 28 28" aria-hidden="true" focusable="false">
  <defs>
    <linearGradient id="brandBg" x1="2" y1="2" x2="26" y2="26" gradientUnits="userSpaceOnUse"><stop stop-color="#0D173A"/><stop offset=".72" stop-color="#1A2E7A"/><stop offset="1" stop-color="#3D66D9"/></linearGradient>
    <linearGradient id="brandInk" x1="5" y1="4" x2="23" y2="23" gradientUnits="userSpaceOnUse"><stop stop-color="#EAF0FF"/><stop offset="1" stop-color="#3D66D9"/></linearGradient>
  </defs>
  <rect x="1" y="1" width="26" height="26" rx="8" fill="url(#brandBg)"/>
  <path d="M-2 21 C4 24 9 8 16 9 C21 10 21 16 30 12" fill="none" stroke="url(#brandInk)" stroke-width="5.2" stroke-linecap="round"/>
  <g fill="#0A1231">
    <rect x="5.8" y="16.5" width="1.8" height="1.2" rx=".45" transform="rotate(-24 6.7 17.1)"/>
    <rect x="11.5" y="7.3" width="1.8" height="1.2" rx=".45" transform="rotate(42 12.4 7.9)"/>
    <rect x="18.3" y="7.9" width="1.8" height="1.2" rx=".45" transform="rotate(42 19.2 8.5)"/>
  </g>
  <circle cx="15.2" cy="14.8" r="6" fill="#0A1231" stroke="#EAF0FF" stroke-opacity=".5" stroke-width=".7"/>
  <path d="M13.1 11.7 C13.9 11.2 14.8 11.4 15.5 11.9 L18 13.7 C18.7 14.2 18.7 15 18.1 15.6 L15.6 17.5 C15 17.9 14.2 17.8 13.6 17.2 C13 16.7 12.8 16.1 12.8 15.5 L12.8 13.4 C12.8 12.9 12.9 12.2 13.1 11.7 Z" fill="#EAF0FF"/>
</svg>`;
