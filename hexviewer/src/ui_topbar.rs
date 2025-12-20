use crate::HexViewerApp;
use crate::loader;
use crate::ui_popup::PopupType;
use eframe::egui;
use intelhex::IntelHex;
use std::error::Error;

enum SaveFormat {
    Binary,
    Hex,
}

fn format_from_extension(path: &std::path::Path) -> Option<SaveFormat> {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())?
        .as_str()
    {
        "bin" => Some(SaveFormat::Binary),
        "hex" => Some(SaveFormat::Hex),
        _ => None,
    }
}

impl HexViewerApp {
    pub(crate) fn show_top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            ui.add_space(3.0);

            egui::MenuBar::new().ui(ui, |ui| {
                ui.horizontal(|ui| {
                    // FILE MENU
                    ui.menu_button("File", |ui| {
                        // OPEN BUTTON
                        if ui.button("Open").clicked()
                            && let Some(path) =
                                rfd::FileDialog::new().set_title("Open File").pick_file()
                        {
                            let mut ih = IntelHex::new();
                            let res = loader::load_file(&path, &mut ih);

                            if let Err(msg) = res {
                                self.error = Some(msg.to_string());
                            } else {
                                // Clear the state of the app
                                self.clear();

                                // Load the IntelHex
                                self.ih = ih;

                                // Fill min/max addresses
                                self.addr.update_range(&self.ih);
                            }
                        }

                        // EXPORT BUTTON
                        if ui.button("Export").clicked()
                            && let Some(path) = rfd::FileDialog::new()
                                .set_title("Save As")
                                .add_filter("Binary", &["bin"])
                                .add_filter("Hex", &["hex"])
                                .save_file()
                        {
                            let format = format_from_extension(&path).unwrap_or(SaveFormat::Hex);

                            let res: Result<(), Box<dyn Error>> = match format {
                                SaveFormat::Binary => self.ih.write_bin(path),
                                SaveFormat::Hex => self.ih.write_hex(path),
                            };
                            match res {
                                Ok(()) => {}
                                Err(msg) => {
                                    self.error = Some(msg.to_string());
                                }
                            }
                        }
                    });

                    // EDIT BUTTON
                    ui.menu_button("Edit", |ui| {
                        // OPEN BUTTON
                        if ui.button("Re-address").clicked() {
                            self.popup.active = true;
                            self.popup.ptype = Some(PopupType::ReAddr);
                        }
                    });

                    // VIEW BUTTON
                    ui.menu_button("View", |ui| {
                        ui.label("Select Bytes per Row:");

                        ui.add_space(3.0);

                        // RadioButtons to select between 16 and 32 bytes per row
                        ui.radio_value(&mut self.bytes_per_row, 16, "16 bytes");
                        ui.add_space(1.0);
                        ui.radio_value(&mut self.bytes_per_row, 32, "32 bytes");
                    });

                    // ABOUT BUTTON
                    let about_button = ui.button("About");

                    if about_button.clicked() {
                        self.popup.active = true;
                        self.popup.ptype = Some(PopupType::About);
                    }
                });
            });

            ui.add_space(2.0);
        });
    }
}
