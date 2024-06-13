pub struct ItemsBoxInit {
    _current_dir: String,
    _show_hidden_items: bool,
}

impl ItemsBoxInit {
    pub fn new(current_dir: &str, show_hidden_items: bool) -> Self {
        Self {
            _current_dir: current_dir.to_owned(),
            _show_hidden_items: show_hidden_items,
        }
    }

    pub fn current_dir(&self) -> &str {
        &self._current_dir
    }

    pub fn show_hidden_items(&self) -> bool {
        self._show_hidden_items
    }
}
