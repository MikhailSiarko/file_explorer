pub struct TopPanelInit {
    _has_parent_dir: bool,
    _show_hidden_items: bool,
}

impl TopPanelInit {
    pub fn new(has_parent_dir: bool, show_hidden_items: bool) -> Self {
        Self {
            _has_parent_dir: has_parent_dir,
            _show_hidden_items: show_hidden_items,
        }
    }

    pub fn has_parent_dir(&self) -> bool {
        self._has_parent_dir
    }

    pub fn show_hidden_items(&self) -> bool {
        self._show_hidden_items
    }
}
