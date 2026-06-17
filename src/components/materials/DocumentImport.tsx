import { useState, useRef } from 'react';
import { useAppContext } from '../../context/AppContext';
import { tauriApi } from '../../api/tauri';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';

export function DocumentImport() {
  const { setAppState } = useAppContext();
  const [isImporting, setIsImporting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileSelect = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setIsImporting(true);
    setError(null);
    setAppState({ status: 'documentImporting' });

    try {
      const content = await tauriApi.importDocument(file.name);
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
        <input
          ref={fileInputRef}
          type="file"
          accept=".pptx,.ppt,.md,.markdown,.pdf"
          onChange={handleFileSelect}
          className="file-input"
          id="document-input"
        />
        <label htmlFor="document-input" className="upload-label">
          <div className="upload-icon">📄</div>
          <p>点击选择文档文件</p>
          <p className="upload-hint">支持 PPTX、Markdown、PDF 格式</p>
        </label>
      </div>

      {isImporting && <LoadingSpinner message="正在导入文档..." />}
      {error && <ErrorMessage message={error} />}
    </div>
  );
}
