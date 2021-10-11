use gdk::Screen;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, CssProvider, StyleContext, Window};

use std::cell::RefCell;
use std::rc::Rc;
use std::env;

mod tournament;
use tournament::{Display, Tournament};

fn main() {
    let application = Application::new(
        Some("tk.olmmcc.tournament"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application initialization failed!");
    application.connect_activate(|application| {
        ApplicationWindow::new(application);
        let css = CssProvider::new();
        css.load_from_data(&include_str!("../tournament.css").as_bytes())
            .unwrap_or_default();
        StyleContext::add_provider_for_screen(&Screen::get_default().unwrap(), &css, 1);
        let builder = Builder::new_from_string(include_str!("../window.ui"));
        let window: Window = builder.get_object("mainWindow").unwrap();
        let tournament = Tournament::new(env::args().collect());
        let mut display = Display::new(builder.clone(), tournament);
        display.display_ranks();
        display.display_race();
        display.display_stage();
        let win_button_1 = display.win_button_1.clone();
        let win_button_2 = display.win_button_2.clone();
        let refresh = display.refresh.clone();
        let display_1 = Rc::new(RefCell::new(display));
        let display_2 = display_1.clone();
        let display_3 = display_2.clone();
        win_button_1.connect_clicked(move |_| {
            display_1.borrow_mut().record_race(true);
        });
        win_button_2.connect_clicked(move |_| {
            display_2.borrow_mut().record_race(false);
        });
        refresh.connect_clicked(move |_| {
            let tournament = Tournament::new(env::args().collect());
            display_3.borrow_mut().reset(tournament);
        });
        window.show_all();
    });
    application.run(&[]);
}
