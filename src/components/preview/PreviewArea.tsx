import { useAppContext } from '../../context/AppContext';
import { convertFileSrc } from '@tauri-apps/api/core';

export function PreviewArea() {
  const { appState } = useAppContext();

  return (
    <div className="preview-area">
      {appState.status === 'idle' && (
        <div className="preview-placeholder">
          <div className="placeholder-icon">🎥</div>
          <p>选择视频文件开始创作</p>
        </div>
      )}

      {appState.status === 'uploading' && (
        <div className="preview-placeholder">
          <div className="placeholder-icon">⏳</div>
          <p>正在加载视频...</p>
        </div>
      )}

      {appState.status === 'ready' && (
        <div className="video-preview">
          <div className="video-container">
            <video
              src={convertFileSrc(appState.videoPath)}
              controls
              className="preview-video"
            />
          </div>
        </div>
      )}

      {appState.status === 'translating' && (
        <div className="preview-placeholder">
          <div className="placeholder-icon">⏳</div>
          <p>正在翻译视频...</p>
        </div>
      )}

      {appState.status === 'completed' && (
        <div className="preview-placeholder">
          <div className="placeholder-icon">✅</div>
          <p>翻译完成</p>
        </div>
      )}

      {appState.status === 'documentReady' && (
        <div className="document-preview">
          <h3>{appState.content.title ?? '未命名文档'}</h3>
          <div className="document-info">
            <span>类型: {appState.content.document_type}</span>
            <span>页数: {appState.content.metadata.page_count}</span>
          </div>
          {appState.content.pages.slice(0, 3).map((page) => (
            <div key={page.page_number} className="document-page-preview">
              <h4>第 {page.page_number} 页</h4>
              <p>{page.text.substring(0, 200)}...</p>
            </div>
          ))}
        </div>
      )}

      {appState.status === 'error' && (
        <div className="preview-placeholder error">
          <div className="placeholder-icon">❌</div>
          <p>{appState.message}</p>
        </div>
      )}
    </div>
  );
}
