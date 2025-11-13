#[derive(Default)]
pub(crate) struct Editor {
    /// Is byte being edited
    pub(crate) in_progress: bool,
    /// Buffer to store byte data during editing
    pub(crate) buffer: String,
    /// Address range of the bytes being edited
    pub(crate) addr: Option<[usize; 2]>,
}

impl Editor {
    /// Clear the editor (edit process complete/canceled)
    pub(crate) fn clear(&mut self) {
        self.in_progress = false;
        self.addr = None;
        self.buffer.clear();
    }

    /// Is provided address same as being edited
    pub(crate) fn is_addr_same(&self, addr: Option<[usize; 2]>) -> bool {
        addr == self.addr
    }
}
