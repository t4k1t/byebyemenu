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
    value: Option<String>,
    default: Option<Vec<String>>,
    idx: usize,
) -> Result<Option<Vec<String>>, String> {
    match value {
        Some(cmd) => match shell_split(&cmd) {
            Ok(parts) if !parts.is_empty() => Ok(Some(parts)),
            Ok(_) => Err(format!("BBMENU_ACTION{idx}_CMD is empty after parsing")),
            Err(e) => Err(format!("Failed to parse BBMENU_ACTION{idx}_CMD: {e}")),
        },
        None => Ok(default),
    }
}

pub fn get_config_from_env() -> Result<Config, String> {
    let css_path = match env::var("BBMENU_CSS_PATH") {
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
            env::var(&cmd_env).ok(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    // Ensure exclusive access to environment variables to prevent interference between parallel tests
    static ENV_VAR_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_parse_command_valid() {
        let res = parse_command_or_error(Some("echo hello world".to_string()), None, 1).unwrap();
        assert_eq!(
            res,
            Some(vec![
                "echo".to_string(),
                "hello".to_string(),
                "world".to_string()
            ])
        );
    }

    #[test]
    fn test_parse_command_empty() {
        let err = parse_command_or_error(Some("".to_string()), None, 2).unwrap_err();
        assert!(err.contains("empty after parsing"));
    }

    #[test]
    fn test_parse_command_with_default() {
        let default = Some(vec!["ls".to_string(), "-l".to_string()]);
        let res = parse_command_or_error(None, default.clone(), 3).unwrap();
        assert_eq!(res, default);
    }

    #[test]
    fn test_parse_command_none_and_no_default() {
        let res = parse_command_or_error(None, None, 4).unwrap();
        assert_eq!(res, None);
    }

    #[test]
    fn test_parse_command_invalid_syntax() {
        let err =
            parse_command_or_error(Some("\"unterminated string".to_string()), None, 5).unwrap_err();
        assert!(err.contains("Failed to parse"));
    }

    #[test]
    fn test_config_env_defaults() {
        let _lock = ENV_VAR_MUTEX.lock().unwrap();
        unsafe {
            env::remove_var("BBMENU_CSS_PATH");
            env::remove_var("XDG_CONFIG_HOME");
            env::set_var("HOME", "/tmp/testhome");
        }
        let config = get_config_from_env().unwrap();
        assert_eq!(
            config.css_path,
            "/tmp/testhome/.config/byebyemenu/style.css"
        );
        assert_eq!(config.actions.len(), 6);
        assert_eq!(config.actions[0].label.as_deref(), Some("_exit"));
        assert!(config.actions[3].command.is_none());
    }

    #[test]
    fn test_config_env_custom_css() {
        let _lock = ENV_VAR_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BBMENU_CSS_PATH", "/custom/path/style.css");
        }
        let config = get_config_from_env().unwrap();
        assert_eq!(config.css_path, "/custom/path/style.css");
    }

    #[test]
    fn test_config_env_custom_action() {
        let _lock = ENV_VAR_MUTEX.lock().unwrap();
        unsafe {
            env::set_var("BBMENU_ACTION2_CMD", "shutdown now");
            env::set_var("BBMENU_ACTION2_LABEL", "Shutdown!");
            env::remove_var("BBMENU_CSS_PATH");
        }
        let config = get_config_from_env().unwrap();
        assert_eq!(
            config.actions[1].command,
            Some(vec!["shutdown".to_string(), "now".to_string()])
        );
        assert_eq!(config.actions[1].label.as_deref(), Some("Shutdown!"));
    }
}
