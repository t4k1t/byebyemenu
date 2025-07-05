use gtk::{glib, prelude::*};
use log::{debug, info};
use shell_words::split as shell_split;
use std::env;
use std::process::{Command, exit};

pub struct Config {
    pub css_path: String,
    pub cmd_4: Option<Vec<String>>,
    pub cmd_5: Option<Vec<String>>,
    pub cmd_6: Option<Vec<String>>,
    pub label_4: Option<String>,
    pub label_5: Option<String>,
    pub label_6: Option<String>,
}

// TODO: Split into multiple files
fn get_config_from_env() -> Config {
    let css_path = match env::var("BYEBYE_CSS_PATH") {
        Ok(path) => {
            debug!("CSS path from env: {}", path);
            path
        }
        Err(_) => "/home/tsk/smithy/byebyemenu/style.css".to_string(),
    };
    let cmd_4 = match env::var("BYEBYE_CMD_4") {
        Ok(cmd) => shell_split(&cmd).ok(),
        Err(_) => None,
    };
    let cmd_5 = match env::var("BYEBYE_CMD_5") {
        Ok(cmd) => shell_split(&cmd).ok(),
        Err(_) => None,
    };
    let cmd_6 = match env::var("BYEBYE_CMD_6") {
        Ok(cmd) => shell_split(&cmd).ok(),
        Err(_) => None,
    };

    Config {
        css_path,
        cmd_4,
        cmd_5,
        cmd_6,
        label_4: Some(String::from("Custom #1\n(1)")),
        label_5: Some(String::from("Custom #2\n(2)")),
        label_6: Some(String::from("Custom #3\n(3)")),
    }
}

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
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    application.connect_activate(build_ui);
    application.run()
}

fn do_exit() {
    info!("selected exit");
    let user = match env::var("USER") {
        Ok(env_user) => env_user,
        Err(_) => "".to_string(),
    };
    let status = Command::new("/usr/bin/loginctl")
        .args(["terminate-user", &user])
        .status();
    debug!("status: {:?}", status);
}

fn do_shutdown() {
    info!("selected shutdown");
    let status = Command::new("/usr/bin/systemctl")
        .args(["poweroff"])
        .status();
    debug!("status: {:?}", status);
}

fn do_reboot() {
    info!("selected reboot");
    let status = Command::new("/usr/bin/systemctl").args(["reboot"]).status();
    debug!("status: {:?}", status);
}

fn build_label(text: String) -> gtk::Label {
    gtk::Label::builder()
        .label(text)
        .justify(gtk::Justification::Center)
        .build()
}

// TODO: Allow customization of all buttons
// TODO: Configuration: custom button labels; custom keybindings; hide/show keybings
fn build_ui(application: &gtk::Application) {
    let config = get_config_from_env();
    // Layout
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    let button_row_1 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let button_row_2 = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    // Default buttons
    let label_exit = build_label("(e)xit".to_string());
    let label_shutdown = build_label("(s)hutdown".to_string());

    let button_exit = gtk::Button::builder().valign(gtk::Align::Center).build();
    button_exit.set_child(Some(&label_exit));
    button_exit.add_css_class("button1");

    let button_shutdown = gtk::Button::builder().build();
    button_shutdown.set_child(Some(&label_shutdown));
    button_shutdown.add_css_class("button2");

    let button_reboot = gtk::Button::builder().build();
    button_reboot.set_child(Some(&build_label("reboot\n(r)".to_string())));
    button_reboot.add_css_class("button3");

    button_row_1.append(&button_exit);
    button_row_1.append(&button_shutdown);
    button_row_1.append(&button_reboot);
    container.append(&button_row_1);

    // Custom buttons
    let mut custom_buttons: Vec<gtk::Button> = vec![];
    if config.cmd_4.is_some() {
        custom_buttons.push(
            gtk::Button::builder()
                .label(config.label_4.unwrap())
                .build(),
        );
    };

    if !custom_buttons.is_empty() {
        container.append(&button_row_2);

        // TODO: Fix commands in case there are no args
        let _ = &custom_buttons[0].connect_clicked(move |_| {
            if let Some(cmd) = &config.cmd_4 {
                let _ = Command::new(&cmd[0]).args(&cmd[1..]).status();
            };
        });
        button_row_2.append(&custom_buttons[0]);
    };
    if custom_buttons.len() > 1 {
        let _ = &custom_buttons[1].connect_clicked(move |_| {
            if let Some(cmd) = &config.cmd_5 {
                let _ = Command::new(&cmd[0]).args(&cmd[1..]).status();
            };
        });
        button_row_2.append(&custom_buttons[1]);
    };
    if custom_buttons.len() > 2 {
        let _ = &custom_buttons[2].connect_clicked(move |_| {
            if let Some(cmd) = &config.cmd_6 {
                let _ = Command::new(&cmd[0]).args(&cmd[1..]).status();
            };
        });
        button_row_2.append(&custom_buttons[2]);
    };

    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("ByeByeMenu")
        .default_width(200)
        .default_height(70)
        .child(&container)
        .visible(true)
        .build();

    let win = window.clone();
    let control_key = gtk::EventControllerKey::new();
    control_key.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::Escape => {
                win.destroy();
                exit(0);
            }
            gdk::Key::e => {
                do_exit();
            }
            gdk::Key::s => {
                do_shutdown();
            }
            gdk::Key::r => {
                do_reboot();
            }
            _ => (),
        }
        glib::Propagation::Stop
    });
    window.add_controller(control_key);

    button_exit.connect_clicked(move |_| {
        do_exit();
    });
    button_shutdown.connect_clicked(move |_| {
        do_shutdown();
    });
    button_reboot.connect_clicked(move |_| {
        do_reboot();
    });

    window.connect_close_request(move |window| {
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        glib::Propagation::Proceed
    });
}
