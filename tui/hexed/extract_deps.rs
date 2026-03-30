#!/usr/bin/env rust-script
//! Extract all workspace dependencies from Cargo.toml files
//! 
//! ```cargo
//! [dependencies]
//! toml = "0.8"
//! walkdir = "2.5"
//! ```

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let mut all_deps = BTreeSet::new();
    
    // Walk through src/file_browser directory
    for entry in WalkDir::new("src/file_browser")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        // Only process Cargo.toml files
        if path.file_name().and_then(|n| n.to_str()) == Some("Cargo.toml") {
            println!("Processing: {}", path.display());
            
            if let Ok(content) = fs::read_to_string(path) {
                extract_workspace_deps(&content, &mut all_deps);
            }
        }
    }
    
    // Also check old yazi-* directories if they still exist
    for entry in WalkDir::new(".")
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        
        if path.file_name().and_then(|n| n.to_str()) == Some("Cargo.toml") {
            if path.starts_with("./yazi-") || path.starts_with(".\\yazi-") {
                println!("Processing old: {}", path.display());
                
                if let Ok(content) = fs::read_to_string(path) {
                    extract_workspace_deps(&content, &mut all_deps);
                }
            }
        }
    }
    
    println!("\n========================================");
    println!("All workspace dependencies found:");
    println!("========================================\n");
    
    for dep in &all_deps {
        println!("{}", dep);
    }
    
    println!("\n========================================");
    println!("Total unique dependencies: {}", all_deps.len());
    println!("========================================");
}

fn extract_workspace_deps(content: &str, deps: &mut BTreeSet<String>) {
    let mut in_dependencies = false;
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Check if we're entering a dependencies section
        if trimmed.starts_with("[dependencies]") 
            || trimmed.starts_with("[dev-dependencies]")
            || trimmed.starts_with("[build-dependencies]") {
            in_dependencies = true;
            continue;
        }
        
        // Check if we're leaving dependencies section
        if trimmed.starts_with('[') && in_dependencies {
            in_dependencies = false;
            continue;
        }
        
        // Extract dependency names that use workspace = true
        if in_dependencies && trimmed.contains("workspace = true") {
            // Parse lines like: dep-name = { workspace = true }
            // or: dep-name.workspace = true
            if let Some(dep_name) = extract_dep_name(trimmed) {
                deps.insert(dep_name);
            }
        }
    }
}

fn extract_dep_name(line: &str) -> Option<String> {
    let line = line.trim();
    
    // Handle: dep-name = { workspace = true, ... }
    if let Some(eq_pos) = line.find('=') {
        let dep_name = line[..eq_pos].trim();
        
        // Remove any version or other attributes
        let dep_name = if let Some(dot_pos) = dep_name.find('.') {
            &dep_name[..dot_pos]
        } else {
            dep_name
        };
        
        // Skip comments
        if dep_name.starts_with('#') {
            return None;
        }
        
        return Some(dep_name.to_string());
    }
    
    None
}
