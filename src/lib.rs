mod ui;

use ui::{item, top_panel};

use iced::{
    executor,
    widget::{column, container, scrollable, Column},
    Alignment, Application, Command, Element, Length, Padding, Theme,
};
use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
pub enum Error {
    IoError(ErrorKind),
}

#[derive(Debug, Clone)]
pub enum Message {
    Next(String),
    Back,
    Home,
    Load(Result<(String, Option<String>, Vec<PathBuf>), Error>),
}

pub struct App {
    home_dir: &'static str,
    items: Vec<PathBuf>,
    parent_dir: Option<String>,
    current_dir: String,
}

async fn load_items(current_dir: String) -> Result<(String, Option<String>, Vec<PathBuf>), Error> {
    match tokio::fs::read_dir(&current_dir).await {
        Ok(mut entries) => {
            let mut items = Vec::new();
            while let Ok(option) = entries.next_entry().await {
                match option {
                    Some(item) => items.push(item.path()),
                    None => break,
                }
            }
            Ok((current_dir.clone(), parent(&current_dir), items))
        }
        Err(error) => Err(Error::IoError(error.kind())),
    }
}

fn parent<'a>(path: &'a str) -> Option<String> {
    Path::new(path).parent().map(|v| v.display().to_string())
}

impl App {
    fn items(&self) -> &Vec<PathBuf> {
        &self.items
    }

    fn update_data(
        &mut self,
        current_dir: String,
        parent_dir: Option<String>,
        items: &mut Vec<PathBuf>,
    ) {
        self.parent_dir = parent_dir;
        self.current_dir = current_dir;
        self.items.clear();
        self.items.append(items);
    }
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let home_dir = env!("HOME");
        let app = App {
            home_dir,
            items: Vec::new(),
            parent_dir: parent(&home_dir.to_string()),
            current_dir: String::from(home_dir),
        };
        (
            app,
            Command::perform(load_items(home_dir.to_string()), Message::Load),
        )
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Next(current_dir) => Command::perform(load_items(current_dir), Message::Load),
            Message::Back => match self.parent_dir.clone() {
                Some(parent) => Command::perform(load_items(parent), Message::Load),
                None => Command::none(),
            },
            Message::Home => Command::perform(load_items(self.home_dir.to_string()), Message::Load),
            Message::Load(Ok((current_dir, parent_dir, mut items))) => {
                self.update_data(current_dir, parent_dir, &mut items);
                Command::none()
            }
            Message::Load(Err(kind)) => {
                println!("Error occured: [{:?}]", kind);
                Command::none()
            }
        }
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
