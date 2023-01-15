use rusty_cli::commands::command::Command;
use crate::commands::build_graph;

pub(crate) fn get_commands() -> Vec<Command> {

    let build_graph = Command::new(
        "Build Graph".to_string(),
        "Builds the graph of the open trip planner".to_string(),
        "Simple".to_string(),
        build_graph::executor,
        "buildGraph".to_string()
    );

    return vec![build_graph];
}