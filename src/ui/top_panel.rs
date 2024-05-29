use iced::{
    widget::{button, container, row, svg},
    Element, Length,
};

use crate::Message;

pub fn view(has_parent_dir: bool) -> Element<'static, Message> {
    let home_icon_handle = svg::Handle::from_path(format!(
        "{}/resources/people.svg",
        env!("CARGO_MANIFEST_DIR")
    ));

    let home = button(svg(home_icon_handle))
        .width(50)
        .height(25)
        .on_press(Message::Home);
    let back = if has_parent_dir {
        let back_icon_handle = svg::Handle::from_path(format!(
            "{}/resources/left-large.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        Some(
            button(svg(back_icon_handle))
                .width(50)
                .height(25)
                .on_press(Message::Back),
        )
    } else {
        None
    };

    container(
        row!()
            .push_maybe(back)
            .push(home)
            .height(Length::FillPortion(1)),
    )
    .padding(10)
    .into()
}
