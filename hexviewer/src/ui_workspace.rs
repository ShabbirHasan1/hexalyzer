use super::HexViewer;
use crate::colors;
use crate::ui_events::EventManager;
use eframe::egui;

impl HexViewer {
    /// Update edit buffer used for temporary storage of user key inputs
    /// during byte editing process
    fn update_edit_buffer(&mut self, typed_char: Option<char>) {
        if self.selection.range.is_some()
            && self.selection.released
            && !self.editor.in_progress
            && let Some(ch) = typed_char
        {
            // Start editing if user types a hex char
            if ch.is_ascii_hexdigit() {
                self.editor.in_progress = true;
                self.editor.addr = self.selection.range;
                self.editor.buffer = ch.to_ascii_uppercase().to_string();
            }
        } else if self.editor.in_progress {
            // If other bytes got selected - clear and return
            if !self.editor.is_addr_same(self.selection.range) {
                self.editor.clear();
            }

            if let Some(ch) = typed_char {
                self.editor.buffer.insert(1, ch);
            }

            // Allow only hex chars
            self.editor.buffer.retain(|c| c.is_ascii_hexdigit());

            // When two hex chars are entered - commit automatically
            if self.editor.buffer.len() == 2 {
                if let Ok(value) = u8::from_str_radix(&self.editor.buffer, 16)
                    && let Some([start, end]) = self.editor.addr
                {
                    // Handle reversed range
                    let (s, e) = if start <= end {
                        (start, end)
                    } else {
                        (end, start)
                    };
                    // Update the bytes in the map
                    for addr in s..=e {
                        if let Some(byte) = self.byte_addr_map.get_mut(&addr) {
                            *byte = value;
                        }
                    }
                }
                self.editor.clear();
            }
        }
    }

    pub(crate) fn show_central_workspace(&mut self, ctx: &egui::Context) {
        // LEFT PANEL (FILE INFORMATION & DATA INSPECTOR)
        egui::SidePanel::left("left_panel")
            .exact_width(280.0)
            .show(ctx, |ui| {
                // FILE INFORMATION
                egui::CollapsingHeader::new("File Information")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add_space(5.0);
                        self.show_file_info_contents(ui);
                        ui.add_space(5.0);
                    });
                // DATA INSPECTOR
                egui::CollapsingHeader::new("Data Inspector")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add_space(5.0);
                        self.show_data_inspector_contents(ui);
                        ui.add_space(5.0);
                    });
            });

        // CENTRAL VIEW
        egui::CentralPanel::default().show(ctx, |ui| {
            let bytes_per_row = 16;
            // Rounds division up
            let total_rows = (self.max_addr - self.min_addr).div_ceil(bytes_per_row);
            // Get row height in pixels (depends on font size)
            let row_height = ui.text_style_height(&egui::TextStyle::Monospace);

            egui::ScrollArea::vertical()
                .scroll_source(egui::containers::scroll_area::ScrollSource {
                    mouse_wheel: true,
                    scroll_bar: true,
                    drag: false,
                })
                .auto_shrink([false; 2])
                .show_rows(ui, row_height, total_rows, |ui, row_range| {
                    // Get state of the mouse click
                    let pointer_down = ui.input(|i| i.pointer.primary_down());
                    let pointer_hover = ui.input(|i| i.pointer.hover_pos());

                    // Detect released clicked
                    if !pointer_down {
                        self.selection.released = true;
                    }

                    // Get state of key press
                    let typed_char = EventManager::get_keyboard_input(ui);

                    // Update byte edit buffer base on the key press
                    self.update_edit_buffer(typed_char);

                    // Cancel byte editing on Esc press
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.editor.clear()
                    }

                    for row in row_range {
                        ui.horizontal(|ui| {
                            // Start and end addresses
                            let start = self.min_addr + row * bytes_per_row;
                            let end = start + bytes_per_row;

                            // Display address (fixed width, monospaced)
                            ui.monospace(format!("{:08X}", start));

                            // Add space before hex block
                            ui.add_space(16.0);

                            // Hex bytes representation row
                            for addr in start..end {
                                // Determine is the current byte selected
                                let byte = self.byte_addr_map.get(&addr).copied();
                                let is_selected =
                                    byte.is_some() && self.selection.is_addr_within_range(addr);

                                // Change color of every other byte for better readability
                                let bg_color = if addr % 2 == 0 {
                                    colors::GRAY_210
                                } else {
                                    colors::GRAY_160
                                };

                                // Determine display value of the byte
                                let display_value = if let Some(b) = byte {
                                    if is_selected && self.editor.in_progress {
                                        self.editor.buffer.clone()
                                    } else {
                                        format!("{:02X}", b)
                                    }
                                } else {
                                    "--".to_string()
                                };

                                // Show byte as a button
                                let button = ui.add_sized(
                                    [21.0, 18.0],
                                    egui::Button::new(
                                        egui::RichText::new(display_value)
                                            .monospace()
                                            .size(12.0)
                                            .color(bg_color),
                                    )
                                    .fill(colors::TRANSPARENT),
                                );

                                // Update the selection range
                                if pointer_down
                                    && pointer_hover.is_some()
                                    && byte.is_some()
                                    && button.rect.contains(pointer_hover.unwrap())
                                {
                                    self.selection.update(addr);
                                }

                                // Highlight byte if selected
                                if is_selected {
                                    ui.painter()
                                        .rect_filled(button.rect, 0.0, colors::LIGHT_BLUE);
                                }

                                // Add space every 8 bytes
                                if (addr + 1) % 8 == 0 {
                                    ui.add_space(5.0);
                                } else {
                                    // Make space between buttons as small as possible
                                    ui.add_space(-6.0);
                                }
                            }

                            // Add space before ASCII row
                            ui.add_space(16.0);

                            // ASCII representation row
                            for addr in start..end {
                                // Determine display char
                                let byte = self.byte_addr_map.get(&addr).copied();
                                let ch = if let Some(b) = byte {
                                    if b.is_ascii_graphic() { b as char } else { '.' }
                                } else {
                                    ' '
                                };

                                // Determine is char selected
                                let is_selected =
                                    byte.is_some() && self.selection.is_addr_within_range(addr);

                                // Show char as label
                                let label = ui.add(egui::Label::new(
                                    egui::RichText::new(ch.to_string())
                                        .color(colors::GRAY_160)
                                        .monospace(),
                                ));

                                // Update the selection range
                                if pointer_down
                                    && pointer_hover.is_some()
                                    && byte.is_some()
                                    && label.rect.contains(pointer_hover.unwrap())
                                {
                                    self.selection.update(addr);
                                }

                                // Highlight char if selected
                                if is_selected {
                                    ui.painter()
                                        .rect_filled(label.rect, 0.0, colors::LIGHT_BLUE);
                                }
                            }
                        });
                    }
                })
        });
    }
}
