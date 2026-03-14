// ============================================================================
// AI Coding Assistant Side View (no input field - you already have it in main TUI)
// Features: Todo/Task sidebar + Tabs (Diff | Markdown | Code Preview) + Effects
// ============================================================================

use std::{
    error::Error,
    io::{self, Stdout},
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListItem, ListState, Paragraph, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Tabs,
    },
};
use similar::{ChangeTag, TextDiff};
use syntect::{
    easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet, util::LinesWithEndings,
};
use tachyonfx::{
    CellFilter, Duration as FxDuration, Effect, EffectRenderer, Interpolation, IntoEffect, Motion,
    SimpleRng, color_from_hsl, fx,
};

mod palette {
    use ratatui::style::Color;

    pub const BG_DARKEST: Color = Color::Rgb(17, 17, 27);
    pub const BG_SURFACE: Color = Color::Rgb(30, 30, 46);

    pub const TEXT: Color = Color::Rgb(205, 214, 244);
    pub const TEXT_DIM: Color = Color::Rgb(147, 153, 178);
    pub const TEXT_MUTED: Color = Color::Rgb(108, 112, 134);

    pub const BLUE: Color = Color::Rgb(137, 180, 250);
    pub const YELLOW: Color = Color::Rgb(249, 226, 175);

    pub const DIFF_ADD_BG: Color = Color::Rgb(30, 50, 30);
    pub const DIFF_DEL_BG: Color = Color::Rgb(55, 25, 25);
    pub const DIFF_ADD_FG: Color = Color::Rgb(166, 227, 161);
    pub const DIFF_DEL_FG: Color = Color::Rgb(243, 139, 168);

