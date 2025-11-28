use crate::byteedit::ByteEdit;
use crate::selection::Selection;
use crate::ui_jumpto::JumpTo;
use crate::ui_search::Search;
use intelhex::IntelHex;
use std::collections::BTreeMap;
use std::ops::Range;

#[derive(Default, PartialEq)]
pub enum Endianness {
    #[default]
    Little,
    Big,
}

#[derive(Default)]
pub struct HexViewer {
    /// IntelHex object returned by intelhex library
    pub ih: IntelHex,
    /// Address-to-byte map
    pub byte_addr_map: BTreeMap<usize, u8>,
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
}
