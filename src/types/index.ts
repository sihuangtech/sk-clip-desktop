// 应用状态类型
export type AppState =
  | { status: 'idle' }
  | { status: 'uploading' }
  | { status: 'ready'; videoPath: string; videoInfo?: VideoMetadata }
  | { status: 'translating'; taskId: string }
  | { status: 'completed'; outputPath: string }
  | { status: 'error'; message: string }
  | { status: 'documentImporting' }
  | { status: 'documentReady'; content: DocumentContent }
  | { status: 'creatingProject' };

// 文档类型
export enum DocumentType {
  PowerPoint = 'PowerPoint',
  Markdown = 'Markdown',
  PDF = 'PDF',
  Word = 'Word',
  Unknown = 'Unknown',
}

export interface DocumentPage {
  page_number: number;
  text: string;
  images: unknown[];
  tables: unknown[];
  layout?: unknown;
}

export interface DocumentContent {
  document_type: DocumentType;
  title?: string;
  author?: string;
  pages: DocumentPage[];
  metadata: {
    file_path: string;
    file_size: number;
    created_at?: string;
    modified_at?: string;
    page_count: number;
    language?: string;
  };
}

// 翻译任务
export interface TranslationTask {
  id: string;
  videoPath: string;
  status: 'Pending' | 'Processing' | 'Completed' | 'Failed';
  sourceLanguage: string;
  targetLanguage: string;
  outputPath?: string;
  errorMessage?: string;
}

export interface TranslationResult {
  source_text: string;
  translated_text: string;
  source_language: string;
  target_language: string;
  confidence: number;
}

export interface SpeechSynthesisResult {
  text: string;
  voice: string;
  output_path: string;
  duration: number;
  sample_rate: number;
}

export interface VideoMetadata {
  file_path: string;
  file_size: number;
  duration: number;
  width: number;
  height: number;
  framerate: number;
  video_codec: string;
  audio_codec?: string;
  bitrate: number;
  sample_rate?: number;
  audio_channels?: number;
  format: string;
  created_at?: string;
  has_audio: boolean;
  has_video: boolean;
}

// 语言
export interface Language {
  code: string;
  name: string;
}

export const SUPPORTED_LANGUAGES: Language[] = [
  { code: 'zh', name: '中文' },
  { code: 'en', name: 'English' },
  { code: 'ja', name: '日本語' },
  { code: 'ko', name: '한국어' },
  { code: 'es', name: 'Español' },
  { code: 'fr', name: 'Français' },
  { code: 'de', name: 'Deutsch' },
  { code: 'ru', name: 'Русский' },
];

// TTS 引擎
export interface TtsEngine {
  id: string;
  name: string;
  category: string;
  footprint: string;
  license: string;
  recommended_use: string;
  repository_url: string;
}

// 应用配置
export interface AiConfig {
  whisper_model_path?: string;
  tts_model_path?: string;
  selected_tts_engine: string;
  available_tts_engines: TtsEngine[];
  translation_model_path?: string;
  default_language: string;
  supported_languages: string[];
}

export interface VideoConfig {
  default_output_format: string;
  default_quality: string;
  max_file_size_mb: number;
  temp_dir?: string;
}

export interface DocumentConfig {
  supported_formats: string[];
  max_file_size_mb: number;
  pdf_dpi: number;
}

export interface UiConfig {
  theme: string;
  language: string;
  window_size: [number, number];
  remember_window_position: boolean;
}

export interface AppConfig {
  ai: AiConfig;
  video: VideoConfig;
  document: DocumentConfig;
  ui: UiConfig;
}

// 音色类型
export type VoiceType = 'female' | 'male' | 'child' | 'elderly';

export const VOICE_OPTIONS: { type: VoiceType; label: string; description: string }[] = [
  { type: 'female', label: '女声', description: '温柔' },
  { type: 'male', label: '男声', description: '沉稳' },
  { type: 'child', label: '童声', description: '活泼' },
  { type: 'elderly', label: '老年', description: '慈祥' },
];

// 素材库标签
export type MaterialTab = 'video' | 'document' | 'audio' | 'image';

// 翻译面板标签
export type TranslationTab = 'translate' | 'synthesis';
