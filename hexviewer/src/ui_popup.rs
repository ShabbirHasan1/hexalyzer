use crate::{HexViewer, color};
use eframe::egui;

//  ========================== Popup Type logic ============================= //

#[derive(Clone, PartialEq, Eq)]
pub enum PopupType {
    Error,
    About,
    ReAddr,
}

impl PopupType {
    pub const fn title(&self) -> &'static str {
        match self {
            PopupType::Error => "Error",
            PopupType::About => "About",
            PopupType::ReAddr => "Re-Address",
        }
    }
}

//  ========================== Popup logic =================================== //

pub struct Popup {
    pub(crate) active: bool,
    pub(crate) ptype: Option<PopupType>,
}

impl Popup {
    pub const fn clear(&mut self) {
        self.active = false;
        self.ptype = None;
    }
}

//  ========================== HexViewer logic ============================= //

impl HexViewer {
    fn display_error(&self, ui: &mut egui::Ui) -> bool {
        ui.label(self.error.as_ref().unwrap());

        // Add space before close button
        ui.add_space(10.0);

        // Keep the window open
        false
    }

    fn display_about(&self, ui: &mut egui::Ui) -> bool {
        ui.vertical(|ui| {
            ui.label("IntelHex");
            ui.label("...");
        });

        // Keep the window open
        false
    }

    fn display_readdr(&mut self, ui: &mut egui::Ui) -> bool {
        ui.text_edit_singleline(&mut self.addr.new_str);

        if ui.button("OK").clicked() {
            self.addr.set_new_start_addr();

            // Close the window
            return true;
        }

        // Keep the window open
        false
    }

    /// Show pop-up
    pub(crate) fn show_popup(&mut self, ctx: &egui::Context) {
        let content_rect = ctx.content_rect();

        // Block interaction with the app
        egui::Area::new(egui::Id::from("modal_blocker"))
            .order(egui::Order::Background)
            .fixed_pos(content_rect.left_top())
            .show(ctx, |ui| {
                ui.allocate_rect(content_rect, egui::Sense::click());
            });

        // Darken the background
        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Background,
            egui::Id::new("modal_bg"),
        ));
        painter.rect_filled(content_rect, 0.0, color::SHADOW);

        let mut is_open = self.popup.active;
        let was_open = self.popup.active;

        // TODO: edge case
        let popup_type = self.popup.ptype.clone().unwrap();

        // Display the pop-up
        let window = egui::Window::new(popup_type.title())
            .open(&mut is_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]);

        let mut close = false;
        window.show(ctx, |ui| match popup_type {
            PopupType::Error => close = self.display_error(ui),
            PopupType::About => close = self.display_about(ui),
            PopupType::ReAddr => close = self.display_readdr(ui),
        });

        // nasty logic...
        is_open = !close && is_open;
        self.popup.active = is_open;

        // If the window got closed this frame
        if was_open && !self.popup.active {
            self.error = None;

            if self.popup.ptype == Some(PopupType::ReAddr) {
                // Re-address the IntelHex
                match self.ih.relocate(self.addr.min) {
                    Ok(()) => {}
                    Err(err) => {
                        self.popup.clear();
                        self.error = Some(err.to_string());
                        return;
                    }
                }

                // Clear addr
                self.addr.clear();

                // Re-calculate address range
                self.addr.update_range(&self.ih);
            }

            self.popup.clear();
        }
    }
}
