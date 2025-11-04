use std::collections::{BTreeMap, HashMap};
use intelhex::IntelHex;


#[derive(Default)]
pub struct HexViewer {
    pub(crate) ih: IntelHex,
    pub(crate) byte_addr_map: BTreeMap<usize, u8>,
    pub(crate) min_addr: usize,
    pub(crate) max_addr: usize,
    // pub(crate) selected: Option<(usize, u8)>,
    // pub(crate) selected: Option<HashMap<usize, u8>>,
    pub(crate) selected: Option<[(usize, u8); 2]>,
    // pub(crate) selection: String,
    pub(crate) error: Option<String>,
    pub(crate) was_released: bool,
}
