mod app;
mod autocomplete;
mod components;
mod effects;
mod gruvbox;
mod input;
mod llm;
mod modal;
mod render;
mod screens;
mod splash;
mod tachyonfx;
mod theme;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = app::ChatApp::new();
    app.run().await
}
