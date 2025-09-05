use log::debug;
use shell_words::split as shell_split;
use std::env;

#[derive(Clone)]
pub struct Action {
    pub command: Option<Vec<String>>,
    pub label: Option<String>,
}

#[derive(Clone)]
pub struct Config {
    pub css_path: String,
    pub actions: Vec<Action>,
}

fn parse_command_or_error(
    env_var: &str,
    default: Option<Vec<String>>,
    idx: usize,
) -> Result<Option<Vec<String>>, String> {
    match env::var(env_var) {
        Ok(cmd) => match shell_split(&cmd) {
            Ok(parts) if !parts.is_empty() => Ok(Some(parts)),
            Ok(_) => Err(format!("BBMENU_ACTION{idx}_CMD is empty after parsing")),
            Err(e) => Err(format!("Failed to parse BBMENU_ACTION{idx}_CMD: {e}")),
        },
        Err(_) => Ok(default),
    }
}

pub fn get_config_from_env() -> Result<Config, String> {
    let css_path = match env::var("BYEBYE_CSS_PATH") {
        Ok(path) => {
            debug!("CSS path from env: {path}");
            path
        }
        Err(_) => {
            let config_home = env::var("XDG_CONFIG_HOME")
                .ok()
                .or_else(|| env::var("HOME").ok().map(|h| format!("{h}/.config")));
            config_home
                .map(|dir| format!("{dir}/byebyemenu/style.css"))
                .unwrap_or_else(|| "style.css".to_string())
        }
    };

    let default_commands = [
        Some(vec![
            "/usr/bin/loginctl".to_string(),
            "terminate-user".to_string(),
            env::var("USER").unwrap_or_default(),
        ]), // exit session
        Some(vec![
            "/usr/bin/systemctl".to_string(),
            "poweroff".to_string(),
        ]), // shutdown
        Some(vec!["/usr/bin/systemctl".to_string(), "reboot".to_string()]), // reboot
        None,
        None,
        None,
    ];

    let default_labels = [
        Some("_exit".to_string()),
        Some("_shutdown".to_string()),
        Some("_reboot".to_string()),
        None,
        None,
        None,
    ];

    let mut actions = Vec::new();

    for i in 1..=6 {
        let cmd_env = format!("BBMENU_ACTION{i}_CMD");
        let label_env = format!("BBMENU_ACTION{i}_LABEL");

        let command = parse_command_or_error(
            &cmd_env,
            default_commands.get(i - 1).cloned().unwrap_or(None),
            i,
        )?;
        let label = env::var(&label_env)
            .ok()
            .or_else(|| default_labels.get(i - 1).cloned().unwrap_or(None));

        actions.push(Action { command, label });
    }

    Ok(Config { css_path, actions })
}
