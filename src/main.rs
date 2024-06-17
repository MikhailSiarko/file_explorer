use file_explorer::App;
use gtk::{
    gdk,
    gio::{self, ActionEntry},
    prelude::*,
    Application,
};
use relm4::RelmApp;

fn initialize_custom_icons() {
    gio::resources_register_include!("icons.gresource").unwrap();

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/com/msiarko/file_explorer/icons");
}

#[cfg(unix)]
static CLOSE_SHORTCUT: &str = "<Meta>Q";
#[cfg(windows)]
static CLOSE_SHORTCUT: &str = "<Alt>F4";

fn setup_shortcuts() {
    let app = relm4::main_application();
    let action_close = ActionEntry::builder("close")
        .activate(|a: &Application, _, _| a.quit())
        .build();
    app.add_action_entries([action_close]);
    app.set_accels_for_action("app.close", &[CLOSE_SHORTCUT]);
}

fn main() {
    let relm_app = RelmApp::new("com.msiarko.file_explorer");
    relm_app.set_global_css(include_str!("../styles/index.css"));
    initialize_custom_icons();
    setup_shortcuts();
    relm_app.run::<App>(());
}
