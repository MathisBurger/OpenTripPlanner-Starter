use std::fmt::format;
use std::process::Command;
use rusty_cli::flags::flag::Flags;
use crate::commands::doctor::find_otp_version;
use crate::config::parse_config;

pub(crate) fn executor(_flags: Flags) {
    let config = parse_config();
    let otp_version = find_otp_version();
    if otp_version.is_none() {
        println!("OTP not found");
        return;
    }
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
            "--build",
            "--serve",
            config.path.data_dir.as_str()
        ])
        .spawn()
        .expect("Cannot start server");

    println!("OTP started successfully")
}