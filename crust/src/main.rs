#!/usr/bin/env run-cargo-script

#![feature(file_create_new)] // This is a nightly feature!
use gtk::gdk::Display;
use gtk::prelude::*;
// use gtk::IconTheme;
use gtk::{
    Application, ApplicationWindow, Builder, CssProvider, StyleContext, TextView,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use std::env::args;
use std::process;
use std::rc::Rc;
use std::{fs, fs::File, io::Read};

static home: String = get_global();

// creates a variable which has no type! noice
fn get_global<T>() -> &'static T {
    fn init() -> T { std::env::var("HOME").unwrap_or_else(|_| {
    eprintln!("Couldn't find $HOME");
    process::exit(1);
}) }
    
    use std::mem::MaybeUninit;
    
    static mut GLOBAL: MaybeUninit<T> = MaybeUninit::uninit();
    static mut ONCE: Once = Once::new();
    
    unsafe {
        ONCE.call_once(|| GLOBAL = MaybeUninit(init()));
        
        &*GLOBAL.as_ptr()
    }
}

fn main() {
    let arguments: Vec<String> = args().collect();
    let application = Application::new(Some("com.github.kobruhh.crust"), Default::default());
    application.connect_startup(move |app| {
        // The CSS "magic" happens here.
        let provider = CssProvider::new();
        provider.load_from_data(std::fs::read_to_string("/home/kobruh/.config/crust/crust.css").expect("Couldn't find crust.css").as_bytes());
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
                File::create_new(".def").unwrap_or_else(|_| {
                    eprintln!("couldn't create a file named: {}", &arguments[1]);
                    process::exit(0);
                })
            })
            .read_to_string(&mut content)
            .unwrap_or(0);
    }

    let clone_arguments = Rc::new(arguments.clone());
    // let clone_args = Rc::clone(&clone_arguments);
    let ui_src = &std::fs::read_to_string("/home/kobruh/.config/crust/crust.ui").expect("Couldn't find crust.ui");
    let builder = Builder::new();
    builder.add_from_string(ui_src).unwrap_or_else(|_| {
        eprintln!("Couldn't add from string");
        process::exit(1);
    });

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    // window.set_opacity(0.8); // -> allows transparency but I dont like the look!

    let text_view: TextView = builder.object("text_view").expect("Couldn't get text_view");
    text_view.add_css_class("view");
    text_view.buffer().set_text(&content);

    // let icon_theme = IconTheme::new();
    // IconTheme::add_resource_path(&icon_theme, "/home/kobruh/github/crust/crust/src/");
    // dbg!(IconTheme::icon_names(&icon_theme));
    // IconTheme::set_theme_name(icons, Some("1f355.png"));
    window.set_icon_name(Some("crust"));

    window.connect_destroy(move |_| {
        let content = get_textview_text(&text_view);
        if &clone_arguments.len() >= &2_usize {
            fs::write(&clone_arguments[1], &content).unwrap_or_else(|_| {
                eprintln!("Couldn't save the file!");
                process::exit(1);
            });
        }
    });
    // let icon_theme = IconTheme::get_default().expect("Failed to get default icon theme");
    window.show();
    // process::exit(1); // -> used to find opening speed!
}

fn get_textview_text(textview: &gtk::TextView) -> String {
    let buffer = textview.buffer();
    let (start, end) = buffer.bounds();
    (buffer.text(&start, &end, true)).to_string()
}
