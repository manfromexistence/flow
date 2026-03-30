use anyhow::Result;
use hound::WavReader;
use std::path::Path;

/// Audio file loader and processor
pub struct AudioLoader;

impl AudioLoader {
    /// Load audio file and convert to 16kHz mono f32 samples
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Vec<f32>> {
        let reader = WavReader::open(path)?;
        let _spec = reader.spec();
        
        let samples: Vec<f32> = reader
            .into_samples::<i16>()
            .map(|s| s.map(|s| s as f32 / i16::MAX as f32))
            .collect::<Result<Vec<_>, _>>()?;
        
        // TODO: Resample to 16kHz if needed
        // TODO: Convert stereo to mono if needed
        
        Ok(samples)
    }
}
