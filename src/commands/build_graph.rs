use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use rusty_cli::flags::flag::Flags;
use crate::commands::doctor::find_otp_version;
use crate::config::parse_config;

/// Command executor to build the graph
pub(crate) fn executor(_flags: Flags) {

    let config = parse_config();
    let otp_version = find_otp_version();
    if otp_version.is_none() {
        println!("OTP not found");
        return;
    }

    // Command to execute the build graph command
    let mut cmd = Command::new("java")
        .args(vec![
            format!("-Xmx{}", config.memory_limit).as_str(),
            "-jar",
            otp_version.unwrap().as_str(),
            "--build",
            "--save",
            config.path.data_dir.as_str()
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Cannot build graph");

    // Fetches the command stdout and prints it to the CLI stdout
    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("{}", line.unwrap());
        }
        cmd.wait().expect("Cannot build graph");


        let _ = fs::create_dir(&config.path.graph_dir);

        // moves generated graph file to graph directory
        fs::rename(
            format!("{}/graph.obj", config.path.data_dir),
            format!("{}/graph.obj", config.path.graph_dir)
        ).expect("Cannot move graph file");
    }
}