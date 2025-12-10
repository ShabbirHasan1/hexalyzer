use crate::hexviewer::HexViewer;
use eframe::egui;

impl HexViewer {
    /// Get scroll offset along Y axis
    pub(crate) fn get_scroll_offset(&self, ui: &egui::Ui, addr: usize) -> f32 {
        // Get y axis target coord
        let row_idx = (addr - self.addr.min) / self.bytes_per_row;
        let row_height =
            ui.text_style_height(&egui::TextStyle::Monospace) + ui.spacing().item_spacing.y;
        let target_y = row_idx as f32 * row_height;
        // Get current position
        let cursor = ui.cursor().min;
        // Calculate offset based on target and current pos
        target_y - cursor.y
    }

    /// Create scroll area (with offset if jump or search is triggered)
    pub(crate) fn create_scroll_area(&mut self, ui: &egui::Ui) -> egui::ScrollArea {
        let mut scroll_area = egui::ScrollArea::vertical();
        if self.search.addr.is_some() {
            let offset = self.get_scroll_offset(ui, self.search.addr.unwrap());
            scroll_area = scroll_area.vertical_scroll_offset(offset);
            self.search.addr = None;
        } else if self.jump_to.addr.is_some() {
            let offset = self.get_scroll_offset(ui, self.jump_to.addr.unwrap());
            scroll_area = scroll_area.vertical_scroll_offset(offset);
            self.jump_to.addr = None;
        }
        scroll_area
    }
}
