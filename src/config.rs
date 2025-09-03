use log::debug;
use shell_words::split as shell_split;
use std::env;

pub struct Config {
    pub css_path: String,
    pub cmd_4: Option<Vec<String>>,
    pub cmd_5: Option<Vec<String>>,
    pub cmd_6: Option<Vec<String>>,
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
