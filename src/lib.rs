mod ui;

use relm4::{
    factory::FactoryVecDeque, Component, ComponentController, ComponentParts, Controller,
    SimpleComponent,
};

use gtk::prelude::*;
use relm4::prelude::*;

use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};
use ui::{
    item::{Item, ItemInit, ItemOutput},
    top_panel::{TopPanel, TopPanelInit, TopPanelInput, TopPanelOutput},
};

trait Hidden {
    fn is_hidden(&self) -> bool;
}

impl Hidden for PathBuf {
    #[cfg(unix)]
    fn is_hidden(&self) -> bool {
        self.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .as_bytes()
            .starts_with(&[b'.'])
    }

    #[cfg(windows)]
    fn is_hidden(&self) -> bool {
        use std::os::windows::prelude::*;

        match self.metadata() {
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
}

#[derive(Debug, Clone)]
pub enum Error {
    IoError(ErrorKind),
}

#[derive(Debug, Clone)]
pub enum Message {
    Next(String),
    Back,
    Home,
    OpenFile(String),
    ShowHidden(bool),
    SelectItem(DynamicIndex),
}

#[tracker::track]
struct CurrentDir {
    value: String,
}

impl From<&str> for CurrentDir {
    fn from(value: &str) -> Self {
        Self {
            value: String::from(value),
            tracker: 0,
        }
    }
}

pub struct App {
    home_dir: String,
    show_hidden_items: bool,
    parent_dir: Option<String>,
    current_dir: CurrentDir,
    top_panel: Controller<TopPanel>,
    items: FactoryVecDeque<Item>,
}

fn load_items(current_dir: &String) -> Result<(Option<String>, Vec<PathBuf>), Error> {
    match std::fs::read_dir(&current_dir) {
        Ok(entries) => {
            let mut items = Vec::new();
            for entry in entries.into_iter() {
                if let Ok(item) = entry {
                    items.push(item.path());
                }
            }
            Ok((parent(&current_dir), items))
        }
        Err(error) => Err(Error::IoError(error.kind())),
    }
}

fn parent<'a>(path: &'a str) -> Option<String> {
    Path::new(path).parent().map(|v| v.display().to_string())
}

impl App {
    fn update_data(
        &mut self,
        current_dir: String,
        parent_dir: &Option<String>,
        items: &Vec<PathBuf>,
    ) {
        self.parent_dir = parent_dir.clone();
        self.current_dir.set_value(current_dir);
        self.items.guard().clear();
        for (index, path_buf) in items
            .into_iter()
            .filter(|p| !p.is_hidden() || (self.show_hidden_items && p.is_hidden()))
            .enumerate()
        {
            self.items.guard().insert(index, ItemInit::from(path_buf));
        }
    }
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = Message;
    type Output = ();

    view! {
        gtk::Window {
            #[track = "model.current_dir.changed(CurrentDir::value())"]
            set_title: Some(model.current_dir.get_value()),
            set_width_request: 680,
            set_height_request: 680,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,

                model.top_panel.widget(),

                gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_vexpand: true,
                    model.items.widget(),
                }
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let home_dir = env!("HOME");
        let parent_dir = parent(&home_dir.to_owned());
        let mut model = Self {
            home_dir: home_dir.to_owned(),
            show_hidden_items: false,
            items: FactoryVecDeque::builder()
                .launch(gtk::Box::new(gtk::Orientation::Vertical, 0))
                .forward(sender.input_sender(), convert_item_response),
            top_panel: TopPanel::builder()
                .launch(TopPanelInit::new(parent_dir.is_some()))
                .forward(sender.input_sender(), convert_top_panel_response),
            parent_dir,
            current_dir: CurrentDir::from(home_dir),
        };

        if let Ok((parent_dir, mut items)) = load_items(&model.home_dir) {
            model.update_data(model.home_dir.clone(), &parent_dir, &mut items);
        }

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _: ComponentSender<Self>) {
        self.current_dir.reset();
        match message {
            Message::Next(current_dir) => {
                if let Ok((parent_dir, mut items)) = load_items(&current_dir) {
                    self.update_data(current_dir, &parent_dir, &mut items);
                    self.top_panel
                        .emit(TopPanelInput::DirectoryLoaded(parent_dir.is_some()))
                }
            }
            Message::Back => {
                if let Some(parent) = self.parent_dir.clone() {
                    if let Ok((parent_dir, mut items)) = load_items(&parent) {
                        self.update_data(parent, &parent_dir, &mut items);
                        self.top_panel
                            .emit(TopPanelInput::DirectoryLoaded(parent_dir.is_some()))
                    }
                }
            }
            Message::Home => {
                if let Ok((parent_dir, mut items)) = load_items(&self.home_dir) {
                    self.update_data(self.home_dir.clone(), &parent_dir, &mut items);
                    self.top_panel
                        .emit(TopPanelInput::DirectoryLoaded(parent_dir.is_some()))
                }
            }
            Message::ShowHidden(show_hidden_items) => {
                self.show_hidden_items = show_hidden_items;
                if let Ok((parent_dir, mut items)) = load_items(&self.current_dir.get_value()) {
                    self.update_data(self.current_dir.value.clone(), &parent_dir, &mut items);
                }
            }
            Message::OpenFile(path) => match open::that(path) {
                Err(error) => {
                    println!("Error occured: [{:?}]", error.kind());
                }
                _ => (),
            },
            Message::SelectItem(index) => {
                self.items
                    .guard()
                    .iter_mut()
                    .filter(|item| item.is_selected())
                    .for_each(|item| item.select(false));

                if let Some(selected_item) = self.items.guard().get_mut(index.current_index()) {
                    selected_item.select(true);
                }
            }
        }
    }
}

fn convert_top_panel_response(output: TopPanelOutput) -> Message {
    match output {
        TopPanelOutput::ShowHiddenItems(value) => Message::ShowHidden(value),
        TopPanelOutput::Home => Message::Home,
        TopPanelOutput::Back => Message::Back,
    }
}

fn convert_item_response(output: ItemOutput) -> Message {
    match output {
        ItemOutput::OpenFile(path) => Message::OpenFile(path),
        ItemOutput::OpenDirectory(path) => Message::Next(path),
        ItemOutput::ItemSelected(index) => Message::SelectItem(index),
    }
}
