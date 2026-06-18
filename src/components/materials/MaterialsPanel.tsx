import { useAppContext } from '../../context/AppContext';
import type { MaterialTab } from '../../types';
import { VideoUpload } from './VideoUpload';
import { DocumentImport } from './DocumentImport';
import { FileText, Image, Music2, Video } from 'lucide-react';

const TABS: { key: MaterialTab; label: string; Icon: typeof Video }[] = [
  { key: 'video', label: '视频', Icon: Video },
  { key: 'document', label: '文档', Icon: FileText },
  { key: 'audio', label: '音频', Icon: Music2 },
  { key: 'image', label: '图片', Icon: Image },
];

export function MaterialsPanel() {
  const { selectedMaterialTab, setSelectedMaterialTab } = useAppContext();

  return (
    <div className="materials-panel">
      <div className="materials-tabs">
        {TABS.map(({ Icon, ...tab }) => (
          <button
            key={tab.key}
            className={`tab-btn ${selectedMaterialTab === tab.key ? 'active' : ''}`}
            onClick={() => setSelectedMaterialTab(tab.key)}
          >
            <span className="tab-icon"><Icon size={18} strokeWidth={2.2} /></span>
            <span className="tab-label">{tab.label}</span>
          </button>
        ))}
      </div>

      <div className="materials-content">
        {selectedMaterialTab === 'video' && <VideoUpload />}
        {selectedMaterialTab === 'document' && <DocumentImport />}
        {selectedMaterialTab === 'audio' && (
          <div className="empty-materials">
            <Music2 className="empty-materials-icon" size={42} strokeWidth={1.8} />
            <p>音频素材</p>
          </div>
        )}
        {selectedMaterialTab === 'image' && (
          <div className="empty-materials">
            <Image className="empty-materials-icon" size={42} strokeWidth={1.8} />
            <p>图片素材</p>
          </div>
        )}
      </div>
    </div>
  );
}
