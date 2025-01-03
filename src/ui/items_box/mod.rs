mod init;
mod input;
mod item;
mod output;

use std::path::PathBuf;

use gtk::prelude::*;
use init::ItemsBoxInit;
pub(crate) use input::ItemsBoxInput;
pub(crate) use output::ItemsBoxOutput;
use relm4::{factory::FactoryVecDeque, prelude::*};

use item::{Item, ItemOutput};

use crate::core::load_items;

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

#[relm4::component(pub, async)]
impl AsyncComponent for ItemsBox {
    type Init = ItemsBoxInit;
    type Input = ItemsBoxInput;
    type Output = ItemsBoxOutput;
    type CommandOutput = ();

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let mut item_box = Self {
            current_dir: init.current_dir().to_owned(),
            show_hidden_items: init.show_hidden_items(),
            items: FactoryVecDeque::builder()
                .launch(gtk::Box::new(gtk::Orientation::Vertical, 0))
                .forward(sender.input_sender(), convert_item_response),
            tracker: 0,
        };

        match load_items(init.current_dir()).await {
            Err(error) => {
                let _ = sender.output(Self::Output::Error(error.to_string()));
            }
            Ok(items) => {
                item_box.update_items(&items);
                let _ = sender.output(Self::Output::DirectoryLoaded(init.current_dir().to_owned()));
            }
        }

        let widgets = view_output!();

        AsyncComponentParts {
            model: item_box,
            widgets,
        }
    }

    view! {
        gtk::ScrolledWindow {
            set_hscrollbar_policy: gtk::PolicyType::Never,
            set_vexpand: true,
            item_box.items.widget(),
        }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _: &Self::Root,
    ) {
        self.reset();
        match message {
            Self::Input::LoadDirectory(current_dir) => match load_items(&current_dir).await {
                Err(error) => {
                    let _ = sender.output(Self::Output::Error(error.to_string()));
                }
                Ok(items) => {
                    self.set_current_dir(current_dir.to_owned());
                    self.update_items(&items);
                    let _ = sender.output(Self::Output::DirectoryLoaded(current_dir.to_owned()));
                }
            },
            Self::Input::OpenFile(path) => {
                if let Err(error) = open::that(&path) {
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
