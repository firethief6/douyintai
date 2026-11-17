# 构建

状态：现行
摘要：Linux、macOS、Windows 的构建依赖、命令和产物。

## 通用前置

- Rust stable，通过 rustup 安装
- Node.js 20+
- pnpm 10+，通过 `corepack enable` 启用
- Git

安装依赖和检查：

```bash
pnpm install --frozen-lockfile
pnpm check
```

打包：

```bash
pnpm tauri build
```

产物在 `target/release/bundle/`。

## Linux

Ubuntu 24.04 安装 WebView 和打包依赖：

```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev \
  libdbus-1-dev libayatana-appindicator3-dev pkg-config
```

构建 deb：

```bash
pnpm tauri build --bundles deb
```

应用包名使用 ASCII，桌面项保持中文。

## macOS

安装 Xcode Command Line Tools：

```bash
xcode-select --install
```

构建 `.app` 和 dmg：

```bash
pnpm tauri build --bundles app,dmg
```

项目不配置开发者签名证书。构建只产生 ad-hoc 签名，首次打开前移除隔离属性：

```bash
xattr -dr com.apple.quarantine /Applications/抖音台.app
```

也可以在 Finder 中右键应用选择“打开”，再到系统设置中允许。

arm64 和 x86_64 分别构建。通用包使用：

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
pnpm tauri build --target universal-apple-darwin
```

## Windows

安装：

- Visual Studio Build Tools，勾选 C++ 桌面开发
- WebView2 Runtime
- Rust stable，目标 `x86_64-pc-windows-msvc`

WebView2 和可选 HEVC 扩展的使用方法见 [Windows 播放组件](WINDOWS-MEDIA.md)。Tauri 使用 `downloadBootstrapper` 检查并安装缺失的 WebView2，不需要把引导程序放进源码树。HEVC 扩展由用户自行安装。

构建：

```powershell
pnpm install --frozen-lockfile
pnpm tauri build
```

产物为 MSI 安装包，需要管理员权限。SmartScreen 提示时点击“更多信息 → 仍要运行”。

## 自动打包

`.github/workflows/package.yml` 在 Linux x86_64、Windows x86_64、macOS arm64 和 macOS x86_64 上执行检查与打包。

- PR：全部平台执行 `pnpm check`、打包、资源校验并上传 Artifact。
- 推送 `v*` 标签：并行打包，全部通过后创建预发布并上传安装包与 `SHA256SUMS`。
- 手动触发：执行打包，不出 Release。
- Artifact 保留 14 天。

任一平台失败时 Release 不创建。安装包不做开发者签名，与本地构建一致。

## 运行期资源

以下资源随包：

- `crates/dysign/resources/libmetasec_ml.so`
- `crates/dysign/resources/libEncryptor.so`
- `crates/dysign/resources/rootfs`
- `crates/dysign/resources/supers.tsv.gz`
- `crates/dysign/resources/device-seq-cold.jsonl`
- `crates/dysign/resources/alloc.tsv`
- `crates/dysign/resources/jni.tsv`

缺少任一资源时，启动后签名接口不可用。检查应用资源目录中的 `sign/`。

## 常见失败

- `webkit2gtk` 找不到：安装 Linux 依赖并重新登录。
- `pkg-config` 失败：确认开发包完整，不使用只含运行库的包。
- 媒体不播放：确认 `src-tauri/src/media.rs` 的回环代理已启动。
- 签名初始化失败：确认 `sign/` 资源完整，运行 `cargo test -p dysign`。
