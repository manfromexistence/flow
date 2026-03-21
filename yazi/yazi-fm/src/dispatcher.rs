use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use tracing::warn;
use yazi_actor::Ctx;
use yazi_config::keymap::Key;
use yazi_macro::{act, emit, succ};
use yazi_shared::{data::Data, event::{ActionCow, Event, NEED_RENDER}};
use yazi_widgets::input::InputMode;

use crate::{Executor, Router, app::App};

pub(super) struct Dispatcher<'a> {
	app: &'a mut App,
}

impl<'a> Dispatcher<'a> {
	#[inline]
	pub(super) fn new(app: &'a mut App) -> Self { Self { app } }

	#[inline]
	pub(super) fn dispatch(&mut self, event: Event) -> Result<()> {
		let result = match event {
			Event::Call(action) => self.dispatch_call(action),
			Event::Seq(actions) => self.dispatch_seq(actions),
			Event::Render(partial) => self.dispatch_render(partial),
			Event::Key(key) => self.dispatch_key(key),
			Event::Mouse(mouse) => self.dispatch_mouse(mouse),
			Event::Resize => self.dispatch_resize(),
			Event::Focus => self.dispatch_focus(),
			Event::Paste(str) => self.dispatch_paste(str),
			Event::Timer => self.dispatch_timer(),
		};

		if let Err(err) = result {
			warn!("Event dispatch error: {err:?}");
		}
		Ok(())
	}

	#[inline]
	fn dispatch_call(&mut self, action: ActionCow) -> Result<Data> {
		Executor::new(self.app).execute(action)
	}

	#[inline]
	fn dispatch_seq(&mut self, mut actions: Vec<ActionCow>) -> Result<Data> {
		if let Some(last) = actions.pop() {
			self.dispatch_call(last)?;
		}
		if !actions.is_empty() {
			emit!(Seq(actions));
		}
		succ!();
	}

	#[inline]
	fn dispatch_render(&mut self, partial: bool) -> Result<Data> {
		if partial {
			_ = NEED_RENDER.compare_exchange(0, 2, Ordering::Relaxed, Ordering::Relaxed);
		} else {
			NEED_RENDER.store(1, Ordering::Relaxed);
		}
		succ!()
	}

	#[inline]
	fn dispatch_key(&mut self, key: KeyEvent) -> Result<Data> {
		use crossterm::event::KeyCode;
		
		// If in animation mode, handle arrow keys for navigation
		if self.app.bridge.chat_state.animation_mode {
			let animations = crate::chat_tui::AnimationType::all();
			
			// Handle Left/Right arrow keys for screen navigation (even on Yazi screen)
			match key.code {
				KeyCode::Left | KeyCode::Backspace => {
					// Previous animation
					if self.app.bridge.chat_state.current_animation_index == 0 {
						self.app.bridge.chat_state.current_animation_index = animations.len() - 1;
					} else {
						self.app.bridge.chat_state.current_animation_index -= 1;
					}
					self.app.bridge.chat_state.animation_start_time = Some(Instant::now());
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!();
				}
				KeyCode::Right | KeyCode::Enter => {
					// Next animation
					self.app.bridge.chat_state.current_animation_index = 
						(self.app.bridge.chat_state.current_animation_index + 1) % animations.len();
					self.app.bridge.chat_state.animation_start_time = Some(Instant::now());
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!();
				}
				_ => {
					// Other keys: only route to yazi if on Yazi screen
					let current_anim = animations[self.app.bridge.chat_state.current_animation_index];
					if current_anim != crate::chat_tui::AnimationType::Yazi {
						// Not on Yazi screen, ignore other keys
						succ!();
					}
					// On Yazi screen, fall through to route to yazi
				}
			}
		}
		
		Router::new(self.app).route(Key::from(key))?;
		succ!();
	}

	#[inline]
	fn dispatch_mouse(&mut self, mouse: MouseEvent) -> Result<Data> {
		let cx = &mut Ctx::active(&mut self.app.core, &mut self.app.term);
		act!(app:mouse, cx, mouse)
	}

	#[inline]
	fn dispatch_resize(&mut self) -> Result<Data> {
		let cx = &mut Ctx::active(&mut self.app.core, &mut self.app.term);
		act!(app:resize, cx, crate::Root::reflow as fn(_) -> _)
	}

	#[inline]
	fn dispatch_focus(&mut self) -> Result<Data> {
		let cx = &mut Ctx::active(&mut self.app.core, &mut self.app.term);
		act!(app:focus, cx)
	}

	#[inline]
	fn dispatch_paste(&mut self, str: String) -> Result<Data> {
		if self.app.core.input.visible {
			let input = &mut self.app.core.input;
			if input.mode() == InputMode::Insert {
				input.type_str(&str)?;
			} else if input.mode() == InputMode::Replace {
				input.replace_str(&str)?;
			}
		}
		succ!();
	}

	#[inline]
	fn dispatch_timer(&mut self) -> Result<Data> {
		// Timer tick for animations - just trigger a render
		// The effects are time-based and will automatically show updated colors
		
		// Update splash font cycling (every 3 seconds)
		if self.app.bridge.chat_state.animation_mode 
			&& self.app.bridge.chat_state.last_font_change.elapsed() >= Duration::from_secs(3)
		{
			let animations = crate::chat_tui::AnimationType::all();
			let current_anim = animations[self.app.bridge.chat_state.current_animation_index];
			if current_anim == crate::chat_tui::AnimationType::Splash {
				self.app.bridge.chat_state.splash_font_index = 
					(self.app.bridge.chat_state.splash_font_index + 1) % 113; // 113 valid fonts
				self.app.bridge.chat_state.last_font_change = Instant::now();
			}
		}
		
		NEED_RENDER.store(1, Ordering::Relaxed);
		succ!();
	}
}
