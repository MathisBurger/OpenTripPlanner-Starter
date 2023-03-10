use rusty_cli::commands::command::Command;
use crate::commands::build_graph;
use crate::commands::doctor;
use crate::commands::start_command;
use crate::commands::stop_command;

/// Gets all commands that can be executed by the CLI
/// in a vector.
pub(crate) fn get_commands() -> Vec<Command> {

    let build_graph = Command::new(
        "Build Graph".to_string(),
        "Builds the graph of the open trip planner".to_string(),
        "Simple".to_string(),
        build_graph::executor,
        "buildGraph".to_string()
    );

    let doctor = Command::new(
        "Doctor".to_string(),
        "Checks if current directory can be used with this CLI".to_string(),
        "Simple".to_string(),
        doctor::executor,
        "doctor".to_string()
    );

    let start_command = Command::new(
        "Start".to_string(),
        "Starts an otp instance".to_string(),
        "Simple".to_string(),
        start_command::executor,
        "start".to_string()
    );

    let stop_command = Command::new(
        "Stop".to_string(),
        "Stops an otp instance".to_string(),
        "Simple".to_string(),
        stop_command::executor,
        "stop".to_string()
    );

    return vec![build_graph, doctor, start_command, stop_command];
}