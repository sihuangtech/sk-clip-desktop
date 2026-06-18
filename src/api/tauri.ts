import { invoke } from '@tauri-apps/api/core';
import type {
  AppConfig,
  DocumentContent,
  SpeechSynthesisResult,
  TranslationResult,
  TranslationTask,
} from '../types';

export const tauriApi = {
  // 基础命令
  greet: (name: string) =>
    invoke<string>('greet', { name }),

  getAppConfig: () =>
    invoke<AppConfig>('get_app_config'),

  updateAppConfig: (config: AppConfig) =>
    invoke<boolean>('update_app_config', { config }),

  // 视频处理
  uploadVideo: (path: string) =>
    invoke<string>('upload_video', { path }),

  getVideoInfo: (path: string) =>
    invoke<string>('get_video_info', { path }),

  extractAudioFromVideo: (videoPath: string, outputAudioPath: string) =>
    invoke<string>('extract_audio_from_video', { videoPath, outputAudioPath }),

  // 翻译和 AI
  translateVideo: (videoPath: string, sourceLanguage: string, targetLanguage: string) =>
    invoke<string>('translate_video', { videoPath, sourceLanguage, targetLanguage }),

  getTranslationTask: (taskId: string) =>
    invoke<TranslationTask>('get_translation_task', { taskId }),

  checkTaskOutput: (taskId: string) =>
    invoke<string>('check_task_output', { taskId }),

  recognizeSpeech: (audioPath: string) =>
    invoke<string>('recognize_speech', { audioPath }),

  translateText: (text: string, sourceLanguage: string, targetLanguage: string) =>
    invoke<TranslationResult>('translate_text', { text, sourceLanguage, targetLanguage }),

  synthesizeSpeech: (text: string, outputPath: string, config?: Record<string, unknown>) =>
    invoke<SpeechSynthesisResult>('synthesize_speech', { text, outputPath, config }),

  getAiModelStatus: () =>
    invoke<string>('get_ai_model_status'),

  initializeAiModels: () =>
    invoke<string>('initialize_ai_models'),

  // 文档处理
  importDocument: (filePath: string) =>
    invoke<DocumentContent>('import_document', { filePath }),

  getSupportedDocumentTypes: () =>
    invoke<string[]>('get_supported_document_types'),

  convertDocumentToAssets: (documentContent: DocumentContent) =>
    invoke<string[]>('convert_document_to_assets', { documentContent }),

  // 时间线项目
  createTimelineProject: (name: string) =>
    invoke<unknown>('create_timeline_project', { projectName: name }),

  saveTimelineProject: (projectId: string, projectData: unknown) =>
    invoke<string>('save_timeline_project', { projectId, projectData }),

  loadTimelineProject: (projectId: string) =>
    invoke<unknown>('load_timeline_project', { projectId }),

  exportTimelineVideo: (projectId: string, outputPath: string, exportSettings: Record<string, unknown>) =>
    invoke<string>('export_timeline_video', { projectId, outputPath, exportSettings }),

  listTimelineProjects: () =>
    invoke<unknown[]>('list_timeline_projects'),

  deleteTimelineProject: (projectId: string) =>
    invoke<boolean>('delete_timeline_project', { projectId }),

  // 代理管理
  getProxyConfig: () =>
    invoke<string>('get_proxy_config'),

  applyProxyProfile: (profile: string) =>
    invoke<string>('apply_proxy_profile', { profileName: profile }),

  disableProxy: () =>
    invoke<string>('disable_proxy'),

  testProxyConnection: (
    proxyType: string,
    host: string,
    port: number,
    username?: string,
    password?: string,
  ) =>
    invoke<boolean>('test_proxy_connection', { proxyType, host, port, username, password }),

  autoDetectProxy: () =>
    invoke<string>('auto_detect_proxy'),

  getMirrorUrl: (originalUrl: string) =>
    invoke<string>('get_mirror_url', { originalUrl }),

  testDownloadConnection: (url: string) =>
    invoke<boolean>('test_download_connection', { url }),
};
