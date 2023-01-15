use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use rusty_cli::flags::flag::Flags;
use crate::commands::doctor::find_otp_version;

pub(crate) fn executor(_flags: Flags) {
    let otp_version = find_otp_version();
    if otp_version.is_none() {
        println!("OTP not found");
        return;
    }
    let mut cmd = Command::new("java")
        .args(vec!["-Xmx10G", "-jar", otp_version.unwrap().as_str(), "--build", "--save", "./data"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Cannot build graph");

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("{}", line.unwrap());
        }
        cmd.wait().expect("Cannot build graph");
    }
}