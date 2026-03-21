//! Font Module
//!
//! This module provides access to figlet fonts for ASCII art text rendering.
//! The fonts are embedded directly in the binary as compressed data.

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io;

// Include the compressed fonts generated at build time
include!(concat!(env!("OUT_DIR"), "/fonts_compressed.rs"));

/// Lists all available figlet font names.
#[allow(dead_code)]
pub fn list_fonts() -> io::Result<Vec<String>> {
    let mut fonts: Vec<String> = FONTS.keys().map(|&s| s.to_string()).collect();
    fonts.sort();
    Ok(fonts)
}

/// Reads and decompresses a font file.
pub fn read_font(name: &str) -> io::Result<Vec<u8>> {
    FONTS
        .get(name)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Font '{}' not found", name),
            )
        })
        .and_then(|compressed| {
            zstd::decode_all(*compressed).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        })
}

/// Returns the total number of available fonts.
#[allow(dead_code)]
pub fn font_count() -> io::Result<usize> {
    Ok(FONTS.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_fonts_not_empty() {
        let fonts = list_fonts().expect("Should be able to list fonts");
        assert!(!fonts.is_empty(), "Should have at least one font");
    }

    #[test]
    fn test_font_count() {
        let count = font_count().expect("Should be able to count fonts");
        assert!(
            count >= 100,
            "Should have at least 100 fonts, got {}",
            count
        );
    }

    #[test]
    fn test_read_font() {
        let fonts = list_fonts().expect("Should be able to list fonts");
        if let Some(first_font) = fonts.first() {
            let content = read_font(first_font);
            assert!(
                content.is_ok(),
                "Should be able to read font '{}'",
                first_font
            );
            assert!(
                !content.unwrap().is_empty(),
                "Font content should not be empty"
            );
        }
    }
}
