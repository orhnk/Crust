#![feature(file_create_new)] // This is a nightly feature!
use gtk::prelude::*;
use std::rc::Rc;
use gtk::gdk::Display;
use gtk::{
    Application, ApplicationWindow, Builder, CssProvider, StyleContext, TextView,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use std::env::args;
use std::process;
use std::{fs, fs::File, io::Read};

fn main() {
    let arguments: Vec<String> = args().collect();
    let application = Application::new(Some("com.github.kobruhh.crust"), Default::default());
    application.connect_startup(move |app| {
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

pub fn build_ui(application: &Application, arguments: &Vec<String>) {
    let mut content = String::new();

    if arguments.len() >= 2 {
        File::open(&arguments[1])
            .unwrap_or_else(|_| {
                eprintln!("File doesn't exist, creating the file");

                fs::write(&arguments[1], &content).unwrap_or_else(|_| {
                    eprintln!("Couldn't create a file! (Probabilty: Permission Denied)");
                    process::exit(1);
                });
                File::create_new(".def").expect("couldn't create a file named: {&arguments[1]}")
            })
            .read_to_string(&mut content)
            .unwrap_or(0);
    }

    let clone_arguments = Rc::new(arguments.clone());
    // let clone_args = Rc::clone(&clone_arguments);
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
    window.connect_destroy(move |_| {
        let content = get_textview_text(&text_view);
        println!("{:#?}, \n{}", &clone_arguments, &content);
        fs::write(&clone_arguments[1], &content).expect("Couldn't save the file!");
    });
}

fn get_textview_text(textview: &gtk::TextView) -> String {
    let buffer = textview.buffer();
    let (start, end) = buffer.bounds();
    (buffer.text(&start, &end, true)).to_string()
}
