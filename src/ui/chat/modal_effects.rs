//! Modal animation effects using tachyonfx
//!
//! This module provides reusable animation effects for modals based on the
//! tachyonfx basic-effects example.

use ratatui::style::{Color, Style};
use std::time::Instant;
use tachyonfx::{
    CellFilter, Duration, Effect, Interpolation, IntoEffect, Motion, color_from_hsl,
    fx::{self, never_complete, parallel, sequence},
};

use super::app_state::ModalType;

/// Creates a sweep-in effect for modal opening
pub fn create_sweep_in_effect(bg: Color, screen_bg: Color) -> Effect {
    let duration = Duration::from_millis(750);
    fx::sweep_in(
        Motion::LeftToRight,
        30,
        0,
        screen_bg,
        (duration, Interpolation::QuadOut),
    )
}

/// Creates a coalesce effect for modal opening
pub fn create_coalesce_effect() -> Effect {
    let medium = Duration::from_millis(500);
    fx::coalesce((medium, Interpolation::CubicOut))
}

/// Creates a slide-in effect for modal opening
pub fn create_slide_in_effect(bg: Color, screen_bg: Color) -> Effect {
    let medium = Duration::from_millis(600);
    fx::parallel(&[
        fx::fade_from_fg(bg, (1200, Interpolation::ExpoInOut)),
        fx::slide_in(Motion::UpToDown, 20, 0, screen_bg, medium),
    ])
}

/// Creates a fade-in effect for modal opening
pub fn create_fade_in_effect(bg: Color) -> Effect {
    let duration = Duration::from_millis(400);
    fx::fade_from_fg(bg, (duration, Interpolation::QuadOut))
}

/// Creates a dissolve effect for modal closing
pub fn create_dissolve_effect(screen_bg: Color) -> Effect {
    let medium = Duration::from_millis(400);
    fx::dissolve_to(Style::default().bg(screen_bg), medium)
}

/// Creates a slide-out effect for modal closing
pub fn create_slide_out_effect(screen_bg: Color) -> Effect {
    let medium = Duration::from_millis(500);
    fx::slide_out(Motion::LeftToRight, 80, 0, screen_bg, medium)
}

/// Creates a sweep-out effect for modal closing
pub fn create_sweep_out_effect(bg: Color) -> Effect {
    fx::sweep_out(Motion::DownToUp, 5, 20, bg, (1000, Interpolation::QuadOut))
}

/// Creates an HSL color shift effect for modal content
pub fn create_hsl_shift_effect() -> Effect {
    let medium = Duration::from_millis(600);
    fx::sequence(&[
        fx::hsl_shift_fg([180.0, 0.0, 0.0], medium),
        fx::hsl_shift_fg([180.0, 0.0, 0.0], medium).reversed(),
    ])
}

/// Creates a custom color cycle effect for modal highlights
pub fn create_color_cycle_effect(base_color: Color) -> Effect {
    fx::effect_fn(
        Instant::now(),
        Duration::from_millis(2000),
        move |state, _ctx, cell_iter| {
            let cycle: f32 = (state.elapsed().as_millis() % 3600) as f32;

            cell_iter
                .filter(|(_, cell)| cell.symbol() != " ")
                .enumerate()
                .for_each(|(i, (_pos, cell))| {
                    let hue = (2.0 * i as f32 + cycle * 0.2) % 360.0;
                    let color = color_from_hsl(hue, 100.0, 50.0);
                    cell.set_fg(color);
                });
        },
    )
    .with_filter(CellFilter::FgColor(base_color))
}

