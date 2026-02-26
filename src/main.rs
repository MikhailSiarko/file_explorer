use anyhow::Result;
use file_explorer::App;
use relm4::RelmApp;
use relm4::gtk::{self, gdk, gio};

fn initialize_custom_icons() -> Result<()> {
    gio::resources_register_include!("icons.gresource")?;
    if let Some(display) = gdk::Display::default() {
        let theme = gtk::IconTheme::for_display(&display);
        theme.add_resource_path("/com/msiarko/file_explorer/icons");
    }
    Ok(())
}

fn main() -> Result<()> {
    let relm_app = RelmApp::new("com.msiarko.file_explorer");
    relm4::set_global_css(include_str!("../styles/index.css"));
    initialize_custom_icons()?;
    relm_app.run::<App>(());
    Ok(())
}