    pub const BORDER_ACTIVE: Color = Color::Rgb(137, 180, 250);
    pub const SUCCESS: Color = Color::Rgb(166, 227, 161);
    pub const ERROR: Color = Color::Rgb(243, 139, 168);
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ActivePanel {
    Sidebar,
    MainContent,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ContentTab {
    Diff,
    Markdown,
    CodePreview,
}

#[derive(Debug, Clone)]
struct Task {
    label: String,
    status: TaskStatus,
    detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TaskStatus {
    Done,
    InProgress,
    Pending,
    Failed,
}

impl TaskStatus {
    fn icon(&self) -> &'static str {
        match self {
            TaskStatus::Done => "✓",
            TaskStatus::InProgress => "◐",
            TaskStatus::Pending => "○",
            TaskStatus::Failed => "✗",
        }
    }

    fn color(&self) -> Color {
        match self {
            TaskStatus::Done => palette::SUCCESS,
            TaskStatus::InProgress => palette::YELLOW,
            TaskStatus::Pending => palette::TEXT_MUTED,
            TaskStatus::Failed => palette::ERROR,
        }
    }
}

struct App {
    active_panel: ActivePanel,
    content_tab: ContentTab,
    task_list_state: ListState,
    tasks: Vec<Task>,
    show_modal: bool,
    show_tachyon_modal: bool,
    modal_task_idx: usize,

    diff_scroll: u16,
    md_scroll: u16,
    code_scroll: u16,

    startup_effect: Option<Effect>,
    modal_effect: Option<Effect>,
    tachyon_modal_effect: Option<Effect>,
    tachyon_active_effect_idx: usize,

    syntax_set: SyntaxSet,
    theme_set: ThemeSet,

    last_tick: FxDuration,
}

struct TachyonEffectsRepository {
    effects: Vec<(&'static str, Effect)>,
}

impl TachyonEffectsRepository {
    fn new() -> Self {
        let screen_bg = palette::BG_DARKEST;
        let bg = palette::BG_SURFACE;

        let slow = FxDuration::from_millis(1250);
        let medium = FxDuration::from_millis(750);

        // fx from lambdas
        let custom_color_cycle = fx::effect_fn(Instant::now(), slow, |state, _ctx, cell_iter| {
            let cycle: f32 = (state.elapsed().as_millis() % 3600) as f32;

            cell_iter
                .filter(|(_, cell)| cell.symbol() != " ")
                .enumerate()
                .for_each(|(i, (_pos, cell))| {
                    let hue = (2.0 * i as f32 + cycle * 0.2) % 360.0;
                    let color = color_from_hsl(hue, 100.0, 50.0);
                    cell.set_fg(color);
                });
        })
        .with_filter(CellFilter::FgColor(palette::TEXT));

        let effects = vec![
            (
                "sweep in",
                fx::sweep_in(
                    Motion::LeftToRight,
                    30,
                    0,
                    screen_bg,
                    (slow, Interpolation::QuadOut),
                ),
            ),
            (
                "smooth expand and reversed",
                fx::sequence(&[
                    fx::expand(
                        tachyonfx::fx::ExpandDirection::Vertical,
                        Style::new().fg(bg).bg(screen_bg),
                        1200,
                    ),
                    fx::sleep(slow),
                    fx::expand(
                        tachyonfx::fx::ExpandDirection::Horizontal,
                        Style::new().fg(bg).bg(screen_bg),
                        1200,
                    )
                    .reversed(),
                ]),
            ),
            (
                "irregular sweep out/sweep in",
                fx::sequence(&[
                    fx::sweep_out(Motion::DownToUp, 5, 20, bg, (2000, Interpolation::QuadOut)),
                    fx::sweep_in(Motion::UpToDown, 5, 20, bg, (2000, Interpolation::QuadOut)),
                    fx::sweep_out(Motion::UpToDown, 5, 20, bg, (2000, Interpolation::QuadOut)),
                    fx::sweep_in(Motion::DownToUp, 5, 20, bg, (2000, Interpolation::QuadOut)),
                ]),
            ),
            (
                "coalesce",
                fx::sequence(&[
                    fx::coalesce((medium, Interpolation::CubicOut)),
                    fx::sleep(medium),
                    fx::prolong_end(
                        medium,
                        fx::dissolve_to(Style::default().bg(screen_bg), medium),
                    ),
                ]),
            ),
            (
                "slide in/out",
                fx::repeating(fx::sequence(&[
                    fx::parallel(&[
                        fx::fade_from_fg(bg, (2000, Interpolation::ExpoInOut)),
                        fx::slide_in(Motion::UpToDown, 20, 0, screen_bg, medium),
                    ]),
                    fx::sleep(medium),
                    fx::prolong_end(
                        medium,
                        fx::slide_out(Motion::LeftToRight, 80, 0, screen_bg, medium),
                    ),
                ])),
            ),
            (
                "change hue, saturation and lightness",
                fx::sequence(&[
                    fx::hsl_shift_fg([360.0, 0.0, 0.0], medium),
                    fx::hsl_shift_fg([0.0, -100.0, 0.0], medium),
                    fx::hsl_shift_fg([0.0, -100.0, 0.0], medium).reversed(),
                    fx::hsl_shift_fg([0.0, 100.0, 0.0], medium),
                    fx::hsl_shift_fg([0.0, 100.0, 0.0], medium).reversed(),
                    fx::hsl_shift_fg([0.0, 0.0, -100.0], medium),
                    fx::hsl_shift_fg([0.0, 0.0, -100.0], medium).reversed(),
                    fx::hsl_shift_fg([0.0, 0.0, 100.0], medium),
                    fx::hsl_shift_fg([0.0, 0.0, 100.0], medium).reversed(),
                ]),
            ),
            ("custom color cycle", fx::never_complete(custom_color_cycle)),
        ];

        Self { effects }
    }

    fn get_effect(&self, idx: usize) -> (&'static str, Effect) {
        self.effects[idx].clone()
    }

    fn len(&self) -> usize {
        self.effects.len()
    }
}

impl App {
    fn new() -> Self {
        let tasks = vec![
            Task {
                label: "Parse user prompt".into(),
                status: TaskStatus::Done,
                detail: "Tokenized + extracted intent...".into(),
            },
            Task {
                label: "Analyze codebase context".into(),
                status: TaskStatus::Done,
                detail: "Scanned 147 files...".into(),
            },
            Task {
                label: "Generate code changes".into(),
                status: TaskStatus::Done,
                detail: "Produced unified diff...".into(),
            },
            Task {
                label: "Run syntax validation".into(),
                status: TaskStatus::InProgress,
                detail: "cargo check in progress...".into(),
            },
            Task {
                label: "Execute test suite".into(),
                status: TaskStatus::Pending,
                detail: "Waiting for validation...".into(),
            },
            Task {
                label: "Apply formatting (rustfmt)".into(),
                status: TaskStatus::Pending,
                detail: "".into(),
            },
            Task {
                label: "Lint with clippy".into(),
                status: TaskStatus::Pending,
                detail: "".into(),
            },
            Task {
                label: "Git stage & commit".into(),
                status: TaskStatus::Pending,
                detail: "".into(),
            },
            Task {
                label: "Security audit".into(),
                status: TaskStatus::Failed,
                detail: "RUSTSEC-2024-0375...".into(),
            },
        ];

        let mut task_list_state = ListState::default();
        task_list_state.select(Some(0));

        Self {
            active_panel: ActivePanel::MainContent,
            content_tab: ContentTab::Diff,
            task_list_state,
            tasks,
            show_modal: false,
            show_tachyon_modal: false,
            modal_task_idx: 0,
            diff_scroll: 0,
            md_scroll: 0,
            code_scroll: 0,
            startup_effect: Some(fx::sweep_in(
                Motion::LeftToRight,
                25,
                0,
                palette::BG_DARKEST,
                (600, tachyonfx::Interpolation::QuadOut),
            )),
            modal_effect: None,
            tachyon_modal_effect: None,
            tachyon_active_effect_idx: 0,
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
            last_tick: FxDuration::ZERO,
        }
    }

    fn selected_task_idx(&self) -> usize {
        self.task_list_state.selected().unwrap_or(0)
    }

    fn toggle_modal(&mut self) {
        self.show_modal = !self.show_modal;
        if self.show_modal {
            self.modal_task_idx = self.selected_task_idx();
            self.modal_effect = Some(fx::coalesce((300, tachyonfx::Interpolation::CubicOut)));
        } else {
            self.modal_effect = None;
        }
    }

    fn toggle_tachyon_modal(&mut self) {
        self.show_tachyon_modal = !self.show_tachyon_modal;
        if self.show_tachyon_modal {
            let effects = TachyonEffectsRepository::new();
            self.tachyon_modal_effect = Some(
                effects
                    .get_effect(self.tachyon_active_effect_idx)
                    .1,
            );
        } else {
            self.tachyon_modal_effect = None;
        }
    }

    fn next_tachyon_effect(&mut self) {
        let effects = TachyonEffectsRepository::new();
        self.tachyon_active_effect_idx = (self.tachyon_active_effect_idx + 1) % effects.len();
        self.tachyon_modal_effect = Some(
            effects
                .get_effect(self.tachyon_active_effect_idx)
                .1,
        );
    }

    fn prev_tachyon_effect(&mut self) {
        let effects = TachyonEffectsRepository::new();
        self.tachyon_active_effect_idx = if self.tachyon_active_effect_idx == 0 {
            effects.len() - 1
        } else {
            self.tachyon_active_effect_idx - 1
        };
        self.tachyon_modal_effect = Some(
            effects
                .get_effect(self.tachyon_active_effect_idx)
                .1,
        );
    }

    fn restart_tachyon_effect(&mut self) {
        let effects = TachyonEffectsRepository::new();
        self.tachyon_modal_effect = Some(
            effects
                .get_effect(self.tachyon_active_effect_idx)
                .1,
        );
    }

    fn random_tachyon_effect(&mut self) {
        let effects = TachyonEffectsRepository::new();
        let mut rng = SimpleRng::default();
        self.tachyon_active_effect_idx = (rng.r#gen() % effects.len() as u32) as usize;
        self.tachyon_modal_effect = Some(
            effects
                .get_effect(self.tachyon_active_effect_idx)
                .1,
        );
    }

    fn scramble_tachyon_effect(&mut self) {
        use tachyonfx::fx::Glitch;
        let duration = FxDuration::from_secs(7);
        self.tachyon_modal_effect = Some(fx::with_duration(
            duration,
            Glitch::builder()
                .cell_glitch_ratio(1f32)
                .action_start_delay_ms(0..3000)
                .action_ms(8000..10_000)
                .build()
                .into_effect(),
        ));
    }

    fn current_scroll_mut(&mut self) -> &mut u16 {
        match self.content_tab {
            ContentTab::Diff => &mut self.diff_scroll,
            ContentTab::Markdown => &mut self.md_scroll,
            ContentTab::CodePreview => &mut self.code_scroll,
        }
    }
}

// Sample data (same as you provided)
const OLD_CODE: &str = r#"use std::collections::HashMap;

pub struct AuthManager {
    tokens: HashMap<String, String>,
    max_retries: u32,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
            max_retries: 3,
        }
    }

    pub fn verify_token(&self, token: &str) -> bool {
        self.tokens.contains_key(token)
    }

    pub fn refresh_token(&mut self, user_id: &str) -> Option<String> {
        // TODO: implement token refresh
        let new_token = format!("token_{}", user_id);
        self.tokens.insert(user_id.to_string(), new_token.clone());
        Some(new_token)
    }
}
"#;

const NEW_CODE: &str = r#"use std::collections::HashMap;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("token expired for user {user_id}")]
    TokenExpired { user_id: String },
    #[error("max retries ({max}) exceeded")]
    MaxRetriesExceeded { max: u32 },
    #[error("invalid token format")]
    InvalidFormat,
}

pub struct TokenEntry {
    pub token: String,
    pub issued_at: Instant,
    pub ttl: Duration,
}

pub struct AuthManager {
    tokens: HashMap<String, TokenEntry>,
    max_retries: u32,
    backoff_base_ms: u64,
}

impl AuthManager {
    pub fn new(max_retries: u32, backoff_base_ms: u64) -> Self {
        Self {
            tokens: HashMap::new(),
            max_retries,
            backoff_base_ms,
        }
    }

