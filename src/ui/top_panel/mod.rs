pub mod init;
mod input;
mod output;

use gtk::prelude::*;
use init::TopPanelInit;
pub(crate) use input::TopPanelInput;
pub(crate) use output::TopPanelOutput;
use relm4::prelude::*;

#[tracker::track]
pub struct TopPanel {
    has_parent_dir: bool,
    show_hidden_items: bool,
}

impl TopPanel {
    pub fn init(has_parent_dir: bool, show_hidden_items: bool) -> TopPanelInit {
        TopPanelInit::new(has_parent_dir, show_hidden_items)
    }
}

#[relm4::component(pub)]
impl SimpleComponent for TopPanel {
    type Init = TopPanelInit;
    type Input = TopPanelInput;
    type Output = TopPanelOutput;

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            has_parent_dir: init.has_parent_dir(),
            show_hidden_items: init.show_hidden_items(),
            tracker: 0,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 5,

            gtk::Button {
                set_icon_name: "left",
                connect_clicked[sender] => move |_| {
                    let _ = sender.output(Self::Output::Back);
                },
                #[track = "model.changed(TopPanel::has_parent_dir())"]
                set_sensitive: model.has_parent_dir,
            },

            gtk::Button {
                set_icon_name: "home",
                connect_clicked[sender] => move |_| {
                    let _ = sender.output(Self::Output::Home);
                },
            },

            gtk::CheckButton {
                set_label: Some("Show hidden items"),
                #[track = "model.changed(TopPanel::show_hidden_items())"]
                set_active: model.show_hidden_items,
                connect_toggled[sender] => move |button| {
                    let _ = sender.output(Self::Output::HiddenItemsToggled(button.is_active()));
                },
            }
        }
    }

    fn update(&mut self, message: Self::Input, _: ComponentSender<Self>) {
        self.reset();
        match message {
            Self::Input::DirectoryLoaded(has_parent_dir) => self.set_has_parent_dir(has_parent_dir),
            Self::Input::ToggleShowHiddenItems => {
                self.set_show_hidden_items(!self.show_hidden_items)
            }
        }
    }
}
