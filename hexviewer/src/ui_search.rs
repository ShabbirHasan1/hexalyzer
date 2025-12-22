use crate::app::HexViewerApp;
use eframe::egui;
use std::collections::btree_map;

#[derive(Default)]
pub struct Search {
    pub(crate) has_focus: bool,
    pub(crate) addr: Option<usize>,
    pub(crate) results: Vec<usize>,
    pub(crate) length: usize,
    input: String,
    last_input: String,
    idx: usize,
    force: bool,
    loose_focus: bool,
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

    pub(crate) const fn loose_focus(&mut self) {
        self.loose_focus = true;
    }
}

impl HexViewerApp {
    /// Show contents of search menu
    pub(crate) fn show_search_contents(&mut self, ui: &mut egui::Ui) {
        let textedit = ui.add(
            egui::TextEdit::singleline(&mut self.search.input)
                .desired_width(ui.available_width() - 30.0),
        );

        if self.search.loose_focus {
            textedit.surrender_focus();
            self.search.loose_focus = false;
        }

        if textedit.has_focus() {
            self.jump_to.has_focus = false;
            self.search.has_focus = true;

            // Clear the selection to avoid modifying bytes
            // while typing in the search area
            self.selection.clear();
        }

        let key = self.events.last_key_released; // get one event per cycle
        if (key == Some(egui::Key::Enter) && self.search.has_focus) || self.search.force {
            // Same input -> move to next result, otherwise -> search again
            if self.search.input == self.search.last_input {
                if !self.search.results.is_empty() {
                    self.search.idx = (self.search.idx + 1) % self.search.results.len();
                }
            } else {
                let input = self.search.input.as_str();

                // Parse str hex representation into Vec<u8>
                let pattern: Option<Vec<u8>> = if input.len().is_multiple_of(2) {
                    (0..input.len())
                        .step_by(2)
                        .map(|i| u8::from_str_radix(&input[i..i + 2], 16).ok())
                        .collect()
                } else {
                    None
                };

                // If pattern valid -> search, otherwise -> clear results
                if let Some(p) = pattern {
                    self.search.results = search_bmh(self.ih.iter(), &p);
                    self.search.length = p.len();
                } else {
                    self.search.results.clear();
                }

                // Reset the state of search
                self.search.idx = 0;
                self.search.last_input = self.search.input.clone();
            }

            // Set address to scroll to (only if not forced)
            if !self.search.force {
                self.search.addr = self.search.results.get(self.search.idx).copied();
            }

            self.search.force = false;
        }

        ui.add_space(5.0);

        // Show matches count if any
        let label_text = if self.search.results.is_empty() {
            "--".to_string()
        } else {
            format!(
                "Hits: {} (Current: {})",
                self.search.results.len(),
                self.search.idx + 1
            )
        };

        ui.label(label_text);
    }
}

// TODO: 1) add SIMD acceleration; 2) Replace with KMP search?

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]
/// Boyer–Moore–Horspool algorithm for `BTreeMap<usize, u8>`.
/// Returns the starting addresses of all matches.
fn search_bmh(map_iter: btree_map::Iter<usize, u8>, pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    if m == 0 || m > u8::MAX as usize {
        return vec![];
    }

    // Consume the iterator once into an indexable representation.
    // This does not clone the BTreeMap, only copies (usize, u8) pairs.
    let haystack: Vec<(usize, u8)> = map_iter.map(|(&addr, &byte)| (addr, byte)).collect();

    // Check if length of address is less than the pattern
    let n = haystack.len();
    if n < m {
        return vec![];
    }

    // Build bad match table
    let mut bad_match = [m as u8; 256];
    for i in 0..m - 1 {
        bad_match[pattern[i] as usize] = (m - 1 - i) as u8;
    }

    // Prepare result collection
    let mut results = Vec::new();

    // Main BMH loop
    let mut i = 0; // index into addrs[]
    while i <= n - m {
        // Compare pattern from right to left
        let mut j = (m - 1) as isize;
        while j >= 0 && haystack[i + j as usize].1 == pattern[j as usize] {
            j -= 1;
        }

        if j < 0 {
            // Match found
            results.push(haystack[i].0);
            i += 1; // advance minimally
        } else {
            // Mismatch -> skip using last byte of window
            let last_byte = haystack[i + m - 1].1;
            i += bad_match[last_byte as usize] as usize;
        }
    }

    results
}
