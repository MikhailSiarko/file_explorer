#[derive(Debug)]
pub enum TopPanelInput {
    DirectoryLoaded(bool),
    ShowHiddenItems(bool),
    ToggleShowHiddenItems,
}
