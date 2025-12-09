use crate::byteedit::ByteEdit;
use crate::selection::Selection;
use crate::ui_jumpto::JumpTo;
use crate::ui_popup::Popup;
use crate::ui_search::Search;
use intelhex::IntelHex;
use std::ops::Range;
use std::time::Instant;

#[derive(PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

pub struct HexViewer {
    /// IntelHex object returned by intelhex library
    pub ih: IntelHex,
    /// Start and end addresses of the hex data
    pub addr_range: Range<usize>,
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
    /// Last frame time (for capping app's FPS)
    pub(crate) last_frame_time: Instant,
    /// Pop up handler
    pub(crate) popup: Popup,
}

impl Default for HexViewer {
    fn default() -> Self {
        Self {
            ih: IntelHex::default(),
            addr_range: Range::default(),
            bytes_per_row: 32,
            endianness: Endianness::Little,
            error: None,
            editor: ByteEdit::default(),
            selection: Selection::default(),
            search: Search::default(),
            jump_to: JumpTo::default(),
            last_frame_time: Instant::now(),
            popup: Popup {
                active: false,
                ptype: None,
            },
        }
    }
}
