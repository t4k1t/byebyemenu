use log::{debug, info};
use std::env;
use std::process::Command;

pub fn do_exit() {
    info!("selected exit");
    let user = match env::var("USER") {
        Ok(env_user) => env_user,
        Err(_) => "".to_string(),
    };
    let status = Command::new("/usr/bin/loginctl")
        .args(["terminate-user", &user])
        .status();
    debug!("status: {status:?}");
}

pub fn do_shutdown() {
    info!("selected shutdown");
    // let status = Command::new("/usr/bin/systemctl")
    //     .args(["poweroff"])
    //     .status();
    // debug!("status: {status:?}");
}

pub fn do_reboot() {
    info!("selected reboot");
    // let status = Command::new("/usr/bin/systemctl").args(["reboot"]).status();
    // debug!("status: {status:?}");
}
