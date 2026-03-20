# 🔌 Metasearch Integration Guide

> How to integrate metasearch functionality into your Rust projects

## 📋 Table of Contents

- [Quick Start](#quick-start)
- [Core Concepts](#core-concepts)
- [Basic Integration](#basic-integration)
- [Advanced Usage](#advanced-usage)
- [Custom Engine Development](#custom-engine-development)
- [Real-World Examples](#real-world-examples)
- [Performance Tips](#performance-tips)

---

## 🚀 Quick Start

### Add to Your Project

Add metasearch crates to your `Cargo.toml`:

```toml
[dependencies]
metasearch-core = { path = "../metasearch/crates/metasearch-core" }
metasearch-engine = { path = "../metasearch/crates/metasearch-engine" }
reqwest = { version = "0.12", features = ["json", "gzip", "brotli", "rustls-tls", "http2"], default-features = false }
tokio = { version = "1.48", features = ["full"] }
async-trait = "0.1"
```

### System Requirements

- Rust 1.85.0 or later (uses edition = "2024")
- Tokio async runtime
- Internet connection for search engines

### Minimal Example (5 Lines!)

```rust
use metasearch_core::query::SearchQuery;
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = EngineRegistry::with_defaults(Client::new());
    let engine = registry.get("google").unwrap();
    let results = engine.search(&SearchQuery::new("rust")).await?;
    
    for r in results {
        println!("{} - {}", r.title, r.url);
    }
    Ok(())
}
```

### Complete Example with Error Handling

```rust
use metasearch_core::{query::SearchQuery, category::SearchCategory};
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client with custom settings
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .user_agent("MyApp/1.0")
        .build()?;
    
    // Initialize engine registry with all 215+ engines
    let registry = EngineRegistry::with_defaults(client);
    println!("Loaded {} engines", registry.count());
    
    // Create search query
    let query = SearchQuery::new("rust programming");
    
    // Get engines for general web search
    let engines = registry.engines_for_category(&SearchCategory::General);
    println!("Found {} general search engines", engines.len());
    
    // Search with first available engine
    if let Some(engine) = engines.first() {
        let metadata = engine.metadata();
        println!("\nSearching with {}...", metadata.display_name);
        
        match engine.search(&query).await {
            Ok(results) => {
                println!("Found {} results\n", results.len());
                for (i, result) in results.iter().take(5).enumerate() {
                    println!("{}. {}", i + 1, result.title);
                    println!("   {}", result.url);
                    if !result.snippet.is_empty() {
                        println!("   {}\n", result.snippet);
                    }
                }
            }
            Err(e) => eprintln!("Search failed: {}", e),
        }
    }
    
    Ok(())
}
```

---

## 🧩 Core Concepts

### 1. Engine Registry

The `EngineRegistry` manages all 215+ search engines. It's the central hub for accessing engines.

**Key Facts:**
- 215 engines total (more than SearXNG's 211!)
- ~60% working rate (124/208 engines tested)
- Engines organized by category
- Thread-safe (uses DashMap internally)

```rust
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;

let client = Client::new();
let registry = EngineRegistry::with_defaults(client);

// Get engine count
println!("Loaded {} engines", registry.count());  // Output: 215

// List all engine names
let names = registry.engine_names();
println!("Engines: {:?}", &names[..5]);  // First 5 engines

// Get specific engine
if let Some(google) = registry.get("google") {
    let meta = google.metadata();
    println!("Engine: {} ({})", meta.display_name, meta.homepage);
}
```

### 2. Search Categories

Engines are organized into 9 categories:

```rust
use metasearch_core::category::SearchCategory;

// Available categories:
SearchCategory::General      // Web search (Google, DuckDuckGo, Brave, etc.)
SearchCategory::Images       // Image search (Bing Images, Google Images, etc.)
SearchCategory::Videos       // Video search (YouTube, Vimeo, etc.)
SearchCategory::News         // News articles (Google News, Bing News, etc.)
SearchCategory::Music        // Music/audio (Spotify, SoundCloud, etc.)
SearchCategory::Files        // File downloads (torrents, archives)
SearchCategory::Science      // Academic papers (arXiv, Google Scholar, etc.)
SearchCategory::It           // IT/programming (GitHub, Stack Overflow, etc.)
SearchCategory::Map          // Maps/locations (OpenStreetMap, Apple Maps)

// Get engines by category
let general_engines = registry.engines_for_category(&SearchCategory::General);
println!("General search engines: {}", general_engines.len());  // ~80+ engines
```

### 3. Search Query

Configure your search with `SearchQuery`:

```rust
use metasearch_core::query::SearchQuery;

// Simple query
let query = SearchQuery::new("rust async");

// Advanced query with options
let mut query = SearchQuery::new("rust programming");
query.page = 2;                              // Pagination (default: 1)
query.language = Some("en".to_string());     // Language filter
query.safe_search = true;                    // Filter adult content (default: false)

// Query is Clone, so you can reuse it
let query2 = query.clone();
```

### 4. Search Results

Results are returned as `Vec<SearchResult>`:

```rust
use metasearch_core::result::SearchResult;

// Result structure:
pub struct SearchResult {
    pub title: String,              // Result title
    pub url: String,                // Result URL
    pub snippet: String,            // Description/excerpt
    pub engine: String,             // Engine name (e.g., "google")
    pub category: String,           // Category (e.g., "General")
    pub engine_rank: u32,           // Position in engine results (1-based)
    pub score: f64,                 // Relevance score (for aggregation)
    pub thumbnail: Option<String>,  // Image URL (for image/video results)
    pub published_date: Option<String>,  // Publication date (for news)
}

// Example usage:
for result in results {
    println!("{} (rank: {})", result.title, result.engine_rank);
    println!("  URL: {}", result.url);
    println!("  From: {}", result.engine);
    if let Some(date) = result.published_date {
        println!("  Published: {}", date);
    }
}
```

### 5. Engine Metadata

Every engine has metadata describing its capabilities:

```rust
use metasearch_core::engine::EngineMetadata;

let engine = registry.get("google").unwrap();
let meta = engine.metadata();

println!("Name: {}", meta.name);                    // "google"
println!("Display: {}", meta.display_name);         // "Google"
println!("Homepage: {}", meta.homepage);            // "https://google.com"
println!("Categories: {:?}", meta.categories);      // [General, Images, News, ...]
println!("Enabled: {}", meta.enabled);              // true/false
println!("Timeout: {}ms", meta.timeout_ms);         // 5000
println!("Weight: {}", meta.weight);                // 1.5 (for ranking)
```

---

## 🔧 Basic Integration

### Single Engine Search

```rust
use metasearch_core::query::SearchQuery;
use metasearch_engine::google::Google;
use reqwest::Client;

async fn search_google(query_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let google = Google::new(client);
    
    let query = SearchQuery::new(query_text);
    let results = google.search(&query).await?;
    
    println!("Found {} results", results.len());
    for result in results {
        println!("• {} - {}", result.title, result.url);
    }
    
    Ok(())
}
```

### Multi-Engine Search

```rust
use metasearch_core::{query::SearchQuery, category::SearchCategory};
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;

async fn search_multiple(query_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);
    
    let query = SearchQuery::new(query_text);
    let engines = registry.engines_for_category(&SearchCategory::General);
    
    // Search with multiple engines
    for engine in engines.iter().take(5) {
        let metadata = engine.metadata();
        println!("\nSearching with {}...", metadata.display_name);
        
        match engine.search(&query).await {
            Ok(results) => {
                println!("✓ Found {} results", results.len());
                for result in results.iter().take(3) {
                    println!("  • {}", result.title);
                }
            }
            Err(e) => println!("✗ Error: {}", e),
        }
    }
    
    Ok(())
}
```

### Parallel Search

```rust
use metasearch_core::{query::SearchQuery, category::SearchCategory, result::SearchResult};
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;
use futures::future::join_all;

async fn parallel_search(query_text: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);
    
    let query = SearchQuery::new(query_text);
    let engines = registry.engines_for_category(&SearchCategory::General);
    
    // Create search tasks
    let tasks: Vec<_> = engines.iter()
        .take(10)
        .map(|engine| {
            let query = query.clone();
            let engine = engine.clone();
            tokio::spawn(async move {
                engine.search(&query).await
            })
        })
        .collect();
    
    // Wait for all searches to complete
    let results = join_all(tasks).await;
    
    // Flatten results
    let mut all_results = Vec::new();
    for result in results {
        if let Ok(Ok(mut engine_results)) = result {
            all_results.append(&mut engine_results);
        }
    }
    
    println!("Total results: {}", all_results.len());
    Ok(all_results)
}
```

---

## 🎯 Advanced Usage

### Result Aggregation & Deduplication

Combine results from multiple engines and remove duplicates:

```rust
use metasearch_core::{
    query::SearchQuery,
    category::SearchCategory,
    ranking::ResultAggregator,
};
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;
use dashmap::DashMap;

async fn aggregated_search(query_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);
    
    // Create aggregator with engine weights (higher = more trusted)
    let weights = DashMap::new();
    weights.insert("google".to_string(), 1.5);
    weights.insert("duckduckgo".to_string(), 1.2);
    weights.insert("brave".to_string(), 1.0);
    weights.insert("bing".to_string(), 1.0);
    
    let aggregator = ResultAggregator::new(weights);
    
    // Search with multiple engines
    let query = SearchQuery::new(query_text);
    let engines = registry.engines_for_category(&SearchCategory::General);
    
    let mut all_results = Vec::new();
    for engine in engines.iter().take(5) {
        if let Ok(results) = engine.search(&query).await {
            all_results.extend(results);
        }
    }
    
    // Aggregate and deduplicate (returns top 50 by default)
    let final_results = aggregator.aggregate(all_results, 50);
    
    println!("Aggregated {} unique results", final_results.len());
    for (i, result) in final_results.iter().enumerate() {
        println!("{}. {} (score: {:.2})", i + 1, result.title, result.score);
    }
    
    Ok(())
}
```

### Parallel Search (Fast!)

Search multiple engines concurrently for maximum speed:

```rust
use metasearch_core::{query::SearchQuery, category::SearchCategory, result::SearchResult};
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;
use futures::future::join_all;

async fn parallel_search(query_text: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);
    
    let query = SearchQuery::new(query_text);
    let engines = registry.engines_for_category(&SearchCategory::General);
    
    // Create search tasks (spawn each engine search)
    let tasks: Vec<_> = engines.iter()
        .take(10)  // Search with 10 engines
        .map(|engine| {
            let query = query.clone();
            let engine = engine.clone();
            tokio::spawn(async move {
                engine.search(&query).await
            })
        })
        .collect();
    
    // Wait for all searches to complete
    let results = join_all(tasks).await;
    
    // Flatten results
    let mut all_results = Vec::new();
    for result in results {
        if let Ok(Ok(mut engine_results)) = result {
            all_results.append(&mut engine_results);
        }
    }
    
    println!("Total results from parallel search: {}", all_results.len());
    Ok(all_results)
}
```

### Autocomplete Integration

Get search suggestions as users type:

```rust
use metasearch_engine::google::Google;
use reqwest::Client;

async fn get_suggestions(partial: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = Client::new();
    let google = Google::new(client);
    
    // Get autocomplete suggestions
    let suggestions = google.autocomplete(partial).await?;
    
    println!("Suggestions for '{}':", partial);
    for suggestion in &suggestions {
        println!("  • {}", suggestion);
    }
    
    Ok(suggestions)
}

// Usage:
// get_suggestions("rust").await?;
// Output:
//   • rust programming
//   • rust game
//   • rust tutorial
//   • rust vs c++
//   • rust language
```

### Category-Specific Search

Search within specific categories:

```rust
use metasearch_core::{query::SearchQuery, category::SearchCategory};
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;

async fn search_images(query_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);
    
    let query = SearchQuery::new(query_text);
    let engines = registry.engines_for_category(&SearchCategory::Images);
    
    println!("Searching images with {} engines", engines.len());
    
    for engine in engines.iter().take(3) {
        let metadata = engine.metadata();
        println!("\n{}", metadata.display_name);
        
        match engine.search(&query).await {
            Ok(results) => {
                for result in results.iter().take(5) {
                    println!("  • {}", result.title);
                    if let Some(thumb) = &result.thumbnail {
                        println!("    Image: {}", thumb);
                    }
                }
            }
            Err(e) => println!("  Error: {}", e),
        }
    }
    
    Ok(())
}

// Search different categories:
// search_by_category("rust", SearchCategory::General).await?;
// search_by_category("nature", SearchCategory::Images).await?;
// search_by_category("tutorial", SearchCategory::Videos).await?;
// search_by_category("covid", SearchCategory::News).await?;
```

### Custom HTTP Client Configuration

Optimize the HTTP client for your needs:

```rust
use reqwest::Client;
use std::time::Duration;

fn create_custom_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .timeout(Duration::from_secs(15))           // Total request timeout
        .connect_timeout(Duration::from_secs(5))    // Connection timeout
        .user_agent("MyApp/1.0")                    // Custom user agent
        .gzip(true)                                 // Enable gzip compression
        .brotli(true)                               // Enable brotli compression
        .redirect(reqwest::redirect::Policy::limited(5))  // Max 5 redirects
        .pool_max_idle_per_host(10)                 // Connection pooling
        .http2_prior_knowledge()                    // Use HTTP/2
        .build()
}

// Use with registry:
let client = create_custom_client()?;
let registry = EngineRegistry::with_defaults(client);
```

### Error Handling Best Practices

```rust
use metasearch_core::error::MetasearchError;

async fn robust_search(query_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);
    let query = SearchQuery::new(query_text);
    
    let engines = registry.engines_for_category(&SearchCategory::General);
    
    for engine in engines.iter().take(5) {
        let metadata = engine.metadata();
        
        match engine.search(&query).await {
            Ok(results) => {
                println!("✓ {} returned {} results", metadata.display_name, results.len());
            }
            Err(MetasearchError::HttpError(e)) => {
                eprintln!("✗ {} HTTP error: {}", metadata.display_name, e);
            }
            Err(MetasearchError::ParseError(e)) => {
                eprintln!("✗ {} parse error: {}", metadata.display_name, e);
            }
            Err(MetasearchError::Timeout) => {
                eprintln!("✗ {} timed out", metadata.display_name);
            }
            Err(e) => {
                eprintln!("✗ {} error: {}", metadata.display_name, e);
            }
        }
    }
    
    Ok(())
}
```

---

## 🛠️ Custom Engine Development

### Implement Your Own Search Engine

```rust
use async_trait::async_trait;
use metasearch_core::{
    engine::{EngineMetadata, SearchEngine},
    category::SearchCategory,
    query::SearchQuery,
    result::SearchResult,
    error::Result,
};
use reqwest::Client;
use smallvec::smallvec;

pub struct MyCustomEngine {
    metadata: EngineMetadata,
    client: Client,
}

impl MyCustomEngine {
    pub fn new(client: Client) -> Self {
        Self {
            metadata: EngineMetadata {
                name: "my_engine".to_string().into(),
                display_name: "My Custom Engine".to_string().into(),
                homepage: "https://example.com".to_string().into(),
                categories: smallvec![SearchCategory::General],
                enabled: true,
                timeout_ms: 5000,
                weight: 1.0,
            },
            client,
        }
    }
}

#[async_trait]
impl SearchEngine for MyCustomEngine {
    fn metadata(&self) -> EngineMetadata {
        self.metadata.clone()
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        // Build search URL
        let url = format!(
            "https://api.example.com/search?q={}",
            urlencoding::encode(&query.query)
        );
        
        // Make HTTP request
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| metasearch_core::error::MetasearchError::HttpError(e.to_string()))?;
        
        let text = response.text().await
            .map_err(|e| metasearch_core::error::MetasearchError::ParseError(e.to_string()))?;
        
        // Parse results (example with JSON)
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| metasearch_core::error::MetasearchError::ParseError(e.to_string()))?;
        
        let mut results = Vec::new();
        
        if let Some(items) = json["results"].as_array() {
            for (i, item) in items.iter().enumerate() {
                let title = item["title"].as_str().unwrap_or("").to_string();
                let url = item["url"].as_str().unwrap_or("").to_string();
                let snippet = item["description"].as_str().unwrap_or("").to_string();
                
                let mut result = SearchResult::new(&title, &url, &snippet, "my_engine");
                result.engine_rank = (i + 1) as u32;
                result.category = SearchCategory::General.to_string();
                
                results.push(result);
            }
        }
        
        Ok(results)
    }
    
    async fn autocomplete(&self, partial: &str) -> Result<Vec<String>> {
        // Optional: implement autocomplete
        let url = format!(
            "https://api.example.com/suggest?q={}",
            urlencoding::encode(partial)
        );
        
        let response = self.client.get(&url).send().await
            .map_err(|e| metasearch_core::error::MetasearchError::HttpError(e.to_string()))?;
        
        let json: serde_json::Value = response.json().await
            .map_err(|e| metasearch_core::error::MetasearchError::ParseError(e.to_string()))?;
        
        let suggestions = json["suggestions"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        
        Ok(suggestions)
    }
}

// Register your custom engine:
use std::sync::Arc;

let client = Client::new();
let mut registry = EngineRegistry::new();
registry.register(Arc::new(MyCustomEngine::new(client.clone())));
```

### HTML Scraping Engine Example

```rust
use scraper::{Html, Selector};

#[async_trait]
impl SearchEngine for MyScrapingEngine {
    fn metadata(&self) -> EngineMetadata {
        // ... metadata ...
    }

    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>> {
        let url = format!("https://example.com/search?q={}", 
            urlencoding::encode(&query.query));
        
        let response = self.client.get(&url).send().await
            .map_err(|e| MetasearchError::HttpError(e.to_string()))?;
        
        let html = response.text().await
            .map_err(|e| MetasearchError::ParseError(e.to_string()))?;
        
        let document = Html::parse_document(&html);
        
        // Define CSS selectors
        let result_sel = Selector::parse(".search-result").unwrap();
        let title_sel = Selector::parse("h3.title").unwrap();
        let link_sel = Selector::parse("a.link").unwrap();
        let snippet_sel = Selector::parse("p.snippet").unwrap();
        
        let mut results = Vec::new();
        
        for (i, item) in document.select(&result_sel).enumerate() {
            let title = item.select(&title_sel).next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            
            let url = item.select(&link_sel).next()
                .and_then(|e| e.value().attr("href"))
                .unwrap_or_default()
                .to_string();
            
            let snippet = item.select(&snippet_sel).next()
                .map(|e| e.text().collect::<String>())
                .unwrap_or_default();
            
            if !title.is_empty() && !url.is_empty() {
                let mut result = SearchResult::new(&title, &url, &snippet, "my_scraper");
                result.engine_rank = (i + 1) as u32;
                results.push(result);
            }
        }
        
        Ok(results)
    }
}
```

---

## 📚 API Reference

### EngineRegistry

```rust
impl EngineRegistry {
    // Create empty registry
    pub fn new() -> Self
    
    // Create registry with all 215+ built-in engines
    pub fn with_defaults(client: Client) -> Self
    
    // Register a custom engine
    pub fn register(&mut self, engine: Arc<dyn SearchEngine>)
    
    // Get engine by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn SearchEngine>>
    
    // Get all engines for a category
    pub fn engines_for_category(&self, category: &SearchCategory) -> Vec<Arc<dyn SearchEngine>>
    
    // List all engine names
    pub fn engine_names(&self) -> Vec<String>
    
    // Get total engine count
    pub fn count(&self) -> usize
}
```

### SearchQuery

```rust
pub struct SearchQuery {
    pub query: String,
    pub page: u32,
    pub language: Option<String>,
    pub safe_search: bool,
}

impl SearchQuery {
    pub fn new(query: impl Into<String>) -> Self
}
```

### SearchResult

```rust
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub engine: String,
    pub category: String,
    pub engine_rank: u32,
    pub score: f64,
    pub thumbnail: Option<String>,
    pub published_date: Option<String>,
}

impl SearchResult {
    pub fn new(title: &str, url: &str, snippet: &str, engine: &str) -> Self
}
```

### SearchEngine Trait

```rust
#[async_trait]
pub trait SearchEngine: Send + Sync {
    fn metadata(&self) -> EngineMetadata;
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>>;
    async fn autocomplete(&self, partial: &str) -> Result<Vec<String>>;
}
```

---

## 💡 Real-World Examples

### Example 1: Simple CLI Search Tool

```rust
use metasearch_core::{query::SearchQuery, category::SearchCategory};
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);
    
    println!("=== Metasearch CLI ===");
    println!("Loaded {} engines\n", registry.count());
    
    loop {
        print!("Search (or 'quit'): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let query_text = input.trim();
        
        if query_text.is_empty() || query_text == "quit" {
            break;
        }
        
        let query = SearchQuery::new(query_text);
        let engines = registry.engines_for_category(&SearchCategory::General);
        
        if let Some(engine) = engines.first() {
            println!("\nSearching with {}...\n", engine.metadata().display_name);
            
            match engine.search(&query).await {
                Ok(results) => {
                    for (i, result) in results.iter().take(10).enumerate() {
                        println!("{}. {}", i + 1, result.title);
                        println!("   {}", result.url);
                        if !result.snippet.is_empty() {
                            println!("   {}\n", result.snippet);
                        }
                    }
                }
                Err(e) => eprintln!("Error: {}\n", e),
            }
        }
    }
    
    println!("Goodbye!");
    Ok(())
}
```

### Example 2: Web API Server with Axum

```rust
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use metasearch_core::{query::SearchQuery, category::SearchCategory, result::SearchResult};
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    registry: Arc<EngineRegistry>,
}

#[derive(Deserialize)]
struct SearchParams {
    q: String,
    #[serde(default)]
    page: u32,
    #[serde(default)]
    engine: Option<String>,
}

#[derive(Serialize)]
struct SearchResponse {
    query: String,
    engine: String,
    results: Vec<SearchResult>,
    total: usize,
}

async fn search_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchParams>,
) -> Json<SearchResponse> {
    let mut query = SearchQuery::new(&params.q);
    if params.page > 0 {
        query.page = params.page;
    }
    
    // Use specified engine or default to first general engine
    let engine = if let Some(engine_name) = params.engine {
        state.registry.get(&engine_name)
    } else {
        state.registry
            .engines_for_category(&SearchCategory::General)
            .first()
            .cloned()
    };
    
    let (engine_name, results) = if let Some(engine) = engine {
        let name = engine.metadata().name.to_string();
        let results = engine.search(&query).await.unwrap_or_default();
        (name, results)
    } else {
        ("none".to_string(), Vec::new())
    };
    
    let total = results.len();
    
    Json(SearchResponse {
        query: params.q,
        engine: engine_name,
        results,
        total,
    })
}

async fn engines_handler(State(state): State<AppState>) -> Json<Vec<String>> {
    Json(state.registry.engine_names())
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let registry = Arc::new(EngineRegistry::with_defaults(client));
    
    let state = AppState { registry };
    
    let app = Router::new()
        .route("/search", get(search_handler))
        .route("/engines", get(engines_handler))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("Server running on http://127.0.0.1:3000");
    println!("Try: http://127.0.0.1:3000/search?q=rust");
    axum::serve(listener, app).await.unwrap();
}
```

### Example 3: Multi-Engine Comparison Tool

```rust
use metasearch_core::{query::SearchQuery, category::SearchCategory};
use metasearch_engine::registry::EngineRegistry;
use reqwest::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let registry = EngineRegistry::with_defaults(client);
    
    let query = SearchQuery::new("rust programming");
    let engines = registry.engines_for_category(&SearchCategory::General);
    
    println!("Comparing {} engines for query: '{}'\n", engines.len(), query.query);
    
    let mut stats: HashMap<String, (usize, u128)> = HashMap::new();
    
    for engine in engines.iter().take(10) {
        let metadata = engine.metadata();
        let start = std::time::Instant::now();
        
        match engine.search(&query).await {
            Ok(results) => {
                let duration = start.elapsed().as_millis();
                stats.insert(
                    metadata.display_name.to_string(),
                    (results.len(), duration)
                );
                println!("✓ {:20} {} results in {}ms", 
                    metadata.display_name, results.len(), duration);
            }
            Err(e) => {
                println!("✗ {:20} Error: {}", metadata.display_name, e);
            }
        }
    }
    
    // Print summary
    println!("\n=== Summary ===");
    let mut sorted: Vec<_> = stats.iter().collect();
    sorted.sort_by_key(|(_, (count, _))| std::cmp::Reverse(*count));
    
    for (name, (count, time)) in sorted {
        println!("{:20} {} results, {}ms", name, count, time);
    }
    
    Ok(())
}
```

---

## ⚡ Performance Tips

### 1. Reuse HTTP Client

**DO THIS:**
```rust
let client = Client::new();
let registry = EngineRegistry::with_defaults(client);
// Use registry for all searches
```

**NOT THIS:**
```rust
// Creating new client for each search is SLOW
for _ in 0..10 {
    let client = Client::new();  // ❌ Bad!
    let registry = EngineRegistry::with_defaults(client);
}
```

### 2. Use Parallel Search

Search multiple engines concurrently:
```rust
// Sequential (slow): ~5 seconds for 5 engines
for engine in engines {
    engine.search(&query).await?;
}

// Parallel (fast): ~1 second for 5 engines
let tasks: Vec<_> = engines.iter()
    .map(|e| tokio::spawn(e.search(&query)))
    .collect();
join_all(tasks).await;
```

### 3. Configure Timeouts

```rust
let client = Client::builder()
    .timeout(Duration::from_secs(10))      // Total timeout
    .connect_timeout(Duration::from_secs(3))  // Connection timeout
    .build()?;
```

### 4. Enable Connection Pooling

```rust
let client = Client::builder()
    .pool_max_idle_per_host(10)  // Reuse connections
    .http2_prior_knowledge()      // Use HTTP/2
    .build()?;
```

### 5. Use Fast Allocator

Add to your `Cargo.toml`:
```toml
[dependencies]
mimalloc = { version = "0.1", default-features = false }

# In your main.rs:
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
```

### 6. Cache Results

```rust
use moka::future::Cache;
use std::time::Duration;

let cache: Cache<String, Vec<SearchResult>> = Cache::builder()
    .max_capacity(1000)
    .time_to_live(Duration::from_secs(300))  // 5 minutes
    .build();

// Check cache first
if let Some(results) = cache.get(&query.query).await {
    return Ok(results);
}

// Search and cache
let results = engine.search(&query).await?;
cache.insert(query.query.clone(), results.clone()).await;
```

### 7. Limit Concurrent Requests

```rust
use tokio::sync::Semaphore;
use std::sync::Arc;

let semaphore = Arc::new(Semaphore::new(5));  // Max 5 concurrent

for engine in engines {
    let permit = semaphore.clone().acquire_owned().await?;
    tokio::spawn(async move {
        let _permit = permit;  // Released when dropped
        engine.search(&query).await
    });
}
```

---

## 🎯 Best Practices

1. **Reuse HTTP Client**: Create one `reqwest::Client` and share it
2. **Handle Errors Gracefully**: Not all engines will succeed
3. **Use Parallel Search**: Search multiple engines concurrently
4. **Deduplicate Results**: Use `ResultAggregator` to remove duplicates
5. **Respect Timeouts**: Configure appropriate timeouts
6. **Cache Results**: Reduce API calls with caching
7. **Rate Limiting**: Implement rate limiting for production
8. **Monitor Performance**: Track which engines are slow/failing

---

## 📊 Engine Statistics

Based on testing (as of March 2026):

- **Total Engines**: 215 (more than SearXNG's 211!)
- **Working Rate**: ~60% (124/208 engines tested)
- **Categories**: 9 (General, Images, Videos, News, Music, Files, Science, IT, Map)
- **Average Response Time**: 0.04s per engine (parallel testing)
- **Top Performers**: voidlinux (309 results), www1x (216), repology (200)

---

## 🔗 Related Documentation

- [README.md](README.md) - Project overview and features
- [QUICK_START.md](QUICK_START.md) - Getting started guide
- [ENGINES.md](ENGINES.md) - Complete list of 215+ engines
- [BRUTAL_TRUTH_REPORT.md](BRUTAL_TRUTH_REPORT.md) - Engine testing results

---

## 📝 License

AGPL-3.0 - Same as the main project

---

**Happy integrating! 🚀**

For questions or issues, check the [GitHub repository](https://github.com/najmus-sakib-hossain/metasearch).
