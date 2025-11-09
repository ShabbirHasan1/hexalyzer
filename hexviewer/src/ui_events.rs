use eframe::egui;
use crate::hexviewer::HexViewer;

impl HexViewer {
    pub(crate) fn get_keyboard_input_event(ui: &egui::Ui) -> Option<char> {
        ui.input(|i| {
            for event in &i.events {
                if let egui::Event::Text(t) = event
                    && let Some(c) = t.chars().next()
                {
                    return Some(c);
                }
            }
            None
        })
    }
}