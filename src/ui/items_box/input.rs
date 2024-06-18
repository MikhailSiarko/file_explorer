use relm4::factory::DynamicIndex;

#[derive(Debug)]
pub enum ItemsBoxInput {
    LoadDirectory(String),
    ToggleShowHiddenItems,
    OpenFile(String),
    SelectItem(DynamicIndex),
}
