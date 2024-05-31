use file_explorer::App;
use iced::{
    font,
    window::{self},
    Application,
};

fn main() -> iced::Result {
    let app_settings: iced::Settings<()> = iced::Settings {
        default_font: iced::Font {
            family: font::Family::Name("FiraCode Nerd Font Mono"),
            weight: font::Weight::Normal,
            stretch: font::Stretch::Normal,
            style: font::Style::Normal,
        },
        window: iced::window::Settings {
            position: window::Position::Centered,
            ..iced::window::Settings::default()
        },
        id: Some(String::from("com.msiarko.file_explorer")),
        ..iced::Settings::default()
    };
    App::run(app_settings)
}
