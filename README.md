# blivedm_rs

一个功能强大的 Bilibili 直播间弹幕 WebSocket 客户端 Rust 库，支持实时弹幕监控、文字转语音（TTS）和浏览器 Cookie 自动检测。

[English Version README](README.en.md)

## 🚀 主要功能

- **🔍 智能 Cookie 检测** - 自动从主流浏览器（Chrome、Firefox、Edge、Opera）检测登录状态
- **💬 实时弹幕监控** - 连接 Bilibili 直播间，实时接收弹幕、礼物、进房等消息
- **🔊 多平台 TTS 支持** - 支持本地 TTS（Windows PowerShell、macOS say、Linux espeak-ng）和远程 TTS 服务器
- **🎛️ 插件化架构** - 模块化设计，支持自定义插件扩展功能
- **🖥️ 跨平台支持** - 原生支持 Windows、macOS、Linux，提供预编译二进制文件
- **⚡ 高性能异步** - 基于 Tokio 的异步架构，低资源占用，高并发处理
- **🔧 灵活配置** - 支持命令行参数配置，可自定义 TTS 音量、语音等参数

## ✨ 新功能：自动浏览器 Cookie 检测

**无需手动提取 Cookie！** 客户端现在可以自动从您的浏览器检测 bilibili cookies。

支持 Linux、macOS 和 Windows 上的 Chrome、Firefox、Edge、Chromium 和 Opera。详情请参阅[浏览器 Cookie 文档](docs/browser-cookies.md)。

## 快速开始

### 预构建二进制文件（推荐）

现在提供预构建二进制文件！从 [Releases 页面](https://github.com/jiahaoxiang2000/blivedm_rs/releases) 下载适合您系统的版本：

- **Windows**: `danmu-windows-x86_64.exe`
- **Linux**: `danmu-linux-x86_64`
- **macOS Intel**: `danmu-macos-x86_64`
- **macOS Apple Silicon**: `danmu-macos-arm64`

下载后直接运行：

```bash
# Windows
danmu-windows-x86_64.exe --room-id 24779526

# Linux/macOS (需要添加执行权限)
chmod +x danmu-linux-x86_64
./danmu-linux-x86_64 --room-id 24779526

# macOS
chmod +x danmu-macos-x86_64
./danmu-macos-x86_64 --room-id 24779526
```

### 使用示例

```bash
# 新功能：自动检测浏览器 cookies（推荐）
./danmu-linux-x86_64 --room-id 12345

# 手动 cookies（必须包含 SESSDATA）
./danmu-linux-x86_64 --cookies "SESSDATA=your_sessdata; other_cookie=..." --room-id 12345

# 使用 TTS REST API 服务器
./danmu-linux-x86_64 --room-id 12345 --tts-server http://localhost:8000 --tts-volume 0.7

# 使用本地 TTS（macOS）
./danmu-macos-x86_64 --room-id 12345 --tts-command say --tts-args "-v,Mei-Jia"

# 使用本地 TTS（Linux）
./danmu-linux-x86_64 --room-id 12345 --tts-command espeak-ng --tts-args "-v,cmn"

# ⚠️ Windows 用户建议：使用 TTS 服务器获得更好的语音体验
# 本地 PowerShell TTS 存在技术限制，推荐使用远程 TTS 服务器：
./danmu-windows-x86_64.exe --room-id 12345 --tts-server http://localhost:8000

# 显示所有可用选项
./danmu-linux-x86_64 --help
```

### TTS 服务器设置（Windows 用户推荐）

**Windows 用户特别推荐使用 TTS 服务器！** 相比受限的本地 PowerShell TTS，服务器提供更好的语音质量和功能。

```bash
# 克隆并设置 TTS 服务器
git clone https://github.com/jiahaoxiang2000/danmu-tts.git
cd danmu-tts
# 按照仓库中的设置说明进行操作
```

**TTS 服务器优势：**
- 🎙️ **高质量语音** - 支持神经网络 TTS 和多种语音引擎
- 🌐 **多语言支持** - 支持中文、英文等多种语言
- ⚙️ **灵活配置** - 可自定义语音参数、音调、语速
- 🔧 **易于部署** - 独立运行，无需复杂配置

详细设置说明请参阅 [danmu-tts 仓库](https://github.com/jiahaoxiang2000/danmu-tts)。

## 从源码构建

如果您希望从源码构建或进行开发，可以按照以下步骤：

```bash
# 克隆仓库
git clone https://github.com/jiahaoxiang2000/blivedm_rs.git
cd blivedm_rs

# 构建项目
cargo build --release

# 运行弹幕客户端（自动检测浏览器 cookies）
./target/release/danmu --room-id 24779526

# 或使用手动 cookies（必须包含 SESSDATA）
./target/release/danmu --cookies "SESSDATA=your_sessdata; other_cookie=..." --room-id 24779526
```

### 系统要求

- **Rust**: 最新稳定版本
- **Linux**: 
  - 音频支持：`sudo apt-get install libasound2-dev`
  - 构建工具：`sudo apt-get install pkg-config libssl-dev`
  - 可选 TTS：`sudo apt-get install espeak-ng`
- **macOS**: 无需额外依赖（使用内置 `say` 命令进行 TTS）
- **Windows**: 无需额外依赖（通过 System.Speech 使用内置 PowerShell TTS）

详细使用指南请参阅 [Danmu](docs/danmu.md) 页面。

## 文档

完整文档位于 [`docs/`](docs/) 文件夹中。主要部分包括：

- [入门指南](docs/README.md)：介绍和设置说明。
- [浏览器 Cookie 自动检测](docs/browser-cookies.md)：**新功能！** 自动 cookie 检测的工作原理。
- [使用指南](docs/usage.md)：如何在您的项目中使用该库。
- [架构](docs/architecture.md)：项目架构和设计。
- [客户端模块](docs/client.md)：客户端实现的详细信息。
- [弹幕模块](docs/danmu.md)：弹幕模块的信息。
- [调度器](docs/scheduler.md)：调度器组件概述。
- [插件](docs/plugins.md)：可用插件及其使用方法。

库文档也可在 [pages](https://jiahaoxiang2000.github.io/blivedm_rs/) 上获取。

## 参考

- [blivedm](https://github.com/xfgryujk/blivedm)：Bilibili 直播弹幕协议的原始 Python 实现。
- [blivedm-rs](https://github.com/yanglul/blivedm_rs)：blivedm 库的 Rust 移植版。
- [bililive-rs](https://github.com/LightQuantumArchive/bililive-rs)：另一个用于 Bilibili 直播的 Rust 实现。
- [bilibili-API-collect](https://github.com/SocialSisterYi/bilibili-API-collect)：SocialSisterYi 的 Bilibili API 集合。