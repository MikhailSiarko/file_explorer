use std::path::PathBuf;

pub struct ItemInit {
    _name: String,
    _path: String,
    _is_file: bool,
}

impl ItemInit {
    pub fn name(&self) -> &str {
        &self._name
    }

    pub fn path(&self) -> &str {
        &self._path
    }

    pub fn is_file(&self) -> bool {
        self._is_file
    }
}

impl From<&PathBuf> for ItemInit {
    fn from(value: &PathBuf) -> Self {
        Self {
            _name: String::from(value.file_name().unwrap().to_str().unwrap()),
            _path: value.display().to_string(),
            _is_file: value.is_file(),
        }
    }
}
