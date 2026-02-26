use relm4::gtk::{Application, gio, prelude::*};

pub fn setup_menubar(app: &Application) {
    let menubar = {
        let file_menu = {
            let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit"));

            let file_menu = gio::Menu::new();
            file_menu.append_item(&quit_menu_item);
            file_menu
        };

        let view_menu = {
            let show_hidden_items =
                gio::MenuItem::new(Some("Show Hidden Items"), Some("app.show_hidden"));

            let view_menu = gio::Menu::new();
            view_menu.append_item(&show_hidden_items);
            view_menu
        };

        let edit_menu = {
            let back = gio::MenuItem::new(Some("Back"), Some("app.back"));
            let home = gio::MenuItem::new(Some("Home"), Some("app.home"));

            let edit_menu = gio::Menu::new();
            edit_menu.append_item(&back);
            edit_menu.append_item(&home);
            edit_menu
        };

        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("File"), &file_menu);
        menubar.append_submenu(Some("Edit"), &edit_menu);
        menubar.append_submenu(Some("View"), &view_menu);

        menubar
    };

    app.set_menubar(Some(&menubar));
}
