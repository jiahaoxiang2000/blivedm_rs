# blivedm_rs

Bilibili 直播间弹幕 WebSocket 客户端 Rust 库。

## ✨ 新功能：自动浏览器 Cookie 检测

**无需手动提取 Cookie！** 客户端现在可以自动从您的浏览器检测 bilibili cookies。

```bash
# 无需 cookies 直接运行 - 将自动检测浏览器中的 cookies！
cargo run --bin danmu -- --room-id 24779526
# 如需要仍支持手动 cookies
cargo run --bin danmu -- --room-id 24779526
# 或者使用显式参数：
cargo run --bin danmu -- --room-id 24779526 --cookies "SESSDATA=your_sessdata; other_cookie=..."
```

支持 Linux、macOS 和 Windows 上的 Chrome、Firefox、Edge、Chromium 和 Opera。详情请参阅[浏览器 Cookie 文档](docs/browser-cookies.md)。

## 快速开始

详细使用指南请参阅 [Danmu](docs/danmu.md) 页面。

### 从源码构建

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

### 预构建二进制文件

预构建二进制文件将在未来版本中提供。目前请使用上述说明从源码构建。

### TTS 服务器设置（可选）

如需高级 TTS 功能，您可以设置 danmu-tts 服务器：

```bash
# 克隆并设置 TTS 服务器
git clone https://github.com/jiahaoxiang2000/danmu-tts.git
cd danmu-tts
# 按照仓库中的设置说明进行操作
```

TTS 服务器提供高质量神经网络语音和多种 TTS 后端。详细设置说明请参阅 [danmu-tts 仓库](https://github.com/jiahaoxiang2000/danmu-tts)。

### 使用示例

```bash
# 新功能：自动检测浏览器 cookies（推荐）
./target/release/danmu --room-id 12345

# 手动 cookies（必须包含 SESSDATA）
./target/release/danmu --cookies "SESSDATA=your_sessdata; other_cookie=..." --room-id 12345

# 使用 TTS REST API 服务器
./target/release/danmu --room-id 12345 --tts-server http://localhost:8000 --tts-volume 0.7

# 使用本地 TTS（macOS）
./target/release/danmu --room-id 12345 --tts-command say --tts-args "-v,Mei-Jia"

# 使用本地 TTS（Linux）
./target/release/danmu --room-id 12345 --tts-command espeak-ng --tts-args "-v,cmn"

# 显示所有可用选项
./target/release/danmu --help
```

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