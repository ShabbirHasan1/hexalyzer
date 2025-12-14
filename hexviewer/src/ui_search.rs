use crate::app::HexViewerApp;
use crate::ui_events::EventManager;
use crate::utils::{parse_hex_str_into_vec, search_bmh};
use eframe::egui;

#[derive(Default)]
pub(crate) struct Search {
    pub(crate) has_focus: bool,
    pub(crate) addr: Option<usize>,
    pub(crate) results: Vec<usize>,
    pub(crate) length: usize,
    input: String,
    last_input: String,
    idx: usize,
    force: bool,
}

impl Search {
    pub(crate) fn clear(&mut self) {
        self.has_focus = false;
        self.addr = None;
        self.results.clear();
        self.length = 0;
        // self.input.clear(); -> do not clear to preserve text box content
        self.last_input.clear();
        self.idx = 0;
        self.force = false;
    }

    pub(crate) fn redo(&mut self) {
        // In case current input field is not valid
        self.input = self.last_input.clone();

        // Clear
        self.clear();

        // Set force flag
        self.force = true;
    }
}

impl HexViewerApp {
    /// Show contents of search menu
    pub(crate) fn show_search_contents(&mut self, ui: &mut egui::Ui) {
        let textedit = ui.add(
            egui::TextEdit::singleline(&mut self.search.input)
                .desired_width(ui.available_width() - 30.0),
        );

        if textedit.has_focus() {
            self.jump_to.has_focus = false;
            self.search.has_focus = true;
        }

        let key = EventManager::get_keyboard_input_key(ui); // get one event per cycle
        if (key.is_some() && key.unwrap() == egui::Key::Enter && self.search.has_focus)
            || self.search.force
        {
            if self.search.input != self.search.last_input {
                let pattern = parse_hex_str_into_vec(self.search.input.as_str());

                if let Some(p) = pattern {
                    // If pattern valid -> search
                    self.search.results = search_bmh(self.ih.iter(), &p);
                    self.search.length = p.len();
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

            // Set address to scroll to (only if not forced)
            if !self.search.force {
                self.search.addr = self.search.results.get(self.search.idx).copied();
            }

            self.search.force = false;
        }

        ui.add_space(5.0);

        let mut label_text = "--".to_string();

        // Show matches count
        if !self.search.results.is_empty() {
            label_text = format!(
                "Hits: {} (Current: {})",
                self.search.results.len(),
                self.search.idx + 1
            );
        }

        ui.label(label_text);
    }
}
