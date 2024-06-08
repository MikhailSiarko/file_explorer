use std::fs;
use std::path::PathBuf;

use iced::{
    theme,
    widget::{container, mouse_area, row, svg, text, Svg},
    Element, Length, Theme,
};

use crate::Message;

use super::styles::SvgStyles;

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    path: String,
    selected: bool,
    is_file: bool,
    metadata: Option<fs::Metadata>,
}

impl Item {
    #[cfg(unix)]
    pub fn is_hidden(&self) -> bool {
        self.name.as_bytes().starts_with(&[b'.'])
    }

    #[cfg(windows)]
    fn is_hidden(&self) -> bool {
        use std::os::windows::prelude::*;

        match self {
            Ok(metadata) => {
                let attributes = metadata.file_attributes();
                if (attributes & 0x2) {
                    true
                }
                false
            }
            Err(err) => println!("Error occured: [{:?}]", err.kind()),
        }
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn unselect(&mut self) {
        self.selected = false;
    }
}

impl From<PathBuf> for Item {
    fn from(value: PathBuf) -> Self {
        Item {
            name: String::from(value.file_name().unwrap().to_str().unwrap()),
            path: value.display().to_string(),
            selected: false,
            is_file: value.is_file(),
            metadata: value.metadata().map_or(None, |m| Some(m)),
        }
    }
}

pub fn view(item: &Item) -> Element<Message> {
    let item_row = row!(
        get_icon(&item).width(Length::FillPortion(1)),
        text(&item.name).width(Length::FillPortion(9)),
    )
    .align_items(iced::Alignment::Start)
    .height(25)
    .width(500);

    let item_container_style = if item.selected {
        theme::Container::Box
    } else {
        theme::Container::Transparent
    };

    let item_container = container(item_row)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
        .style(item_container_style);

    let message = if item.selected {
        if item.is_file {
            Message::OpenFile(item.path.clone())
        } else {
            Message::Next(item.path.clone())
        }
    } else {
        Message::SelectItem(item.name.clone())
    };
    mouse_area(item_container).on_press(message).into()
}

fn get_icon(item: &Item) -> Svg<Theme> {
    if item.is_file {
        let file_icon_handle =
            svg::Handle::from_memory(include_bytes!("../../resources/paper.svg"));
        svg(file_icon_handle).style(theme::Svg::Custom(Box::new(SvgStyles::Themed)))
    } else {
        let dir_icon_handle =
            svg::Handle::from_memory(include_bytes!("../../resources/folder-open.svg"));
        svg(dir_icon_handle).style(theme::Svg::Custom(Box::new(SvgStyles::Themed)))
    }
}
