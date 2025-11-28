use crate::HexViewer;
use eframe::egui;
// use eframe::egui::debug_text::print;
use intelhex::IntelHex;

impl HexViewer {
    pub(crate) fn show_top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // FILE MENU
                ui.menu_button("File", |ui| {
                    // OPEN BUTTON
                    if ui.button("Open").clicked()
                        && let Some(path) = rfd::FileDialog::new()
                            .set_title("Open Hex File")
                            .pick_file()
                    {
                        let ih = IntelHex::from_hex(path);

                        if let Err(msg) = ih {
                            self.error = Some(msg.to_string());
                        } else {
                            self.ih = ih.unwrap();
                            // Clear the map if another hex was loaded before
                            self.byte_addr_map.clear();
                            // Fill data array (TODO: don't store the data at all - access directly via ih)
                            for (addr, byte) in &self.ih.to_btree_map() {
                                self.byte_addr_map.insert(*addr, *byte);
                            }
                            // Fill address
                            self.addr_range.start = *self.byte_addr_map.keys().min().unwrap();
                            self.addr_range.end = *self.byte_addr_map.keys().max().unwrap();
                        }
                    }

                    // EXPORT BUTTON
                    if ui.button("Export").clicked()
                        && let Some(path) = rfd::FileDialog::new().set_title("Save As").save_file()
                    {
                        // TODO: handle saving going wrong
                        // TODO: implement proper solution
                        let vec: Vec<(usize, u8)> =
                            self.byte_addr_map.iter().map(|(&k, &v)| (k, v)).collect();
                        match self.ih.update_buffer_slice(vec.as_slice()) {
                            Ok(_) => {}
                            Err(msg) => {
                                self.error = Some(msg.to_string());
                            }
                        }

                        // println!("{:?}", self.byte_addr_map);

                        match self.ih.write_hex(path) {
                            Ok(_) => {}
                            Err(msg) => {
                                self.error = Some(msg.to_string());
                            }
                        }
                    }
                });

                // TODO: HELP BUTTON
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        println!("About clicked");
                    }
                });
            });
        });
    }
}
