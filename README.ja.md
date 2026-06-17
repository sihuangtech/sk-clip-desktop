# 彩旗クリップ

[English](README.md) | [简体中文](README.zh.md)

彩旗クリップは、ドキュメント駆動のビデオ作成のためのクロスプラットフォームデスクトップアプリケーションです。Tauri 2、Rust、React を基盤として構築され、ローカルビデオ素材、プレゼンテーションドキュメント、字幕、翻訳、音声合成を単一のオフラインワークフローに統合します。

本プロジェクトはローカルファーストの処理理念を採用しており、機密性の高いメディアやドキュメントデータは常にユーザーのデバイスに保存され、クラウドサービスにアップロードする必要はありません。

## 機能

- **ビデオインポートと基本編集**
  - ローカルビデオファイルをメイン素材またはクリップとしてインポート
  - FFmpeg によるトリミング、分割、時間調整、サイズ変更、字幕オーバーレイ、音声抽出、サムネイル生成、ビデオ結合

- **ビジュアルタイムラインエディタ**
  - ビデオ、オーディオ、ドキュメント、字幕、音声合成トラックをタイムラインに配置
  - タイムライン要素をドラッグし、開始時間と継続時間を制御
  - ドキュメントページ、インポートクリップ、字幕、合成ナレーションから構造化ビデオを構築

- **ドキュメントインポートと統合**
  - PPTX、Markdown、PDF 形式のインポートをサポート
  - ドキュメントコンテンツをページ、テキスト、画像、レイアウトメタデータに解析
  - ドキュメントコンテンツをビデオクリップと同期できる視覚素材に変換

- **ローカル音声合成**
  - 設定で複数のローカルオープンソース TTS エンジンから選択
  - 軽量エンジン：Piper、KittenTTS、Kokoro ONNX、sherpa-onnx
  - 高度な声のクローン：GPT-SoVITS、F5-TTS（サイドカー統合）

- **ビデオ翻訳ワークフロー**
  - ローカル音声認識（Whisper ベース）
  - テキスト翻訳（オープンソース機械翻訳モデル）
  - 字幕生成と吹き替え合成

- **オフラインファースト、クロスプラットフォーム**
  - Tauri ベースで Windows、macOS、Linux をサポート
  - 重い処理タスクはローカルで実行し、プライバシーを保護し安定したパフォーマンスを確保

## 技術スタック

| レイヤー | 技術 |
| -------- | ---- |
| フロントエンド | React 19、TypeScript、Vite 7 |
| デスクトップ/バックエンド | Tauri 2、Rust |
| ビデオ処理 | FFmpeg（コマンドライン呼び出し） |
| AI モデル | Candle（Rust ML）、ローカル ONNX モデル |
| ドキュメント処理 | Rust パーサー（PPTX、Markdown、PDF） |

## TTS エンジンオプション

| エンジン | 定位 | サイズ | ライセンス | 適用場面 |
| -------- | ---- | ---- | ---------- | -------- |
| [eSpeak NG](https://github.com/espeak-ng/espeak-ng) | 超軽量ルールベース | 数 MB | GPL-3.0 | 代替読み上げ、デバッグ |
| [Piper](https://github.com/rhasspy/piper) | 軽量ニューラル | 音声パックによる | MIT/GPL | デフォルトオフライン読み上げ |
| [KittenTTS](https://github.com/KittenML/KittenTTS) | 軽量ニューラル | 25-80 MB | Apache-2.0 | 自然なローカル読み上げ |
| [Kokoro ONNX](https://github.com/thewh1teagle/kokoro-onnx) | 中軽量高品質 | ~80 MB | Apache-2.0 | 高品質読み上げ |
| [GPT-SoVITS](https://github.com/RVC-Boss/GPT-SoVITS) | 声のクローン | 重量（サイドカー） | MIT | 吹き替え、クローン |
| [F5-TTS](https://github.com/SWivid/F5-TTS) | 声のクローン | 重量（サイドカー） | MIT | 高度なクローン |

## 開発ガイド

### 環境要件

- [Node.js](https://nodejs.org/)（v18+）
- [Rust](https://www.rust-lang.org/tools/install)（stable）
- [Tauri CLI](https://tauri.app/start/prerequisites/)

```bash
# Tauri CLI のインストール
cargo install tauri-cli --version "^2.0.0" --locked
```

### 開発実行

```bash
# フロントエンド依存関係のインストール
npm install

# 開発サーバーの起動
npm run tauri dev
```

### プロダクションビルド

```bash
# フロントエンドのビルド
npm run build

# デスクトップアプリケーションのビルド
npm run tauri build
```

ビルド成果物は `src-tauri/target/release/bundle/` ディレクトリに出力されます。

### 依存関係の更新

```bash
# Rust の更新
rustup update stable

# npm パッケージの更新
npm update

# Cargo 依存関係の更新
cd src-tauri && cargo update
```

## プロジェクト構造

```text
sk-clip-desktop/
├── src/                    # React + TypeScript フロントエンド
│   ├── api/               # Tauri API ラッパー
│   ├── components/        # UI コンポーネント
│   ├── context/           # React Context（状態管理）
│   ├── styles/            # CSS スタイル
│   └── types/             # TypeScript 型定義
├── src-tauri/             # Tauri + Rust バックエンド
│   ├── src/
│   │   ├── ai/           # AI サービス（ASR、MT、TTS）
│   │   ├── commands/     # Tauri コマンド
│   │   ├── document/     # ドキュメント処理
│   │   ├── model_manager/# モデルダウンロード/管理
│   │   ├── video/        # ビデオ処理（FFmpeg）
│   │   └── utils/        # ユーティリティ関数
│   └── Cargo.toml        # Rust 依存関係
├── package.json           # Node.js 依存関係
└── vite.config.ts         # Vite 設定
```

## 現在のステータス

本プロジェクトはプロトタイプ段階であり、一部の機能はシミュレーション実装です：

- コアビデオ処理は FFmpeg を使用
- AI パイプライン（ASR、MT、TTS）はシミュレーションバックエンドを使用
- タイムラインエディタは基本機能を備えている
- ドキュメントインポートは PPTX、Markdown、PDF をサポート

## ライセンス

MIT ライセンス。詳細は [LICENSE](LICENSE) をご覧ください。

## お問い合わせ

- メール：developer@skstudio.cn
- GitHub：[sihuangtech/sk-clip-desktop](https://github.com/sihuangtech/sk-clip-desktop)

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=sihuangtech/sk-clip-desktop&type=Date)](https://star-history.com/#sihuangtech/sk-clip-desktop&Date)
