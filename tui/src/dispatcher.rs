use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use fb_actor::Ctx;
use fb_config::keymap::Key;
use fb_macro::{act, emit, succ};
use fb_shared::{
	data::Data,
	event::{ActionCow, Event, NEED_RENDER},
};
use fb_widgets::input::InputMode;
use tracing::warn;

use crate::file_browser::{Executor, Router, app::App};

// Helper function to format key events into readable shortcut strings
fn format_key_event(key: &KeyEvent) -> String {
	let mut parts = Vec::new();

	// For Char keys, check if Shift is needed (uppercase letters need Shift)
	let is_char = matches!(key.code, KeyCode::Char(_));
	let needs_shift = if let KeyCode::Char(c) = key.code {
		c.is_uppercase() || "!@#$%^&*()_+{}|:\"<>?".contains(c)
	} else {
		false
	};

	if key.modifiers.contains(KeyModifiers::CONTROL) {
		parts.push("Ctrl");
	}

	// Only add Shift if it's not a char, or if it's a char that needs explicit Shift
	if key.modifiers.contains(KeyModifiers::SHIFT) && (!is_char || needs_shift) {
		parts.push("Shift");
	}

	if key.modifiers.contains(KeyModifiers::ALT) {
		parts.push("Alt");
	}

	let key_str = match key.code {
		KeyCode::Char(c) => {
			// Always use uppercase for letters
			if c.is_alphabetic() { c.to_uppercase().to_string() } else { c.to_string() }
		}
		KeyCode::F(n) => format!("F{}", n),
		KeyCode::Backspace => "Backspace".to_string(),
		KeyCode::Enter => "Enter".to_string(),
		KeyCode::Left => "Left".to_string(),
		KeyCode::Right => "Right".to_string(),
		KeyCode::Up => "Up".to_string(),
		KeyCode::Down => "Down".to_string(),
		KeyCode::Home => "Home".to_string(),
		KeyCode::End => "End".to_string(),
		KeyCode::PageUp => "PageUp".to_string(),
		KeyCode::PageDown => "PageDown".to_string(),
		KeyCode::Tab => "Tab".to_string(),
		KeyCode::BackTab => "BackTab".to_string(),
		KeyCode::Delete => "Delete".to_string(),
		KeyCode::Insert => "Insert".to_string(),
		KeyCode::Esc => "Esc".to_string(),
		_ => return "Unknown".to_string(),
	};

	parts.push(&key_str);
	parts.join("+")
}

pub(super) struct Dispatcher<'a> {
	app: &'a mut App,
}

impl<'a> Dispatcher<'a> {
	#[inline]
	pub(super) fn new(app: &'a mut App) -> Self {
		Self { app }
	}

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
		use crate::input::InputAction;
		use crate::menu::MenuAction;
		use crossterm::event::KeyCode;

