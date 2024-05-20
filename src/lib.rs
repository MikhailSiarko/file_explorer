mod ui;

use ui::{item, top_panel};

use iced::{
    executor,
    widget::{column, container, scrollable, Column},
    Alignment, Application, Command, Element, Length, Padding, Theme,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub enum Message {
    Next(String),
    Back,
    Home,
}

pub struct App {
    home_dir: &'static str,
    items: Vec<PathBuf>,
    parent_dir: Option<String>,
    current_dir: String,
}

impl App {
    fn fetch_items<'a>(&mut self, current_dir: &'a str) {
        self.items.clear();
        match fs::read_dir(current_dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(info) => self.items.push(info.path()),
                        Err(_) => println!("Error during reading the directory"),
                    }
                }
            }
            Err(_) => println!("Error during creation of the application"),
        }
    }

    fn items(&self) -> &Vec<PathBuf> {
        &self.items
    }

    fn parent(path: &String) -> Option<String> {
        Path::new(path).parent().map(|v| v.display().to_string())
    }

    fn load_dir(&mut self, path: String) {
        self.fetch_items(&path);
        self.parent_dir = App::parent(&path);
        self.current_dir = path;
    }
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let home_dir = env!("HOME");
        let mut app = App {
            home_dir,
            items: Vec::new(),
            parent_dir: App::parent(&home_dir.to_string()),
            current_dir: String::from(home_dir),
        };
        app.fetch_items(&app.home_dir);
        (app, Command::none())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Next(current_dir) => self.load_dir(current_dir),
            Message::Back => {
                if let Some(parent) = self.parent_dir.clone() {
                    self.load_dir(parent);
                }
            }
            Message::Home => self.load_dir(env!("HOME").to_string()),
        }

        Command::none()
    }

    fn title(&self) -> String {
        self.current_dir.clone()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let items: Vec<Element<_>> = self.items().into_iter().map(|i| item::view(i)).collect();

        let explorer = container(
            scrollable(Column::from_vec(items).padding(Padding::new(20.0))).width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(5));

        column!(top_panel::view(self.parent_dir.is_some()), explorer)
            .align_items(Alignment::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
