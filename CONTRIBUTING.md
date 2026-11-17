# 参与开发

状态：现行
摘要：开发环境、检查、代码约定、PR 和文档回流。

## 环境

- Rust stable
- Node.js 20+
- pnpm 10+
- 对应平台 WebView 依赖，见 `docs/BUILD.md`

## 检查

提交前必须通过：

```bash
pnpm check
```

`pnpm check` 包含 TypeScript 类型检查、前端构建、UI 单测、Rust fmt、clippy `-D warnings` 和 Rust 单测。

## 代码约定

- 数据层只用 Rust，不增加 Python、Node、Java 或 ffmpeg 运行时依赖。
- 视图层只经 `src/lib/api.ts` 调 `api_call`。
- 中文界面文案集中在 `copy.ts`。
- 颜色和间距使用 `src/styles.css` 顶部变量。
- 破坏性操作必须可撤销或二次确认。
- 新增平台差异时三端共用代码，按 Linux、Windows/macOS 顺序验证。

## 分支与提交

- `main` 只接收 PR；空仓库首次初始化除外。
- 从最新 `main` 建短分支。
- PR 内允许追加提交。
- 合并只使用 squash，合并后主分支只增加一个提交。
- 标签在合并后打在主分支提交上。
- PR 描述写复现、改动、验证和平台。

## 合并与回流

PR 合并后由维护者回带改动，下一次发布带入；主分支不直接承载长期开发。

## 推送

维护者推送走 `scripts/push.sh`：把 PAT 写进仓库根 `.env`（按 `.env.example`），再执行：

```bash
./scripts/push.sh main
./scripts/push.sh <标签>
```

`fetch` 用 `./scripts/push.sh fetch`。`.env` 已被忽略，不会入库。

## 文档

README 和 `docs/` 随代码一起维护，修改后与代码同一个 PR 提交。

## 许可证

提交即表示贡献按 Apache-2.0 授权。无需签署 CLA。
