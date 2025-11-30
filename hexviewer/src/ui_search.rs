use crate::hexviewer::HexViewer;
use crate::ui_events::EventManager;
use crate::utils::{parse_hex_str_into_vec, search_bmh};
use eframe::egui;

#[derive(Default)]
pub(crate) struct Search {
    pub(crate) addr: Option<usize>,
    idx: usize,
    results: Vec<usize>,
    input: String,
    last_input: String,
    pub(crate) has_focus: bool,
}

impl HexViewer {
    /// Show contents of search menu
    pub(crate) fn show_search_contents(&mut self, ui: &mut egui::Ui) {
        let textedit = ui.text_edit_singleline(&mut self.search.input);

        if textedit.has_focus() {
            self.jump_to.has_focus = false;
            self.search.has_focus = true;
        }

        if let Some(key) = EventManager::get_keyboard_input_key(ui)
            && key == egui::Key::Enter
            && self.search.has_focus
        {
            if self.search.input != self.search.last_input {
                let pattern = parse_hex_str_into_vec(self.search.input.as_str());
                if let Some(p) = pattern {
                    // If pattern valid -> search
                    self.search.results = search_bmh(&self.ih.to_btree_map(), &p);
                } else {
                    // If pattern not valid -> clear results
                    self.search.results.clear();
                }
                // Reset the state of search
                self.search.idx = 0;
                self.search.last_input = self.search.input.clone();
            } else {
                // Same input -> move to next result
                if !self.search.results.is_empty() {
                    self.search.idx = (self.search.idx + 1) % self.search.results.len();
                }
            }
            // Set address to scroll to
            self.search.addr = self.search.results.get(self.search.idx).copied();
        }
        // Show label with matches count
        if !self.search.results.is_empty() {
            ui.add_space(5.0);
            ui.label(format!(
                "Hits: {} (Current: {})",
                self.search.results.len(),
                self.search.idx + 1
            ));
        }
    }
}
