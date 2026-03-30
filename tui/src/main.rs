#![allow(unsafe_code)]
#![allow(clippy::all)]
#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(clippy::cargo)]

#[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

mod file_browser;
mod menu;

mod animations;
mod autocomplete;
mod bridge;
mod chat;
mod chat_components;
mod chat_input;
mod components;
mod dispatcher;
mod effects;
mod exit_animation;
mod font;
mod input;
mod llm;
mod logs;
mod modal;
mod panic;
mod perf;
mod render;
mod root;
mod signals;
mod splash;
mod state;
mod theme;

pub use bridge::{AppMode, YaziChatBridge};
use logs::Logs;
use panic::Panic;
pub use root::Root;
pub use state::AnimationType;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	Panic::install();
	fb_shared::init();

	Logs::start()?;
	_ = fdlimit::raise_fd_limit();

	fb_tty::init();
	fb_term::init();
	fb_fs::init();
	fb_config::init()?;
	fb_vfs::init();
	fb_adapter::init()?;
	fb_boot::init();
	fb_dds::init();
	fb_widgets::init();
	fb_watcher::init();
	fb_plugin::init()?;
	fb_dds::serve();

	fb_shared::LOCAL_SET.run_until(file_browser::app::App::serve()).await
}
