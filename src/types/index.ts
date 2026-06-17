// 应用状态类型
export type AppState =
  | { status: 'idle' }
  | { status: 'uploading' }
  | { status: 'ready'; videoPath: string }
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
  Pdf = 'Pdf',
}

export interface DocumentPage {
  pageNumber: number;
  title?: string;
  textContent: string;
  imagePaths: string[];
  notes?: string;
}

export interface DocumentContent {
  documentType: DocumentType;
  title: string;
  pages: DocumentPage[];
  totalPages: number;
  sourcePath: string;
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
  recommendedUse: string;
  repositoryUrl: string;
}

// 应用配置
export interface AiConfig {
  selectedTtsEngine: string;
  availableTtsEngines: TtsEngine[];
  defaultSourceLanguage: string;
  defaultTargetLanguage: string;
  whisperModelPath: string;
}

export interface VideoConfig {
  defaultOutputFormat: string;
  defaultQuality: string;
  maxFileSize: number;
}

export interface DocumentConfig {
  pdfDpi: number;
  maxDocumentSize: number;
}

export interface AppConfig {
  ai: AiConfig;
  video: VideoConfig;
  document: DocumentConfig;
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
