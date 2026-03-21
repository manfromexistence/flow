use ratatui::{layout::Rect, Frame, widgets::Widget, buffer::Buffer};
use crate::app::ChatApp;

// Yazi Root widget - uses the actual yazi Lua-based rendering
pub struct YaziRoot<'a> {
    core: &'a yazi_core::Core,
}

impl<'a> YaziRoot<'a> {
    pub fn new(core: &'a yazi_core::Core) -> Self {
        Self { core }
    }
}

impl Widget for YaziRoot<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use mlua::ObjectLike;
        use yazi_binding::elements::render_once;
        use yazi_plugin::LUA;
        
        // Try to render using Lua
        let result = (|| {
            let area_binding = yazi_binding::elements::Rect::from(area);
            let root = LUA.globals().raw_get::<mlua::Table>("Root")?
                .call_method::<mlua::Table>("new", area_binding)?;
            
            render_once(root.call_method("redraw", ())?, buf, |p| self.core.mgr.area(p));
            Ok::<_, mlua::Error>(())
        })();
        
        // If Lua rendering fails, show error
        if let Err(e) = result {
            use ratatui::widgets::Paragraph;
            use ratatui::style::{Style, Color};
            
            let error_msg = format!("Yazi Lua rendering failed:\n{}\n\nLua plugin system not initialized", e);
            Paragraph::new(error_msg)
                .style(Style::default().fg(Color::Red))
                .render(area, buf);
        }
    }
}

impl ChatApp {
    pub fn render_yazi_in_area(&mut self, area: Rect, frame: &mut Frame) {
        if let Some(ref yazi_core) = self.yazi_core {
            // Use the actual yazi Root widget with Lua rendering
            frame.render_widget(YaziRoot::new(yazi_core), area);
        } else {
            use ratatui::widgets::Paragraph;
            use ratatui::style::{Style, Color};
            
            frame.render_widget(
                Paragraph::new("📁 Initializing Yazi...")
                    .style(Style::default().fg(Color::Cyan)),
                area,
            );
        }
    }
}
