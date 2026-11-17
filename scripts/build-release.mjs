import { spawn } from "node:child_process";
import { resolve } from "node:path";

// 去掉构建机绝对路径，安装包里只留仓库内相对路径。
const root = resolve(import.meta.dirname, "..");
const flags = [process.env.RUSTFLAGS, `--remap-path-prefix=${root}=/douyintai`].filter(Boolean).join(" ");
const child = spawn("pnpm", ["tauri", "build", ...process.argv.slice(2)], {
  cwd: root,
  stdio: "inherit",
  env: { ...process.env, RUSTFLAGS: flags },
});
child.on("exit", (code) => process.exit(code ?? 1));
