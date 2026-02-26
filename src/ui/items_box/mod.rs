mod init;
mod input;
mod item;
mod output;

use std::path::PathBuf;

use init::ItemsBoxInit;
pub(crate) use input::ItemsBoxInput;
pub(crate) use output::ItemsBoxOutput;
use relm4::gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use item::{Item, ItemOutput};

use crate::core::{load_items, open_item};

trait Hidden {
    fn is_hidden(&self) -> bool;
}

#[tracker::track]
pub struct ItemsBox {
    #[tracker::do_not_track]
    items: FactoryVecDeque<Item>,
    show_hidden_items: bool,
    current_dir: String,
}

impl ItemsBox {
    fn update_items(&mut self, items: &[PathBuf]) {
        self.items.guard().clear();
        for (index, path_buf) in items
            .iter()
            .filter(|p| !p.is_hidden() || (self.show_hidden_items && p.is_hidden()))
            .enumerate()
        {
            self.items.guard().insert(index, Item::init(path_buf));
        }
    }

    fn sort_by_type(a: &PathBuf, b: &PathBuf) -> std::cmp::Ordering {
        if a.is_dir() && b.is_file() {
            std::cmp::Ordering::Less
        } else if a.is_file() && b.is_dir() {
            std::cmp::Ordering::Greater
        } else {
            a.file_name().cmp(&b.file_name())
        }
    }

    pub fn init(current_dir: &str, show_hidden_items: bool) -> ItemsBoxInit {
        ItemsBoxInit::new(current_dir.to_owned(), show_hidden_items)
    }
}

impl Hidden for PathBuf {
    #[cfg(unix)]
    fn is_hidden(&self) -> bool {
        self.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .as_bytes()
            .starts_with(b".")
    }

    #[cfg(windows)]
    fn is_hidden(&self) -> bool {
        use std::os::windows::prelude::*;
        const FILE_ATTRIBUTE_HIDDEN: u32 = 0x00000002;

        match self.metadata() {
            Ok(metadata) => {
                let attributes = metadata.file_attributes();
                if attributes & FILE_ATTRIBUTE_HIDDEN != 0 {
                    return true;
                }

                false
            }
            Err(err) => {
                println!("Error occured: [{:?}]", err.kind());
                false
            }
        }
    }
}

#[relm4::component(pub)]
impl Component for ItemsBox {
    type Init = ItemsBoxInit;
    type Input = ItemsBoxInput;
    type Output = ItemsBoxOutput;
    type CommandOutput = ();

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut item_box = Self {
            current_dir: init.current_dir().to_owned(),
            show_hidden_items: init.show_hidden_items(),
            items: FactoryVecDeque::builder()
                .launch(gtk::Box::new(gtk::Orientation::Vertical, 0))
                .forward(sender.input_sender(), convert_item_response),
            tracker: 0,
        };

        match load_items(init.current_dir()) {
            Err(error) => {
                let _ = sender.output(Self::Output::Error(error.to_string()));
            }
            Ok(mut items) => {
                items.sort_by(Self::sort_by_type);
                item_box.update_items(&items);
                let _ = sender.output(Self::Output::DirectoryLoaded(init.current_dir().to_owned()));
            }
        }

        let widgets = view_output!();

        ComponentParts {
            model: item_box,
            widgets,
        }
    }

    view! {
        gtk::ScrolledWindow {
            set_hscrollbar_policy: gtk::PolicyType::Automatic,
            set_vexpand: true,
            item_box.items.widget(),
        }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _: &Self::Root) {
        self.reset();
        match message {
            Self::Input::LoadDirectory(current_dir) => match load_items(&current_dir) {
                Err(error) => {
                    let _ = sender.output(Self::Output::Error(error.to_string()));
                }
                Ok(mut items) => {
                    self.set_current_dir(current_dir.to_owned());
                    items.sort_by(Self::sort_by_type);
                    self.update_items(&items);
                    let _ = sender.output(Self::Output::DirectoryLoaded(current_dir.to_owned()));
                }
            },
            Self::Input::OpenFile(path) => {
                if let Err(error) = open_item(&path) {
                    let _ = sender.output(Self::Output::Error(error.to_string()));
                }
            }
            Self::Input::SelectItem(index) => {
                self.items
                    .guard()
                    .iter_mut()
                    .filter(|item| item.is_selected())
                    .for_each(|item| item.select(false));

                if let Some(selected_item) = self.items.guard().get_mut(index.current_index()) {
                    selected_item.select(true);
                }
            }
            Self::Input::ToggleShowHiddenItems => {
                self.set_show_hidden_items(!self.show_hidden_items);
                sender.input(Self::Input::LoadDirectory(self.current_dir.clone()));
            }
            Self::Input::ShowHiddenItems(show) => {
                self.set_show_hidden_items(show);
                if self.changed(ItemsBox::show_hidden_items()) {
                    sender.input(Self::Input::LoadDirectory(self.current_dir.clone()))
                }
            }
        }
    }
}

fn convert_item_response(output: ItemOutput) -> ItemsBoxInput {
    match output {
        ItemOutput::OpenFile(path) => ItemsBoxInput::OpenFile(path),
        ItemOutput::OpenDirectory(path) => ItemsBoxInput::LoadDirectory(path),
        ItemOutput::ItemSelected(index) => ItemsBoxInput::SelectItem(index),
    }
}
