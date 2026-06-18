import { useState, useEffect } from 'react';
import { useAppContext } from '../../context/AppContext';
import { tauriApi } from '../../api/tauri';
import type { AppConfig } from '../../types';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { SuccessMessage } from '../common/SuccessMessage';

export function SettingsPanel() {
  const { showSettings, setShowSettings } = useAppContext();
  const [config, setConfig] = useState<AppConfig | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  useEffect(() => {
    if (showSettings) {
      loadConfig();
    }
  }, [showSettings]);

  const loadConfig = async () => {
    setLoading(true);
    setError(null);
    try {
      const appConfig = await tauriApi.getAppConfig();
      setConfig(appConfig);
    } catch (err) {
      setError('加载配置失败');
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    if (!config) return;
    setSaving(true);
    setError(null);
    setSuccess(null);
    try {
      await tauriApi.updateAppConfig(config);
      setSuccess('配置已保存');
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      setError('保存配置失败');
    } finally {
      setSaving(false);
    }
  };

  if (!showSettings) return null;

  return (
    <div className="settings-overlay">
      <div className="settings-panel">
        <div className="settings-header">
          <h2>设置</h2>
          <button className="close-btn" onClick={() => setShowSettings(false)}>
            ✕
          </button>
        </div>

        <div className="settings-body">
          {loading && <LoadingSpinner message="加载配置..." />}
          {error && <ErrorMessage message={error} />}
          {success && <SuccessMessage message={success} />}

          {config && (
            <>
              <div className="settings-section">
                <h3>AI 模型设置</h3>
                <div className="setting-item">
                  <label>默认源语言</label>
                  <select
                    value={config.ai.default_language}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        ai: { ...config.ai, default_language: e.target.value },
                      })
                    }
                  >
                    <option value="zh">中文</option>
                    <option value="en">English</option>
                    <option value="ja">日本語</option>
                  </select>
                </div>

                <div className="setting-item">
                  <label>TTS 引擎</label>
                  <select
                    value={config.ai.selected_tts_engine}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        ai: { ...config.ai, selected_tts_engine: e.target.value },
                      })
                    }
                  >
                    {config.ai.available_tts_engines.map((engine) => (
                      <option key={engine.id} value={engine.id}>
                        {engine.name} ({engine.category})
                      </option>
                    ))}
                  </select>
                </div>

                {config.ai.available_tts_engines.length > 0 && (
                  <div className="engine-info">
                    {(() => {
                      const engine = config.ai.available_tts_engines.find(
                        (e) => e.id === config.ai.selected_tts_engine
                      );
                      if (!engine) return null;
                      return (
                        <>
                          <p><strong>类型:</strong> {engine.category}</p>
                          <p><strong>体积:</strong> {engine.footprint}</p>
                          <p><strong>许可证:</strong> {engine.license}</p>
                          <p><strong>适用场景:</strong> {engine.recommended_use}</p>
                        </>
                      );
                    })()}
                  </div>
                )}
              </div>

              <div className="settings-section">
                <h3>视频处理设置</h3>
                <div className="setting-item">
                  <label>默认输出格式</label>
                  <select
                    value={config.video.default_output_format}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        video: { ...config.video, default_output_format: e.target.value },
                      })
                    }
                  >
                    <option value="mp4">MP4</option>
                    <option value="avi">AVI</option>
                    <option value="mov">MOV</option>
                    <option value="mkv">MKV</option>
                  </select>
                </div>

                <div className="setting-item">
                  <label>默认视频质量</label>
                  <select
                    value={config.video.default_quality}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        video: { ...config.video, default_quality: e.target.value },
                      })
                    }
                  >
                    <option value="low">低</option>
                    <option value="medium">中</option>
                    <option value="high">高</option>
                    <option value="ultra">超高</option>
                  </select>
                </div>
              </div>

              <div className="settings-section">
                <h3>文档处理设置</h3>
                <div className="setting-item">
                  <label>PDF DPI</label>
                  <input
                    type="number"
                    min="72"
                    max="600"
                    value={config.document.pdf_dpi}
                    onChange={(e) =>
                      setConfig({
                        ...config,
                        document: { ...config.document, pdf_dpi: parseInt(e.target.value) },
                      })
                    }
                  />
                </div>
              </div>
            </>
          )}
        </div>

        <div className="settings-footer">
          <button className="btn btn-secondary" onClick={() => setShowSettings(false)}>
            取消
          </button>
          <button className="btn btn-primary" onClick={handleSave} disabled={saving}>
            {saving ? '保存中...' : '保存'}
          </button>
        </div>
      </div>
    </div>
  );
}
