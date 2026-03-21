use std::env;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=../../figlet");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("fonts_compressed.rs");
    let bin_path = Path::new(&out_dir).join("fonts.bin");
    
    let mut bin_output = BufWriter::new(File::create(&bin_path).unwrap());
    let mut rs_output = BufWriter::new(File::create(&dest_path).unwrap());

    let figlet_dir = Path::new("../../figlet");
    if !figlet_dir.exists() {
        println!("cargo:warning=figlet directory not found, skipping font compression");
        writeln!(rs_output, "pub const FONTS_DATA: &[u8] = &[];").unwrap();
        return;
    }

    let mut entries: Vec<_> = fs::read_dir(figlet_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "dx")
                .unwrap_or(false)
        })
        .collect();

    entries.sort_by_key(|e| e.path());

    let count = entries.len() as u32;
    bin_output.write_all(&count.to_le_bytes()).unwrap();

    for entry in entries {
        let path = entry.path();
        let name = path.file_stem().unwrap().to_str().unwrap();
        let content = fs::read(&path).unwrap();

        let compressed = zstd::encode_all(&content[..], 3).unwrap();

        let name_bytes = name.as_bytes();
        bin_output
            .write_all(&(name_bytes.len() as u32).to_le_bytes())
            .unwrap();
        bin_output.write_all(name_bytes).unwrap();

        bin_output
            .write_all(&(compressed.len() as u32).to_le_bytes())
            .unwrap();
        bin_output.write_all(&compressed).unwrap();
    }

    drop(bin_output);
    let bin_data = fs::read(&bin_path).unwrap();
    
    writeln!(rs_output, "pub const FONTS_DATA: &[u8] = &[").unwrap();
    for chunk in bin_data.chunks(16) {
        write!(rs_output, "    ").unwrap();
        for byte in chunk {
            write!(rs_output, "{:#04x}, ", byte).unwrap();
        }
        writeln!(rs_output).unwrap();
    }
    writeln!(rs_output, "];").unwrap();
}