    pub fn verify_token(&self, token: &str) -> Result<bool, AuthError> {
        if token.is_empty() {
            return Err(AuthError::InvalidFormat);
        }
        if let Some(entry) = self.tokens.values().find(|e| e.token == token) {
            if entry.issued_at.elapsed() > entry.ttl {
                return Err(AuthError::TokenExpired {
                    user_id: token.to_string(),
                });
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn refresh_token(&mut self, user_id: &str) -> Result<String, AuthError> {
        for attempt in 0..self.max_retries {
            let backoff = Duration::from_millis(
                self.backoff_base_ms * 2u64.pow(attempt),
            );
            std::thread::sleep(backoff);

            let new_token = format!("tok_{}_{}", user_id, attempt);
            let entry = TokenEntry {
                token: new_token.clone(),
                issued_at: Instant::now(),
                ttl: Duration::from_secs(3600),
            };
            self.tokens.insert(user_id.to_string(), entry);
            return Ok(new_token);
        }
        Err(AuthError::MaxRetriesExceeded {
            max: self.max_retries,
        })
    }
}
"#;
const MARKDOWN_RESPONSE: &str = r#"# 🔧 Auth Module Refactoring

## Summary
Refactored authentication with proper error handling, token expiration, and exponential backoff.

## Key Changes
- Added `AuthError` enum
- New `TokenEntry` struct with TTL
- Improved `refresh_token` with backoff
"#;

// Helper: Render nice unified diff
fn build_diff_lines() -> Vec<Line<'static>> {
    let diff = TextDiff::from_lines(OLD_CODE, NEW_CODE);
    let mut lines = vec![];

    lines.push(Line::styled(
        "--- old/auth.rs",
        Style::default().fg(palette::TEXT_DIM),
    ));
    lines.push(Line::styled(
        "+++ new/auth.rs",
        Style::default().fg(palette::TEXT_DIM),
    ));
    lines.push(Line::raw(""));

    for change in diff.iter_all_changes() {
        let (prefix, style) = match change.tag() {
            ChangeTag::Delete => (
                "-",
                Style::default()
                    .fg(palette::DIFF_DEL_FG)
                    .bg(palette::DIFF_DEL_BG),
            ),
            ChangeTag::Insert => (
                "+",
                Style::default()
                    .fg(palette::DIFF_ADD_FG)
                    .bg(palette::DIFF_ADD_BG),
            ),
            ChangeTag::Equal => (" ", Style::default().fg(palette::TEXT)),
        };

        let mut content = change.to_string();
        if content.ends_with('\n') {
            content.pop();
        }

        let mut spans = vec![Span::styled(prefix, style)];
        spans.push(Span::styled(content, style));
        lines.push(Line::from(spans));
    }
    lines
}

// Helper: Syntax highlight code (Rust)
fn highlight_rust(code: &str, syntax_set: &SyntaxSet, theme_set: &ThemeSet) -> Vec<Line<'static>> {
    let syntax = syntax_set
        .find_syntax_by_extension("rs")
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());
    let mut highlighter = HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);

