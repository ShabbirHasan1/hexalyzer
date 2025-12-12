use crate::HexViewer;
use crate::loader;
use crate::ui_popup::PopupType;
use eframe::egui;
use intelhex::IntelHex;

impl HexViewer {
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
                                self.ih = ih;
                                self.editor.reset();
                                // Fill min/max addresses
                                self.addr.update_range(&self.ih);
                            }
                        }

                        // EXPORT BUTTON
                        if ui.button("Export").clicked()
                            && let Some(path) = rfd::FileDialog::new().set_title("Save As").save_file()
                        {
                            match self.ih.write_hex(path) {
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