		// PRIORITY 1: Global menu navigation keys - work when menu is visible on ANY screen
		if self.app.bridge.chat_state.show_tachyon_menu {
			// Check if we're in recording mode in keyboard shortcuts submenu
			if self.app.bridge.chat_state.menu.recording_mode
				&& self.app.bridge.chat_state.menu.current_submenu == Some(1)
			{
				// Get the selected shortcut index (skip "Back" and "Toggle Recording Mode")
				if let Some(action_index) = self.app.bridge.chat_state.menu.get_selected_shortcut_index() {
					// Format the key press into a shortcut string
					let shortcut = format_key_event(&key);

					// Don't record navigation keys or special menu keys
					if !matches!(
						key.code,
						KeyCode::Up
							| KeyCode::Down
							| KeyCode::PageUp
							| KeyCode::PageDown
							| KeyCode::Home
							| KeyCode::End
							| KeyCode::Esc
							| KeyCode::Enter
							| KeyCode::Char('j')
							| KeyCode::Char('k')
							| KeyCode::Char('g')
							| KeyCode::Char('G')
					) {
						// Update the keyboard shortcut
						self.app.bridge.chat_state.menu.update_keyboard_shortcut(action_index, shortcut);
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
				}
			}

			match key.code {
				KeyCode::Up | KeyCode::Char('k') => {
					self.app.bridge.chat_state.menu.select_prev_menu_item();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self
							.app
							.bridge
							.chat_state
							.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Down | KeyCode::Char('j') => {
					self.app.bridge.chat_state.menu.select_next_menu_item();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self
							.app
							.bridge
							.chat_state
							.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::PageUp => {
					self.app.bridge.chat_state.menu.page_up(10);
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self
							.app
							.bridge
							.chat_state
							.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::PageDown => {
					self.app.bridge.chat_state.menu.page_down(10);
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self
							.app
							.bridge
							.chat_state
							.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Home | KeyCode::Char('g') => {
					self.app.bridge.chat_state.menu.jump_to_top();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self
							.app
							.bridge
							.chat_state
							.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::End | KeyCode::Char('G') => {
					self.app.bridge.chat_state.menu.jump_to_bottom();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self
							.app
							.bridge
							.chat_state
							.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Char('t') | KeyCode::Char('T') => {
					// Toggle light/dark mode when in theme submenu
					if self.app.bridge.chat_state.menu.current_submenu == Some(0) {
						self.app.bridge.chat_state.toggle_theme_mode();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
				}
				KeyCode::Enter => {
					// Check if toggle mode button is selected
					if self.app.bridge.chat_state.menu.is_toggle_mode_selected() {
						// Toggle the theme mode
						self.app.bridge.chat_state.toggle_theme_mode();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}

					// Check if toggle recording button is selected
					if self.app.bridge.chat_state.menu.is_toggle_recording_selected() {
						// Toggle the recording mode
						self.app.bridge.chat_state.menu.toggle_recording_mode();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}

					// Get the current theme name before selecting
					let theme_name = self.app.bridge.chat_state.menu.get_selected_theme_name();

					// Select current menu item (enter submenu or execute action)
					let _should_close = !self.app.bridge.chat_state.menu.select_current_item();

					// If we were in theme submenu and selected a theme, just close the menu
					// (theme is already applied from navigation/hover)
					if theme_name.is_some() {
						self.app.bridge.chat_state.menu_is_closing = true;
						self.app.bridge.chat_state.menu.pick_closing_effect();
						self.app.bridge.chat_state.show_tachyon_menu = false;
					}

					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Esc => {
					// Go back to main menu if in submenu, otherwise close menu
					if self.app.bridge.chat_state.menu.current_submenu.is_some() {
						self.app.bridge.chat_state.menu.go_back_to_main();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					} else {
						// Close menu
						self.app.bridge.chat_state.menu_is_closing = true;
						self.app.bridge.chat_state.menu.pick_closing_effect();
						self.app.bridge.chat_state.show_tachyon_menu = false;
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
				}
				_ => {}
			}
		}

		// PRIORITY 2: If in animation mode, handle navigation keys but allow typing
		if self.app.bridge.chat_state.animation_mode {
			let all_animations = crate::AnimationType::all();
			let current_anim = all_animations[self.app.bridge.chat_state.current_animation_index];

			// Handle navigation keys based on current screen
			match key.code {
				KeyCode::Left => {
					if current_anim == crate::AnimationType::Splash {
						// From Splash → Go to first carousel animation (Matrix)
						let carousel = crate::AnimationType::carousel_animations();
						// Find Matrix in all_animations
						if let Some(matrix_idx) = all_animations.iter().position(|a| *a == carousel[0]) {
							self.app.bridge.chat_state.current_animation_index = matrix_idx;
						}
					} else if current_anim == crate::AnimationType::Yazi {
						// From Yazi → Go back to Splash
						self.app.bridge.chat_state.current_animation_index = 0;
					} else {
						// In carousel → Navigate to previous carousel animation
						let carousel = crate::AnimationType::carousel_animations();
						if let Some(current_carousel_idx) = carousel.iter().position(|a| *a == current_anim) {
							let prev_carousel_idx = if current_carousel_idx == 0 {
								carousel.len() - 1
							} else {
								current_carousel_idx - 1
							};
							let prev_anim = carousel[prev_carousel_idx];
							// Find this animation in all_animations
							if let Some(idx) = all_animations.iter().position(|a| *a == prev_anim) {
								self.app.bridge.chat_state.current_animation_index = idx;
							}
						}
					}
					self.app.bridge.chat_state.animation_start_time = Some(Instant::now());
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Right => {
					if current_anim == crate::AnimationType::Splash {
						// From Splash → Go to Yazi (file browser)
						if let Some(yazi_idx) =
							all_animations.iter().position(|a| *a == crate::AnimationType::Yazi)
						{
							self.app.bridge.chat_state.current_animation_index = yazi_idx;
						}
					} else if current_anim == crate::AnimationType::Yazi {
						// From Yazi → Go back to Splash
						self.app.bridge.chat_state.current_animation_index = 0;
					} else {
						// In carousel → Go back to Splash
						self.app.bridge.chat_state.current_animation_index = 0;
					}
					self.app.bridge.chat_state.animation_start_time = Some(Instant::now());
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Up => {
					// Set current animation as intro animation (only in carousel)
					if current_anim != crate::AnimationType::Splash
						&& current_anim != crate::AnimationType::Yazi
					{
						self.app.bridge.chat_state.intro_animation = current_anim;
						let anim_name = current_anim.name();
						self.app.bridge.chat_state.show_toast(format!("✓ Intro: {}", anim_name));
						NEED_RENDER.store(1, Ordering::Relaxed);
					}
					succ!()
				}
				KeyCode::Down => {
					// Set current animation as outro animation (only in carousel)
					if current_anim != crate::AnimationType::Splash
						&& current_anim != crate::AnimationType::Yazi
					{
						self.app.bridge.chat_state.outro_animation = current_anim;
						let anim_name = current_anim.name();
						self.app.bridge.chat_state.show_toast(format!("✓ Outro: {}", anim_name));
						NEED_RENDER.store(1, Ordering::Relaxed);
					}
					succ!()
				}
				_ => {
					// For other keys, fall through to handle input
					// This allows typing while viewing animations
				}
			}
		}

		// Global '0' key handler - toggle menu overlay on ANY screen
		if key.code == KeyCode::Char('0') {
			if self.app.bridge.chat_state.show_tachyon_menu {
				// Closing menu - pick random closing animation
				self.app.bridge.chat_state.menu_is_closing = true;
				self.app.bridge.chat_state.menu.pick_closing_effect();
				self.app.bridge.chat_state.show_tachyon_menu = false;
			} else {
				// Opening menu - pick random opening animation
				self.app.bridge.chat_state.menu_is_closing = false;
				self.app.bridge.chat_state.show_tachyon_menu = true;
				self.app.bridge.chat_state.menu.pick_opening_effect();
			}

			NEED_RENDER.store(1, Ordering::Relaxed);
			succ!()
		}

		// Global keyboard shortcuts - check if any registered shortcut matches
		let pressed_key = format_key_event(&key);
		let mappings = &self.app.bridge.chat_state.menu.keyboard_mappings;

		// Check each action to see if its shortcut matches
		for action in MenuAction::all_actions() {
			let shortcut = mappings.get(action);

			// Handle shortcuts with "or" (e.g., "0 or Ctrl+P")
			let matches = if shortcut.contains(" or ") {
				shortcut.split(" or ").any(|s| s.trim() == pressed_key)
			} else {
				shortcut == pressed_key
			};

			if matches {
				let submenu_index = match action {
					MenuAction::ContextControlPanel => None, // Special case - just toggle menu
					MenuAction::Theme => Some(0),
					MenuAction::KeyboardShortcuts => Some(1),
					MenuAction::Providers => Some(2),
					MenuAction::PluginsApps => Some(3),
					MenuAction::Skills => Some(4),
					MenuAction::Sandbox => Some(5),
					MenuAction::WebSearch => Some(6),
					MenuAction::McpServers => Some(7),
					MenuAction::MemoryHistory => Some(8),
					MenuAction::MultiAgent => Some(9),
					MenuAction::Notifications => Some(10),
					MenuAction::VoiceRealtime => Some(11),
					MenuAction::ImageVision => Some(12),
					MenuAction::Profiles => Some(13),
					MenuAction::Worktree => Some(14),
					MenuAction::Authentication => Some(15),
					MenuAction::NetworkProxy => Some(16),
					MenuAction::HooksEvents => Some(17),
					MenuAction::SessionResume => Some(18),
					MenuAction::ApprovalPolicy => Some(19),
					MenuAction::ShellEnvironment => Some(20),
					MenuAction::ExecutionRules => Some(21),
					MenuAction::ProjectTrust => Some(22),
					MenuAction::DeveloperInstructions => Some(23),
					MenuAction::FeatureFlags => Some(24),
				};

				// Check if menu is already open with this submenu
				let is_same_submenu = if let Some(idx) = submenu_index {
					self.app.bridge.chat_state.show_tachyon_menu
						&& self.app.bridge.chat_state.menu.current_submenu == Some(idx)
						&& self.app.bridge.chat_state.menu.opened_directly
				} else {
					// For ContextControlPanel, check if menu is open at main level
					self.app.bridge.chat_state.show_tachyon_menu
						&& self.app.bridge.chat_state.menu.current_submenu.is_none()
				};

				if is_same_submenu {
					// Toggle off - close the menu
					self.app.bridge.chat_state.menu_is_closing = true;
					self.app.bridge.chat_state.menu.pick_closing_effect();
					self.app.bridge.chat_state.show_tachyon_menu = false;
				} else {
					// Open menu if not already open
					if !self.app.bridge.chat_state.show_tachyon_menu {
						self.app.bridge.chat_state.menu_is_closing = false;
						self.app.bridge.chat_state.show_tachyon_menu = true;
						self.app.bridge.chat_state.menu.pick_opening_effect();
					}

					// Navigate to the submenu directly (without "Back" button)
					if let Some(idx) = submenu_index {
						self.app.bridge.chat_state.menu.enter_submenu_directly(idx);
					} else {
						// ContextControlPanel - go to main menu
						self.app.bridge.chat_state.menu.go_back_to_main();
					}
				}

				NEED_RENDER.store(1, Ordering::Relaxed);
				succ!()
			}
		}

		// Handle chat input when in Chat mode or FilePicker mode (chat input is visible)
		if self.app.bridge.mode == crate::AppMode::Chat
			|| self.app.bridge.mode == crate::AppMode::FilePicker
		{
			// Handle Space key for voice mode - hybrid hold detection
			if key.code == KeyCode::Char(' ') && key.modifiers.is_empty() {
				use crossterm::event::KeyEventKind;
				
				let now = Instant::now();
				let is_repeat = key.kind == KeyEventKind::Repeat;
				
				// Check if this is a rapid repeat using timing (fallback for terminals without enhancement flags)
				let is_timing_repeat = if let Some(last_press) = self.app.bridge.chat_state.last_space_press {
					last_press.elapsed() < Duration::from_millis(100)
				} else {
					false
				};
				
				if is_repeat || is_timing_repeat {
					// Key is being held! Activate voice mode (spinner)
					if !self.app.bridge.chat_state.space_held {
						// Save cursor position before reverting (this is where cursor is AFTER typing space)
						let old_cursor_pos = self.app.bridge.chat_state.input.cursor_position;
						
						self.app.bridge.chat_state.cursor_revert_from_pos = old_cursor_pos;
						
						// Remove ALL trailing spaces that were typed during the hold detection
						// We need to remove potentially 2 spaces: the first press + the repeat that triggered detection
						let mut new_pos = old_cursor_pos;
						let mut spaces_removed = 0;
						
						while new_pos > 0 && spaces_removed < 2 {
							let content_before = &self.app.bridge.chat_state.input.content[..new_pos];
							if content_before.ends_with(' ') {
								new_pos -= 1;
								self.app.bridge.chat_state.input.content.remove(new_pos);
								spaces_removed += 1;
							} else {
								break;
							}
						}
						
						if spaces_removed > 0 {
							self.app.bridge.chat_state.input.cursor_position = new_pos;
						}
						
						// Start cursor revert animation
						self.app.bridge.chat_state.cursor_revert_animation = true;
						self.app.bridge.chat_state.cursor_revert_start = Some(now);
						
						// Activate voice mode
						self.app.bridge.chat_state.space_held = true;
						self.app.bridge.chat_state.space_hold_start = Some(now);
					}
					self.app.bridge.chat_state.last_space_press = Some(now);
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!() // Don't type spaces while holding
				} else if key.kind == KeyEventKind::Release {
					// Space released - deactivate voice mode
					if self.app.bridge.chat_state.space_held {
						self.app.bridge.chat_state.space_held = false;
						self.app.bridge.chat_state.space_hold_start = None;
						self.app.bridge.chat_state.last_space_press = None;
						self.app.bridge.chat_state.cursor_revert_animation = false;
						self.app.bridge.chat_state.cursor_revert_start = None;
						NEED_RENDER.store(1, Ordering::Relaxed);
					}
					succ!()
				} else {
					// First press - type the space normally
					self.app.bridge.chat_state.last_space_press = Some(now);
					let action = self.app.bridge.chat_state.input.handle_key(key);
					match action {
						InputAction::Changed => {
							NEED_RENDER.store(1, Ordering::Relaxed);
							succ!()
						}
						_ => succ!()
					}
				}
			} else {
				// Any other key pressed - deactivate voice mode
				if self.app.bridge.chat_state.space_held {
					self.app.bridge.chat_state.space_held = false;
					self.app.bridge.chat_state.space_hold_start = None;
					self.app.bridge.chat_state.last_space_press = None;
					self.app.bridge.chat_state.cursor_revert_animation = false;
					self.app.bridge.chat_state.cursor_revert_start = None;
					NEED_RENDER.store(1, Ordering::Relaxed);
				}
			}

			// If voice mode is active (spinner showing), don't process any input - just return
			if self.app.bridge.chat_state.space_held {
				NEED_RENDER.store(1, Ordering::Relaxed);
				succ!()
			}

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
					// If we have messages (in chat mode), play outro animation first
					if !self.app.bridge.chat_state.messages.is_empty() {
						self.app.bridge.chat_state.play_outro_animation();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					} else {
						// No messages, just show farewell and exit
						crate::exit_animation::show_train_farewell();
						std::process::exit(0);
					}
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
		use crossterm::event::MouseEventKind;

		// Handle menu mouse events globally when menu is visible
		if self.app.bridge.chat_state.show_tachyon_menu {
			match mouse.kind {
				MouseEventKind::Moved => {
					// Handle hover - always process and render if state changed
					if self.app.bridge.chat_state.menu.handle_mouse(mouse.column, mouse.row, false) {
						// Apply theme preview if hovering over a theme
						if let Some(theme_name) = self.app.bridge.chat_state.menu.get_hovered_theme_name() {
							self
								.app
								.bridge
								.chat_state
								.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
						}
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
				}
				MouseEventKind::Down(_) => {
					// Handle click - select and potentially enter submenu
					if self.app.bridge.chat_state.menu.handle_mouse(mouse.column, mouse.row, true) {
						// Check if toggle mode button is clicked
						if self.app.bridge.chat_state.menu.is_toggle_mode_selected() {
							// Toggle the theme mode
							self.app.bridge.chat_state.toggle_theme_mode();
							NEED_RENDER.store(1, Ordering::Relaxed);
							succ!()
						}

						// Check if toggle recording button is clicked
						if self.app.bridge.chat_state.menu.is_toggle_recording_selected() {
							// Toggle the recording mode
							self.app.bridge.chat_state.menu.toggle_recording_mode();
							NEED_RENDER.store(1, Ordering::Relaxed);
							succ!()
						}

						// Get the current theme name before selecting
						let theme_name = self.app.bridge.chat_state.menu.get_selected_theme_name();

						// Item was clicked - now select it (enter submenu or execute)
						let _should_close = !self.app.bridge.chat_state.menu.select_current_item();

						// If we were in theme submenu and clicked a theme, just close the menu
						// (theme is already applied from hover)
						if theme_name.is_some() {
							self.app.bridge.chat_state.menu_is_closing = true;
							self.app.bridge.chat_state.menu.pick_closing_effect();
							self.app.bridge.chat_state.show_tachyon_menu = false;
						}

						NEED_RENDER.store(1, Ordering::Relaxed);
					}
				}
				MouseEventKind::ScrollUp => {
					// Scroll up (previous items)
					self.app.bridge.chat_state.menu.select_prev_menu_item();
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				MouseEventKind::ScrollDown => {
					// Scroll down (next items)
					self.app.bridge.chat_state.menu.select_next_menu_item();
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				_ => {}
			}
		}

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
			let animations = crate::AnimationType::all();
			let current_anim = animations[self.app.bridge.chat_state.current_animation_index];
			if current_anim == crate::AnimationType::Splash {
				self.app.bridge.chat_state.splash_font_index =
					(self.app.bridge.chat_state.splash_font_index + 1) % 113; // 113 valid fonts
				self.app.bridge.chat_state.last_font_change = Instant::now();
			}
		}

		// Update Menu timing
		let elapsed = self.app.bridge.chat_state.last_frame_instant.elapsed();
		self.app.bridge.chat_state.menu.update(elapsed);
		self.app.bridge.chat_state.last_frame_instant = Instant::now();

		NEED_RENDER.store(1, Ordering::Relaxed);
		succ!();
	}
}
