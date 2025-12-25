use crate::byteedit::ByteEdit;
use crate::events::EventState;
use crate::selection::Selection;
use crate::ui_jumpto::JumpTo;
use crate::ui_popup::Popup;
use crate::ui_search::Search;
use intelhexlib::IntelHex;
use std::ops::RangeInclusive;

pub mod colors {
    use eframe::egui::Color32;

    // pub const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);
    pub const LIGHT_BLUE: Color32 = Color32::from_rgba_premultiplied(33, 81, 109, 20);
    pub const MUD: Color32 = Color32::from_rgba_premultiplied(54, 44, 19, 20);
    pub const GREEN: Color32 = Color32::from_rgba_premultiplied(35, 53, 38, 20);
    pub const GRAY_160: Color32 = Color32::from_gray(160);
    pub const GRAY_210: Color32 = Color32::from_gray(210);
    pub const SHADOW: Color32 = Color32::from_black_alpha(150);
}

#[derive(PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

pub struct HexViewerApp {
    /// `IntelHex` object returned by `intelhexlib`
    pub ih: IntelHex,
    /// Address range of the hex data
    pub addr: RangeInclusive<usize>,
    /// Displayed bytes per row
    pub bytes_per_row: usize,
    /// Endianness of the hex data
    pub endianness: Endianness,
    /// Errors during parsing, editing, or writing `IntelHex` file
    pub error: Option<String>,
    /// Handler for bytes editing
    pub editor: ByteEdit,
    /// Handler for GUI feature of bytes selection
    pub selection: Selection,
    /// Handler for GUI feature to search for byte string
    pub search: Search,
    /// Handler for GUI feature to jump to selected address
    pub jump_to: JumpTo,
    /// Pop up handler
    pub popup: Popup,
    /// Per-frame state of user inputs
    pub events: EventState,
}

impl Default for HexViewerApp {
    fn default() -> Self {
        Self {
            ih: IntelHex::default(),
            addr: 0..=0,
            bytes_per_row: 16,
            endianness: Endianness::Little,
            error: None,
            editor: ByteEdit::default(),
            selection: Selection::default(),
            search: Search::default(),
            jump_to: JumpTo::default(),
            popup: Popup::default(),
            events: EventState::default(),
        }
    }
}

impl HexViewerApp {
    pub(crate) fn clear(&mut self) {
        self.ih = IntelHex::default();
        self.addr = 0..=0;
        self.error = None;
        self.editor = ByteEdit::default();
        self.selection = Selection::default();
        self.search = Search::default();
        self.jump_to = JumpTo::default();
        self.popup = Popup::default();
        self.events = EventState::default();
    }
}