    let mut output = vec![];

    for line in LinesWithEndings::from(code) {
        let ranges = highlighter
            .highlight_line(line, syntax_set)
            .unwrap();
        let spans: Vec<Span> = ranges
            .iter()
            .map(|(style, text)| {
                let fg = Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                Span::styled(text.to_string(), Style::default().fg(fg))
            })
            .collect();
        output.push(Line::from(spans));
    }
    output
}

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let mut app = App::new();

    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();
    let mut last_frame_instant = Instant::now();

    loop {
        app.last_tick = last_frame_instant.elapsed().into();
        last_frame_instant = Instant::now();

        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if !event::poll(timeout)? {
            last_tick = Instant::now();
            continue;
        }

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
            && handle_input(&mut app, key)
        {
            break;
        }

        last_tick = Instant::now();
    }

    restore_terminal()?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn handle_input(app: &mut App, key: event::KeyEvent) -> bool {
    // Handle tachyon modal keys first
    if app.show_tachyon_modal {
        match key.code {
            KeyCode::Char('0') | KeyCode::Esc => {
                app.toggle_tachyon_modal();
                return false;
            },
            KeyCode::Enter => {
                app.next_tachyon_effect();
                return false;
            },
            KeyCode::Backspace => {
                app.prev_tachyon_effect();
                return false;
            },
            KeyCode::Char(' ') => {
                app.restart_tachyon_effect();
                return false;
            },
            KeyCode::Char('r') => {
                app.random_tachyon_effect();
                return false;
            },
            KeyCode::Char('s') => {
                app.scramble_tachyon_effect();
                return false;
            },
            _ => return false,
        }
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => return true,

        KeyCode::Tab => {
            app.active_panel = match app.active_panel {
                ActivePanel::Sidebar => ActivePanel::MainContent,
                ActivePanel::MainContent => ActivePanel::Sidebar,
            };
        },

        KeyCode::Char('h') | KeyCode::Left => app.content_tab = ContentTab::Diff,
        KeyCode::Char('j') | KeyCode::Down => {
            if app.active_panel == ActivePanel::Sidebar {
                let i = app
                    .selected_task_idx()
                    .saturating_add(1)
                    .min(app.tasks.len() - 1);
                app.task_list_state.select(Some(i));
            } else {
                let s = app.current_scroll_mut();
                *s = s.saturating_add(1);
            }
        },
        KeyCode::Char('k') | KeyCode::Up => {
            if app.active_panel == ActivePanel::Sidebar {
                let i = app.selected_task_idx().saturating_sub(1);
                app.task_list_state.select(Some(i));
            } else {
                let s = app.current_scroll_mut();
                *s = s.saturating_sub(1);
            }
        },
        KeyCode::Char('l') | KeyCode::Right => app.content_tab = ContentTab::CodePreview,
        KeyCode::Char('m') => app.content_tab = ContentTab::Markdown,

        KeyCode::Enter => {
            if app.active_panel == ActivePanel::Sidebar {
                app.toggle_modal();
            }
        },

        KeyCode::Char('0') => {
            app.toggle_tachyon_modal();
        },

        _ => {},
    }
    false
}

