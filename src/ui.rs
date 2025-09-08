use crate::config::Config;
use gtk::{self, glib, prelude::*};
use log::warn;
use std::process::Command;
use std::process::exit;

fn execute_command(cmd: &[String]) {
    if let Err(e) = Command::new(&cmd[0]).args(&cmd[1..]).status() {
        warn!("Failed to execute command {cmd:?}: {e}");
    }
}

fn build_label(text: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(text)
        .justify(gtk::Justification::Center)
        .use_underline(true)
        .build()
}

pub fn build_ui(application: &gtk::Application, config: &Config) {
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    let button_row_default = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let button_row_custom = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    // Default buttons
    for i in 0..3 {
        let action = &config.actions[i];
        let button = gtk::Button::builder().valign(gtk::Align::Center).build();
        button.set_child(Some(&build_label(
            action.label.as_deref().unwrap_or("_unnamed"),
        )));
        button.add_css_class(&format!("button{}", i + 1));
        let cmd = action.command.clone().unwrap();
        button.connect_clicked(move |_| {
            execute_command(&cmd);
        });
        button_row_default.append(&button);
    }
    container.append(&button_row_default);

    // Optional custom buttons
    let mut custom_buttons = Vec::new();
    for i in 3..6 {
        let action = &config.actions[i];
        if let (Some(cmd), Some(label)) = (&action.command, &action.label) {
            let button = gtk::Button::builder().label(label).build();
            let cmd = cmd.clone();
            button.connect_clicked(move |_| {
                execute_command(&cmd);
            });
            button_row_custom.append(&button);
            custom_buttons.push(button);
        }
    }
    if !custom_buttons.is_empty() {
        container.append(&button_row_custom);
    }

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
        if key == gdk::Key::Escape {
            win.destroy();
            exit(0);
        }
        glib::Propagation::Proceed
    });
    window.add_controller(control_key);

    window.connect_close_request(move |window| {
        if let Some(application) = window.application() {
            application.remove_window(window);
        }
        glib::Propagation::Proceed
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_label_sets_text_and_properties() {
        let _ = gtk::init();
        let label = build_label("_Bye Bye");

        // Styling
        assert_eq!(label.justify(), gtk::Justification::Center);
        // Mnemonic
        assert_eq!(label.label(), "_Bye Bye");
        assert_eq!(label.text(), "Bye Bye");
        assert_eq!(label.mnemonic_keyval(), gdk::Key::B);
    }

    #[test]
    fn test_execute_command_handles_failure() {
        let cmd = vec!["nonexistent_command_xyz".to_string()];
        // Don't panic on invalid command
        execute_command(&cmd);
    }
}
