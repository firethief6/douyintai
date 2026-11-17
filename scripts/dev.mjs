// 开发启动：清场 → 起 vite → 编译 debug → 装进带正式 bundle id 的 .app → 打开。
// 用 .app 而不是裸二进制，LaunchServices 才能按 bundle id 找到调试实例，
// 配合 tauri-plugin-single-instance 保证任何时候只有一个窗口。
import { execFileSync, spawn } from "node:child_process";
import { copyFileSync, mkdirSync, rmSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { platform } from "node:os";

const mac = platform() === "darwin";
const win = platform() === "win32";
const nap = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

const root = resolve(import.meta.dirname, "..");
const appPath = resolve(root, "target/debug/抖音台.app");
const binPath = resolve(root, "target/debug/douyintai");
const BIN = mac ? "target/debug/douyintai" : "target/debug/douyintai.exe";
const MARKERS = mac
  ? [BIN, "target/debug/抖音台.app/Contents/MacOS/douyintai"]
  : [BIN];

function pids(markers) {
  if (win) {
    const out = execFileSync("powershell.exe", ["-NoProfile", "-Command",
      "Get-CimInstance Win32_Process | Where-Object { $_.Name -in @('node.exe', 'douyintai.exe') } | Select-Object ProcessId,CommandLine | ConvertTo-Json -Compress",
    ], { encoding: "utf8", windowsHide: true });
    const normalize = (s) => s.replaceAll("\\", "/").toLowerCase();
    const needles = markers.map(normalize);
    return [].concat(JSON.parse(out || "[]"))
      .filter((p) => p.ProcessId !== process.pid && needles.some((m) => normalize(p.CommandLine || "").includes(m)))
      .map((p) => p.ProcessId);
  }
  const out = execFileSync("ps", ["-Ao", "pid=,command="], { encoding: "utf8" });
  return out
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => markers.some((m) => line.includes(m)))
    .map((line) => Number(line.split(/\s+/, 1)[0]))
    .filter((pid) => Number.isInteger(pid) && pid !== process.pid);
}

function kill(markers) {
  for (const sig of ["SIGTERM", "SIGKILL"]) {
    const list = pids(markers);
    if (!list.length) return;
    for (const pid of list) {
      try { process.kill(pid, sig); } catch { /* 已退出 */ }
    }
    const until = Date.now() + (sig === "SIGTERM" ? 3000 : 1000);
    while (Date.now() < until && pids(markers).length) nap(100);
  }
  const left = pids(markers);
  if (left.length) {
    console.error(`残留进程无法结束：${left.join(", ")}`);
    process.exit(1);
  }
}

const plist = `<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleDevelopmentRegion</key><string>Chinese</string>
<key>CFBundleDisplayName</key><string>抖音台</string>
<key>CFBundleExecutable</key><string>douyintai</string>
<key>CFBundleIdentifier</key><string>io.firethief6.douyintai</string>
<key>CFBundleInfoDictionaryVersion</key><string>6.0</string>
<key>CFBundleName</key><string>抖音台</string>
<key>CFBundleIconFile</key><string>icon.icns</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>CFBundleShortVersionString</key><string>0.1.0</string>
<key>CFBundleVersion</key><string>0.1.0</string>
<key>LSMinimumSystemVersion</key><string>10.13</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>
`;

kill(MARKERS);
kill(["@tauri-apps/cli/tauri.js dev", "vite/bin/vite.js --port 1420"]);
rmSync("/tmp/io_firethief6_douyintai_si.sock", { force: true });

const devArgs = mac ? ["dev"] : ["tauri", "dev"];
// npm 的 npm_execpath 是 JS；pnpm 12 的 Homebrew 包是原生二进制，不能再交给 node。
const cli = process.env.npm_execpath;
const jsCli = !!cli && /\.(?:c|m)?js$/i.test(cli);
const child = spawn(jsCli ? process.execPath : cli || (win ? "pnpm.cmd" : "pnpm"), jsCli ? [cli, ...devArgs] : devArgs, {
  cwd: root, stdio: "inherit", detached: !win, windowsHide: true, shell: win && !jsCli,
});
const stop = (sig) => {
  kill(MARKERS);
  try {
    if (win) execFileSync("taskkill", ["/PID", String(child.pid), "/T", "/F"], { stdio: "ignore", windowsHide: true });
    else process.kill(-child.pid, sig);
  } catch { /* 已退出 */ }
};
for (const sig of ["SIGINT", "SIGTERM"]) process.on(sig, () => { stop(sig); process.exit(0); });
if (!mac) child.on("exit", (code) => { stop("SIGTERM"); process.exit(code ?? 0); });

if (mac) {
  const url = `http://localhost:1420/?v=${crypto.randomUUID()}`;
  for (let i = 0; i < 60; i++) {
    try { execFileSync("curl", ["-sf", "-o", "/dev/null", url]); break; } catch { nap(500); }
  }

  execFileSync("cargo", ["build", "--no-default-features", "--color", "always"], {
    cwd: root, stdio: "inherit",
  });

  rmSync(appPath, { recursive: true, force: true });
  mkdirSync(resolve(appPath, "Contents/MacOS"), { recursive: true });
  mkdirSync(resolve(appPath, "Contents/Resources"), { recursive: true });
  copyFileSync(binPath, resolve(appPath, "Contents/MacOS/douyintai"));
  writeFileSync(resolve(appPath, "Contents/Info.plist"), plist);
  try {
    copyFileSync(resolve(root, "src-tauri/icons/icon.icns"), resolve(appPath, "Contents/Resources/icon.icns"));
  } catch { /* 无图标也能启动 */ }
  try {
    execFileSync("/System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister", ["-f", appPath], { stdio: "ignore" });
  } catch { /* 注册失败仍可启动 */ }

  execFileSync("open", ["-n", appPath], { stdio: "inherit" });
  console.log(`已启动：${appPath}`);
}