fn ui(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(area);

    // Header
    let header = Block::default()
        .title(" AstraCoder • auth.rs refactoring ")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(palette::BLUE));
    frame.render_widget(header, vertical[0]);

    // Main horizontal split
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(28), Constraint::Percentage(72)])
        .split(vertical[1]);

    render_sidebar(frame, app, horizontal[0]);
    render_main_content(frame, app, horizontal[1]);

    // Footer
    let footer =
        Paragraph::new(" Tab: focus • ↑↓: navigate • h/j/k/l: tabs • Enter: details • q: quit")
            .style(Style::default().fg(palette::TEXT_MUTED));
    frame.render_widget(footer, vertical[2]);

    // Modal
    if app.show_modal {
        render_modal(frame, app);
    }

    // Tachyon Modal
    if app.show_tachyon_modal {
        render_tachyon_modal(frame, app);
    }

    // Startup effect
    if let Some(_effect) = &app.startup_effect {
        // In tachyonfx 0.25 you typically apply effects after rendering via the buffer
        // For simplicity we keep it minimal here
    }
}

fn render_sidebar(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" AI Agent Tasks ")
        .borders(Borders::ALL)
        .border_style(if app.active_panel == ActivePanel::Sidebar {
            Style::default().fg(palette::BORDER_ACTIVE)
        } else {
            Style::default().fg(palette::TEXT_MUTED)
        });

    let items: Vec<ListItem> = app
        .tasks
        .iter()
        .map(|task| {
            let icon = task.status.icon();
            let style = Style::default().fg(task.status.color());
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", icon), style),
                Span::styled(&task.label, Style::default().fg(palette::TEXT)),
            ]))
        })
        .collect();

    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(palette::BLUE),
    );

    frame.render_stateful_widget(list, area, &mut app.task_list_state);
}

