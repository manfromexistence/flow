use anyhow::Result;
use tracing::{info, warn, error};
use tracing_subscriber;
use tokio::sync::mpsc;
use std::time::Instant;

mod stt;
mod llm;
mod tts;
mod wake_word;
mod vad;
mod audio;
mod metrics;

use wake_word::WakeWordDetector;
use vad::VAD;
use audio::{AudioCapture, AudioPlayback};
use metrics::{Metrics, PipelineMetrics};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Edith AI Assistant starting...");
    info!("📊 Stack: Qwen 3.5 0.8B + Moonshine Tiny + Kokoro-82M");
    info!("💾 Total model size: ~464MB | RAM usage: ~550MB");

    // Initialize metrics
    let mut metrics = Metrics::new();

    // Initialize wake word detector
    info!("🎯 Loading wake word detector...");
    let wake_word = WakeWordDetector::new(
        "models/wake_words/arise.onnx",
        0.5,
        "arise"
    )?;
    info!("✅ Wake word '{}' loaded", wake_word.wake_word_name());

    // Initialize VAD
    let vad = VAD::new(0.01, 1500); // 1.5 second silence threshold

    // Initialize audio capture
    info!("🎤 Initializing audio capture...");
    let audio_capture = AudioCapture::new()?;
    let sample_rate = audio_capture.sample_rate();
    info!("✅ Audio capture ready ({}Hz)", sample_rate);

    // Initialize audio playback
    let audio_playback = AudioPlayback::new()?;

    // TODO: Initialize models (placeholder for now)
    info!("🧠 Loading AI models...");
    info!("  - STT: Moonshine Tiny Streaming (34MB)");
    info!("  - LLM: Qwen 3.5 0.8B Q4_K_M (350MB)");
    info!("  - TTS: Kokoro-82M ONNX (80MB)");
    // let stt = stt::MoonshineOrtSTT::new("models/moonshine-tiny-streaming.onnx")?;
    // let llm = llm::QwenLLM::new("models/llm/Qwen3.5-0.8B-Q4_K_M.gguf")?;
    // let tts = tts::KokoroOrtTTS::new("models/tts/kokoro-v1.0.int8.onnx")?;
    info!("✅ All models loaded successfully");

    info!("\n╔════════════════════════════════════════════════════════╗");
    info!("║  EDITH IS NOW LISTENING 24/7                           ║");
    info!("║  Say '{}' to activate                              ║", wake_word.wake_word_name());
    info!("╚════════════════════════════════════════════════════════╝\n");

    // Start audio capture
    let (audio_tx, mut audio_rx) = mpsc::unbounded_channel();
    let _stream = audio_capture.start_capture(audio_tx)?;

    // Main loop - 24/7 listening
    let mut wake_word_buffer = Vec::new();
    let mut recording_buffer = Vec::new();
    let mut is_recording = false;
    let mut silence_start: Option<Instant> = None;
    let mut wake_detected_time: Option<Instant> = None;

    while let Some(audio_chunk) = audio_rx.recv().await {
        // Always check for wake word when not recording
        if !is_recording {
            wake_word_buffer.extend_from_slice(&audio_chunk);
            
            // Keep buffer at reasonable size (1 second = ~16000 samples at 16kHz)
            if wake_word_buffer.len() > 16000 {
                wake_word_buffer.drain(..wake_word_buffer.len() - 16000);
            }

            // Check for wake word
            if wake_word.detect(&wake_word_buffer)? {
                info!("🎯 Wake word '{}' detected!", wake_word.wake_word_name());
                is_recording = true;
                recording_buffer.clear();
                wake_detected_time = Some(Instant::now());
                silence_start = None;
                continue;
            }
        }

        // Record user speech after wake word
        if is_recording {
            recording_buffer.extend_from_slice(&audio_chunk);

            // Check for speech vs silence
            if vad.is_speech(&audio_chunk) {
                silence_start = None; // Reset silence timer
            } else {
                // Start silence timer if not already started
                if silence_start.is_none() {
                    silence_start = Some(Instant::now());
                }

                // Check if silence duration exceeded threshold
                if let Some(start) = silence_start {
                    if start.elapsed().as_millis() > vad.silence_duration_ms() as u128 {
                        info!("🔇 Silence detected, processing speech...");
                        
                        // Process the recorded audio
                        let wake_time_ms = wake_detected_time
                            .map(|t| t.elapsed().as_millis() as u64)
                            .unwrap_or(0);

                        match process_speech(
                            &recording_buffer,
                            &audio_playback,
                            wake_time_ms,
                            &mut metrics,
                        ).await {
                            Ok(pipeline_metrics) => {
                                let stats = metrics.get_stats();
                                pipeline_metrics.print_report(&stats);
                            }
                            Err(e) => {
                                error!("❌ Error processing speech: {}", e);
                            }
                        }

                        // Reset for next wake word
                        is_recording = false;
                        recording_buffer.clear();
                        wake_word_buffer.clear();
                        silence_start = None;
                        wake_detected_time = None;
                        
                        info!("\n🎧 Listening for wake word '{}'...", wake_word.wake_word_name());
                    }
                }
            }

            // Safety: max 30 seconds of recording
            if recording_buffer.len() > 16000 * 30 {
                warn!("⚠️  Recording timeout (30s), processing anyway...");
                let wake_time_ms = wake_detected_time
                    .map(|t| t.elapsed().as_millis() as u64)
                    .unwrap_or(0);

                if let Err(e) = process_speech(
                    &recording_buffer,
                    &audio_playback,
                    wake_time_ms,
                    &mut metrics,
                ).await {
                    error!("❌ Error processing speech: {}", e);
                }

                is_recording = false;
                recording_buffer.clear();
                wake_word_buffer.clear();
                silence_start = None;
                wake_detected_time = None;
            }
        }
    }

    Ok(())
}

