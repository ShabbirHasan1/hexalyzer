use crate::{HexViewer, color};
use eframe::egui;

//  ========================== Popup Type logic ============================= //

#[derive(Clone)]
pub enum PopupType {
    Error,
    About,
}

impl PopupType {
    pub fn title(&self) -> &'static str {
        match self {
            PopupType::Error => "Error",
            PopupType::About => "About",
        }
    }
}

//  ========================== Popup logic =================================== //

pub struct Popup {
    pub(crate) active: bool,
    pub(crate) ptype: Option<PopupType>,
}

impl Popup {
    pub fn clear(&mut self) {
        self.active = false;
        self.ptype = None;
    }
}

//  ========================== HexViewer logic ============================= //

impl HexViewer {
    fn show_error(&self, ui: &mut egui::Ui) {
        ui.label(self.error.as_ref().unwrap());

        // Add space before close button
        ui.add_space(10.0);
    }

    fn show_about(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label("IntelHex");
            ui.label("...");
        });
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
        let popup_type = self.popup.ptype.clone().unwrap();

        // Display the pop-up
        let window = egui::Window::new(popup_type.title())
            .open(&mut is_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]);

        window.show(ctx, |ui| match popup_type {
            PopupType::Error => {
                self.show_error(ui);
            }
            PopupType::About => {
                self.show_about(ui);
            }
        });

        self.popup.active = is_open;

        // If the window got closed this frame
        if was_open && !self.popup.active {
            self.error = None;
            self.popup.clear();
        }
    }
}
