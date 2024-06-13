use relm4::factory::DynamicIndex;

#[derive(Debug)]
pub enum ItemOutput {
    OpenFile(String),
    OpenDirectory(String),
    ItemSelected(DynamicIndex),
}
