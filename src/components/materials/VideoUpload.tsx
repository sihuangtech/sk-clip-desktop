import { useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { useAppContext } from '../../context/AppContext';
import { SUPPORTED_LANGUAGES } from '../../types';
import { tauriApi } from '../../api/tauri';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';

export function VideoUpload() {
  const { appState, setAppState, sourceLanguage, setSourceLanguage, targetLanguage, setTargetLanguage } = useAppContext();
  const [error, setError] = useState<string | null>(null);

  const handleFileSelect = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Video',
          extensions: ['mp4', 'avi', 'mov', 'mkv', 'wmv', 'flv', 'webm'],
        },
      ],
    });

    if (typeof selected !== 'string') return;

    setError(null);
    setAppState({ status: 'uploading' });

    try {
      const importedPath = await tauriApi.uploadVideo(selected);
      setAppState({ status: 'ready', videoPath: importedPath });
    } catch (err) {
      const message = err instanceof Error ? err.message : '上传失败';
      setError(message);
      setAppState({ status: 'error', message });
    }
  };

  const handleTranslate = async () => {
    if (appState.status !== 'ready') return;

    setAppState({ status: 'translating', taskId: '' });
    try {
      const taskId = await tauriApi.translateVideo(
        appState.videoPath,
        sourceLanguage,
        targetLanguage
      );
      setAppState({ status: 'translating', taskId });
    } catch (err) {
      const message = err instanceof Error ? err.message : '翻译失败';
      setAppState({ status: 'error', message });
    }
  };

  const handleReset = () => {
    setAppState({ status: 'idle' });
    setError(null);
  };

  return (
    <div className="video-upload">
      {appState.status === 'idle' && (
        <div className="upload-area">
          <button type="button" className="upload-label" onClick={handleFileSelect}>
            <div className="upload-icon">🎬</div>
            <p>点击选择视频文件</p>
            <p className="upload-hint">支持 MP4、AVI、MOV 等格式</p>
          </button>
        </div>
      )}

      {appState.status === 'uploading' && (
        <LoadingSpinner message="正在加载视频..." />
      )}

      {(appState.status === 'ready' || appState.status === 'translating') && (
        <div className="video-controls">
          <div className="language-selectors">
            <div className="language-select">
              <label>源语言</label>
              <select
                value={sourceLanguage}
                onChange={(e) => setSourceLanguage(e.target.value)}
              >
                {SUPPORTED_LANGUAGES.map((lang) => (
                  <option key={lang.code} value={lang.code}>
                    {lang.name}
                  </option>
                ))}
              </select>
            </div>
            <span className="arrow">→</span>
            <div className="language-select">
              <label>目标语言</label>
              <select
                value={targetLanguage}
                onChange={(e) => setTargetLanguage(e.target.value)}
              >
                {SUPPORTED_LANGUAGES.map((lang) => (
                  <option key={lang.code} value={lang.code}>
                    {lang.name}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {appState.status === 'ready' && (
            <button className="btn btn-primary" onClick={handleTranslate}>
              开始翻译
            </button>
          )}

          {appState.status === 'translating' && (
            <LoadingSpinner message="正在翻译..." />
          )}
        </div>
      )}

      {appState.status === 'completed' && (
        <div className="completed-actions">
          <p className="success-text">✓ 翻译完成</p>
          <div className="action-buttons">
            <button className="btn btn-secondary" onClick={handleReset}>
              重新开始
            </button>
          </div>
        </div>
      )}

      {error && <ErrorMessage message={error} />}

      {appState.status === 'error' && (
        <div className="error-actions">
          <button className="btn btn-secondary" onClick={handleReset}>
            重试
          </button>
        </div>
      )}
    </div>
  );
}
