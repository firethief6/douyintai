import { $, $$, toast } from "../lib/dom";
import { proxyGet, proxySet, proxyTest } from "../lib/api";
import { isTauri, saveSettings } from "../lib/store";
import type { View } from "../router";

const view: View = {
  title: "代理",
  render: () => `
    <section class="view active">
      <div class="pagehead">
        <div><h1 class="h1">出口代理</h1><div class="sub">接口被当前网络限流时，把请求分散到多个出口</div></div>
      </div>
      <div class="settings" style="grid-template-columns:1fr">
        <div class="sbody">
          <div class="fieldrow"><label>状态</label><div class="row">
            <button class="btn sm" data-px="on">启用</button>
            <button class="btn sm" data-px="off">停用</button>
            <span class="dim" id="pxstate">${isTauri() ? "读取中…" : "浏览器预览不生效"}</span>
          </div></div>
          <div class="fieldrow" style="align-items:flex-start"><label>列表</label>
            <textarea class="input mono" id="fProxy" rows="12" style="flex:1;min-height:220px" placeholder="127.0.0.1:7890&#10;http://10.0.0.2:3128&#10;socks5://10.0.0.3:1080"></textarea>
          </div>
          <div class="fieldrow"><label>CDN</label><label class="row" style="gap:6px"><input type="checkbox" id="fProxyCdn"> 视频与图集也走代理</label></div>
          <div class="row" style="margin-top:12px">
            <button class="btn sm primary" data-act="save">保存并应用</button>
            <button class="btn sm" data-act="test">实测全部</button>
          </div>
          <div id="pxtest" class="dim" style="margin-top:12px;white-space:pre-line"></div>
          <div class="setnote">一行一个，支持 host:port 与 http://、socks5:// 前缀；留空或停用则直连。失败出口自动冷却 1–10 分钟，恢复后重新参与轮换。</div>
          <div class="setnote">CDN 不校验身份，默认直连更快；仅在视频、图片也拉不动时打开。</div>
        </div>
      </div>
    </section>`,
  mount(ctx) {
    let on = false;
    const ta = () => $("#fProxy", ctx.content) as HTMLTextAreaElement;
    const cdn = () => $("#fProxyCdn", ctx.content) as HTMLInputElement;
    const count = () => ta().value.split("\n").filter((x) => x.trim()).length;
    const state = (next: boolean) => {
      on = next;
      const el = $("#pxstate", ctx.content);
      if (el) el.textContent = on ? `已启用 · ${count()} 个出口` : `未启用${count() ? ` · 已存 ${count()} 个` : ""}`;
      $$("[data-px]", ctx.content).forEach((b) => {
        b.classList.toggle("primary", (b as HTMLElement).dataset.px === "on" === on);
      });
    };
    if (isTauri()) {
      void proxyGet().then((r) => {
        ta().value = r.list;
        cdn().checked = r.cdn;
        state(r.enabled);
      }).catch((e) => { const el = $("#pxstate", ctx.content); if (el) el.textContent = `读取失败：${String((e as Error).message)}`; });
    }
    const apply = async (next: boolean, say: (r: { count: number; enabled: boolean }) => string) => {
      saveSettings({ proxyEnabled: next, proxyList: ta().value, proxyCdn: cdn().checked });
      try {
        const r = await proxySet(ta().value, next, cdn().checked);
        state(r.enabled);
        toast(say(r));
      } catch (err) { toast(`保存失败：${String((err as Error).message)}`); }
    };
    ctx.content.addEventListener("click", async (e) => {
      const t = (e.target as HTMLElement).closest<HTMLElement>("[data-px],[data-act]");
      if (!t) return;
      if (t.dataset.px) {
        const on = t.dataset.px === "on";
        await apply(on, (r) => (on ? `已启用 ${r.count} 个出口` : "已停用代理"));
        return;
      }
      if (t.dataset.act === "save") {
        await apply(on, (r) => (r.count ? `已应用 ${r.count} 个出口` : "列表为空，将直连"));
        return;
      }
      if (t.dataset.act === "test") {
        const out = $("#pxtest", ctx.content);
        if (!out) return;
        out.textContent = "正在实测，最多十几秒…";
        try {
          const r = await proxyTest(ta().value);
          out.textContent = r.rows.length
            ? `可用 ${r.usable} / ${r.total}\n` + r.rows.map((x) => `${x.ok ? "通" : "不通"} ${x.url}${x.ok ? ` ${x.ms}ms` : ""}`).join("\n")
            : "列表为空";
        } catch (err) { out.textContent = `实测失败：${String((err as Error).message)}`; }
      }
    });
  },
};

export default view;
