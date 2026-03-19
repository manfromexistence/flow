mod app;
mod autocomplete;
mod components;
mod effects;
mod font;
mod gruvbox;
mod input;
mod llm;
mod modal;
mod perf;
mod render; // Now a module directory with submodules
// mod screens; // Unused - only for standalone screen demos
mod splash;
mod tachyonfx;
mod theme;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = app::ChatApp::new();
    app.run().await
}
