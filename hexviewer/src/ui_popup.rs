use crate::HexViewerApp;
use crate::app::colors;
use crate::events::collect_ui_events;
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
            Self::Error => "Error",
            Self::About => "About",
            Self::ReAddr => "Re-Address",
        }
    }
}

//  ========================== Popup logic =================================== //

#[derive(Default)]
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

impl HexViewerApp {
    fn display_error(ui: &mut egui::Ui, msg: &str) -> bool {
        ui.label(msg);

        // Add space before close button
        ui.add_space(10.0);

        // Keep the window open
        false
    }

    fn display_about(ui: &mut egui::Ui) -> bool {
        ui.vertical(|ui| {
            ui.add_space(5.0);

            ui.heading("Hexalyzer");
            ui.label("Cross-platform hex viewing and editing app");

            ui.add_space(3.0);
            ui.separator();
            ui.add_space(3.0);

            ui.label(
                "The app is built with *egui* - immediate-mode GUI library.\
            The hex parsing and writing is handled by IntelHex library, built as part of the \
            same project.\n\nThe app does not support partial file loading (yet?) so RAM usage \
            while working with very large files will be high.",
            );

            ui.add_space(5.0);
        });

        // Keep the window open
        false
    }

    fn display_readdr(&mut self, ui: &mut egui::Ui) -> bool {
        ui.horizontal(|ui| {
            ui.label("New start address:");
            ui.add_space(1.5);
            ui.add(
                egui::TextEdit::singleline(&mut self.addr.new_start)
                    .desired_width(ui.available_width() - 70.0),
            );
        });

        ui.add_space(10.0);

        if ui.button("OK").clicked() || self.events.enter_released {
            self.addr.set_new_start_addr();

            // Redo search
            self.search.redo();

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

                // Collect input events once per frame and store in the app state
                self.events = collect_ui_events(ui);
            });

        // Darken the background
        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Background,
            egui::Id::new("modal_bg"),
        ));
        painter.rect_filled(content_rect, 0.0, colors::SHADOW);

        let mut is_open = self.popup.active;
        let was_open = self.popup.active;

        let Some(popup_type) = self.popup.ptype.clone() else {
            self.popup.clear();
            return;
        };

        // Display the pop-up
        let window = egui::Window::new(popup_type.title())
            .open(&mut is_open)
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0]);

        // Track OK button or Enter press
        let mut close_confirm = false;

        window.show(ctx, |ui| match popup_type {
            PopupType::Error => {
                let error = self.error.as_deref().unwrap_or("?");
                close_confirm = Self::display_error(ui, error);
            }
            PopupType::About => close_confirm = Self::display_about(ui),
            PopupType::ReAddr => close_confirm = self.display_readdr(ui),
        });

        self.popup.active = !close_confirm && is_open && !self.events.escape_pressed;

        // If the window got closed this frame
        if was_open && !self.popup.active {
            self.error = None;

            if self.popup.ptype == Some(PopupType::ReAddr) && close_confirm {
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