fn render_main_content(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Tabs
    let tab_titles = vec![" Diff ", " Markdown ", " Code Preview "];
    let tabs = Tabs::new(tab_titles)
        .select(match app.content_tab {
            ContentTab::Diff => 0,
            ContentTab::Markdown => 1,
            ContentTab::CodePreview => 2,
        })
        .block(Block::default().borders(Borders::BOTTOM))
        .highlight_style(Style::default().fg(palette::BLUE).bold());

    frame.render_widget(tabs, chunks[0]);

    let content_area = chunks[1];

    match app.content_tab {
        ContentTab::Diff => render_diff_view(frame, app, content_area),
        ContentTab::Markdown => render_markdown_view(frame, app, content_area),
        ContentTab::CodePreview => render_code_view(frame, app, content_area),
    }
}

fn render_diff_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" Unified Diff ")
        .borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let diff_lines = build_diff_lines();
    let text = Text::from(diff_lines.clone());

    let para = Paragraph::new(text)
        .scroll((app.diff_scroll, 0))
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(para, inner);

    // Scrollbar
    let mut scrollbar_state =
        ScrollbarState::new(diff_lines.len() as usize).position(app.diff_scroll as usize);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight),
        area,
        &mut scrollbar_state,
    );
}

fn render_markdown_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" AI Explanation ")
        .borders(Borders::ALL);
    let para = Paragraph::new(MARKDOWN_RESPONSE)
        .block(block)
        .scroll((app.md_scroll, 0))
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(para, area);
}

