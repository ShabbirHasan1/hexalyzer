#[derive(Debug, Default)]
pub struct Selection {
    /// Range is a start and end addresses of selected bytes.
    /// Inverted if selection is moving right-to-left.
    pub(crate) range: Option<[usize; 2]>,
    /// Is cursor click removed after being pressed
    pub(crate) released: bool,
}

impl Selection {
    /// Check if provided addr is within the selection range
    pub(crate) const fn is_addr_within_range(&self, addr: usize) -> bool {
        if let Some(range) = self.range {
            if range[0] < range[1] {
                return range[0] <= addr && range[1] >= addr;
            }
            return range[1] <= addr && range[0] >= addr;
        }
        false
    }
    /// Extend selection range with provided addr
    pub(crate) fn update(&mut self, addr: usize) {
        if self.released {
            self.released = false;
            self.range = None;
        }
        let sel = self.range.get_or_insert([addr, addr]);
        sel[1] = addr;
    }

    pub(crate) const fn clear(&mut self) {
        self.range = None;
        self.released = false;
    }
}
