import { useAppContext } from '../../context/AppContext';
import type { MaterialTab } from '../../types';
import { VideoUpload } from './VideoUpload';
import { DocumentImport } from './DocumentImport';

const TABS: { key: MaterialTab; label: string; icon: string }[] = [
  { key: 'video', label: '视频', icon: '🎬' },
  { key: 'document', label: '文档', icon: '📄' },
  { key: 'audio', label: '音频', icon: '🎵' },
  { key: 'image', label: '图片', icon: '🖼️' },
];

export function MaterialsPanel() {
  const { selectedMaterialTab, setSelectedMaterialTab } = useAppContext();

  return (
    <div className="materials-panel">
      <div className="materials-tabs">
        {TABS.map((tab) => (
          <button
            key={tab.key}
            className={`tab-btn ${selectedMaterialTab === tab.key ? 'active' : ''}`}
            onClick={() => setSelectedMaterialTab(tab.key)}
          >
            <span className="tab-icon">{tab.icon}</span>
            <span className="tab-label">{tab.label}</span>
          </button>
        ))}
      </div>

      <div className="materials-content">
        {selectedMaterialTab === 'video' && <VideoUpload />}
        {selectedMaterialTab === 'document' && <DocumentImport />}
        {selectedMaterialTab === 'audio' && (
          <div className="empty-materials">
            <p>🎵</p>
            <p>音频素材</p>
          </div>
        )}
        {selectedMaterialTab === 'image' && (
          <div className="empty-materials">
            <p>🖼️</p>
            <p>图片素材</p>
          </div>
        )}
      </div>
    </div>
  );
}
