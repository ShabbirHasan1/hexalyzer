use crate::app::HexViewerApp;
use eframe::egui;

impl HexViewerApp {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation
    )]
    /// Get scroll offset along Y axis
    pub(crate) fn get_scroll_offset(&mut self, ui: &egui::Ui, addr: usize) -> f32 {
        let row_idx = (addr - self.addr.min) / self.bytes_per_row;

        // Handle edge case
        if row_idx > f32::MAX as usize {
            self.error = Some("Row index larger than f32 max value - display failed.".to_string());
            return 0.0;
        }

        let row_height =
            ui.text_style_height(&egui::TextStyle::Monospace) + ui.spacing().item_spacing.y;

        // Get y axis target coord
        let target_y = row_idx as f32 * row_height;

        // Get current position
        let cursor = ui.cursor().min;

        // Calculate offset based on target and current pos
        target_y - cursor.y
    }

    /// Create scroll area (with offset if jump or search is triggered)
    pub(crate) fn create_scroll_area(&mut self, ui: &egui::Ui) -> egui::ScrollArea {
        let mut scroll_area = egui::ScrollArea::vertical();
        if let Some(addr) = self.search.addr {
            let offset = self.get_scroll_offset(ui, addr);
            scroll_area = scroll_area.vertical_scroll_offset(offset);
            self.search.addr = None;
        } else if let Some(addr) = self.jump_to.addr {
            let offset = self.get_scroll_offset(ui, addr);
            scroll_area = scroll_area.vertical_scroll_offset(offset);
            self.jump_to.addr = None;
        }
        scroll_area
    }
}
