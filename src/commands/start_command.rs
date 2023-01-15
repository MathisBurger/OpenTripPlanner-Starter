use std::process::Command;
use rusty_cli::flags::flag::Flags;
use crate::commands::doctor::find_otp_version;

pub(crate) fn executor(_flags: Flags) {
    let otp_version = find_otp_version();
    if otp_version.is_none() {
        println!("OTP not found");
        return;
    }
    Command::new("nohup")
        .args(vec!["java", "-Xmx10G", "-jar", otp_version.unwrap().as_str(), "--build", "--serve", "./data"])
        .spawn()
        .expect("Cannot start server");

    println!("OTP started successfully")
}