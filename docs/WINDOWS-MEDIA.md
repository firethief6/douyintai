# Windows 播放组件

状态：现行
摘要：Windows WebView2 与 HEVC 扩展的用途、安装、卸载和画质行为。

应用不会自动安装 HEVC 扩展，也不跳转商店。按需安装以下组件。

| 组件 | 用途 | 是否必需 |
|---|---|---|
| `MicrosoftEdgeWebview2Setup.exe` | 安装 Microsoft Edge WebView2 Runtime，应用用它显示界面 | 系统没有 WebView2 时必需 |
| `HEVC视频扩展-双击安装.appx` | 为 Windows 增加 H.265 / HEVC 视频解码能力 | 可选；没有它仍可选择支持的 H.264 资源 |

HEVC 扩展不等于 WebView2；只安装 WebView2 不能播放 HEVC。

## 下载

- WebView2：微软 [Evergreen 下载页](https://developer.microsoft.com/microsoft-edge/webview2/#download-section) 的 Standalone Installer。联网环境也可以使用小体积引导程序，由它下载并安装运行时。
- HEVC 扩展：Microsoft Store 的 [HEVC 视频扩展（来自设备制造商）](https://apps.microsoft.com/detail/9n4wgh0z6vhq)，免费；或付费版 [HEVC 视频扩展](https://apps.microsoft.com/detail/9nmzlz57r3t7)。

安装包必须匹配系统架构。

## 安装 WebView2

1. 完全退出抖音台（托盘菜单选择“退出”，仅关闭窗口会隐藏到托盘）。
2. 双击 `MicrosoftEdgeWebview2Setup.exe`，按提示完成安装。
3. 重新打开抖音台。

已安装 WebView2 的系统不需要重复安装。WebView2 可能被其他应用共用，不必为了检查 HEVC 而卸载它。

MSI 安装包使用 Tauri 默认的 `downloadBootstrapper` 模式：缺少运行时时由安装程序下载并安装。直接运行主程序的用户需要自行确保 WebView2 已安装。

## 安装 HEVC 扩展

1. 完全退出抖音台。
2. 双击下载的 `.appx` 文件，在 Windows 应用安装界面完成安装。商店直接安装时文件名不同，效果相同。
3. 重新打开抖音台。仅刷新页面不足以重新识别系统解码器。

也可以在 PowerShell 中为当前用户安装（路径按实际下载位置调整）：

```powershell
Add-AppxPackage -Path '.\HEVC视频扩展-双击安装.appx'
Get-AppxPackage -Name Microsoft.HEVCVideoExtension |
    Select-Object Name, Version, Status
```

安装前可以检查签名：

```powershell
Get-AuthenticodeSignature -LiteralPath '.\HEVC视频扩展-双击安装.appx' |
    Select-Object Status, SignerCertificate
```

有效签名显示 `Valid`，签名者为 Microsoft Corporation。

系统策略不允许安装时，继续使用支持的 H.264 资源。

## 卸载 HEVC 扩展

1. 完全退出抖音台。
2. 在 Windows“设置 → 应用 → 已安装的应用”中找到 HEVC 视频扩展并卸载，或执行：

```powershell
Get-AppxPackage -Name Microsoft.HEVCVideoExtension | Remove-AppxPackage
```

3. 重新打开抖音台。播放器重新检测解码能力，隐藏不支持的画质。

卸载影响当前用户其他依赖这个扩展播放 HEVC 的应用；以后可用同一安装包重新安装。

## 画质菜单

播放器底部、倍速按钮左侧显示当前画质，点击展开选择：

- 有三个及以上不同分辨率：高 / 中 / 低，最多三档。
- 只有两个不同分辨率：中 / 低。
- 只有一个分辨率：仅低。

同一分辨率不重复展示，自动选择该分辨率的最优支持资源。默认播放实际支持的最高分辨率，即使菜单名称是“中”或“低”。切换画质保留播放位置、暂停状态和倍速。

档位来自当前作品提供的资源，安装 HEVC 不会凭空增加分辨率。例如同一作品最高 H.264 是 1080P、最高 HEVC 是 1440P 时，未安装扩展默认 1080P；安装扩展且设备支持后默认 1440P。两种编码最高都是 720P 时，安装扩展后最高仍是 720P。

画质检测只影响预览。下载页独立选择档位，允许下载暂时无法解码的 HEVC 文件。库内本地文件预览需要系统支持其编码。

## 画质行为

| 系统状态（均重启应用） | 默认画质 | 结果 |
|---|---|---|
| 未安装 HEVC 扩展 | H.264 1080P | 正常解码 |
| 安装 HEVC 扩展 | HEVC 1440P | 正常解码 |
| 卸载 HEVC 扩展 | H.264 1080P | 正常解码 |

其他设备还会受到系统版本、显卡和驱动影响。播放器按运行时检测筛选，不按操作系统名称固定禁用 HEVC。
