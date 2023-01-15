use rusty_cli::command_handler::CommandHandlerArguments;
use rusty_cli::runner::Runner;

mod command_list;
mod commands;
mod config;

fn main() {

    // Initially parse config to check if it is valid
    config::parse_config();

    let commands = command_list::get_commands();

    let mut runner = Runner::new();
    runner.enable_command_handler(CommandHandlerArguments {
        commands,
        default_no_argument_callback: None,
        flags: vec![]
    });
    runner.run();
}
