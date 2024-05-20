use iced::{
    widget::{button, row},
    Element, Length,
};

use crate::Message;

pub fn view(has_parent_dir: bool) -> Element<'static, Message> {
    let home = button("Home").on_press(Message::Home);
    let back = if has_parent_dir {
        Some(button("Back").on_press(Message::Back))
    } else {
        None
    };
    row!(home)
        .push_maybe(back)
        .height(Length::FillPortion(1))
        .into()
}
