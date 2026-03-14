/// Performance metrics and monitoring
use std::time::{Duration, Instant};
use sysinfo::System;

pub struct Metrics {
    system: System,
    start_time: Instant,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
            start_time: Instant::now(),
        }
    }

    pub fn start_timer(&self) -> Instant {
        Instant::now()
    }

    pub fn elapsed(&self, start: Instant) -> Duration {
        start.elapsed()
    }

    pub fn update(&mut self) {
        self.system.refresh_all();
    }

    pub fn get_stats(&mut self) -> SystemStats {
        self.update();
        
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let cpu_usage = self.system.global_cpu_usage();
        
        SystemStats {
            memory_used_mb: used_memory / 1024 / 1024,
            memory_total_mb: total_memory / 1024 / 1024,
            cpu_usage_percent: cpu_usage,
            uptime_secs: self.start_time.elapsed().as_secs(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemStats {
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub cpu_usage_percent: f32,
    pub uptime_secs: u64,
}

#[derive(Debug, Clone)]
pub struct PipelineMetrics {
    pub wake_word_detected_ms: u64,
    pub stt_duration_ms: u64,
    pub llm_duration_ms: u64,
    pub tts_duration_ms: u64,
    pub total_duration_ms: u64,
    pub audio_length_ms: u64,
}

impl PipelineMetrics {
    pub fn print_report(&self, stats: &SystemStats) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║           EDITH PERFORMANCE REPORT                     ║");
        println!("╠════════════════════════════════════════════════════════╣");
        println!("║ Pipeline Timing:                                       ║");
        println!("║   Wake Word Detection: {:>6} ms                       ║", self.wake_word_detected_ms);
        println!("║   Speech-to-Text:      {:>6} ms                       ║", self.stt_duration_ms);
        println!("║   LLM Processing:      {:>6} ms                       ║", self.llm_duration_ms);
        println!("║   Text-to-Speech:      {:>6} ms                       ║", self.tts_duration_ms);
        println!("║   ─────────────────────────────                        ║");
        println!("║   Total Response Time: {:>6} ms                       ║", self.total_duration_ms);
        println!("║                                                        ║");
        println!("║ Audio Stats:                                           ║");
        println!("║   Input Audio Length:  {:>6} ms                       ║", self.audio_length_ms);
        println!("║   Real-time Factor:    {:>6.2}x                       ║", 
                 self.audio_length_ms as f32 / self.total_duration_ms as f32);
        println!("║                                                        ║");
        println!("║ System Resources:                                      ║");
        println!("║   Memory Usage:        {:>4}/{:>4} MB ({:>3.1}%)        ║", 
                 stats.memory_used_mb, 
                 stats.memory_total_mb,
                 (stats.memory_used_mb as f32 / stats.memory_total_mb as f32) * 100.0);
        println!("║   CPU Usage:           {:>6.1}%                        ║", stats.cpu_usage_percent);
        println!("║   Uptime:              {:>6} seconds                  ║", stats.uptime_secs);
        println!("╚════════════════════════════════════════════════════════╝\n");
    }
}
