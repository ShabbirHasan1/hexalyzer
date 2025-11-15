use crate::hexviewer::HexViewer;
use crate::ui_events::EventManager;
use eframe::egui;
use std::collections::BTreeMap;

#[derive(Default)]
pub(crate) struct Search {
    pub(crate) scroll_addr: Option<usize>,
    idx: usize,
    results: Vec<usize>,
    input: String,
    last_input: String,
}

impl HexViewer {
    /// Show contents of search menu
    pub(crate) fn show_search_contents(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(&mut self.search.input);

        if let Some(key) = EventManager::get_keyboard_input_key(ui)
            && key == egui::Key::Enter
        {
            if self.search.input != self.search.last_input {
                let pattern = parse_hex_pattern(self.search.input.as_str());
                if let Some(p) = pattern {
                    // If pattern valid -> search
                    self.search.results = search_bmh(&self.byte_addr_map, &p);
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
            self.search.scroll_addr = self.search.results.get(self.search.idx).copied();
        }
        // Show label with matches count
        if !self.search.results.is_empty() {
            ui.add_space(5.0);
            ui.label(format!("Hits: {}", self.search.results.len()));
        }
    }

    /// Get scroll offset along Y axis
    pub(crate) fn get_scroll_offset(&mut self, ui: &mut egui::Ui, bytes_per_row: usize) -> f32 {
        // Reset scroll address
        self.search.scroll_addr = None;
        // Get y axis target coord
        let row_height =
            ui.text_style_height(&egui::TextStyle::Monospace) + ui.spacing().item_spacing.y;
        let row_idx = (self.search.scroll_addr.unwrap() - self.min_addr) / bytes_per_row;
        let target_y = row_idx as f32 * row_height;
        // Get current position
        let cursor = ui.cursor().min;
        // Calculate offset based on target and current pos
        target_y - cursor.y
    }
}

/// Parse str hex representation into Vec<u8>
fn parse_hex_pattern(input: &str) -> Option<Vec<u8>> {
    // Must have even length
    if !input.len().is_multiple_of(2) {
        return None;
    }

    // Convert each 2-char chunk to a byte
    (0..input.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).ok())
        .collect()
}

/// Boyer–Moore–Horspool algorithm for BTreeMap<usize, u8>.
/// Returns the starting addresses of all matches.
///
/// TODO: add SIMD acceleration
fn search_bmh(map: &BTreeMap<usize, u8>, pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    if m == 0 || map.is_empty() {
        return vec![];
    }

    // Build bad match table
    let mut bad_match = [m as u8; 256];
    for i in 0..m - 1 {
        bad_match[pattern[i] as usize] = (m - 1 - i) as u8;
    }

    // Prepare result collection
    let mut results = Vec::new();

    // Build a Vec of (addr,byte) indices into map for sequential access.
    // NOTE: only address are stored in this Vec, not the bytes.
    let addrs: Vec<usize> = map.keys().copied().collect();

    // Check if length of address is less than the pattern
    let n = addrs.len();
    if n < m {
        return results;
    }

    let bytes_iter = |i: usize| -> u8 {
        // safe: address exists since index is in addrs
        map[&addrs[i]]
    };

    // Main BMH loop
    let mut i = 0; // index into addrs[]

    while i <= n - m {
        // Compare pattern from right to left
        let mut j = (m - 1) as isize;
        while j >= 0 && bytes_iter(i + j as usize) == pattern[j as usize] {
            j -= 1;
        }

        if j < 0 {
            // Match found
            results.push(addrs[i]);
            i += 1; // advance minimally
        } else {
            // Mismatch -> skip using last byte of window
            let last_byte = bytes_iter(i + m - 1);
            i += bad_match[last_byte as usize] as usize;
        }
    }
    results
}
