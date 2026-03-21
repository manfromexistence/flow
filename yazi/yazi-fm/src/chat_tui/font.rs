//! Font module for yazi integration
//! Fonts are embedded at build time from figlet directory

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io;

// Include the compressed fonts generated at build time
include!(concat!(env!("OUT_DIR"), "/fonts_compressed.rs"));

// Parse the binary font data into a HashMap
static FONTS: Lazy<HashMap<String, &'static [u8]>> = Lazy::new(|| {
    let mut fonts = HashMap::new();
    let mut offset = 0;
    
    if FONTS_DATA.len() < 4 {
        return fonts;
    }
    
    let count = u32::from_le_bytes([
        FONTS_DATA[0],
        FONTS_DATA[1],
        FONTS_DATA[2],
        FONTS_DATA[3],
    ]) as usize;
    offset += 4;
    
    for _ in 0..count {
        if offset + 4 > FONTS_DATA.len() {
            break;
        }
        
        let name_len = u32::from_le_bytes([
            FONTS_DATA[offset],
            FONTS_DATA[offset + 1],
            FONTS_DATA[offset + 2],
            FONTS_DATA[offset + 3],
        ]) as usize;
        offset += 4;
        
        if offset + name_len > FONTS_DATA.len() {
            break;
        }
        
        let name = String::from_utf8_lossy(&FONTS_DATA[offset..offset + name_len]).to_string();
        offset += name_len;
        
        if offset + 4 > FONTS_DATA.len() {
            break;
        }
        
        let data_len = u32::from_le_bytes([
            FONTS_DATA[offset],
            FONTS_DATA[offset + 1],
            FONTS_DATA[offset + 2],
            FONTS_DATA[offset + 3],
        ]) as usize;
        offset += 4;
        
        if offset + data_len > FONTS_DATA.len() {
            break;
        }
        
        let data = &FONTS_DATA[offset..offset + data_len];
        offset += data_len;
        
        fonts.insert(name, data);
    }
    
    fonts
});

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

/// Lists all available figlet font names.
#[allow(dead_code)]
pub fn list_fonts() -> io::Result<Vec<String>> {
    let mut fonts: Vec<String> = FONTS.keys().cloned().collect();
    fonts.sort();
    Ok(fonts)
}

/// Returns the total number of available fonts.
#[allow(dead_code)]
pub fn font_count() -> io::Result<usize> {
    Ok(FONTS.len())
}
