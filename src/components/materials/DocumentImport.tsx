import { useState } from 'react';
import { open } from '@tauri-apps/plugin-dialog';
import { useAppContext } from '../../context/AppContext';
import { tauriApi } from '../../api/tauri';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';

export function DocumentImport() {
  const { setAppState } = useAppContext();
  const [isImporting, setIsImporting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleFileSelect = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: 'Document',
          extensions: ['pptx', 'ppt', 'md', 'markdown', 'pdf', 'docx', 'doc'],
        },
      ],
    });

    if (typeof selected !== 'string') return;

    setIsImporting(true);
    setError(null);
    setAppState({ status: 'documentImporting' });

    try {
      const content = await tauriApi.importDocument(selected);
      setAppState({ status: 'documentReady', content });
    } catch (err) {
      const message = err instanceof Error ? err.message : '导入失败';
      setError(message);
      setAppState({ status: 'error', message });
    } finally {
      setIsImporting(false);
    }
  };

  return (
    <div className="document-import">
      <div className="upload-area">
        <button type="button" className="upload-label" onClick={handleFileSelect}>
          <div className="upload-icon">📄</div>
          <p>点击选择文档文件</p>
          <p className="upload-hint">支持 PPTX、Markdown、PDF 格式</p>
        </button>
      </div>

      {isImporting && <LoadingSpinner message="正在导入文档..." />}
      {error && <ErrorMessage message={error} />}
    </div>
  );
}
