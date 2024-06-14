mod ui;

use relm4::{Component, ComponentController, ComponentParts, Controller};

use gtk::prelude::*;
use relm4::prelude::*;

use std::{io::ErrorKind, path::Path};
use ui::{
    items_box::{ItemsBox, ItemsBoxInput, ItemsBoxOutput},
    top_panel::{TopPanel, TopPanelInput, TopPanelOutput},
};

#[derive(Debug)]
pub enum Error {
    IoError(ErrorKind),
}

#[derive(Debug)]
pub enum AppInput {
    UpdateCurrentDirectory(String),
    Back,
    Home,
    ShowHidden(bool),
}

#[tracker::track]
pub struct App {
    #[tracker::do_not_track]
    home_dir: String,
    #[tracker::do_not_track]
    parent_dir: Option<String>,
    current_dir: String,
    #[tracker::do_not_track]
    top_panel: Controller<TopPanel>,
    #[tracker::do_not_track]
    items_box: AsyncController<ItemsBox>,
}

fn parent<'a>(path: &'a str) -> Option<String> {
    Path::new(path).parent().map(|v| v.display().to_string())
}

#[relm4::component(pub)]
impl Component for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Window {
            #[track = "model.changed(App::current_dir())"]
            set_title: Some(model.get_current_dir()),
            set_width_request: 680,
            set_height_request: 680,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,

                model.top_panel.widget(),
                model.items_box.widget(),
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let home_dir = env!("HOME");
        let parent_dir = parent(&home_dir);
        let model = Self {
            home_dir: home_dir.to_owned(),
            items_box: ItemsBox::builder()
                .launch(ItemsBox::init(home_dir, false))
                .forward(sender.input_sender(), convert_items_box_response),
            top_panel: TopPanel::builder()
                .launch(TopPanel::init(parent_dir.is_some()))
                .forward(sender.input_sender(), convert_top_panel_response),
            parent_dir,
            current_dir: home_dir.to_owned(),
            tracker: 0,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _: ComponentSender<Self>, _: &Self::Root) {
        self.reset();
        match message {
            AppInput::UpdateCurrentDirectory(current_dir) => {
                self.set_current_dir(current_dir);
                self.parent_dir = parent(self.get_current_dir());
                self.top_panel
                    .emit(TopPanelInput::DirectoryLoaded(self.parent_dir.is_some()));
            }
            AppInput::Back => {
                if let Some(parent) = self.parent_dir.clone() {
                    self.items_box.emit(ItemsBoxInput::LoadDirectory(parent));
                }
            }
            AppInput::Home => {
                self.items_box
                    .emit(ItemsBoxInput::LoadDirectory(self.home_dir.clone()));
            }
            AppInput::ShowHidden(show_hidden_items) => {
                self.items_box
                    .emit(ItemsBoxInput::ShowHiddenItems(show_hidden_items));
            }
        }
    }
}

fn convert_top_panel_response(output: TopPanelOutput) -> AppInput {
    match output {
        TopPanelOutput::ShowHiddenItems(value) => AppInput::ShowHidden(value),
        TopPanelOutput::Home => AppInput::Home,
        TopPanelOutput::Back => AppInput::Back,
    }
}

fn convert_items_box_response(output: ItemsBoxOutput) -> AppInput {
    match output {
        ItemsBoxOutput::DirectoryLoaded(current_dir) => {
            AppInput::UpdateCurrentDirectory(current_dir)
        }
    }
}
