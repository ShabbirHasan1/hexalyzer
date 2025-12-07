use crate::HexViewer;
use eframe::egui;
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
                            self.editor.reset();
                            // Fill min/max address
                            self.addr_range.start = self.ih.get_min_addr().unwrap();
                            self.addr_range.end = self.ih.get_max_addr().unwrap();
                        }
                    }

                    // EXPORT BUTTON
                    if ui.button("Export").clicked()
                        && let Some(path) = rfd::FileDialog::new().set_title("Save As").save_file()
                    {
                        match self.ih.write_hex(path) {
                            Ok(_) => {}
                            Err(msg) => {
                                self.error = Some(msg.to_string());
                            }
                        }
                    }
                });

                // ABOUT BUTTON
                let about_button = ui.button("About");

                if about_button.clicked() {
                    self.help_menu_open = true;
                }

                let window = egui::Window::new("About")
                    .open(&mut self.help_menu_open)
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]);
                    // .title_bar(false);

                window.show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("IntelHex");
                        ui.label("...");
                    });
                });
            });
        });
    }
}
