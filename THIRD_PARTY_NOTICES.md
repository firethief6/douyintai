# 第三方资源说明

状态：现行
摘要：随包和随源码第三方资源的许可证说明。

本项目随包或随源码包含以下第三方资源。各资源权利归原权利人所有。

## 源码依赖

- `vendor/unidroid`：Apache-2.0，来源 `https://github.com/zhoupb01/unidroid`。
- `unidbg`：Apache-2.0，来源 `https://github.com/zhkl0228/unidbg`。
- `hls.js`：Apache-2.0。
- Tauri、Rust crates、npm 依赖：许可证随各自包提供，完整依赖树见 `Cargo.lock` 与 `pnpm-lock.yaml`。

## 界面资源

- Tabler Icons：MIT，Copyright (c) 2020-2026 Paweł Kuna。用于界面图标，全文见 `src-tauri/icons/TABLER-LICENSE.txt`。
- Inter：SIL Open Font License 1.1。用于西文与数字界面字体，全文见 `src/assets/fonts/Inter-LICENSE.txt`。

## Android 运行库

`crates/dysign/resources/rootfs/lib` 与 `lib64` 保存 Android 6.0（API 23）系统库，取自 unidbg 的 `android/sdk23` 资源，原始来源为 Android Open Source Project，按 Apache-2.0 分发。

## 抖音签名资源

- `crates/dysign/resources/libmetasec_ml.so`
- `crates/dysign/resources/supers.tsv.gz`
- `crates/dysign/resources/device-seq-cold.jsonl`

这些文件取自抖音客户端分发文件，用于本机签名复现。项目未随文件提供独立许可证文本，再分发依据以字节跳动及原客户端条款为准。资源可能随客户端版本变化，升级后需要重新提取和校验。

## 本机身份数据

匿名设备身份在首次运行时生成，写入用户数据目录，不随包分发。身份只含设备字段，不含抖音账号密码。

## 图片与样片

`src/assets/media/cover-5.jpg` 来自推荐流，仅作界面占位。权利归原发布者所有。
