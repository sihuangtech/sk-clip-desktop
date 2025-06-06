/// 语音合成配置结构体
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TtsConfig {
    /// 语言代码
    pub language: String,
    /// 音色/说话人ID
    pub voice_id: String,
    /// 语速 (0.5 - 2.0)
    pub speed: f32,
    /// 音调 (-20.0 - 20.0)
    pub pitch: f32,
    /// 音量 (0.0 - 1.0)
    pub volume: f32,
}

impl Default for TtsConfig {
    fn default() -> Self {
        Self {
            language: "zh".to_string(),
            voice_id: "default".to_string(),
            speed: 1.0,
            pitch: 0.0,
            volume: 0.8,
        }
    }
}

impl TtsConfig {
    /// 创建新的TTS配置
    pub fn new(language: &str, voice_id: &str) -> Self {
        Self {
            language: language.to_string(),
            voice_id: voice_id.to_string(),
            ..Default::default()
        }
    }

    /// 设置语速
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed.clamp(0.5, 2.0);
        self
    }

    /// 设置音调
    pub fn with_pitch(mut self, pitch: f32) -> Self {
        self.pitch = pitch.clamp(-20.0, 20.0);
        self
    }

    /// 设置音量
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 1.0);
        self
    }

    /// 验证配置参数
    pub fn validate(&self) -> Result<(), String> {
        if self.speed < 0.5 || self.speed > 2.0 {
            return Err("语速必须在0.5-2.0之间".to_string());
        }
        
        if self.pitch < -20.0 || self.pitch > 20.0 {
            return Err("音调必须在-20.0-20.0之间".to_string());
        }
        
        if self.volume < 0.0 || self.volume > 1.0 {
            return Err("音量必须在0.0-1.0之间".to_string());
        }
        
        Ok(())
    }
} 