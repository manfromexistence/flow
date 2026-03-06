/// Audio capture and playback module
use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

pub struct AudioCapture {
    device: Device,
    config: StreamConfig,
}

impl AudioCapture {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;
        
        let config = device.default_input_config()?.into();
        
        Ok(Self { device, config })
    }

    /// Start capturing audio and send to channel
    pub fn start_capture(&self, tx: mpsc::UnboundedSender<Vec<f32>>) -> Result<Stream> {
        let buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = buffer.clone();
        
        let stream = self.device.build_input_stream(
            &self.config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buf = buffer_clone.lock().unwrap();
                buf.extend_from_slice(data);
                
                // Send chunks of 512 samples (32ms at 16kHz)
                if buf.len() >= 512 {
                    let chunk: Vec<f32> = buf.drain(..512).collect();
                    let _ = tx.send(chunk);
                }
            },
            |err| eprintln!("Audio capture error: {}", err),
            None,
        )?;
        
        stream.play()?;
        Ok(stream)
    }

    pub fn sample_rate(&self) -> u32 {
        self.config.sample_rate.0
    }
}

pub struct AudioPlayback {
    device: Device,
    config: StreamConfig,
}

impl AudioPlayback {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;
        
        let config = device.default_output_config()?.into();
        
        Ok(Self { device, config })
    }

    /// Play audio samples
    pub fn play(&self, audio: Vec<f32>) -> Result<()> {
        let audio = Arc::new(Mutex::new(audio));
        let audio_clone = audio.clone();
        let mut position = 0;
        
        let stream = self.device.build_output_stream(
            &self.config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let audio = audio_clone.lock().unwrap();
                for sample in data.iter_mut() {
                    *sample = if position < audio.len() {
                        let val = audio[position];
                        position += 1;
                        val
                    } else {
                        0.0
                    };
                }
            },
            |err| eprintln!("Audio playback error: {}", err),
            None,
        )?;
        
        stream.play()?;
        
        // Wait for playback to finish
        std::thread::sleep(std::time::Duration::from_secs_f32(
            audio.lock().unwrap().len() as f32 / self.config.sample_rate.0 as f32
        ));
        
        Ok(())
    }
}
