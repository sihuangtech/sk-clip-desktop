import { invoke } from '@tauri-apps/api/core';
import type { AppConfig, DocumentContent } from '../types';

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

  extractAudioFromVideo: (videoPath: string) =>
    invoke<string>('extract_audio_from_video', { videoPath }),

  // 翻译和 AI
  translateVideo: (videoPath: string, sourceLanguage: string, targetLanguage: string) =>
    invoke<string>('translate_video', { videoPath, sourceLanguage, targetLanguage }),

  getTranslationTask: (taskId: string) =>
    invoke<string>('get_translation_task', { taskId }),

  checkTaskOutput: (taskId: string) =>
    invoke<string>('check_task_output', { taskId }),

  recognizeSpeech: (audioPath: string) =>
    invoke<string>('recognize_speech', { audioPath }),

  translateText: (text: string, sourceLanguage: string, targetLanguage: string) =>
    invoke<string>('translate_text', { text, sourceLanguage, targetLanguage }),

  synthesizeSpeech: (text: string, voiceType: string, speed: number) =>
    invoke<string>('synthesize_speech', { text, voiceType, speed }),

  getAiModelStatus: () =>
    invoke<string>('get_ai_model_status'),

  initializeAiModels: () =>
    invoke<boolean>('initialize_ai_models'),

  // 文档处理
  importDocument: (filePath: string) =>
    invoke<DocumentContent>('import_document', { filePath }),

  getSupportedDocumentTypes: () =>
    invoke<string[]>('get_supported_document_types'),

  convertDocumentToAssets: (documentPath: string) =>
    invoke<string>('convert_document_to_assets', { documentPath }),

  // 时间线项目
  createTimelineProject: (name: string) =>
    invoke<string>('create_timeline_project', { name }),

  saveTimelineProject: (project: string) =>
    invoke<boolean>('save_timeline_project', { project }),

  loadTimelineProject: (projectId: string) =>
    invoke<string>('load_timeline_project', { projectId }),

  exportTimelineVideo: (projectId: string) =>
    invoke<string>('export_timeline_video', { projectId }),

  listTimelineProjects: () =>
    invoke<string[]>('list_timeline_projects'),

  deleteTimelineProject: (projectId: string) =>
    invoke<boolean>('delete_timeline_project', { projectId }),

  // 代理管理
  getProxyConfig: () =>
    invoke<string>('get_proxy_config'),

  applyProxyProfile: (profile: string) =>
    invoke<boolean>('apply_proxy_profile', { profile }),

  disableProxy: () =>
    invoke<boolean>('disable_proxy'),

  testProxyConnection: () =>
    invoke<boolean>('test_proxy_connection'),

  autoDetectProxy: () =>
    invoke<string>('auto_detect_proxy'),

  getMirrorUrl: (originalUrl: string) =>
    invoke<string>('get_mirror_url', { originalUrl }),

  testDownloadConnection: (url: string) =>
    invoke<boolean>('test_download_connection', { url }),
};
