use crate::address::Address;
use crate::byteedit::ByteEdit;
use crate::selection::Selection;
use crate::ui_jumpto::JumpTo;
use crate::ui_popup::Popup;
use crate::ui_search::Search;
use intelhex::IntelHex;

#[derive(PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

pub struct HexViewerApp {
    /// `IntelHex` object returned by intelhex library
    pub ih: IntelHex,
    /// Address handling of the hex data
    pub addr: Address,
    /// Bytes per row to display
    pub bytes_per_row: usize, // TODO: make configurable
    /// Endianness of the hex data
    pub endianness: Endianness,
    /// Error during intelhex parsing
    pub error: Option<String>,
    /// Handler for byte editing
    pub editor: ByteEdit,
    /// Handler for GUI feature of bytes selection
    pub selection: Selection,
    /// Handler for GUI feature to search for byte string
    pub search: Search,
    /// Handler for GUI feature to jump to selected address
    pub jump_to: JumpTo,
    /// Pop up handler
    pub popup: Popup,
}

impl Default for HexViewerApp {
    fn default() -> Self {
        Self {
            ih: IntelHex::default(),
            addr: Address::default(),
            bytes_per_row: 16,
            endianness: Endianness::Little,
            error: None,
            editor: ByteEdit::default(),
            selection: Selection::default(),
            search: Search::default(),
            jump_to: JumpTo::default(),
            popup: Popup {
                active: false,
                ptype: None,
            },
        }
    }
}