fn render_code_view(frame: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" New Code (syntax highlighted) ")
        .borders(Borders::ALL);
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let highlighted = highlight_rust(NEW_CODE, &app.syntax_set, &app.theme_set);
    let para = Paragraph::new(Text::from(highlighted)).scroll((app.code_scroll, 0));

    frame.render_widget(para, inner);
}

fn render_modal(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(70, 60, frame.area());

    let block = Block::default()
        .title(" Task Details ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette::BLUE));

    let task = &app.tasks[app.modal_task_idx];

    let content = format!(
        "Status: {}\n\n{}\n\nDetail:\n{}",
        task.status.icon(),
        task.label,
        task.detail
    );

    let para = Paragraph::new(content)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(para, area);
}

fn render_tachyon_modal(frame: &mut Frame, app: &mut App) {
    let screen_bg = palette::BG_DARKEST;
    let bg = palette::BG_SURFACE;

    let content_area = centered_rect(80, 60, frame.area());

    // Clear and render background
    frame.render_widget(Clear, frame.area());
    frame.render_widget(
        Block::default().style(Style::default().bg(screen_bg)),
        frame.area(),
    );

    frame.render_widget(
        Block::default().style(Style::default().bg(bg)),
        content_area,
    );

    let layout =
        Layout::vertical([Constraint::Length(2), Constraint::Length(7), Constraint::Length(6)])
            .split(content_area.inner(ratatui::layout::Margin::new(1, 1)));

    let anim_style = [Style::default().fg(palette::YELLOW), Style::default().fg(palette::BLUE)];
    let text_style = Style::default().fg(palette::TEXT);
    let shortcut_style = [
        Style::default()
            .fg(palette::YELLOW)
            .add_modifier(Modifier::BOLD),
        Style::default().fg(palette::TEXT_DIM),
    ];

    let effects = TachyonEffectsRepository::new();
    let active_effect_name = effects
        .get_effect(app.tachyon_active_effect_idx)
        .0;

    let active_animation = Line::from(vec![
        Span::from("Active animation: ").style(anim_style[0]),
        Span::from(active_effect_name).style(anim_style[1]),
    ]);

    let main_text = Text::from(vec![
        Line::from("Many effects are composable, e.g. `parallel`, `sequence`, `repeating`."),
        Line::from("Most effects have a lifetime, after which they report done()."),
        Line::from("Effects such as `never_complete`, `temporary` influence or override this."),
        Line::from(""),
        Line::from("The text in this window will undergo a random transition"),
        Line::from("when any of the following keys are pressed:"),
    ])
    .style(text_style);

    let shortcut = |key: &'static str, desc: &'static str| {
        Line::from(vec![
            Span::from(key).style(shortcut_style[0]),
            Span::from(desc).style(shortcut_style[1]),
        ])
    };

    let shortcuts = Text::from(vec![
        shortcut("↵   ", "next transition"),
        shortcut("⌫   ", "previous transition"),
        shortcut("␣   ", "restart transition"),
        shortcut("r   ", "random transition"),
        shortcut("s   ", "scramble text toggle"),
        shortcut("0   ", "close this modal"),
    ]);

    frame.render_widget(active_animation, layout[0]);
    frame.render_widget(main_text, layout[1]);
    frame.render_widget(shortcuts, layout[2]);

    // Apply effect if running
    if let Some(effect) = &mut app.tachyon_modal_effect
        && effect.running()
    {
        frame.render_effect(effect, content_area, app.last_tick);
    }
}

// Helper for centered modal
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
