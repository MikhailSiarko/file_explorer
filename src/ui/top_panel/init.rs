pub struct TopPanelInit {
    _has_parent_dir: bool,
}

impl TopPanelInit {
    pub fn new(has_parent_dir: bool) -> Self {
        Self {
            _has_parent_dir: has_parent_dir,
        }
    }

    pub fn has_parent_dir(&self) -> bool {
        self._has_parent_dir
    }
}
