import { $, esc } from "../lib/dom";
import { health, version } from "../lib/api";
import { saveSettings, settings } from "../lib/store";
import { brandMark } from "../brand";
import type { View } from "../router";

const view: View = {
  title: "连接",
  render: () => {
    const s = settings();
    return `
    <div class="connect">
      <div class="left">
        <div class="bigmark">${brandMark("mark big")}
          <div><b style="font-size:18px">抖音台</b><div class="dim" style="font-size:11px;letter-spacing:.14em">DOUYIN DESK</div></div></div>
        <div><h1>自己的<br>抖音工作台。<em>不追。</em></h1>
          <p class="lead">推荐货架、搜索、用户、短剧、直播与下载队列，在一个键盘可用的桌面窗口里。数据在应用进程内直接取，匿名即可。</p>
          <div class="points">
            <div><svg class="i"><use href="#i-check"/></svg>桌面端进程内直取；浏览器联调才连本机 <span class="mono">8765</span></div>
            <div><svg class="i"><use href="#i-check"/></svg>不登录也能浏览、下载、收藏</div>
            <div><svg class="i"><use href="#i-check"/></svg>接口、签名与身份池都在进程内</div>
          </div></div>
        <div class="dim" style="font-size:11px">v${__APP_VERSION__} · 未连接后端</div>
      </div>
      <div class="right"><div class="card2">
        <h2>连接后端</h2>
        <div class="hint">桌面端后端随应用启动；浏览器开发才需外部 <span class="mono">douyin serve</span>。</div>
        <div class="lab"><span>地址</span><span class="dim">桌面端可留空；浏览器开发用 8765</span></div>
        <input class="input mono" id="cUrl" value="${esc(s.baseUrl)}">
        <div class="lab"><span>访问令牌</span><span class="dim">serve start --token</span></div>
        <input class="input mono" id="cToken" type="password" value="${esc(s.token)}">
        <div class="row" style="margin-top:16px"><button class="btn primary" style="flex:1" data-act="go">测试并进入</button></div>
        <div id="cout" class="dim" style="font-size:12px;margin-top:12px"></div>
        <details class="diag"><summary><span>连不上？三步排查</span><svg class="i" style="width:14px;height:14px"><use href="#i-chevdown"/></svg></summary>
          <div class="dbody">
            <p><b>1</b><span>确认后端在跑：<span class="mono">douyin serve status</span></span></p>
            <p><b>2</b><span>端口不同就一起改，别只改一边</span></p>
            <p><b>3</b><span>令牌错会回 401；重新生成后整段粘贴</span></p>
          </div></details>
        <div class="footlink">令牌只存在本机</div>
      </div></div>
    </div>`;
  },
  mount() {
    const go = async () => {
      saveSettings({
        baseUrl: ($("#cUrl") as HTMLInputElement).value.trim(),
        token: ($("#cToken") as HTMLInputElement).value.trim(),
      });
      const out = $("#cout")!;
      out.textContent = "测试中…";
      try {
        await health();
        const v = await version();
        saveSettings({ firstRun: false });
        out.textContent = `连接正常 · ${String(v.lib ?? "")} · 签名 ${String(v.sign || "—")}`;
        out.style.color = "var(--green)";
        setTimeout(() => (location.hash = "#/feed"), 300);
      } catch (e) {
        out.textContent = "失败：" + String((e as Error).message);
        out.style.color = "var(--red)";
      }
    };
    $('[data-act="go"]')!.addEventListener("click", go);
    ($("#cToken") as HTMLInputElement).addEventListener("keydown", (e) => { if (e.key === "Enter") go(); });
  },
};
export default view;
