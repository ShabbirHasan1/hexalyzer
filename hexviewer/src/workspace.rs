use eframe::egui;
use super::HexViewer;


impl HexViewer {
    pub(crate) fn show_central_workspace(&mut self, ctx: &egui::Context) {
        // Get filename
        let filename = self.ih.filepath
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "-".to_string());

        // LEFT PANEL
        egui::SidePanel::left("left_panel")
            .exact_width(250.0)
            .show(ctx, |ui| {

                // File Info
                egui::CollapsingHeader::new("File Information")
                    .default_open(true)
                    .show(ui, |ui| {
                        egui::Grid::new("file_info_grid")
                            .num_columns(2) // two columns: label + value
                            .spacing([30.0, 4.0]) // horizontal & vertical spacing
                            .show(ui, |ui| {
                                ui.label("File Name");
                                ui.label(filename);
                                ui.end_row();
                                ui.label("File Size");
                                ui.label(format!("{} bytes", self.ih.size));
                                ui.end_row();
                            });
                    });

                // Data Inspector
                egui::CollapsingHeader::new("Data Inspector")
                    .default_open(true)
                    .show(ui, |ui| {
                        egui::Grid::new("data_inspector_grid")
                            .num_columns(2) // two columns: label & value
                            .spacing([20.0, 4.0]) // horizontal & vertical spacing
                            .show(ui, |ui| {
                                ui.heading("Type");
                                ui.heading("Value");
                                ui.end_row();

                                if self.selected.is_none() {
                                    ui.label("--");
                                    ui.label("--");
                                    ui.end_row();
                                    return;
                                }

                                let sel = self.selected.as_ref().unwrap();
                                let bytes: Vec<u8> = self.byte_addr_map
                                    .range(sel[0].0..=sel[1].0)
                                    .map(|(_, &b)| b)
                                    .collect();

                                match bytes.len() {
                                    1 => {
                                        ui.label("u8");
                                        ui.label(u8::from_le_bytes([bytes[0]]).to_string());
                                        ui.end_row();
                                        ui.label("i8");
                                        ui.label(i8::from_le_bytes([bytes[0]]).to_string());
                                        ui.end_row();
                                    }
                                    2 => {
                                        ui.label("u16");
                                        ui.label(u16::from_le_bytes(bytes.clone().try_into().unwrap()).to_string());
                                        ui.end_row();
                                        ui.label("i16");
                                        ui.label(i16::from_le_bytes(bytes.try_into().unwrap()).to_string());
                                        ui.end_row();
                                    }
                                    4 => {
                                        ui.label("u32");
                                        ui.label(u32::from_le_bytes(bytes.clone().try_into().unwrap()).to_string());
                                        ui.end_row();
                                        ui.label("i32");
                                        ui.label(i32::from_le_bytes(bytes.clone().try_into().unwrap()).to_string());
                                        ui.end_row();
                                        // TODO: fix display of f32
                                        // ui.label("f32");
                                        // ui.label(f32::from_le_bytes(bytes.clone().try_into().unwrap()).to_string());
                                        // ui.end_row();
                                    }
                                    8 => {
                                        ui.label("u64");
                                        ui.label(u64::from_le_bytes(bytes.clone().try_into().unwrap()).to_string());
                                        ui.end_row();
                                        ui.label("i64");
                                        ui.label(i64::from_le_bytes(bytes.clone().try_into().unwrap()).to_string());
                                        ui.end_row();
                                        // TODO: fix display of f64
                                        // ui.label("f64");
                                        // ui.label(f64::from_le_bytes(bytes.clone().try_into().unwrap()).to_string());
                                        // ui.end_row();
                                    }
                                    _ => {
                                        ui.label("--");
                                        ui.label("--");
                                        ui.end_row();
                                    }
                                }
                            });
                    });
        });

        // RIGHT PANEL
        egui::SidePanel::right("search_panel").show(ctx, |ui| {
            ui.label("Search panel");
        });

