mod ui;

use ui::{
    item::{self, Item},
    top_panel,
};

use iced::{
    executor,
    widget::{column, scrollable, Column},
    Alignment, Application, Command, Element, Length, Padding, Theme,
};
use std::{io::ErrorKind, path::Path};

#[derive(Debug, Clone)]
pub enum Error {
    IoError(ErrorKind),
}

#[derive(Debug, Clone)]
pub enum Message {
    Next(String),
    Back,
    Home,
    Load(Result<(String, Option<String>, Vec<Item>), Error>),
    OpenFile(String),
    ShowHidden(bool),
    SelectItem(String),
}

pub struct App<'a> {
    home_dir: &'a str,
    show_hidden_items: bool,
    items: Vec<Item>,
    parent_dir: Option<String>,
    current_dir: String,
    selected_item_index: Option<usize>,
}

async fn load_items(current_dir: String) -> Result<(String, Option<String>, Vec<Item>), Error> {
    match tokio::fs::read_dir(&current_dir).await {
        Ok(mut entries) => {
            let mut items = Vec::new();
            while let Ok(option) = entries.next_entry().await {
                match option {
                    Some(item) => items.push(Item::from(item.path())),
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

impl App<'_> {
    fn items(&self) -> &Vec<Item> {
        &self.items
    }

    fn update_data(
        &mut self,
        current_dir: String,
        parent_dir: Option<String>,
        items: &mut Vec<Item>,
    ) {
        self.parent_dir = parent_dir;
        self.current_dir = current_dir;
        self.items.clear();
        self.items.append(items);
    }
}

impl Application for App<'_> {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let home_dir = env!("HOME");
        let app = App {
            home_dir,
            show_hidden_items: false,
            items: Vec::new(),
            parent_dir: parent(&home_dir.to_string()),
            current_dir: String::from(home_dir),
            selected_item_index: None,
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
            Message::ShowHidden(show_hidden_items) => {
                self.show_hidden_items = show_hidden_items;
                Command::none()
            }
            Message::Load(Err(kind)) => {
                println!("Error occured: [{:?}]", kind);
                Command::none()
            }
            Message::OpenFile(path) => match open::that(path) {
                Ok(_) => Command::none(),
                Err(error) => {
                    println!("Error occured: [{:?}]", error.kind());
                    Command::none()
                }
            },
            Message::SelectItem(file_name) => {
                if let Some(selected_item_index) = self.selected_item_index {
                    if let Some(selected) = self.items.get_mut(selected_item_index) {
                        selected.unselect();
                    }
                }
                if let Some(index) = self.items.iter().position(|i| i.name == file_name) {
                    if let Some(item) = self.items.get_mut(index) {
                        item.select();
                        self.selected_item_index = Some(index);
                    }
                }
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
        let items: Vec<Element<_>> = self
            .items()
            .into_iter()
            .filter(|i| !i.is_hidden() || (self.show_hidden_items && i.is_hidden()))
            .map(|i| item::view(i))
            .collect();

        let explorer = scrollable(
            Column::from_vec(items)
                .padding(Padding::new(20.0))
                .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::FillPortion(5));

        column!(
            top_panel::view(self.parent_dir.is_some(), self.show_hidden_items),
            explorer
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}
