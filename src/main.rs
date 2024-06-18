use file_explorer::App;
use gtk::{
    gdk,
    gio::{self},
};
use relm4::RelmApp;

fn initialize_custom_icons() {
    gio::resources_register_include!("icons.gresource").unwrap();

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/com/msiarko/file_explorer/icons");
}

fn main() {
    let relm_app = RelmApp::new("com.msiarko.file_explorer");
    relm_app.set_global_css(include_str!("../styles/index.css"));
    initialize_custom_icons();
    relm_app.run::<App>(());
}
