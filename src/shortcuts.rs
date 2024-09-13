use gtk::{gio::ActionEntry, prelude::*, Application};
use relm4::ComponentSender;

use crate::{App, AppInput};

#[cfg(unix)]
static CLOSE_APP: &str = "<Meta>Q";
#[cfg(windows)]
static CLOSE_APP: &str = "<Alt>F4";

#[cfg(unix)]
static TOGGLE_HIDDEN_ITEMS: &str = "<Meta>I";
#[cfg(windows)]
static TOGGLE_HIDDEN_ITEMS: &str = "<Alt>I";

static HOME: &str = "Home";
static BACK: &str = "BackSpace";

pub fn setup_shortcuts(app: &Application, sender: &ComponentSender<App>) {
    let close = ActionEntry::builder("close")
        .activate(|a: &Application, _, _| a.quit())
        .build();

    let show_hidden = ActionEntry::builder("show_hidden")
        .activate({
            let sender_clone = sender.clone();
            move |_: &Application, _, _| {
                sender_clone.input(AppInput::ToggleShowHiddenItems);
            }
        })
        .build();

    let home = ActionEntry::builder("home")
        .activate({
            let sender_clone = sender.clone();
            move |_: &Application, _, _| {
                sender_clone.input(AppInput::Home);
            }
        })
        .build();

    let back = ActionEntry::builder("back")
        .activate({
            let sender_clone = sender.clone();
            move |_: &Application, _, _| {
                sender_clone.input(AppInput::Back);
            }
        })
        .build();

    app.add_action_entries([close, show_hidden, home, back]);
    app.set_accels_for_action("app.close", &[CLOSE_APP]);
    app.set_accels_for_action("app.show_hidden", &[TOGGLE_HIDDEN_ITEMS]);
    app.set_accels_for_action("app.home", &[HOME]);
    app.set_accels_for_action("app.back", &[BACK]);
}
