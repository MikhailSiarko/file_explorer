use std::path::PathBuf;

use iced::{
    theme,
    widget::{mouse_area, row, svg, text, Svg},
    Element, Length, Theme,
};

use crate::Message;

use super::styles::SvgStyles;

pub fn view(path_buf: &PathBuf) -> Element<Message> {
    let container = row!(
        get_icon(&path_buf).width(Length::FillPortion(1)),
        text(path_buf.file_name().unwrap().to_str().unwrap()).width(Length::FillPortion(9)),
    )
    .align_items(iced::Alignment::Start)
    .height(25)
    .width(500);
    let message = if path_buf.is_file() {
        Message::OpenFile(path_buf.display().to_string())
    } else {
        Message::Next(path_buf.display().to_string())
    };
    mouse_area(container).on_press(message).into()
}

fn get_icon(path_buf: &PathBuf) -> Svg<Theme> {
    if path_buf.is_dir() {
        let dir_icon_handle =
            svg::Handle::from_memory(include_bytes!("../../resources/folder-open.svg"));
        svg(dir_icon_handle).style(theme::Svg::Custom(Box::new(SvgStyles::Themed)))
    } else {
        let file_icon_handle =
            svg::Handle::from_memory(include_bytes!("../../resources/paper.svg"));
        svg(file_icon_handle).style(theme::Svg::Custom(Box::new(SvgStyles::Themed)))
    }
}
