use std::process::{Command, Stdio};
use rusty_cli::flags::flag::Flags;
use java_locator::locate_java_home;
use std::{fs, env};

pub(crate) fn executor(_flags: Flags) {
    println!("Running doctor test:");
    let java_version = get_java_version();
    if java_version {
        println!("✅  Java 17 installed!");
    } else {
        println!("❌  Java 17 not installed");
    }
    let otp_version = find_otp_version();
    if otp_version.is_some() {
        println!("✅  Version {} found", otp_version.unwrap());
    } else {
        println!("❌  No OTP version found");
    }
    let data_dir = get_data_dir();
    if data_dir {
        println!("✅  Data directory found");
    } else {
        println!("❌  Data directory missing");
    }
}

fn get_java_version() -> bool {
    let java_home = locate_java_home().unwrap();
    return java_home.contains("17");
}

pub(crate) fn find_otp_version() -> Option<String> {
    let current_dir = env::current_dir().unwrap();
    let supported_versions = vec!["otp-2.1.0-shaded.jar", "otp-2.2.0-shaded.jar"];
    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().ok_or("None").unwrap().to_str().unwrap();
        if supported_versions.contains(&file_name) {
            return Some(file_name.parse().unwrap());
        }
    }
    None
}

fn get_data_dir() -> bool {
    let current_dir = env::current_dir().unwrap();
    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let metadata = fs::metadata(&path).unwrap();
        let file_name = path.file_name().ok_or("None").unwrap().to_str().unwrap();
        if metadata.is_dir() && file_name == "data" {
             return true;
        }

    }
    return false;
}