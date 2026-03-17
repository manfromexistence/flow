//! Animation screens module

#![allow(dead_code)]

pub mod bubbles;
pub mod fire;
pub mod plasma;
pub mod rainbow;
pub mod spinners;
pub mod starfield;

pub use bubbles::BubblesScreen;
pub use fire::FireScreen;
pub use plasma::PlasmaScreen;
pub use rainbow::RainbowScreen;
pub use spinners::SpinnersScreen;
pub use starfield::StarfieldScreen;

use std::io;
use std::thread;
use std::time::Duration;

/// Run all animation screens in sequence
#[allow(clippy::type_complexity)]
pub fn run_all_animations() -> io::Result<()> {
    let animations: Vec<(&str, Box<dyn Fn(usize) -> io::Result<()>>)> = vec![
        (
            "🔥 Fire Animation",
            Box::new(|frame| {
                let screen = FireScreen::new();
                screen.render(frame)
            }),
        ),
        (
            "🌊 Plasma Effect",
            Box::new(|frame| {
                let screen = PlasmaScreen::new();
                screen.render(frame)
            }),
        ),
        (
            "🫧 Bubbles",
            Box::new(|frame| {
                let screen = BubblesScreen::new();
                screen.render(frame)
            }),
        ),
        (
            "⏳ Spinners",
            Box::new(|frame| {
                let screen = SpinnersScreen::new();
                screen.render(frame)
            }),
        ),
        (
            "🌈 Rainbow Text",
            Box::new(|frame| {
                let screen = RainbowScreen::new();
                screen.render(frame)
            }),
        ),
        (
            "🌌 Starfield",
            Box::new(|frame| {
                let screen = StarfieldScreen::new();
                screen.render(frame)
            }),
        ),
    ];

    for (name, render_fn) in &animations {
        println!("Starting {}", name);

        // Run animation for 3 seconds (15 frames at 200ms each)
        for frame in 0..15 {
            render_fn(frame)?;
            thread::sleep(Duration::from_millis(200));
        }

        thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
