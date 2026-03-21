use mlua::{ObjectLike, Table};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use tracing::error;
use yazi_binding::elements::render_once;
use yazi_core::Core;
use yazi_plugin::LUA;

use crate::chat_tui::{YaziChatBridge, AppMode};
use super::{cmp, confirm, help, input, mgr, pick, spot, tasks, which};

pub(super) struct Root<'a> {
	core: &'a Core,
	bridge: &'a mut YaziChatBridge,
}

impl<'a> Root<'a> {
	pub(super) fn new(core: &'a Core, bridge: &'a mut YaziChatBridge) -> Self { 
		Self { core, bridge } 
	}

	pub(super) fn reflow(area: Rect) -> mlua::Result<Table> {
		let area = yazi_binding::elements::Rect::from(area);
		let root = LUA.globals().raw_get::<Table>("Root")?.call_method::<Table>("new", area)?;
		root.call_method("reflow", ())
	}
}

impl Widget for Root<'_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		// Check mode and render accordingly
		match self.bridge.mode {
			AppMode::Chat => {
				// Full chat mode - render chat TUI
				self.bridge.chat_state.render(area, buf);
			}
			AppMode::FilePicker => {
				// Split screen: 85% yazi, 15% chat input
				let chunks = ratatui::layout::Layout::default()
					.direction(ratatui::layout::Direction::Vertical)
					.constraints([
						ratatui::layout::Constraint::Percentage(85),  // Yazi file picker (more space)
						ratatui::layout::Constraint::Percentage(15),  // Chat input (compact)
					])
					.split(area);
				
				let yazi_area = chunks[0];
				let chat_area = chunks[1];
				
				// Render yazi in the top area
				let mut f = || {
					let area = yazi_binding::elements::Rect::from(yazi_area);
					let root = LUA.globals().raw_get::<Table>("Root")?.call_method::<Table>("new", area)?;

					render_once(root.call_method("redraw", ())?, buf, |p| self.core.mgr.area(p));
					Ok::<_, mlua::Error>(())
				};
				if let Err(e) = f() {
					error!("Failed to redraw the `Root` component:\n{e}");
				}

				mgr::Preview::new(self.core).render(yazi_area, buf);
				mgr::Modal::new(self.core).render(yazi_area, buf);

				if self.core.tasks.visible {
					tasks::Tasks::new(self.core).render(yazi_area, buf);
				}

				if self.core.active().spot.visible() {
					spot::Spot::new(self.core).render(yazi_area, buf);
				}

				if self.core.pick.visible {
					pick::Pick::new(self.core).render(yazi_area, buf);
				}

				if self.core.input.visible {
					input::Input::new(self.core).render(yazi_area, buf);
				}

				if self.core.confirm.visible {
					confirm::Confirm::new(self.core).render(yazi_area, buf);
				}

				if self.core.help.visible {
					help::Help::new(self.core).render(yazi_area, buf);
				}

				if self.core.cmp.visible {
					cmp::Cmp::new(self.core).render(yazi_area, buf);
				}

				if self.core.which.active {
					which::Which::new(self.core).render(yazi_area, buf);
				}
				
				// Render dimmed chat at the bottom
				self.bridge.chat_state.render_dimmed(chat_area, buf);
			}
		}
	}
}
