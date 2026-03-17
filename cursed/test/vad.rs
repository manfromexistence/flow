/// Voice Activity Detection module
/// Detects when user is speaking vs silence
pub struct VAD {
    // Simple energy-based VAD for now
    threshold: f32,
    silence_duration_ms: u64,
}

impl VAD {
    pub fn new(threshold: f32, silence_duration_ms: u64) -> Self {
        Self {
            threshold,
            silence_duration_ms,
        }
    }

    /// Check if audio frame contains speech
    pub fn is_speech(&self, audio: &[f32]) -> bool {
        let energy: f32 = audio.iter().map(|x| x * x).sum::<f32>() / audio.len() as f32;
        energy > self.threshold
    }

    /// Get silence duration threshold in ms
    pub fn silence_duration_ms(&self) -> u64 {
        self.silence_duration_ms
    }
}