/// Creates a cycling effect that shows all the basic-effects examples one by one
pub fn create_cycling_modal_effect() -> Effect {
    use tachyonfx::fx::{parallel, sequence};

    let medium = Duration::from_millis(750);
    let slow = Duration::from_millis(1250);

    // Create a sequence of all the effects from the basic-effects example
    fx::sequence(&[
        // 1. Sweep in
        fx::sweep_in(
            Motion::LeftToRight,
            30,
            0,
            Color::Black,
            (medium, Interpolation::QuadOut),
        ),
        fx::sleep(Duration::from_millis(500)),
        // 2. Coalesce
        fx::coalesce((medium, Interpolation::CubicOut)),
        fx::sleep(Duration::from_millis(500)),
        // 3. Slide in/out
        fx::parallel(&[
            fx::fade_from_fg(Color::DarkGray, (1200, Interpolation::ExpoInOut)),
            fx::slide_in(Motion::UpToDown, 20, 0, Color::Black, medium),
        ]),
        fx::sleep(Duration::from_millis(500)),
        // 4. HSL color shift
        fx::sequence(&[
            fx::hsl_shift_fg([180.0, 0.0, 0.0], medium),
            fx::hsl_shift_fg([180.0, 0.0, 0.0], medium).reversed(),
        ]),
        fx::sleep(Duration::from_millis(500)),
        // 5. Sweep out/in sequence
        fx::sequence(&[
            fx::sweep_out(
                Motion::DownToUp,
                5,
                20,
                Color::DarkGray,
                (1000, Interpolation::QuadOut),
            ),
            fx::sweep_in(
                Motion::UpToDown,
                5,
                20,
                Color::DarkGray,
                (1000, Interpolation::QuadOut),
            ),
        ]),
    ])
}
/// Get the appropriate opening effect for a modal type
pub fn get_modal_open_effect(modal_type: ModalType, bg: Color, screen_bg: Color) -> Effect {
    match modal_type {
        ModalType::Focus => create_sweep_in_effect(bg, screen_bg),
        ModalType::Add => create_slide_in_effect(bg, screen_bg),
        ModalType::Plan => create_coalesce_effect(),
        ModalType::Model => create_cycling_modal_effect(), // Use cycling effect for model modal
        ModalType::Local => create_fade_in_effect(bg),
        ModalType::Changes => create_sweep_in_effect(bg, screen_bg),
        ModalType::Tasks => create_slide_in_effect(bg, screen_bg),
        ModalType::Agents => create_coalesce_effect(),
        ModalType::Memory => create_fade_in_effect(bg),
        ModalType::Tools => create_sweep_in_effect(bg, screen_bg),
        ModalType::More => create_slide_in_effect(bg, screen_bg),
        ModalType::GoogleApi => create_fade_in_effect(bg),
        ModalType::ElevenlabsApi => create_fade_in_effect(bg),
        ModalType::EffectsDemo => create_coalesce_effect(),
    }
}

/// Get the appropriate closing effect for a modal type
pub fn get_modal_close_effect(modal_type: ModalType, bg: Color, screen_bg: Color) -> Effect {
    match modal_type {
        ModalType::Focus => create_dissolve_effect(screen_bg),
        ModalType::Add => create_slide_out_effect(screen_bg),
        ModalType::Plan => create_dissolve_effect(screen_bg),
        ModalType::Model => create_sweep_out_effect(bg),
        ModalType::Local => create_dissolve_effect(screen_bg),
        ModalType::Changes => create_slide_out_effect(screen_bg),
        ModalType::Tasks => create_dissolve_effect(screen_bg),
        ModalType::Agents => create_slide_out_effect(screen_bg),
        ModalType::Memory => create_dissolve_effect(screen_bg),
        ModalType::Tools => create_sweep_out_effect(bg),
        ModalType::More => create_dissolve_effect(screen_bg),
        ModalType::GoogleApi => create_dissolve_effect(screen_bg),
        ModalType::ElevenlabsApi => create_dissolve_effect(screen_bg),
        ModalType::EffectsDemo => create_dissolve_effect(screen_bg),
    }
}

/// Creates a pulsing highlight effect for selected items
pub fn create_pulse_effect(accent_color: Color) -> Effect {
    let duration = Duration::from_millis(1000);
    fx::effect_fn(Instant::now(), duration, move |state, _ctx, cell_iter| {
        let elapsed = state.elapsed().as_millis() as f32;
        let cycle = (elapsed % 1000.0) / 1000.0;
        let opacity = 0.5 + 0.5 * (cycle * std::f32::consts::PI * 2.0).sin();

        cell_iter.for_each(|(_pos, cell)| {
            if let Color::Rgb(r, g, b) = accent_color {
                let new_r = (r as f32 * opacity) as u8;
                let new_g = (g as f32 * opacity) as u8;
                let new_b = (b as f32 * opacity) as u8;
                cell.set_bg(Color::Rgb(new_r, new_g, new_b));
            }
        });
    })
}

/// Creates a shimmer effect for search boxes
pub fn create_search_shimmer_effect(accent_color: Color) -> Effect {
    let duration = Duration::from_millis(1500);
    fx::effect_fn(Instant::now(), duration, move |state, _ctx, cell_iter| {
        let elapsed = state.elapsed().as_millis() as f32;
        let cycle = (elapsed % 1500.0) / 1500.0;

        cell_iter.enumerate().for_each(|(i, (_pos, cell))| {
            let offset = (i as f32 * 0.1 + cycle) % 1.0;
            let brightness = 0.6 + 0.4 * (offset * std::f32::consts::PI * 2.0).sin();

            if let Color::Rgb(r, g, b) = accent_color {
                let new_r = (r as f32 * brightness) as u8;
                let new_g = (g as f32 * brightness) as u8;
                let new_b = (b as f32 * brightness) as u8;
                cell.set_fg(Color::Rgb(new_r, new_g, new_b));
            }
        });
    })
}
