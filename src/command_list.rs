use rusty_cli::commands::command::Command;
use crate::commands::build_graph;
use crate::commands::doctor;

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

    return vec![build_graph, doctor];
}