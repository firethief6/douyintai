import { $, $$, ago, esc, toast } from "../lib/dom";
import { caps, health, identityClear, identityPause, identityPrepare, identitySetTarget, identityStatus, probe, setSignBackend, signStatus, version, type IdentityPool } from "../lib/api";
import { applyTheme, isTauri, saveSettings, settings, type LogLevel } from "../lib/store";
import { exportDiagnostics, log, logPath, revealPath, setLogLevel } from "../lib/log";
import { replace, type View } from "../router";

const TABS = [["conn", "服务"], ["identity", "身份"], ["dl", "下载"], ["look", "外观"], ["caps", "能力"], ["diag", "诊断"], ["about", "关于"]] as const;

const view: View = {
  title: "设置",
  render(ctx) {
    const tab = ctx.query.get("tab") ?? "conn";
    return `
    <section class="view active">
      <div class="pagehead"><h1 class="h1">设置</h1><div class="sub">改动即时保存到本机</div></div>
      <div class="settings">
        <nav class="snav">${TABS.map(([k, n]) => `<button class="${k === tab ? "active" : ""}" data-tab="${k}">${n}</button>`).join("")}</nav>
        <div class="sbody" id="sbody"></div>
      </div>
    </section>`;
  },
  mount(ctx) {
    const body = $("#sbody", ctx.content)!;
    const checkService = async () => {
      const out = $("#testout", body);
      const dot = $("#testdot", body);
      if (!out || !dot) return;
      out.textContent = "正在检查服务…";
      dot.className = "dot amber";
      try {
        const v = await version();
        await health();
        out.textContent = `服务正常 · v${String(v.lib ?? __APP_VERSION__)}`;
        dot.className = "dot";
      } catch (err) {
        out.textContent = `服务不可用 · ${String((err as Error).message)}`;
        dot.className = "dot red";
        log("err", "服务检查失败：" + String((err as Error).message));
      }
    };
    const paintSign = async () => {
      const out = $("#signstate", body);
      if (!out || !isTauri()) return;
      try {
        const r = await signStatus();
        const st = r.sign;
        $$("[data-sign]", body).forEach((b) => b.classList.toggle("primary", (b as HTMLElement).dataset.sign === st.backend));
        const used = st.backend === "native" ? st.native : st.emu;
        const other = st.backend === "native" ? st.emu : st.native;
        out.textContent = `当前${st.backend === "native" ? "原生" : "兼容"} · 成功 ${used.ok} / 失败 ${used.err}`
          + (other.ok + other.err ? ` · 另一实现 成功 ${other.ok} / 失败 ${other.err}` : "")
          + (st.env ? ` · 环境变量 DYSIGN_IMPL 固定为 ${st.env}，重启后仍按它` : "");
        if (st.last) {
          const line = document.createElement("div");
          line.className = "dim";
          line.textContent = `最近失败 ${ago(st.last.at)}（${st.last.backend === "native" ? "原生" : "兼容"}）：${st.last.message}`;
          out.append(line);
        }
      } catch (e) {
        out.textContent = `读取签名状态失败：${String((e as Error).message)}`;
      }
    };
    let poolTimer: number | undefined;
    const drawPool = (p: IdentityPool) => {
      const box = $("#poolstate", body);
      if (!box) return;
      box.innerHTML = `<div class="health">
        <div><span>目标</span><b>${p.target}</b></div><div><span>已有</span><b>${p.total}</b></div>
        <div><span>可用</span><b>${p.available}</b></div><div><span>冷却</span><b>${p.cooling}</b></div>
        <div><span>退役</span><b>${p.retired}</b></div><div><span>失效</span><b>${p.dead}</b></div>
        <div><span>正在创建</span><b>${p.creating}</b></div></div>
        ${p.last_error ? `<div class="notice warn">${esc(p.last_error)}</div>` : ""}`;
      $$("[data-act=idpause]", body).forEach((b) => {
        b.textContent = p.paused ? "继续准备" : "暂停准备";
      });
    };
    const refreshPool = async () => {
      if (!isTauri()) return;
      try { drawPool((await identityStatus()).pool); }
      catch (e) { const box = $("#poolstate", body); if (box) box.textContent = `读取失败：${String((e as Error).message)}`; }
    };
    const paint = async (tab: string) => {
      const s = settings();
      if (poolTimer) { clearInterval(poolTimer); poolTimer = undefined; }
      $$("[data-tab]", ctx.content).forEach((b) => b.classList.toggle("active", (b as HTMLElement).dataset.tab === tab));
      if (tab === "conn") {
        const local = isTauri();
        body.innerHTML = `
          <h2 class="h2">服务</h2>
          <p class="setnote">${local ? "抖音台直接在这台电脑上运行，不需要填写服务器地址或令牌。" : "浏览器预览需要连接本机的抖音台服务；桌面版会自动配置好。"}</p>
          <div class="servicecard">
            <span class="dot amber" id="testdot"></span>
            <div><b id="testout">${local ? "正在检查服务…" : "尚未检查服务"}</b><div class="dim">${local ? "浏览、采集和下载的数据都只保存在这台电脑上。" : "填写地址并检查连接后即可使用。"}</div></div>
            <button class="btn sm ghost" data-act="test">检查服务</button>
          </div>
          ${local ? "" : `<div class="fieldrow"><label>地址</label><input class="input mono" id="fUrl" value="${esc(s.baseUrl)}"></div>
          <div class="fieldrow"><label>令牌</label><input class="input mono" id="fToken" type="password" value="${esc(s.token)}" placeholder="有访问令牌时填写"></div>`}
          ${local ? `<div class="fieldrow" style="margin-top:18px"><label>签名</label><div class="row">
            <button class="btn sm" data-sign="native">原生（快）</button>
            <button class="btn sm" data-sign="emu">兼容（模拟器）</button>
          </div></div>
          <div class="setnote">默认使用原生。签名报错时在这里切到兼容，切换只影响之后的请求，不会自动回退。</div>
          <div class="setnote" id="signstate"></div>` : ""}`;
        if (local) void checkService();
        if (local) void paintSign();
      } else if (tab === "identity") {
        body.innerHTML = `
          <h2 class="h2">身份</h2>
          <p class="setnote">身份全部在本机网络里生成，谁使用谁生成。新号会被服务端按新设备风控，一般 1–2 天后恢复；期间直播和评论可能提示稍后重试。身份只保存在本机。</p>
          <div class="fieldrow"><label>目标数量</label><input class="input mono" id="fPoolSize" style="width:110px" type="number" min="1" max="128" step="1" value="${s.identityPoolSize}">
            <button class="btn sm" data-act="idsize">应用</button></div>
          <div id="poolstate" class="dim">${isTauri() ? "正在读取…" : "浏览器预览不管理身份，请使用桌面版。"}</div>
          <div class="row" style="margin-top:16px">
            <button class="btn sm primary" data-act="idprepare">立即准备</button>
            <button class="btn sm" data-act="idpause">暂停准备</button>
            <button class="btn sm" data-act="idclear">重建身份池</button>
          </div>
          <div class="setnote">目标数量是可用身份的维持水位。失效、退役或打满使用次数后会自动补回。重建会清空本机名单并从头生成，旧身份立即停用。</div>`;
        if (isTauri()) {
          void refreshPool();
          poolTimer = window.setInterval(refreshPool, 2000);
        }
      } else if (tab === "dl") {
        body.innerHTML = `
          <h2 class="h2">下载</h2><div class="sub">桌面端直接落盘；浏览器预览只打开直链。</div>
          <div class="fieldrow"><label>目录</label><input class="input mono" id="fDir" value="${esc(s.downloadDir)}"><button class="btn sm" data-act="pick">选择</button></div>
          <div class="fieldrow"><label>命名</label><input class="input mono" id="fName" value="${esc(s.naming)}"></div>
          <div class="fieldrow"><label>同时下载</label><input class="input mono" id="fConc" style="width:110px" type="number" min="1" max="8" step="1" value="${s.concurrency}"></div>
          <div class="fieldrow"><label>已下载跳过</label><label class="row" style="gap:6px"><input type="checkbox" id="fSkip" ${s.skipExisting ? "checked" : ""}> 同一作品已有文件时不再重下</label></div>
          <div class="hint" style="font-size:11px;color:var(--text-3)">可用：{author} {desc} {id} {date} {ext}；并发范围 1–8；同名不同作品自动追加 (2)</div>`;
      } else if (tab === "look") {
        body.innerHTML = `
          <h2 class="h2">外观</h2>
          <div class="fieldrow"><label>主题</label><div class="row"><button class="btn sm ${s.theme === "dark" ? "primary" : ""}" data-theme="dark">深色</button><button class="btn sm ${s.theme === "light" ? "primary" : ""}" data-theme="light">浅色</button></div></div>
          <div class="fieldrow"><label>弹幕</label><div class="row"><button class="btn sm ${s.danmaku ? "primary" : ""}" data-dmk="on">开</button><button class="btn sm ${s.danmaku ? "" : "primary"}" data-dmk="off">关</button></div></div>
          <div class="fieldrow"><label>透明度</label><input class="input mono" id="fOp" style="width:110px" type="number" min="0.2" max="1" step="0.1" value="${s.danmakuOpacity}"></div>
          <div class="fieldrow"><label>字号</label><input class="input mono" id="fSize" style="width:110px" type="number" min="12" max="28" step="1" value="${s.danmakuSize}"></div>`;
      } else if (tab === "caps") {
        body.innerHTML = `<h2 class="h2">能力</h2><div class="sub">当前可用的数据与操作；状态为“可用”表示可以正常使用。</div><div id="capsList" class="capwrap"><div class="capload">读取中…</div></div>`;
        try {
          const list = await caps();
          const rows = list.map((c) => `<div class="caprow"><code class="capname" title="${esc(c.name)}">${esc(c.name)}</code><span class="caphelp" title="${esc(c.help || c.route)}">${esc(c.help || c.route)}</span><span class="badge src">可用</span></div>`).join("");
          const listEl = $("#capsList", body)!;
          listEl.innerHTML = rows
            ? `<div class="caphead"><span>能力</span><span>说明</span><span>状态</span></div>${rows}`
            : `<div class="capload">没有可用能力</div>`;
          listEl.addEventListener("click", (e) => e.stopPropagation());
        } catch (e) { $("#capsList", body)!.textContent = String((e as Error).message); }
      } else if (tab === "diag") {
        body.innerHTML = `
          <h2 class="h2">日志与诊断</h2>
          <p class="setnote">遇到问题时先选“debug”，重新操作一次并导出诊断包，把 zip 附到 Issue。日志记录请求路径、耗时、网络与签名结果；令牌和代理账号会自动抹掉。排查结束后切回 info。</p>
          <div class="fieldrow"><label>日志级别</label><div class="row">${([["debug", "Debug"], ["info", "Info"], ["warn", "Warn"], ["error", "Error"]] as const).map(([v, n]) => `<button class="btn sm ${s.logLevel === v ? "primary" : ""}" data-log="${v}" aria-pressed="${s.logLevel === v}">${n}</button>`).join("")}</div></div>
          <div class="setnote">Debug：请求明细与网络结果 · Info：访问记录与生命周期 · Warn：异常但可继续 · Error：仅失败。</div>
          <div class="row" style="margin:14px 0"><button class="btn sm primary" data-act="export">导出诊断包</button><button class="btn sm ghost" data-act="issue">新建 Issue</button><button class="btn sm ghost" data-act="openlog">打开日志文件</button></div>
          <div class="setnote" id="exportstate" role="status"></div>
          <div class="row" style="margin:18px 0 10px"><button class="btn sm" data-act="health">/health</button><button class="btn sm ghost" data-act="probe">/probe</button><button class="btn sm ghost" data-act="ver">版本</button></div>
          <pre class="diagout" id="diagout">点上面的按钮。</pre>`;
      } else {
        body.innerHTML = `<h2 class="h2">关于</h2>
          <div class="sub">抖音台 v${__APP_VERSION__}</div>
          <p class="aboutcopy">一款用于浏览、收藏和采集抖音内容的桌面工具。可以查看推荐、热榜、直播和短剧，搜索视频与作者，把作品、图集和评论保存到本机，并在资料库中集中管理与播放。</p>
          <p class="aboutcopy">适合日常浏览、素材收集和离线查看。浏览过程只读，不会自动发布、评论或点赞。</p>`;
      }
      for (const row of $$<HTMLElement>(".fieldrow", body)) {
        const label = row.querySelector<HTMLLabelElement>("label");
        const control = row.querySelector<HTMLElement>("input,select,textarea");
        if (label && control?.id && !label.contains(control)) label.htmlFor = control.id;
      }
    };

    ctx.content.addEventListener("click", async (e) => {
      const t = (e.target as HTMLElement).closest<HTMLElement>("[data-tab],[data-act],[data-theme],[data-dmk],[data-sign],[data-log]");
      // 主题值也写在根节点 HTML 上；普通字段点击不能被当成主题按钮。
      if (!t || !ctx.content.contains(t)) return;
      if (t.dataset.tab) {
        replace(`/settings?tab=${t.dataset.tab}`);
        paint(t.dataset.tab); return;
      }
      if (t.dataset.log) {
        const lv = t.dataset.log as LogLevel;
        saveSettings({ logLevel: lv });
        try { await setLogLevel(lv); toast(lv === "debug" ? "已开启 debug，复现问题后导出诊断包" : "日志级别已保存"); }
        catch (err) { toast(`设置失败：${String((err as Error).message)}`); }
        paint("diag"); return;
      }
      if (t.dataset.dmk) { saveSettings({ danmaku: t.dataset.dmk === "on" }); paint("look"); return; }
      if (t.dataset.theme) {
        saveSettings({ theme: t.dataset.theme as "dark" | "light" });
        applyTheme();
        paint("look"); return;
      }
      if (t.dataset.sign) {
        const mode = t.dataset.sign as "native" | "emu";
        try {
          await setSignBackend(mode);
          saveSettings({ signBackend: mode });
          toast("已切换签名实现");
          void paintSign();
        } catch (e) {
          toast(`切换失败：${String((e as Error).message)}`);
        }
        return;
      }
      if (t.dataset.act === "idsize") {
        const n = Number(($("#fPoolSize") as HTMLInputElement).value);
        if (!Number.isFinite(n) || n < 1 || n > 128) { toast("目标数量需在 1–128"); return; }
        saveSettings({ identityPoolSize: Math.round(n) });
        try { drawPool((await identitySetTarget(Math.round(n))).pool); toast("已更新目标数量"); }
        catch (e) { toast(`更新失败：${String((e as Error).message)}`); }
        return;
      }
      if (t.dataset.act === "idprepare") {
        try { drawPool((await identityPrepare()).pool); } catch (e) { toast(`操作失败：${String((e as Error).message)}`); }
        return;
      }
      if (t.dataset.act === "idpause") {
        const paused = t.textContent?.trim() === "暂停准备";
        try { drawPool((await identityPause(paused)).pool); } catch (e) { toast(`操作失败：${String((e as Error).message)}`); }
        return;
      }
      if (t.dataset.act === "idclear") {
        if (!confirm("删除本机全部身份并重新准备？旧身份将立即停用。")) return;
        try { drawPool((await identityClear()).pool); void identityPrepare(); toast("已清空，开始重新生成"); }
        catch (e) { toast(`重建失败：${String((e as Error).message)}`); }
        return;
      }
      if (t.dataset.act === "test") {
        if (!isTauri()) saveSettings({ baseUrl: ($("#fUrl") as HTMLInputElement).value.trim(), token: ($("#fToken") as HTMLInputElement).value.trim() });
        await checkService();
        return;
      }
      if (t.dataset.act === "dmkopts") { paint("look"); return; }
      if (t.dataset.act === "export") {
        const out = $("#exportstate", body);
        if (out) out.textContent = "正在整理日志…";
        try {
          const p = await exportDiagnostics();
          if (out) out.innerHTML = `已导出：<span class="mono">${esc(p)}</span> <button class="btn sm ghost" data-act="reveal" data-path="${esc(p)}">打开所在目录</button>`;
          toast("诊断包已导出");
        } catch (err) {
          if (out) out.textContent = `导出失败：${String((err as Error).message)}`;
        }
        return;
      }
      if (t.dataset.act === "issue") {
        const { invoke } = await import("@tauri-apps/api/core");
        await invoke("open_external", { url: "https://github.com/firethief6/douyintai/issues/new" }).catch(() => {});
        return;
      }
      if (t.dataset.act === "openlog") {
        const p = await logPath();
        if (p) await revealPath(p);
        return;
      }
      if (t.dataset.act === "reveal") {
        if (t.dataset.path) await revealPath(t.dataset.path);
        return;
      }
      if (t.dataset.act === "pick") {
        const { open } = await import("@tauri-apps/plugin-dialog");
        const dir = await open({ directory: true });
        if (typeof dir === "string") { ($("#fDir") as HTMLInputElement).value = dir; saveSettings({ downloadDir: dir }); }
        return;
      }
      if (t.dataset.act === "health" || t.dataset.act === "ver") {
        try { const v = await (t.dataset.act === "health" ? health() : version()); $("#diagout", body)!.textContent = JSON.stringify(v, null, 2); }
        catch (err) { $("#diagout", body)!.textContent = String((err as Error).message); }
        return;
      }
      if (t.dataset.act === "probe") {
        try { $("#diagout", body)!.textContent = JSON.stringify(await probe(), null, 2); }
        catch (err) { $("#diagout", body)!.textContent = String((err as Error).message); }
      }
    });

    body.addEventListener("change", (e) => {
      const el = e.target as HTMLInputElement;
      if (el.id === "fUrl") saveSettings({ baseUrl: el.value.trim() });
      if (el.id === "fToken") saveSettings({ token: el.value.trim() });
      if (el.id === "fDir") saveSettings({ downloadDir: el.value });
      if (el.id === "fName") saveSettings({ naming: el.value });
      if (el.id === "fConc") saveSettings({ concurrency: Math.min(8, Math.max(1, Number(el.value) || 3)) });
      if (el.id === "fSkip") saveSettings({ skipExisting: el.checked });
      if (el.id === "fOp") saveSettings({ danmakuOpacity: Math.min(1, Math.max(0.2, Number(el.value) || 0.9)) });
      if (el.id === "fSize") saveSettings({ danmakuSize: Math.min(28, Math.max(12, Number(el.value) || 16)) });
    });
    const tab = ctx.query.get("tab") ?? "conn";
    paint(TABS.some(([k]) => k === tab) ? tab : "conn");
    if (tab === "dl") setTimeout(() => $("#fDir", body)?.addEventListener("change", () => toast("已保存")), 0);
  },
};
export default view;
