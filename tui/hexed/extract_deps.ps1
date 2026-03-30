# PowerShell script to extract all workspace dependencies from Cargo.toml files

$allDeps = @{}

# Function to extract dependencies from a Cargo.toml file
function Extract-WorkspaceDeps {
    param($filePath)
    
    $content = Get-Content $filePath -Raw
    $inDependencies = $false
    
    foreach ($line in $content -split "`n") {
        $trimmed = $line.Trim()
        
        # Check if entering dependencies section
        if ($trimmed -match '^\[(dependencies|dev-dependencies|build-dependencies)\]') {
            $inDependencies = $true
            continue
        }
        
        # Check if leaving dependencies section
        if ($trimmed -match '^\[' -and $inDependencies) {
            $inDependencies = $false
            continue
        }
        
        # Extract dependency with workspace = true
        if ($inDependencies -and $trimmed -match 'workspace\s*=\s*true') {
            if ($trimmed -match '^([a-zA-Z0-9_\-]+)\s*[=\.]') {
                $depName = $matches[1]
                if (-not $allDeps.ContainsKey($depName)) {
                    $allDeps[$depName] = $filePath
                }
            }
        }
    }
}

Write-Host "Scanning src/file_browser/ for Cargo.toml files..." -ForegroundColor Cyan
Write-Host ""

# Scan src/file_browser directory
$cargoFiles = Get-ChildItem -Path "src/file_browser" -Filter "Cargo.toml" -Recurse -ErrorAction SilentlyContinue

foreach ($file in $cargoFiles) {
    Write-Host "Processing: $($file.FullName)" -ForegroundColor Gray
    Extract-WorkspaceDeps -filePath $file.FullName
}

# Also scan old dx-* directories if they exist
$dxDirs = Get-ChildItem -Path "." -Filter "dx-*" -Directory -ErrorAction SilentlyContinue

foreach ($dir in $dxDirs) {
    $cargoFile = Join-Path $dir.FullName "Cargo.toml"
    if (Test-Path $cargoFile) {
        Write-Host "Processing old: $cargoFile" -ForegroundColor Gray
        Extract-WorkspaceDeps -filePath $cargoFile
    }
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "All workspace dependencies found:" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

$sortedDeps = $allDeps.Keys | Sort-Object

foreach ($dep in $sortedDeps) {
    Write-Host $dep -ForegroundColor Yellow
}

Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "Total unique dependencies: $($allDeps.Count)" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""

# Generate workspace.dependencies section
Write-Host "Generating [workspace.dependencies] section..." -ForegroundColor Cyan
Write-Host ""

$output = "[workspace.dependencies]`n"

foreach ($dep in $sortedDeps) {
    # Try to find version from existing Cargo.toml
    $version = "1.0"  # Default
    
    # Common known versions
    $knownVersions = @{
        "ansi-to-tui" = "7.0"
        "anyhow" = "1.0"
        "arc-swap" = "1.7"
        "async-channel" = "2.3"
        "async-fs" = "2.1"
        "async-trait" = "0.1"
        "base64" = "0.22"
        "better-panic" = "0.3"
        "bitflags" = "2.6"
        "bstr" = "1.10"
        "byteorder" = "1.5"
        "bytes" = "1.8"
        "chrono" = "0.4"
        "clap" = "4.5"
        "color-eyre" = "0.6"
        "crossterm" = "0.28"
        "dashmap" = "6.1"
        "dirs" = "5.0"
        "dyn-clone" = "1.0"
        "either" = "1.13"
        "encoding_rs" = "0.8"
        "fd-lock" = "4.0"
        "fdlimit" = "0.3.0"
        "flume" = "0.11"
        "foldhash" = "0.1"
        "futures" = "0.3"
        "globset" = "0.4"
        "hashbrown" = "0.15"
        "indexmap" = "2.6"
        "libc" = "0.2"
        "log" = "0.4"
        "lru" = "0.12"
        "md5" = "0.7"
        "memchr" = "2.7"
        "mlua" = "0.10"
        "nix" = "0.29"
        "notify" = "7.0"
        "once_cell" = "1.20"
        "parking_lot" = "0.12"
        "paste" = "1.0"
        "percent-encoding" = "2.3"
        "ratatui" = "0.29"
        "rayon" = "1.10"
        "regex" = "1.11"
        "scopeguard" = "1.2"
        "serde" = "1.0"
        "serde_json" = "1.0"
        "shell-words" = "1.1"
        "signal-hook" = "0.3"
        "smallvec" = "1.13"
        "syntect" = "5.2"
        "tachyonfx" = "0.8"
        "tempfile" = "3.14"
        "thiserror" = "2.0"
        "tokio" = "1.42"
        "tokio-stream" = "0.1"
        "tokio-util" = "0.7"
        "toml" = "0.8"
        "tracing" = "0.1"
        "tracing-subscriber" = "0.3"
        "unicode-width" = "0.2"
        "url" = "2.5"
        "uuid" = "1.11"
        "which" = "7.0"
    }
    
    if ($knownVersions.ContainsKey($dep)) {
        $version = $knownVersions[$dep]
    }
    
    $output += "$dep = `"$version`"`n"
}

Write-Host $output

# Save to file
$output | Out-File "workspace_dependencies.toml" -Encoding UTF8
Write-Host "Saved to: workspace_dependencies.toml" -ForegroundColor Green
