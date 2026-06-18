import { useState } from 'react';
import { save } from '@tauri-apps/plugin-dialog';
import { useAppContext } from '../../context/AppContext';
import { SUPPORTED_LANGUAGES, VOICE_OPTIONS, type VoiceType, type TranslationTab } from '../../types';
import { tauriApi } from '../../api/tauri';
import { LoadingSpinner } from '../common/LoadingSpinner';

export function TranslationPanel() {
  const { appState, sourceLanguage, setSourceLanguage, targetLanguage, setTargetLanguage } = useAppContext();
  const [activeTab, setActiveTab] = useState<TranslationTab>('translate');
  const [sourceText, setSourceText] = useState('');
  const [translatedText, setTranslatedText] = useState('');
  const [isTranslating, setIsTranslating] = useState(false);
  const [isSynthesizing, setIsSynthesizing] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [voiceType, setVoiceType] = useState<VoiceType>('female');
  const [speechSpeed, setSpeechSpeed] = useState(1.0);

  const handleTranslate = async () => {
    if (!sourceText.trim()) return;
    setIsTranslating(true);
    setError(null);
    try {
      const result = await tauriApi.translateText(sourceText, sourceLanguage, targetLanguage);
      setTranslatedText(result.translated_text);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsTranslating(false);
    }
  };

  const handleSynthesize = async () => {
    if (!translatedText.trim()) return;
    const outputPath = await save({
      defaultPath: 'speech.wav',
      filters: [{ name: 'Audio', extensions: ['wav'] }],
    });
    if (!outputPath) return;

    setIsSynthesizing(true);
    setError(null);
    try {
      await tauriApi.synthesizeSpeech(translatedText, outputPath, {
        voiceType,
        speed: speechSpeed,
      });
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setIsSynthesizing(false);
    }
  };

  return (
    <div className="translation-panel">
      <div className="panel-tabs">
        <button
          className={`tab-btn ${activeTab === 'translate' ? 'active' : ''}`}
          onClick={() => setActiveTab('translate')}
        >
          翻译
        </button>
        <button
          className={`tab-btn ${activeTab === 'synthesis' ? 'active' : ''}`}
          onClick={() => setActiveTab('synthesis')}
        >
          合成
        </button>
      </div>

      {activeTab === 'translate' && (
        <div className="translate-tab">
          {error && <div className="error-message"><span>{error}</span></div>}

          <div className="language-selectors">
            <select value={sourceLanguage} onChange={(e) => setSourceLanguage(e.target.value)}>
              {SUPPORTED_LANGUAGES.map((lang) => (
                <option key={lang.code} value={lang.code}>{lang.name}</option>
              ))}
            </select>
            <span className="arrow">→</span>
            <select value={targetLanguage} onChange={(e) => setTargetLanguage(e.target.value)}>
              {SUPPORTED_LANGUAGES.map((lang) => (
                <option key={lang.code} value={lang.code}>{lang.name}</option>
              ))}
            </select>
          </div>

          <textarea
            className="source-text"
            placeholder="输入要翻译的文本..."
            value={sourceText}
            onChange={(e) => setSourceText(e.target.value)}
          />

          <button
            className="btn btn-primary"
            onClick={handleTranslate}
            disabled={isTranslating || !sourceText.trim()}
          >
            {isTranslating ? '翻译中...' : '翻译'}
          </button>

          {isTranslating && <LoadingSpinner message="正在翻译..." />}

          {translatedText && (
            <div className="translated-result">
              <label>译文</label>
              <textarea
                className="translated-text"
                value={translatedText}
                readOnly
              />
            </div>
          )}
        </div>
      )}

      {activeTab === 'synthesis' && (
        <div className="synthesis-tab">
          {error && <div className="error-message"><span>{error}</span></div>}

          <div className="voice-options">
            <label>音色选择</label>
            <div className="voice-grid">
              {VOICE_OPTIONS.map((option) => (
                <button
                  key={option.type}
                  className={`voice-btn ${voiceType === option.type ? 'active' : ''}`}
                  onClick={() => setVoiceType(option.type)}
                >
                  <span className="voice-name">{option.label}</span>
                  <span className="voice-desc">{option.description}</span>
                </button>
              ))}
            </div>
          </div>

          <div className="speed-control">
            <label>语速: {speechSpeed.toFixed(1)}x</label>
            <input
              type="range"
              min="0.5"
              max="2.0"
              step="0.1"
              value={speechSpeed}
              onChange={(e) => setSpeechSpeed(parseFloat(e.target.value))}
            />
          </div>

          {translatedText && (
            <div className="synthesis-preview">
              <label>合成文本预览</label>
              <div className="preview-text">{translatedText}</div>
            </div>
          )}

          <button
            className="btn btn-primary"
            onClick={handleSynthesize}
            disabled={isSynthesizing || !translatedText.trim()}
          >
            {isSynthesizing ? '生成中...' : '生成语音'}
          </button>

          {isSynthesizing && <LoadingSpinner message="正在生成语音..." />}
        </div>
      )}

      {appState.status === 'ready' && (
        <div className="video-translate-section">
          <h4>视频翻译</h4>
          <p className="hint">将视频中的语音翻译为目标语言</p>
        </div>
      )}
    </div>
  );
}
