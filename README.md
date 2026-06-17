# SK Clip (彩旗剪辑)

SK Clip is a cross-platform desktop application for document-driven video creation. Built with Tauri 2, Rust, and React, it combines local video assets, presentation documents, subtitles, translation, and speech synthesis into a single offline workflow.

The project emphasizes local-first processing. Sensitive media and documents stay on the user's machine instead of being uploaded to cloud services.

## Features

- **Video import and basic editing**
  - Import local video files as primary media or insert clips.
  - Prepare clips with basic trimming, splitting, duration adjustment, resizing, subtitle overlay, audio extraction, thumbnails, and merging through FFmpeg.

- **Visual timeline editor**
  - Arrange video, audio, document, subtitle, and generated speech tracks on a timeline.
  - Drag timeline elements and control their start time and duration.
  - Build structured videos from document pages, imported clips, subtitles, and synthesized narration.

- **Document import and integration**
  - Support for PPTX, Markdown, and PDF imports.
  - Parse document content into pages, text, images, and layout metadata.
  - Convert document content into visual assets synchronized with video clips.

- **Local text-to-speech**
  - Choose from multiple local open-source TTS engines in settings.
  - Lightweight engines: Piper, KittenTTS, Kokoro ONNX, sherpa-onnx.
  - Advanced voice cloning: GPT-SoVITS, F5-TTS (sidecar integration).

- **Video translation workflow**
  - Local speech recognition (Whisper-based).
  - Text translation with open-source MT models.
  - Subtitle generation and dubbed audio synthesis.

- **Offline-first and cross-platform**
  - Built on Tauri for Windows, macOS, and Linux.
  - Heavy processing runs locally for privacy and predictable performance.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | React 19, TypeScript, Vite 7 |
| Desktop/Backend | Tauri 2, Rust |
| Video Processing | FFmpeg (command-line) |
| AI Models | Candle (Rust ML), local ONNX models |
| Document Processing | Rust parsers (PPTX, Markdown, PDF) |

## TTS Engine Options

| Engine | Role | Footprint | License | Best Fit |
|--------|------|-----------|---------|----------|
| [eSpeak NG](https://github.com/espeak-ng/espeak-ng) | Ultra-light rule-based | A few MB | GPL-3.0 | Fallback, debugging |
| [Piper](https://github.com/rhasspy/piper) | Lightweight neural | Voice packages | MIT/GPL | Default offline narration |
| [KittenTTS](https://github.com/KittenML/KittenTTS) | Lightweight neural | 25-80 MB | Apache-2.0 | Natural local narration |
| [Kokoro ONNX](https://github.com/thewh1teagle/kokoro-onnx) | Mid-light high-quality | ~80 MB | Apache-2.0 | Quality narration |
| [GPT-SoVITS](https://github.com/RVC-Boss/GPT-SoVITS) | Voice cloning | Heavy (sidecar) | MIT | Dubbing, cloning |
| [F5-TTS](https://github.com/SWivid/F5-TTS) | Voice cloning | Heavy (sidecar) | MIT | Advanced cloning |

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Tauri CLI](https://tauri.app/start/prerequisites/)

```bash
# Install Tauri CLI
cargo install tauri-cli --version "^2.0.0" --locked
```

### Run in Development

```bash
# Install frontend dependencies
npm install

# Start development server
npm run tauri dev
```

### Build for Production

```bash
# Build frontend
npm run build

# Build desktop application
npm run tauri build
```

Output artifacts are in `src-tauri/target/release/bundle/`.

### Upgrade Dependencies

```bash
# Update Rust
rustup update stable

# Update npm packages
npm update

# Update Cargo dependencies
cd src-tauri && cargo update
```

## Project Structure

```
sk-clip-desktop/
├── src/                    # React + TypeScript frontend
│   ├── api/               # Tauri API wrappers
│   ├── components/        # UI components
│   ├── context/           # React context (state management)
│   ├── styles/            # CSS styles
│   └── types/             # TypeScript type definitions
├── src-tauri/             # Tauri + Rust backend
│   ├── src/
│   │   ├── ai/           # AI services (ASR, MT, TTS)
│   │   ├── commands/     # Tauri commands
│   │   ├── document/     # Document processing
│   │   ├── model_manager/# Model download/management
│   │   ├── video/        # Video processing (FFmpeg)
│   │   └── utils/        # Utility functions
│   └── Cargo.toml        # Rust dependencies
├── package.json           # Node.js dependencies
└── vite.config.ts         # Vite configuration
```

## Current Status

This is a prototype. Several features are partially implemented or mocked:

- Core video processing uses FFmpeg
- AI pipeline (ASR, MT, TTS) uses simulated backends
- Timeline editor has basic functionality
- Document import supports PPTX, Markdown, PDF

## License

MIT License. See [LICENSE](LICENSE) for details.
