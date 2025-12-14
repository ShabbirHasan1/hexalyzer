use crate::hexviewer::HexViewer;
use crate::ui_events::EventManager;
use eframe::egui;

#[derive(Default)]
pub(crate) struct JumpTo {
    /// Is the text edit window in focus
    pub(crate) has_focus: bool,
    /// Address to jump to
    pub(crate) addr: Option<usize>,
    /// User input string
    input: String,
}

impl HexViewer {
    /// Show contents of jumpto menu
    pub(crate) fn show_jumpto_contents(&mut self, ui: &mut egui::Ui) {
        let textedit = ui.add(
            egui::TextEdit::singleline(&mut self.jump_to.input)
                .desired_width(ui.available_width() - 30.0),
        );

        if textedit.has_focus() {
            self.search.has_focus = false;
            self.jump_to.has_focus = true;
        }

        if let Some(key) = EventManager::get_keyboard_input_key(ui)
            && key == egui::Key::Enter
            && self.jump_to.has_focus
        {
            self.jump_to.addr = usize::from_str_radix(&self.jump_to.input, 16).ok();
        }
    }
}
