//! Main rendering module - coordinates all rendering logic

mod animations;
mod controls;
mod input;
// mod suggestions; // Commented out - autocomplete disabled

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::Widget,
};

use crate::app::{AnimationType, ChatApp};
use crate::components::MessageList;

impl ChatApp {
    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        // Update tachyon effects timing
        let _elapsed = self.last_render.elapsed();

        // Both animations show in chat area only, keeping input visible
        if self.show_train_animation || self.show_matrix_animation {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            self.input_area = chunks[1];

            // Render appropriate animation in the chat area
            if self.show_train_animation {
                self.render_train_animation_in_area(chunks[0], frame);
            } else if self.show_matrix_animation {
                self.render_matrix_animation_in_area(chunks[0], frame);
            }

            // Still render the input bar and bottom controls
            self.render_input_box(chunks[1], frame.buffer_mut());

            // Render autocomplete suggestions if visible
            // if self.autocomplete.suggestion_list().is_visible() {
            //     self.render_suggestions(frame, chunks[1]);
            // }

            let (plan_area, model_area, _token_area, local_area) =
                self.render_bottom_controls(chunks[2], frame.buffer_mut());

            self.plan_button_area = plan_area;
            self.model_button_area = model_area;
            self.local_button_area = local_area;
            return;
        }

        // Animation carousel mode
        if self.animation_mode {
            let animations = AnimationType::all();
            let current_anim = animations[self.current_animation_index];

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            self.input_area = chunks[1];

            // Render the current animation in the chat area
            match current_anim {
                AnimationType::Splash => {
                    crate::splash::render(
                        chunks[0],
                        frame.buffer_mut(),
                        &self.theme,
                        self.splash_font_index,
                        &self.rainbow_animation,
                    );
                }
                AnimationType::Matrix => {
                    self.render_matrix_animation_in_area(chunks[0], frame);
                }
                AnimationType::Train => {
                    self.render_train_animation_in_area(chunks[0], frame);
                }
                AnimationType::Confetti => {
                    self.render_confetti_animation_in_area(chunks[0], frame);
                }
                AnimationType::GameOfLife => {
                    self.render_gameoflife_animation_in_area(chunks[0], frame);
                }
                AnimationType::Starfield => {
                    self.render_starfield_animation_in_area(chunks[0], frame);
                }
                AnimationType::Rain => {
                    self.render_rain_animation_in_area(chunks[0], frame);
                }
                AnimationType::NyanCat => {
                    self.render_nyancat_animation_in_area(chunks[0], frame);
                }
                AnimationType::DVDLogo => {
                    self.render_dvdlogo_animation_in_area(chunks[0], frame);
                }
                AnimationType::TachyonDemo => {
                    self.tachyon_demo.render(frame);
                }
                AnimationType::Fire => {
                    self.render_fire_animation_in_area(chunks[0], frame);
                }
                AnimationType::Plasma => {
                    self.render_plasma_animation_in_area(chunks[0], frame);
                }
                AnimationType::Spinners => {
                    self.render_spinners_animation_in_area(chunks[0], frame);
                }
                AnimationType::Waves => {
                    self.render_waves_animation_in_area(chunks[0], frame);
                }
                AnimationType::Fireworks => {
                    self.render_fireworks_animation_in_area(chunks[0], frame);
                }
            }

            // Render input box and bottom controls
            self.render_input_box(chunks[1], frame.buffer_mut());

            // Render autocomplete suggestions if visible
            // if self.autocomplete.suggestion_list().is_visible() {
            //     self.render_suggestions(frame, chunks[1]);
            // }

            let (plan_area, model_area, _token_area, local_area) =
                self.render_bottom_controls(chunks[2], frame.buffer_mut());

            self.plan_button_area = plan_area;
            self.model_button_area = model_area;
            self.local_button_area = local_area;
            return;
        }

        if self.show_dx_splash {
            // Show DX splash screen
            crate::splash::render(
                frame.area(),
                frame.buffer_mut(),
                &self.theme,
                self.splash_font_index,
                &self.rainbow_animation,
            );
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(frame.area());

        self.input_area = chunks[1];

        if self.messages.is_empty() {
            crate::splash::render(
                chunks[0],
                frame.buffer_mut(),
                &self.theme,
                self.splash_font_index,
                &self.rainbow_animation,
            );

            // Show TachyonFX modal on top of splash screen
            if self.show_tachyon_modal {
                self.tachyon_demo.render(frame);
            }
        } else {
            MessageList::with_effects(
                &self.messages,
                &self.theme,
                self.chat_scroll_offset,
                &self.shimmer,
                &self.typing_indicator,
            )
            .render(chunks[0], frame.buffer_mut());
        }

        self.render_input_box(chunks[1], frame.buffer_mut());

        // Render autocomplete suggestions if visible
        // if self.autocomplete.suggestion_list().is_visible() {
        //     self.render_suggestions(frame, chunks[1]);
        // }

        let (plan_area, model_area, _token_area, local_area) =
            self.render_bottom_controls(chunks[2], frame.buffer_mut());

        self.plan_button_area = plan_area;
        self.model_button_area = model_area;
        self.local_button_area = local_area;
    }
}
