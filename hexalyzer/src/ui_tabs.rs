use crate::app::HexViewerApp;
use crate::ui_button;
use eframe::egui;

impl HexViewerApp {
    /// Show tabs with the list of open files.
    /// Tabs are constrained to fit into the available space.
    /// If the number of tabs does not exceed the maximum allowed, the "Open New File" tab is added.
    pub(crate) fn show_tabs(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tabs_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut tab_to_close = None;

                // Modify spacing between tabs
                let spacing = 2.0;
                ui.spacing_mut().item_spacing.x = spacing;

                // Calculate ideal widths for all tabs
                let mut ideal_widths = Vec::new();
                let font_id = egui::TextStyle::Body.resolve(ui.style());

                // Estimate width of each tab: name width + padding + close button space
                for session in &self.sessions {
                    let galley = ui.painter().layout_no_wrap(
                        session.name.clone(),
                        font_id.clone(),
                        ui.visuals().widgets.active.text_color(),
                    );
                    let text_width = galley.size().x;
                    let ideal_w = text_width + 45.0; // 45px for margins and "×" button
                    ideal_widths.push(ideal_w);
                }

                // Determine scaling
                let add_button_width = if self.sessions.len() < self.max_tabs {
                    70.0
                } else {
                    40.0
                };
                let available_width = ui.available_width() - add_button_width;
                let total_ideal_width: f32 = ideal_widths.iter().sum();
                // Only scale down if we actually exceed the available space
                let scale_factor = if total_ideal_width > available_width {
                    available_width / total_ideal_width
                } else {
                    1.0
                };

                for (i, session) in self.sessions.iter().enumerate() {
                    let is_active = Some(i) == self.active_index;

                    // Get width for this tab, scaled by the scaling factor
                    let dynamic_width = ideal_widths[i] * scale_factor;

                    // Create a constrained UI for each tab (to fit all tabs into the tab bar)
                    ui.allocate_ui(egui::vec2(dynamic_width, ui.available_height()), |ui| {
                        let (response, close_clicked) =
                            ui_button::tab_style_button(ui, ("tab", i), is_active, |ui| {
                                // Truncate the name if it is too long for the calculated width
                                let truncated_name = egui::RichText::new(&session.name);
                                ui.add(egui::Label::new(truncated_name).truncate());

                                // Close button
                                ui.button("×").clicked()
                            });

                        if close_clicked {
                            tab_to_close = Some(i);
                        } else if response.clicked() {
                            self.active_index = Some(i);
                        }
                    });
                }

                // Handle closing tabs after the loop to avoid borrow checker issues
                if let Some(i) = tab_to_close {
                    self.close_file(i);
                }

                // "Open New File" tab button
                if self.sessions.len() < self.max_tabs {
                    let (response, ()) = ui_button::tab_style_button(ui, "add_tab", false, |ui| {
                        ui.label(egui::RichText::new(" + ").strong());
                    });
                    if response.on_hover_text("Open New File").clicked()
                        && let Some(path) =
                            rfd::FileDialog::new().set_title("Open File").pick_file()
                    {
                        self.load_file(&path);
                    }
                }
            });
        });
    }
}
