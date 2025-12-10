use crate::hexviewer::HexViewer;
use eframe::egui;

impl HexViewer {
    pub(crate) fn show_side_panel(&mut self, ctx: &egui::Context) {
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
                // JUMP TO ADDRESS
                egui::CollapsingHeader::new("Jump To Address")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add_space(5.0);
                        self.show_jumpto_contents(ui);
                        ui.add_space(5.0);
                    });
                // SEARCH
                egui::CollapsingHeader::new("Search")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add_space(5.0);
                        self.show_search_contents(ui);
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
    }
}
