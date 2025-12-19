// #![warn(clippy::all)]
// #![warn(clippy::pedantic)]
// #![warn(clippy::nursery)]
// // Optional stricter rules
// #![warn(clippy::unwrap_used)]
// #![warn(clippy::expect_used)]
// #![warn(clippy::panic)]

mod address;
mod app;
mod byteedit;
mod loader;
mod selection;
mod ui_centralpanel;
mod ui_events;
mod ui_fileinfo;
mod ui_inspector;
mod ui_jumpto;
mod ui_popup;
mod ui_scrollarea;
mod ui_search;
mod ui_sidepanel;
mod ui_topbar;
mod utils;

use crate::ui_popup::PopupType;
use app::HexViewerApp;
use eframe::egui;
use eframe::egui::ViewportBuilder;

pub(crate) mod colors {
    use eframe::egui::Color32;

    pub const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);
    pub const LIGHT_BLUE: Color32 = Color32::from_rgba_premultiplied(33, 81, 109, 20);
    pub const MUD: Color32 = Color32::from_rgba_premultiplied(54, 44, 19, 20);
    pub const GREEN: Color32 = Color32::from_rgba_premultiplied(35, 53, 38, 20);
    pub const GRAY_160: Color32 = Color32::from_gray(160);
    pub const GRAY_210: Color32 = Color32::from_gray(210);
    pub const SHADOW: Color32 = Color32::from_black_alpha(150);
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        vsync: false,
        viewport: ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hexalyzer",
        options,
        Box::new(|_cc| Ok(Box::new(HexViewerApp::default()))),
    )
}

impl eframe::App for HexViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_top_bar(ctx);

        // TODO: move this somewhere
        if self.error.is_some() {
            self.popup.active = true;
            self.popup.ptype = Some(PopupType::Error);
        }

        self.show_side_panel(ctx);

        if self.popup.active {
            self.show_popup(ctx);
        } else {
            self.show_central_panel(ctx);
        }
    }
}

// TODO for MVP:
// Edge cases - re-addr when no ih, etc..
// Drag and drop files?
// Verify export works OK
// Add content to help
// Verify performance acceptable (cap if needed)
// Polish up code
// Add documentation

// TODO further:
// Use LayoutJob or other methods to do custom bytes display instead of widget (BIG TASK)
// Prefetch the visible window into a Vec and render from that cache.
// Instead of rendering gaps fully, use egui Separator or other thing to show the address gap
