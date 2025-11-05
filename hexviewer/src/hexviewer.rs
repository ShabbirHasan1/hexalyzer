use intelhex::IntelHex;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub(crate) struct Selection {
    pub(crate) range: Option<[usize; 2]>,
    pub(crate) released: bool,
}

#[derive(Default)]
pub struct HexViewer {
    pub ih: IntelHex,
    pub byte_addr_map: BTreeMap<usize, u8>,
    pub min_addr: usize,
    pub max_addr: usize,
    pub selected: Selection,
    pub error: Option<String>,
}
