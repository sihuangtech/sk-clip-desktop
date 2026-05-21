# SK Clip

[简体中文](README.zh.md)

SK Clip is a cross-platform desktop application built with Tauri, Rust, and Yew. It is designed for document-driven video creation: combining local video assets, presentation or document content, subtitles, translation, and speech synthesis into a single offline workflow for presentations, online courses, training videos, and narrated explainers.

The project emphasizes local-first processing. Sensitive media and documents are intended to stay on the user's machine instead of being uploaded to cloud services.

## Features

- **Video import and basic editing**
  - Import local video files as primary media or insert clips.
  - Prepare clips with basic trimming, splitting, duration adjustment, resizing, subtitle overlay, audio extraction, thumbnails, and merging through FFmpeg.
  - Keep source media organized for a document-driven editing workflow.

- **Visual timeline editor**
  - Arrange video, audio, document, subtitle, and generated speech tracks on a timeline.
  - Drag timeline elements and control their start time and duration.
  - Build structured videos from document pages, imported clips, subtitles, and synthesized narration.

- **Document import and integration**
  - Planned support for PPTX, Markdown, PDF, and DOCX imports.
  - Parse document content into pages, text, images, tables, and layout metadata.
  - Convert document content into visual assets that can be synchronized with video clips.

- **Local text-to-speech**
  - Choose from multiple local open-source TTS engines in the app settings.
  - Lightweight reading engines include Piper, KittenTTS, Kokoro ONNX, and sherpa-onnx.
  - Advanced voice cloning or dubbing engines can be added through GPT-SoVITS, F5-TTS, or similar sidecar integrations.

- **Video translation workflow**
  - Planned local speech recognition, text translation, subtitles, and dubbed audio.
  - ASR is planned around Whisper or compatible local models.
  - Translation is planned around open-source MT models such as Helsinki-NLP/OPUS-MT, potentially through CTranslate2 or Rust-friendly inference runtimes.

- **Offline-first and cross-platform**
  - Built on Tauri for Windows, macOS, and Linux.
  - Heavy processing is designed to run locally for privacy and predictable performance.

## Tech Stack

- Frontend: Rust, WebAssembly, Yew, Trunk.
- Desktop/backend: Tauri 2 and Rust.
- Video processing: FFmpeg command-line integration.
- Document processing: Rust parsers and planned conversion tooling.
- AI models:
  - TTS: configurable local engines, from lightweight ONNX engines to voice-cloning sidecars.
  - ASR: planned Whisper-compatible local recognition.
  - MT: planned open-source machine translation models.

## Local Open-Source TTS Options

The app includes built-in metadata for several local TTS engine candidates. Users can select the default TTS engine from the settings panel. The current implementation wires the configuration and UI entry point first; actual inference backends will be implemented incrementally.

| Engine | Role | Footprint | License Notes | Best Fit |
| --- | --- | --- | --- | --- |
| [eSpeak NG](https://github.com/espeak-ng/espeak-ng) | Ultra-light rule-based engine | A few MB, CPU only | GPL-3.0 | Fallback narration, debugging, broad language coverage |
| [Flite](https://github.com/festvox/flite) | Ultra-light embeddable engine | Small CPU runtime | BSD-like | English fallback narration, low-resource devices |
| [TinyTTS](https://github.com/tronghieuit/tiny-tts) | Ultra-light neural TTS | About 3.4 MB ONNX, CPU | MIT | English lightweight experiments, very small packages |
| [KittenTTS](https://github.com/KittenML/KittenTTS) | Lightweight neural TTS | About 25-80 MB ONNX, CPU | Apache-2.0 | Natural local narration, desktop default candidate |
| [Piper](https://github.com/rhasspy/piper) | Lightweight neural TTS | Voice packages downloaded as needed, CPU | MIT / GPL branches require separate review | Stable offline narration, default engine candidate |
| [Kokoro ONNX](https://github.com/thewh1teagle/kokoro-onnx) | Mid-light high-quality neural TTS | Quantized models around 80 MB, CPU capable | Apache-2.0 | More natural explainer narration |
| [sherpa-onnx](https://github.com/k2-fsa/sherpa-onnx) | Cross-platform ONNX runtime layer | Depends on selected model, CPU capable | Apache-2.0 | Unified local runtime for Piper, Kokoro, VITS, and related models |
| [GPT-SoVITS](https://github.com/RVC-Boss/GPT-SoVITS) | Voice cloning / advanced dubbing | Heavier, best as Python sidecar, GPU optional | MIT | Video translation dubbing and few-shot voice cloning |
| [F5-TTS](https://github.com/SWivid/F5-TTS) | Voice cloning / advanced dubbing | Heavier, best as Python sidecar, GPU optional | Code is MIT; pretrained weights require separate review | High-quality cloning experiments and non-default advanced engine |

Recommended integration path:

1. Default lightweight narration: Piper, KittenTTS, or Kokoro ONNX.
2. Cross-platform runtime layer: evaluate sherpa-onnx for managing ONNX-based models.
3. Advanced voice cloning: integrate GPT-SoVITS or F5-TTS as sidecar services or CLI commands rather than embedding PyTorch directly in the Rust process.

## Development, Build, and Packaging

### Prerequisites

This project is a Tauri 2 + Yew WebAssembly app. Install Rust, the WASM target, Tauri CLI, and Trunk before development:

```bash
rustup update stable
rustup target add wasm32-unknown-unknown
cargo install tauri-cli trunk
```

The repository defaults to the Shanghai Jiao Tong University SJTUG crates.io sparse index in `.cargo/config.toml`:

```toml
[source.crates-io]
replace-with = "sjtug"

[source.sjtug]
registry = "sparse+https://mirrors.sjtug.sjtu.edu.cn/crates.io-index/"
```

### Run in Development

From the repository root:

```bash
cargo tauri dev
```

This starts the Trunk/Yew frontend dev server and opens the Tauri desktop window.

### Build Checks

Check the full Rust workspace:

```bash
cargo check --workspace
```

Build frontend assets:

```bash
trunk build --release
```

Build the Rust workspace:

```bash
cargo build --workspace
```

### Package a Release

Build distributable desktop bundles:

```bash
cargo tauri build
```

Generated artifacts are usually placed under:

```text
src-tauri/target/release/bundle/
```

The exact output depends on the host platform and Tauri configuration. Examples include `.app`, `.dmg`, `.msi`, `.deb`, and `.AppImage`.

### Upgrade Dependencies

```bash
rustup update stable
cargo install cargo-edit
cargo upgrade --incompatible allow --recursive true
cargo upgrade --manifest-path src-tauri/Cargo.toml --incompatible allow --recursive true
cargo update
cargo check --workspace
```

## Usage

See [docs/usage.md](docs/usage.md) for workflow-level usage notes.

## Current Status

- This is still a prototype. Several advanced features, including full timeline rendering, document-to-video conversion, speech recognition, translation, and real TTS inference, are partially implemented or mocked.
- Core video utility commands use FFmpeg, but the complete AI-assisted video translation pipeline is still under active design.
- Model integration, voice cloning, and lip-sync are planned but not production-ready.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
