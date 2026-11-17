// macOS 图标按系统网格留出透明边：824/1024 内容，避免 Dock 中比其它应用大一圈。
// 用法：node scripts/mac-icon.mjs [源 PNG]，默认 design/icon.png。
import { execFileSync } from "node:child_process";
import { copyFileSync, mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { resolve } from "node:path";

if (process.platform !== "darwin") {
  console.log("只在 macOS 生成 icon.icns");
  process.exit(0);
}

const root = resolve(import.meta.dirname, "..");
const source = resolve(root, process.argv[2] || "design/icon.png");
const work = mkdtempSync(resolve(tmpdir(), "douyintai-icon-"));
const master = resolve(work, "master.png");
const iconset = resolve(work, "icon.iconset");
const out = resolve(root, "src-tauri/icons/icon.icns");

try {
  execFileSync("uv", ["run", "--with", "pillow", "python", "-"], {
    input: `from PIL import Image\nim = Image.open(${JSON.stringify(source)}).convert("RGBA").resize((824, 824), Image.LANCZOS)\nout = Image.new("RGBA", (1024, 1024), (0, 0, 0, 0))\nout.paste(im, (100, 100), im)\nout.save(${JSON.stringify(master)})\n`,
    stdio: ["pipe", "inherit", "inherit"],
  });
  execFileSync("mkdir", ["-p", iconset]);
  const sizes = [16, 32, 32, 64, 128, 256, 512, 1024];
  const names = ["16x16", "16x16@2x", "32x32", "32x32@2x", "128x128", "128x128@2x", "256x256", "256x256@2x", "512x512", "512x512@2x"];
  const targets = [[16, "16x16"], [32, "16x16@2x"], [32, "32x32"], [64, "32x32@2x"], [128, "128x128"], [256, "128x128@2x"], [256, "256x256"], [512, "256x256@2x"], [512, "512x512"], [1024, "512x512@2x"]];
  for (const [px, name] of targets) {
    execFileSync("sips", ["-z", String(px), String(px), master, "--out", resolve(iconset, `icon_${name}.png`)], { stdio: "ignore" });
  }
  execFileSync("iconutil", ["-c", "icns", iconset, "-o", out], { stdio: "inherit" });
  copyFileSync(master, resolve(work, "preview.png"));
  console.log(`已生成 ${out}`);
  console.log(`预览：${resolve(work, "preview.png")}`);
  console.log(`尺寸：${sizes.length}/${names.length}`);
} finally {
  // 保留临时目录供查看预览；失败时不保留。
  if (process.exitCode) rmSync(work, { recursive: true, force: true });
}
