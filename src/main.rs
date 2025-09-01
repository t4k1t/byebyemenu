use gtk::{glib, prelude::*};
use log::{debug, info};

mod actions;
mod config;
mod ui;

use config::get_config_from_env;
use ui::build_ui;

fn main() -> glib::ExitCode {
    env_logger::init();

    let config = get_config_from_env();
    let application = gtk::Application::builder()
        .application_id("com.github.t4k1t.byebyemenu")
        .build();
    application.connect_startup(move |_| {
        let provider = gtk::CssProvider::new();
        provider.load_from_path(&config.css_path);
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to default display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    application.connect_activate(build_ui);
    application.run()
}
