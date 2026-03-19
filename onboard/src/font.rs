//! Font Module for Onboard
//!
//! This module provides access to figlet fonts for ASCII art text rendering.
//! The fonts are embedded directly in the project for better distribution.

use std::fs;
use std::io;
use std::path::PathBuf;

/// Returns the path to the figlet fonts directory.
pub fn fonts_dir() -> PathBuf {
    PathBuf::from("figlet")
}

/// Lists all available figlet font names.
pub fn list_fonts() -> io::Result<Vec<String>> {
    let dir = fonts_dir();
    let mut fonts = Vec::new();

    if dir.exists() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "dx")
                && let Some(stem) = path.file_stem()
                && let Some(name) = stem.to_str()
            {
                fonts.push(name.to_string());
            }
        }
    }

    fonts.sort();
    Ok(fonts)
}

/// Returns the path to a specific font file.
pub fn font_path(name: &str) -> Option<PathBuf> {
    let path = fonts_dir().join(format!("{}.dx", name));
    if path.exists() { Some(path) } else { None }
}

/// Reads the content of a font file.
pub fn read_font(name: &str) -> io::Result<Vec<u8>> {
    let path = font_path(name).ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, format!("Font '{}' not found", name))
    })?;
    fs::read(path)
}

/// Returns the total number of available fonts.
pub fn font_count() -> io::Result<usize> {
    list_fonts().map(|fonts| fonts.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fonts_dir_exists() {
        let dir = fonts_dir();
        assert!(dir.exists(), "Figlet fonts directory should exist at {:?}", dir);
    }

    #[test]
    fn test_list_fonts_not_empty() {
        let fonts = list_fonts().expect("Should be able to list fonts");
        assert!(!fonts.is_empty(), "Should have at least one font");
    }

    #[test]
    fn test_font_count() {
        let count = font_count().expect("Should be able to count fonts");
        assert!(count >= 400, "Should have at least 400 fonts, got {}", count);
    }
}