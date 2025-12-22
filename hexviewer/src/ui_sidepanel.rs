use crate::app::HexViewerApp;
use crate::ui_inspector::format_with_separators;
use eframe::egui;

impl HexViewerApp {
    pub(crate) fn show_side_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_panel")
            .exact_width(280.0)
            .show(ctx, |ui| {
                ui.add_space(3.0);

                // FILE INFORMATION
                egui::CollapsingHeader::new("File Information")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add_space(5.0);

                        let filepath = self.ih.filepath.to_string_lossy().into_owned();
                        let filename = self
                            .ih
                            .filepath
                            .file_name()
                            .map_or_else(|| "--".to_string(), |n| n.to_string_lossy().into_owned());

                        egui::Grid::new("file_info_grid")
                            .num_columns(2) // two columns: label + value
                            .spacing([30.0, 4.0]) // horizontal & vertical spacing
                            .show(ui, |ui| {
                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::LEFT),
                                    |ui| {
                                        ui.label("File Name");
                                    },
                                );
                                // Wrap the name + show the filepath on hover
                                let response = ui.add(
                                    egui::Label::new(filename)
                                        .wrap()
                                        .sense(egui::Sense::hover()),
                                );
                                if !filepath.is_empty() {
                                    response.on_hover_text(&filepath);
                                }
                                ui.end_row();

                                ui.with_layout(
                                    egui::Layout::left_to_right(egui::Align::LEFT),
                                    |ui| {
                                        ui.label("File Size");
                                    },
                                );
                                let size = format_with_separators(self.ih.size);
                                ui.label(format!("{size} bytes"));
                                ui.end_row();
                            });

                        ui.add_space(5.0);
                    });

                ui.add_space(3.0);

                // JUMP TO ADDRESS
                egui::CollapsingHeader::new("Jump To Address")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add_space(5.0);
                        self.show_jumpto_contents(ui);
                        ui.add_space(5.0);
                    });

                ui.add_space(3.0);

                // SEARCH
                egui::CollapsingHeader::new("Search")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add_space(5.0);
                        self.show_search_contents(ui);
                        ui.add_space(5.0);
                    });

                ui.add_space(3.0);

                // DATA INSPECTOR
                egui::CollapsingHeader::new("Data Inspector")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add_space(5.0);
                        self.show_data_inspector_contents(ui);
                        ui.add_space(5.0);
                    });
            });
    }
}
