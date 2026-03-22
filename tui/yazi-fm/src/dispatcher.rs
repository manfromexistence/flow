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
		use crate::chat_tui::input::InputAction;
		
		// If in animation mode, handle navigation keys but allow typing
		if self.app.bridge.chat_state.animation_mode {
			let animations = crate::chat_tui::AnimationType::all();
			let current_anim = animations[self.app.bridge.chat_state.current_animation_index];
			
			// Handle TachyonDemo special keys
			if current_anim == crate::chat_tui::AnimationType::TachyonDemo {
				match key.code {
					KeyCode::Char(' ') => {
						self.app.bridge.chat_state.tachyon_demo.restart_effect();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					KeyCode::Char('r') => {
						self.app.bridge.chat_state.tachyon_demo.random_effect(&mut self.app.bridge.chat_state.tachyon_rng);
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					KeyCode::Char('s') => {
						self.app.bridge.chat_state.tachyon_demo.scramble_effect();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					_ => {}
				}
			}
			// Handle navigation keys for animation carousel
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
					succ!()
				}
				KeyCode::Right => {
					// Next animation (but not Enter - Enter submits input)
					self.app.bridge.chat_state.current_animation_index = 
						(self.app.bridge.chat_state.current_animation_index + 1) % animations.len();
					self.app.bridge.chat_state.animation_start_time = Some(Instant::now());
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				_ => {
					// For other keys, fall through to handle input
					// This allows typing while viewing animations
				}
			}
		}
		// Handle chat input when in Chat mode or FilePicker mode (chat input is visible)
		if self.app.bridge.mode == crate::chat_tui::AppMode::Chat 
			|| self.app.bridge.mode == crate::chat_tui::AppMode::FilePicker 
		{
			// Handle scrolling when messages exist and input is empty
			if !self.app.bridge.chat_state.messages.is_empty() 
				&& self.app.bridge.chat_state.input.content.is_empty() 
			{
				match key.code {
					KeyCode::Up => {
						self.app.bridge.chat_state.chat_scroll_offset = 
							self.app.bridge.chat_state.chat_scroll_offset.saturating_sub(1);
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					KeyCode::Down => {
						self.app.bridge.chat_state.chat_scroll_offset += 1;
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					_ => {}
				}
			}
			// Handle thinking accordion toggle with '0' key
			if key.code == KeyCode::Char('0') && !self.app.bridge.chat_state.messages.is_empty() {
				// Toggle thinking expansion for the last assistant message
				for msg in self.app.bridge.chat_state.messages.iter_mut().rev() {
					if msg.role == crate::chat_tui::MessageRole::Assistant {
						msg.thinking_expanded = !msg.thinking_expanded;
						break;
					}
				}
				NEED_RENDER.store(1, Ordering::Relaxed);
				succ!()
			}
			// Route key to chat input
			let action = self.app.bridge.chat_state.input.handle_key(key);
			
			match action {
				InputAction::Submit(msg) => {
					// Add message to chat - this exits animation mode
					self.app.bridge.chat_state.add_user_message(msg);
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				InputAction::Exit => {
					// Show farewell train animation
					crate::chat_tui::exit_animation::show_train_farewell();
					// Exit the application
					std::process::exit(0);
				}
				InputAction::Changed => {
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				InputAction::PreviousHistory | InputAction::NextHistory => {
					// TODO: Implement history navigation
					succ!()
				}
				InputAction::None => {
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
			}
		}
		// Route to yazi's normal key handling
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
		
		// Update chat state (process LLM responses)
		self.app.bridge.chat_state.update();
		
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
		
		// Update TachyonDemo timing
		let elapsed = self.app.bridge.chat_state.last_frame_instant.elapsed();
		self.app.bridge.chat_state.tachyon_demo.update(elapsed);
		self.app.bridge.chat_state.last_frame_instant = Instant::now();
		
		NEED_RENDER.store(1, Ordering::Relaxed);
		succ!();
	}
}
