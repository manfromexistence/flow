//! Simplified font module for yazi integration
//! This is a stub that returns empty data - fonts are not embedded in yazi build

use std::io;

/// Stub function that returns an error - fonts not available in yazi build
pub fn read_font(_name: &str) -> io::Result<Vec<u8>> {
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Fonts not embedded in yazi build",
    ))
}
