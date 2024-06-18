use relm4::factory::DynamicIndex;

#[derive(Debug)]
pub enum ItemsBoxInput {
    LoadDirectory(String),
    ToggleShowHiddenItems,
    ShowHiddenItems(bool),
    OpenFile(String),
    SelectItem(DynamicIndex),
}
