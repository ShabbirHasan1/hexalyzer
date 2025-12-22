use intelhex::IntelHex;

#[derive(Default)]
pub struct Address {
    pub(crate) min: usize,
    pub(crate) max: usize,
    pub(crate) new_start: String,
}

impl Address {
    pub(crate) fn clear(&mut self) {
        self.min = 0;
        self.max = 0;
        self.new_start = String::new();
    }

    pub(crate) fn update_range(&mut self, ih: &IntelHex) {
        self.min = ih.get_min_addr().unwrap_or(0);
        self.max = ih.get_max_addr().unwrap_or(0);
    }

    pub(crate) fn set_new_start_addr(&mut self) {
        if let Ok(addr) = usize::from_str_radix(&self.new_start, 16) {
            self.min = addr;
        }
    }
}