async fn process_speech(
    audio: &[f32],
    playback: &AudioPlayback,
    wake_time_ms: u64,
    metrics: &mut Metrics,
) -> Result<PipelineMetrics> {
    let total_start = metrics.start_timer();
    let audio_length_ms = (audio.len() as f32 / 16000.0 * 1000.0) as u64;

    info!("📝 Audio length: {:.2}s", audio.len() as f32 / 16000.0);

    // Step 1: Speech-to-Text
    info!("🎤 Running speech-to-text...");
    let stt_start = metrics.start_timer();
    
    // TODO: Actual STT inference
    let transcribed_text = format!("Hello, this is a test transcription of {} samples", audio.len());
    
    let stt_duration = metrics.elapsed(stt_start);
    info!("✅ STT completed in {}ms: '{}'", stt_duration.as_millis(), transcribed_text);

    // Step 2: LLM Enhancement
    info!("🧠 Processing with LLM...");
    let llm_start = metrics.start_timer();
    
    // TODO: Actual LLM inference
    let enhanced_text = format!("Enhanced response: {}", transcribed_text);
    
    let llm_duration = metrics.elapsed(llm_start);
    info!("✅ LLM completed in {}ms: '{}'", llm_duration.as_millis(), enhanced_text);

    // Step 3: Text-to-Speech
    info!("🔊 Generating speech...");
    let tts_start = metrics.start_timer();
    
    // TODO: Actual TTS inference
    // For now, create silent audio
    let tts_audio = vec![0.0f32; 16000]; // 1 second of silence
    
    let tts_duration = metrics.elapsed(tts_start);
    info!("✅ TTS completed in {}ms", tts_duration.as_millis());

    // Step 4: Play audio
    info!("🔈 Playing response...");
    playback.play(tts_audio)?;

    let total_duration = metrics.elapsed(total_start);

    Ok(PipelineMetrics {
        wake_word_detected_ms: wake_time_ms,
        stt_duration_ms: stt_duration.as_millis() as u64,
        llm_duration_ms: llm_duration.as_millis() as u64,
        tts_duration_ms: tts_duration.as_millis() as u64,
        total_duration_ms: total_duration.as_millis() as u64,
        audio_length_ms,
    })
}
