use std::path::PathBuf;

use iced::{
    widget::{mouse_area, text},
    Element,
};

use crate::Message;

pub fn view(path_buf: &PathBuf) -> Element<Message> {
    mouse_area(text(path_buf.file_name().unwrap().to_str().unwrap()))
        .on_press(Message::Next(path_buf.display().to_string()))
        .into()
}
