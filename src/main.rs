use gtk::{glib, prelude::*};
use log::warn;
use std::cell::Cell;
use std::path::Path;
use std::rc::Rc;

mod config;
mod ui;

use config::get_config_from_env;

const FALLBACK_CSS: &str = "
box {
    padding: 4px;
}
button {
    min-width: 120px;
    min-height: 120px;
    margin: 4px;
    font-size: 1.2rem;
}
";

fn load_css_provider(css_path: &str) -> gtk::CssProvider {
    let provider = gtk::CssProvider::new();
    let had_error = Rc::new(Cell::new(false));

    {
        let had_error_cloned = had_error.clone();
        provider.connect_parsing_error(move |_, section, error| {
            let file_str = section
                .file()
                .and_then(|f| f.path())
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|| "<unknown>".to_string());
            warn!("CSS parsing error in {file_str}: {error}");
            had_error_cloned.set(true);
        });
    }

    if Path::new(css_path).exists() {
        provider.load_from_path(css_path);
    } else {
        warn!("CSS file {css_path} does not exist, falling back to embedded CSS.");
        provider.load_from_string(FALLBACK_CSS);
    }

    if had_error.get() {
        warn!("Failed to load CSS from path: {css_path}, falling back to embedded CSS.");
        had_error.set(false);
        provider.load_from_string(FALLBACK_CSS);
        if had_error.get() {
            warn!("Failed to load fallback embedded CSS.");
        }
    }

    provider
}

fn main() -> glib::ExitCode {
    env_logger::init();

    let config = match get_config_from_env() {
        Ok(cfg) => Rc::new(cfg),
        Err(e) => {
            eprintln!("Configuration error: {e}");
            return glib::ExitCode::FAILURE;
        }
    };

    let application = gtk::Application::builder()
        .application_id("com.github.t4k1t.byebyemenu")
        .build();

    {
        let config = config.clone();
        application.connect_startup(move |_| {
            let provider = load_css_provider(&config.css_path);
            gtk::style_context_add_provider_for_display(
                &gdk::Display::default().expect("Could not connect to default display."),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        });
    }

    {
        let config = config.clone();
        application.connect_activate(move |app| {
            crate::ui::build_ui(app, &config);
        });
    }

    application.run()
}
