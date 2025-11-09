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

// Get labels for default buttons
fn get_default_button_labels(config: &Config) -> Vec<String> {
    config
        .actions
        .iter()
        .take(3)
        .map(|action| {
            action
                .label
                .clone()
                .unwrap_or_else(|| "unnamed".to_string())
        })
        .collect()
}

// Get labels and actions for custom buttons
fn get_custom_buttons(config: &Config) -> Vec<(String, Vec<String>)> {
    config
        .actions
        .iter()
        .skip(3)
        .take(3)
        .filter_map(|action| {
            if let (Some(label), Some(command)) = (&action.label, &action.command) {
                Some((label.clone(), command.clone()))
            } else {
                None
            }
        })
        .collect()
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
    let default_labels = get_default_button_labels(config);
    for (i, (action, label_text)) in config
        .actions
        .iter()
        .take(3)
        .zip(default_labels.iter())
        .enumerate()
    {
        let button = gtk::Button::builder()
            .valign(gtk::Align::Center)
            .label(label_text)
            .use_underline(true)
            .build();
        button.add_css_class(&format!("button{}", i + 1));
        if let Some(cmd) = &action.command {
            let cmd = cmd.clone();
            button.connect_clicked(move |_| {
                execute_command(&cmd);
            });
        }
        button_row_default.append(&button);
    }
    container.append(&button_row_default);

    // Optional custom buttons
    let mut custom_buttons = Vec::new();
    for (label, cmd) in get_custom_buttons(config) {
        let button = gtk::Button::builder()
            .label(&label)
            .use_underline(true)
            .build();
        let cmd = cmd.clone();
        button.connect_clicked(move |_| {
            execute_command(&cmd);
        });
        button_row_custom.append(&button);
        custom_buttons.push(button);
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
    use crate::config::{Action, Config};

    // Config helper
    fn make_config(labels: Vec<Option<&str>>, commands: Vec<Option<&str>>) -> Config {
        let actions = labels
            .into_iter()
            .zip(commands.into_iter())
            .map(|(label, command)| Action {
                label: label.map(|s| s.to_string()),
                command: command.map(|s| vec![s.to_string()]),
            })
            .collect();
        Config {
            actions,
            css_path: "".to_string(),
        }
    }

    #[test]
    fn test_get_default_button_labels_with_missing_labels() {
        // Ensures missing labels are replaced with "unnamed"
        let config = make_config(
            vec![None, Some("Label2"), Some("Label3"), None, None, None],
            vec![Some("true"), Some("true"), Some("true"), None, None, None],
        );
        let labels = get_default_button_labels(&config);
        assert_eq!(labels, vec!["unnamed", "Label2", "Label3"]);
    }

    #[test]
    fn test_get_custom_buttons_filters_correctly() {
        // Ensures only actions with both label and command are included
        let config = make_config(
            vec![
                Some("A1"),
                Some("A2"),
                Some("A3"),
                Some("C1"),
                None,
                Some("C3"),
            ],
            vec![
                Some("true"),
                Some("true"),
                Some("true"),
                Some("cmd1"),
                Some("cmd2"),
                None,
            ],
        );
        let custom = get_custom_buttons(&config);
        assert_eq!(custom, vec![("C1".to_string(), vec!["cmd1".to_string()])]);
    }

    #[test]
    fn test_execute_command_handles_failure() {
        // Ensures no panic on invalid command
        let cmd = vec!["nonexistent_command_xyz".to_string()];
        execute_command(&cmd);
    }

    #[test]
    fn test_execute_command_success() {
        // Ensures valid command executes without error
        let cmd = vec!["true".to_string()];
        execute_command(&cmd);
    }
}
