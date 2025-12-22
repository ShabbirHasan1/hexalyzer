use crate::app::HexViewerApp;
use eframe::egui;

#[derive(Default)]
pub struct JumpTo {
    /// Is the text edit window in focus
    pub(crate) has_focus: bool,
    /// Address to jump to
    pub(crate) addr: Option<usize>,
    /// User input string
    input: String,
    loose_focus: bool,
}

impl JumpTo {
    pub(crate) const fn loose_focus(&mut self) {
        self.loose_focus = true;
    }
}

impl HexViewerApp {
    /// Show contents of jumpto menu
    pub(crate) fn show_jumpto_contents(&mut self, ui: &mut egui::Ui) {
        let textedit = ui.add(
            egui::TextEdit::singleline(&mut self.jump_to.input)
                .desired_width(ui.available_width() - 30.0),
        );

        if self.jump_to.loose_focus {
            textedit.surrender_focus();
            self.jump_to.loose_focus = false;
        }

        if textedit.has_focus() {
            self.search.has_focus = false;
            self.jump_to.has_focus = true;

            // Clear the selection to avoid modifying bytes
            // while typing in the jumpto area
            self.selection.clear();
        }

        if let Some(key) = self.events.last_key_released
            && key == egui::Key::Enter
            && self.jump_to.has_focus
        {
            self.jump_to.addr = usize::from_str_radix(&self.jump_to.input, 16).ok();
        }
    }
}
