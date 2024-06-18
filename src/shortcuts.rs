#[cfg(unix)]
pub static CLOSE_APP: &str = "<Meta>Q";
#[cfg(windows)]
pub static CLOSE_APP: &str = "<Alt>F4";

#[cfg(unix)]
pub static TOGGLE_HIDDEN_ITEMS: &str = "<Meta>H";
#[cfg(windows)]
pub static TOGGLE_HIDDEN_ITEMS: &str = "<Alt>H";
