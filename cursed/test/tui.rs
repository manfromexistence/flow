#![allow(warnings)]

mod prompts;
mod theme;
mod ui;

#[tokio::main]
async fn main() {
    let mut app = ui::chat::ChatApp::new();
    app.initialize_llm();
    if let Err(err) = app.run() {
        eprintln!("Error: {err:#}");
        std::process::exit(1);
    }
}
