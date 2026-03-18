//! Autocomplete suggestions rendering

use ratatui::layout::Rect;

use crate::app::ChatApp;

impl ChatApp {
    /// Render autocomplete suggestions overlay using animated modal
    pub fn render_suggestions(&mut self, frame: &mut ratatui::Frame, input_area: Rect) {
        // Only render if suggestions are visible
        if !self.autocomplete.suggestion_list().is_visible() {
            return;
        }

        // Calculate suggestion box position (above input box)
        // Use a fixed reasonable height for the modal
        let max_height = 8; // Fixed height for better visibility
        let suggestion_area = Rect {
            x: input_area.x,
            y: input_area.y.saturating_sub(max_height),
            width: input_area.width,
            height: max_height,
        };

        // Use the animated suggestion list from autocomplete
        self.autocomplete
            .suggestion_list_mut()
            .render(frame, suggestion_area);
    }
}
