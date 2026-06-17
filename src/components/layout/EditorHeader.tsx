import { useAppContext } from '../../context/AppContext';

export function EditorHeader() {
  const { setShowSettings } = useAppContext();

  return (
    <header className="editor-header">
      <div className="header-left">
        <div className="logo">
          <span className="logo-icon">🎬</span>
          <span className="logo-text">彩旗剪辑</span>
        </div>
      </div>

      <div className="header-center">
        <div className="playback-controls">
          <button className="playback-btn" title="播放/暂停">
            ▶
          </button>
          <span className="timecode">00:00:00:00</span>
        </div>
      </div>

      <div className="header-right">
        <button className="header-btn export-btn" title="导出">
          📤 导出
        </button>
        <button
          className="header-btn settings-btn"
          title="设置"
          onClick={() => setShowSettings(true)}
        >
          ⚙️ 设置
        </button>
      </div>
    </header>
  );
}
