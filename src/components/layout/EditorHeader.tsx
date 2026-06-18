import { useAppContext } from '../../context/AppContext';
import { Clapperboard, Download, Play, Settings } from 'lucide-react';

export function EditorHeader() {
  const { setShowSettings } = useAppContext();

  return (
    <header className="editor-header">
      <div className="header-left">
        <div className="logo">
          <span className="logo-icon"><Clapperboard size={18} strokeWidth={2.2} /></span>
          <span className="logo-text">彩旗剪辑</span>
        </div>
      </div>

      <div className="header-center">
        <div className="playback-controls">
          <button className="playback-btn" title="播放/暂停">
            <Play size={15} fill="currentColor" strokeWidth={2.4} />
          </button>
          <span className="timecode">00:00:00:00</span>
        </div>
      </div>

      <div className="header-right">
        <button className="header-btn export-btn" title="导出">
          <Download size={15} />
          <span>导出</span>
        </button>
        <button
          className="header-btn settings-btn"
          title="设置"
          onClick={() => setShowSettings(true)}
        >
          <Settings size={15} />
          <span>设置</span>
        </button>
      </div>
    </header>
  );
}
