use iced::{
    theme,
    widget::{button, container, row, svg, tooltip},
    Element, Length,
};

use crate::Message;

use super::styles::SvgStyles;

pub fn view(has_parent_dir: bool) -> Element<'static, Message> {
    let home_icon_handle = svg::Handle::from_memory(include_bytes!("../../resources/people.svg"));

    let home = tooltip(
        button(svg(home_icon_handle).style(theme::Svg::Custom(Box::new(SvgStyles::Themed))))
            .width(50)
            .height(25)
            .on_press(Message::Home),
        "Home",
        tooltip::Position::FollowCursor,
    );
    let back = if has_parent_dir {
        let back_icon_handle =
            svg::Handle::from_memory(include_bytes!("../../resources/left-large.svg"));
        Some(tooltip(
            button(svg(back_icon_handle).style(theme::Svg::Custom(Box::new(SvgStyles::Themed))))
                .width(50)
                .height(25)
                .on_press(Message::Back),
            "Back",
            tooltip::Position::FollowCursor,
        ))
    } else {
        None
    };

    container(
        row!()
            .push_maybe(back)
            .push(home)
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)
            .spacing(10)
            .height(Length::Shrink),
    )
    .padding(5)
    .into()
}
