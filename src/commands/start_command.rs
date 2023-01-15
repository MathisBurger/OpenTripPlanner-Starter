use std::process::Command;
use rusty_cli::flags::flag::Flags;
use crate::commands::doctor::find_otp_version;
use crate::config::{Config, parse_config};

/// Command executor to start otp
pub(crate) fn executor(flags: Flags) {
    let config = parse_config();
    let otp_version = find_otp_version();
    if otp_version.is_none() {
        println!("OTP not found");
        return;
    }

    // Command to start OTP
    Command::new("nohup")
        .args(vec![
            "java",
            format!("-Xmx{}", config.memory_limit).as_str(),
            "-jar",
            otp_version.unwrap().as_str(),
            "--port",
            format!("{}", config.http.port).as_str(),
            "--securePort",
            format!("{}", config.http.secure_port).as_str(),
            get_target_command(&flags),
            "--serve",
            get_target_dir(flags, config).as_str()
        ])
        .spawn()
        .expect("Cannot start server");

    println!("OTP started successfully")
}

/// Decides on the flags if the server should be started
/// by using a generated graph file or if the
/// graph should be generated on startup
fn get_target_command(flags: &Flags) -> &'static str {
    if flags.get("buildOnStartup").is_some() {
        return "--build"
    }
    return "--load";
}

/// Gets the target directory for the action
fn get_target_dir(flags: Flags, config: Config) -> String{
    if flags.get("buildOnStartup").is_some() {
        return config.path.data_dir.clone();
    }
    return config.path.graph_dir.clone();
}