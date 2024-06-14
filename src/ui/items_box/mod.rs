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

use crate::Error;

use item::{Item, ItemOutput};

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
    async fn load(&mut self, current_dir: &str, sender: &AsyncComponentSender<Self>) {
        match self.load_items(current_dir).await {
            Err(error) => {
                println!("Error occured: [{:?}]", error);
            }
            Ok(items) => {
                self.set_current_dir(current_dir.to_owned());
                self.update_items(&items);
                let _ = sender.output(ItemsBoxOutput::DirectoryLoaded(current_dir.to_owned()));
            }
        }
    }

    async fn load_items(&mut self, current_dir: &str) -> Result<Vec<PathBuf>, Error> {
        match tokio::fs::read_dir(&current_dir).await {
            Ok(mut entries) => {
                let mut items = Vec::new();
                while let Ok(option) = entries.next_entry().await {
                    match option {
                        Some(item) => items.push(item.path()),
                        None => break,
                    }
                }
                Ok(items)
            }
            Err(error) => Err(Error::IoError(error.kind())),
        }
    }

    fn update_items(&mut self, items: &Vec<PathBuf>) {
        self.items.guard().clear();
        for (index, path_buf) in items
            .into_iter()
            .filter(|p| !p.is_hidden() || (self.show_hidden_items && p.is_hidden()))
            .enumerate()
        {
            self.items.guard().insert(index, Item::init(path_buf));
        }
    }

    pub fn init(current_dir: &str, show_hidden_items: bool) -> ItemsBoxInit {
        ItemsBoxInit::new(current_dir, show_hidden_items)
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

        item_box.load(init.current_dir(), &sender).await;

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
            Self::Input::LoadDirectory(current_dir) => self.load(&current_dir, &sender).await,
            Self::Input::OpenFile(path) => match open::that(path) {
                Err(error) => {
                    println!("Error occured: [{:?}]", error.kind());
                }
                _ => (),
            },
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
            Self::Input::ShowHiddenItems(show_hidden_items) => {
                self.set_show_hidden_items(show_hidden_items);
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
