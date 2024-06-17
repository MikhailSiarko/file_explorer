use file_explorer::App;
use gtk::{gdk, gio};
use relm4::RelmApp;

fn initialize_custom_icons() {
    gio::resources_register_include!("icons.gresource").unwrap();

    let display = gdk::Display::default().unwrap();
    let theme = gtk::IconTheme::for_display(&display);
    theme.add_resource_path("/com/msiarko/file_explorer/icons");
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let application = gtk::Application::builder()
        .application_id("com.msiarko.file_explorer")
        .build();

    let relm_app = RelmApp::from_app(application);
    relm_app.set_global_css(include_str!("../styles/index.css"));
    initialize_custom_icons();
    relm_app.run::<App>(());
    Ok(())
}
