use gtk::prelude::*;
use std::path::PathBuf;

use relm4::prelude::*;

#[tracker::track]
pub struct Item {
    index: DynamicIndex,
    name: String,
    path: String,
    selected: bool,
    is_file: bool,
    icon_name: &'static str,
}

#[derive(Debug)]
pub enum ItemOutput {
    OpenFile(String),
    OpenDirectory(String),
    ItemSelected(DynamicIndex),
}

#[derive(Debug)]
pub enum ItemInput {
    ItemClicked,
}

pub struct ItemInit {
    name: String,
    path: String,
    is_file: bool,
}

impl From<&PathBuf> for ItemInit {
    fn from(value: &PathBuf) -> Self {
        Self {
            name: String::from(value.file_name().unwrap().to_str().unwrap()),
            path: value.display().to_string(),
            is_file: value.is_file(),
        }
    }
}

impl Item {
    pub fn select(&mut self, selected: bool) {
        self.set_selected(selected);
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }
}

#[relm4::factory(pub)]
impl FactoryComponent for Item {
    type ParentWidget = gtk::Box;
    type CommandOutput = ();
    type Input = ItemInput;
    type Output = ItemOutput;
    type Init = ItemInit;

    fn init_model(init: Self::Init, index: &Self::Index, _: FactorySender<Self>) -> Self {
        let icon_name = if init.is_file { "file" } else { "folder" };
        Self {
            index: index.clone(),
            name: init.name,
            path: init.path,
            selected: false,
            is_file: init.is_file,
            icon_name,
            tracker: 0,
        }
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            Self::Input::ItemClicked => {
                if self.selected {
                    let output = if self.is_file {
                        Self::Output::OpenFile(self.path.clone())
                    } else {
                        Self::Output::OpenDirectory(self.path.clone())
                    };
                    let _ = sender.output(output);
                } else {
                    let _ = sender.output(Self::Output::ItemSelected(self.index.clone()));
                }
            }
        }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            #[track = "self.changed(Item::selected())"]
            set_class_active: ("selected", self.selected),
            set_css_classes: &["item-box"],
            add_controller = gtk::GestureClick {
                connect_released[sender] => move |gesture, _, _, _| {
                    gesture.set_state(gtk::EventSequenceState::Claimed);
                    let _ = sender.input(Self::Input::ItemClicked);
                },
            },
            gtk::Image {
                set_icon_name: Some(self.icon_name),
            },
            gtk::Label {
                set_label: &self.name,
            }
        }
    }
}
