use gio::prelude::*;
use gtk::prelude::*;
use gdk::Screen;
use gtk::{Application, ApplicationWindow, Builder, CssProvider, StyleContext, Window};

mod tournament;
use tournament::{Display, Tournament};

fn main() {
    let application = Application::new(Some("tk.olmmcc.tournament"), gio::ApplicationFlags::FLAGS_NONE)
        .expect("Application initialization failed!");
    application.connect_activate(|application| {
        ApplicationWindow::new(application);
        let css = CssProvider::new();
        css.load_from_data(&include_str!("../tournament.css").as_bytes())
        .unwrap_or_default();
        StyleContext::add_provider_for_screen(&Screen::get_default().unwrap(), &css, 1);
        let builder = Builder::new_from_string(include_str!("../window.ui"));
        let window: Window = builder.get_object("mainWindow").unwrap();
        let tournament = Tournament::new();
        let mut display = Display::new(builder, tournament);
        display.connect_refresh();
        display.connect_buttons_clicked();
        display.display_ranks();
        window.show_all();
    });
    application.run(&[]);
}
