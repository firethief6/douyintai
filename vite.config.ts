import { readFileSync } from "node:fs";
import { Readable } from "node:stream";
import { defineConfig } from "vite";

const pkg = JSON.parse(readFileSync(new URL("./package.json", import.meta.url), "utf8"));

/** 浏览器开发期的媒体代理：CDN 会拒掉本地 Referer。生产走 Rust 回环代理。 */
const mediaProxy = {
  name: "media-proxy",
  configureServer(server: any) {
    server.middlewares.use("/media", async (req: any, res: any) => {
      const u = new URL(req.url || "/", "http://x").searchParams.get("u");
      if (!u) { res.statusCode = 400; res.end(); return; }
      try {
        const headers: Record<string, string> = {
          Referer: "https://www.douyin.com/",
          "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15",
        };
        if (req.headers.range) headers.Range = req.headers.range;
        const r = await fetch(u, { headers });
        res.statusCode = r.status;
        const ct = r.headers.get("content-type") || "";
        if (ct.includes("mpegurl") || new URL(r.url || u).pathname.endsWith(".m3u8")) {
          // HLS 清单里的分片也指回本代理。
          const link = (x: string) => {
            const abs = new URL(x, r.url || u);
            const ext = (abs.pathname.split("/").pop() || "").split(".").pop() || "bin";
            return `/media/m.${ext}?u=${encodeURIComponent(abs.href)}`;
          };
          const body = (await r.text()).split("\n").map((line) => {
            const t = line.trim();
            if (!t) return "";
            if (t.startsWith("#")) return t.replace(/URI="([^"]+)"/g, (_m, x) => `URI="${link(x)}"`);
            return link(t);
          }).join("\n");
          res.setHeader("content-type", "application/vnd.apple.mpegurl");
          res.setHeader("cache-control", "no-cache");
          res.end(body);
          return;
        }
        for (const h of ["content-type", "content-range", "content-length"]) {
          const v = r.headers.get(h); if (v) res.setHeader(h, v);
        }
        res.setHeader("accept-ranges", "bytes");
        if (r.body) {
          // 换网/切代理会掐断上游连接，不吞掉错误会让 Vite 进程整个退出。
          const body = Readable.fromWeb(r.body as any);
          body.on("error", () => res.destroy());
          res.on("close", () => body.destroy());
          body.pipe(res);
        } else {
          res.end();
        }
      } catch { res.statusCode = 502; res.end(); }
    });
  },
};

export default defineConfig({
  plugins: [mediaProxy],
  clearScreen: false,
  define: { __APP_VERSION__: JSON.stringify(pkg.version) },
  server: {
    port: 1420,
    strictPort: true,
    host: "127.0.0.1",
    watch: { ignored: ["**/src-tauri/**", "**/design/**"] },
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    target: "chrome105",
    chunkSizeWarningLimit: 650,
    minify: process.env.TAURI_ENV_DEBUG ? false : "esbuild",
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes("hls.js")) return "hls";
          if (id.includes("@tauri-apps")) return "tauri";
        },
      },
    },
  },
});