        // CENTRAL VIEW
        egui::CentralPanel::default().show(ctx, |ui| {
            let bytes_per_row = 16;
            // Same as (self.max_addr - self.min_addr) / bytes_per_row
            // but division rounds result up
            let total_rows = ((self.max_addr - self.min_addr) + bytes_per_row - 1) /
                bytes_per_row;
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
                    //
                    for row in row_range {
                        ui.horizontal(|ui| {
                            // Start and end addresses
                            let start = self.min_addr + row * bytes_per_row;
                            let end = start + bytes_per_row;

                            // Display address (fixed width, monospaced)
                            ui.monospace(format!("{:08X}", start));

                            // Add space before hex block
                            ui.add_space(16.0);

                            // Hex bytes
                            for addr in start..end {
                                let byte = self.byte_addr_map.get(&addr).copied();

                                let mut is_selected = false;
                                if let Some(s) = &self.selected {
                                    if let Some(b) = byte {
                                        is_selected = s[0].0 <= addr && s[1].0 >= addr;
                                    }
                                };

                                // Change color of every other byte for better readability
                                let bg_color = if addr % 2 == 0 {
                                    egui::Color32::from_gray(210)
                                } else {
                                    egui::Color32::from_gray(160) // light gray
                                };

                                // Each byte is a button
                                let mut display_value = "--".to_string();
                                if let Some(b) = byte {
                                    display_value = format!("{:02X}", b);
                                }
                                let button = ui.add_sized(
                                    [21.0, 18.0],
                                    egui::Button::new(egui::RichText::new(display_value)
                                                          .monospace()
                                                          .size(12.0)
                                                          .color(bg_color),
                                    ).fill(egui::Color32::from_white_alpha(0)), // fully transparent,
                                );

                                let pointer_down = ui.input(|i| i.pointer.primary_down());
                                let pointer_hover = ui.input(|i| i.pointer.hover_pos());

                                if pointer_down && pointer_hover.is_some() && byte.is_some() && button.rect.contains(pointer_hover.unwrap()) {
                                    if self.was_released {
                                        self.was_released = false;
                                        self.selected = None;
                                    }
                                    let sel = self.selected
                                        .get_or_insert_with(|| [(addr, byte.unwrap()), (addr, byte.unwrap())]);
                                    sel[1] = (addr, byte.unwrap());
                                    // println!("{:?}", self.selected)
                                }

                                if !pointer_down {
                                    self.was_released = true;
                                }

                                if is_selected {
                                    // Highlight the selected byte
                                    ui.painter().rect_filled(
                                        button.rect,
                                        0.0,
                                        egui::Color32::from_rgba_premultiplied(33, 81, 109, 20),
                                        // 31, 53, 68
                                    );
                                }

                                // Add space every 8 bytes
                                if (addr + 1) % 8 == 0 {
                                    ui.add_space(5.0);
                                } else {
                                    // Make space between buttons as small as possible
                                    ui.add_space(-6.0);
                                }
                            }

                            // Add space before ASCII column
                            ui.add_space(16.0);

                            // ASCII representation
                            for addr in start..end {
                                let mut ch = ' ';
                                let mut byte = None;
                                if let Some(b) = self.byte_addr_map.get(&addr).copied() {
                                    byte = Some(b);
                                    ch = if b.is_ascii_graphic() {
                                        b as char
                                    } else {
                                        '.'
                                    }
                                }

                                let mut is_selected = false;
                                if let Some(s) = &self.selected {
                                    if let Some(b) = byte {
                                        is_selected = s[0].0 <= addr && s[1].0 >= addr;
                                    }
                                };

                                let label = ui.add(egui::Label::new(
                                    egui::RichText::new(ch.to_string())
                                        .color(egui::Color32::from_gray(160))
                                        .monospace(),
                                ));

                                let pointer_down = ui.input(|i| i.pointer.primary_down());
                                let pointer_hover = ui.input(|i| i.pointer.hover_pos());

                                if pointer_down && pointer_hover.is_some() && byte.is_some() && label.rect.contains(pointer_hover.unwrap()) {
                                    if self.was_released {
                                        self.was_released = false;
                                        self.selected = None;
                                    }
                                    let sel = self.selected
                                        .get_or_insert_with(|| [(addr, byte.unwrap()), (addr, byte.unwrap())]);
                                    sel[1] = (addr, byte.unwrap());
                                    // println!("{:?}", self.selected)
                                }

                                if !pointer_down {
                                    self.was_released = true;
                                }

                                if is_selected {
                                    // Highlight the selected byte
                                    ui.painter().rect_filled(
                                        label.rect,
                                        0.0,
                                        egui::Color32::from_rgba_premultiplied(33, 81, 109, 20),
                                        // 31, 53, 68
                                    );
                                }
                            }
                        });
                    };
                })
        });
    }
}