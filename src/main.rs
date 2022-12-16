use gtk::prelude::*;
use std::fmt::Arguments;
use std::io::prelude::*;
// use std::os::unix::process;
use std::{fs::File, io::Read, fs};
use std::env::args;
use std::process;
use gtk::{
    Application, ApplicationWindow, Builder, TextView, CssProvider, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk::gdk::Display;

fn main() {
    let arguments:Vec<String> = args().collect();
    let application = Application::new(
        Some("com.github.kobruhh.crust"),
        Default::default(),
    );
    let content:&str = "";
    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(include_bytes!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // We build the application UI.
        build_ui(app, &arguments);
    });

    application.run();
}

pub fn build_ui(application: &Application, arguments:&Vec<String>) -> (&'static str, ApplicationWindow) {
    let mut content = String::new();

    if arguments.len() >= 2 {
        println!("{}", arguments[0]);
        File::open(&arguments[1]).unwrap_or_else(|_| {
            eprintln!("couldn't open file!");
            process::exit(0);
        }).read_to_string(&mut content).unwrap_or_else(|_| {
                0
            }
            );
    }

    let ui_src = include_str!("crust.ui");
    let builder = Builder::new();
    builder
        .add_from_string(ui_src)
        .expect("Couldn't add from string");

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    // window.set_opacity(0.8); // -> allows transparency but I dont like the look!
    let text_view: TextView = builder.object("text_view").expect("Couldn't get text_view");
    text_view.add_css_class("view");
    text_view.buffer().set_text(&content);
    window.show();
    (&text_view.to_string(), window)

        window.connect_destroy(move |_| {
            fs::write(&arguments[1], &content).expect("Couldn't save the file!");
        });
}
