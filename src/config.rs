use log::debug;
use shell_words::split as shell_split;
use std::env;

pub struct Config {
    pub css_path: String,
    pub cmd_1: Option<Vec<String>>,
    pub cmd_2: Option<Vec<String>>,
    pub cmd_3: Option<Vec<String>>,
    pub cmd_4: Option<Vec<String>>,
    pub cmd_5: Option<Vec<String>>,
    pub cmd_6: Option<Vec<String>>,
    pub label_1: Option<String>,
    pub label_2: Option<String>,
    pub label_3: Option<String>,
    pub label_4: Option<String>,
    pub label_5: Option<String>,
    pub label_6: Option<String>,
}

pub fn get_config_from_env() -> Config {
    let css_path = match env::var("BYEBYE_CSS_PATH") {
        Ok(path) => {
            debug!("CSS path from env: {path}");
            path
        }
        Err(_) => {
            let config_home = env::var("XDG_CONFIG_HOME")
                .ok()
                .or_else(|| env::var("HOME").ok().map(|h| format!("{h}/.config")));
            let default_path = config_home
                .map(|dir| format!("{dir}/byebyemenu/style.css"))
                .unwrap_or_else(|| "style.css".to_string());
            default_path
        }
    };
    // TODO: Refactor (variable names, better way to read from env, code duplication)
    let cmd_1 = match env::var("BBMENU_ACTION1_CMD") {
        Ok(cmd) => shell_split(&cmd).ok(),
        Err(_) => Some(vec![
            "/usr/bin/loginctl".to_string(),
            "terminate-user".to_string(),
            env::var("USER").unwrap_or_default(),
        ]), // Default: exit session
    };
    let cmd_2 = match env::var("BBMENU_ACTION2_CMD") {
        Ok(cmd) => shell_split(&cmd).ok(),
        Err(_) => Some(vec![
            "/usr/bin/systemctl".to_string(),
            "poweroff".to_string(),
        ]), // Default: shutdown
    };
    let cmd_3 = match env::var("BBMENU_ACTION3_CMD") {
        Ok(cmd) => shell_split(&cmd).ok(),
        Err(_) => Some(vec!["/usr/bin/systemctl".to_string(), "reboot".to_string()]), // Default: reboot
    };
    let cmd_4 = match env::var("BBMENU_ACTION4_CMD") {
        Ok(cmd) => shell_split(&cmd).ok(),
        Err(_) => None,
    };
    let cmd_5 = match env::var("BBMENU_ACTION5_CMD") {
        Ok(cmd) => shell_split(&cmd).ok(),
        Err(_) => None,
    };
    let cmd_6 = match env::var("BBMENU_ACTION6_CMD") {
        Ok(cmd) => shell_split(&cmd).ok(),
        Err(_) => None,
    };

    let label_1 = env::var("BBMENU_ACTION1_LABEL")
        .ok()
        .or(Some("_exit".to_string()));
    let label_2 = env::var("BBMENU_ACTION2_LABEL")
        .ok()
        .or(Some("_shutdown".to_string()));
    let label_3 = env::var("BBMENU_ACTION3_LABEL")
        .ok()
        .or(Some("_reboot".to_string()));
    let label_4 = env::var("BBMENU_ACTION4_LABEL").ok();
    let label_5 = env::var("BBMENU_ACTION5_LABEL").ok();
    let label_6 = env::var("BBMENU_ACTION6_LABEL").ok();

    Config {
        css_path,
        cmd_1,
        cmd_2,
        cmd_3,
        cmd_4,
        cmd_5,
        cmd_6,
        label_1,
        label_2,
        label_3,
        label_4,
        label_5,
        label_6,
    }
}
