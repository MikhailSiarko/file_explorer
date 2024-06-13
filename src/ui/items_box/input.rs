use relm4::factory::DynamicIndex;

#[derive(Debug)]
pub enum ItemsBoxInput {
    LoadDirectory(String),
    ShowHiddenItems(bool),
    OpenFile(String),
    SelectItem(DynamicIndex),
}
