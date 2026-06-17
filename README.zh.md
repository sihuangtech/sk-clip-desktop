# SK Clip (彩旗剪辑)

[English](README.md)

彩旗剪辑是一款跨平台桌面视频创作应用。基于 Tauri 2、Rust 和 React 构建，将本地视频素材、演示文档、字幕、翻译和语音合成为一体化的离线工作流。

项目采用本地优先处理理念，敏感的媒体和文档数据始终保留在用户设备上，无需上传至云端服务。

## 功能特点

- **视频导入与基础编辑**
  - 导入本地视频文件作为主素材或插入片段
  - 通过 FFmpeg 进行裁剪、分割、时长调整、尺寸调整、字幕叠加、音频提取、缩略图生成和视频合并

- **可视化时间线编辑器**
  - 在时间线上排列视频、音频、文档、字幕和语音合成轨道
  - 拖拽时间线元素，控制起始时间和持续时长
  - 从文档页面、导入片段、字幕和合成旁白构建结构化视频

- **文档导入与集成**
  - 支持 PPTX、Markdown 和 PDF 格式导入
  - 解析文档内容为页面、文本、图片和布局元数据
  - 将文档内容转换为可与视频片段同步的视觉素材

- **本地语音合成**
  - 在设置中选择多种本地开源 TTS 引擎
  - 轻量级引擎：Piper、KittenTTS、Kokoro ONNX、sherpa-onnx
  - 高级声音克隆：GPT-SoVITS、F5-TTS（sidecar 集成）

- **视频翻译工作流**
  - 本地语音识别（基于 Whisper）
  - 文本翻译（开源机器翻译模型）
  - 字幕生成和配音合成

- **离线优先、跨平台**
  - 基于 Tauri 构建，支持 Windows、macOS 和 Linux
  - 重度处理任务本地运行，保护隐私并确保稳定性能

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | React 19、TypeScript、Vite 7 |
| 桌面/后端 | Tauri 2、Rust |
| 视频处理 | FFmpeg（命令行调用） |
| AI 模型 | Candle（Rust ML）、本地 ONNX 模型 |
| 文档处理 | Rust 解析器（PPTX、Markdown、PDF） |

## TTS 引擎选项

| 引擎 | 定位 | 体积 | 许可证 | 适用场景 |
|------|------|------|--------|----------|
| [eSpeak NG](https://github.com/espeak-ng/espeak-ng) | 超轻量规则引擎 | 几 MB | GPL-3.0 | 备用朗读、调试 |
| [Piper](https://github.com/rhasspy/piper) | 轻量神经网络 | 按语音包下载 | MIT/GPL | 默认离线朗读 |
| [KittenTTS](https://github.com/KittenML/KittenTTS) | 轻量神经网络 | 25-80 MB | Apache-2.0 | 自然本地朗读 |
| [Kokoro ONNX](https://github.com/thewh1teagle/kokoro-onnx) | 中轻量高质量 | ~80 MB | Apache-2.0 | 高质量朗读 |
| [GPT-SoVITS](https://github.com/RVC-Boss/GPT-SoVITS) | 声音克隆 | 较重（sidecar） | MIT | 配音、克隆 |
| [F5-TTS](https://github.com/SWivid/F5-TTS) | 声音克隆 | 较重（sidecar） | MIT | 高级克隆 |

## 开发指南

### 环境要求

- [Node.js](https://nodejs.org/)（v18+）
- [Rust](https://www.rust-lang.org/tools/install)（stable）
- [Tauri CLI](https://tauri.app/start/prerequisites/)

```bash
# 安装 Tauri CLI
cargo install tauri-cli --version "^2.0.0" --locked
```

### 开发运行

```bash
# 安装前端依赖
npm install

# 启动开发服务器
npm run tauri dev
```

### 生产构建

```bash
# 构建前端
npm run build

# 构建桌面应用
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/` 目录。

### 更新依赖

```bash
# 更新 Rust
rustup update stable

# 更新 npm 包
npm update

# 更新 Cargo 依赖
cd src-tauri && cargo update
```

## 项目结构

```
sk-clip-desktop/
├── src/                    # React + TypeScript 前端
│   ├── api/               # Tauri API 封装
│   ├── components/        # UI 组件
│   ├── context/           # React Context（状态管理）
│   ├── styles/            # CSS 样式
│   └── types/             # TypeScript 类型定义
├── src-tauri/             # Tauri + Rust 后端
│   ├── src/
│   │   ├── ai/           # AI 服务（ASR、MT、TTS）
│   │   ├── commands/     # Tauri 命令
│   │   ├── document/     # 文档处理
│   │   ├── model_manager/# 模型下载/管理
│   │   ├── video/        # 视频处理（FFmpeg）
│   │   └── utils/        # 工具函数
│   └── Cargo.toml        # Rust 依赖
├── package.json           # Node.js 依赖
└── vite.config.ts         # Vite 配置
```

## 当前状态

本项目处于原型阶段，部分功能为模拟实现：

- 核心视频处理使用 FFmpeg
- AI 管线（ASR、MT、TTS）使用模拟后端
- 时间线编辑器具备基础功能
- 文档导入支持 PPTX、Markdown、PDF

## 许可证

MIT 许可证，详见 [LICENSE](LICENSE)。
