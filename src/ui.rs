use crate::config::get_config_from_env;
use gtk::{self, glib, prelude::*};
use std::process::Command;
use std::process::exit;

fn build_label(text: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(text)
        .justify(gtk::Justification::Center)
        .use_underline(true)
        .build()
}

// TODO: Allow customization of all buttons
// TODO: Configuration: custom button labels; custom keybindings; hide/show keybings
pub fn build_ui(application: &gtk::Application) {
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
    let button_exit = gtk::Button::builder().valign(gtk::Align::Center).build();
    button_exit.set_child(Some(&build_label(
        config.label_1.as_deref().unwrap_or("_exit"),
    )));
    button_exit.add_css_class("button1");

    let button_shutdown = gtk::Button::builder().build();
    button_shutdown.set_child(Some(&build_label(
        config.label_2.as_deref().unwrap_or("_shutdown"),
    )));
    button_shutdown.add_css_class("button2");

    let button_reboot = gtk::Button::builder().build();
    button_reboot.set_child(Some(&build_label(
        config.label_3.as_deref().unwrap_or("_reboot"),
    )));
    button_reboot.add_css_class("button3");

    button_row_1.append(&button_exit);
    button_row_1.append(&button_shutdown);
    button_row_1.append(&button_reboot);
    container.append(&button_row_1);

    // Custom buttons
    let mut custom_buttons: Vec<gtk::Button> = vec![];
    if let (Some(cmd), Some(label)) = (&config.cmd_4, &config.label_4) {
        let btn = gtk::Button::builder().label(label).build();
        let cmd = cmd.clone();
        btn.connect_clicked(move |_| {
            let _ = Command::new(&cmd[0]).args(&cmd[1..]).status();
        });
        custom_buttons.push(btn);
    }
    if let (Some(cmd), Some(label)) = (&config.cmd_5, &config.label_5) {
        let btn = gtk::Button::builder().label(label).build();
        let cmd = cmd.clone();
        btn.connect_clicked(move |_| {
            let _ = Command::new(&cmd[0]).args(&cmd[1..]).status();
        });
        custom_buttons.push(btn);
    }
    if let (Some(cmd), Some(label)) = (&config.cmd_6, &config.label_6) {
        let btn = gtk::Button::builder().label(label).build();
        let cmd = cmd.clone();
        btn.connect_clicked(move |_| {
            let _ = Command::new(&cmd[0]).args(&cmd[1..]).status();
        });
        custom_buttons.push(btn);
    }

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
            _ => (),
        }
        glib::Propagation::Stop
    });
    window.add_controller(control_key);

    button_exit.connect_clicked(move |_| {
        if let Some(cmd) = &config.cmd_1 {
            let _ = Command::new(&cmd[0]).args(&cmd[1..]).status();
        }
    });
    button_shutdown.connect_clicked(move |_| {
        if let Some(cmd) = &config.cmd_2 {
            let _ = Command::new(&cmd[0]).args(&cmd[1..]).status();
        }
    });
    button_reboot.connect_clicked(move |_| {
        if let Some(cmd) = &config.cmd_3 {
            let _ = Command::new(&cmd[0]).args(&cmd[1..]).status();
        }
    });

    window.connect_close_request(move |window| {
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        glib::Propagation::Proceed
    });
}
