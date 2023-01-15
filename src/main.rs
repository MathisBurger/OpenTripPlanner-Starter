use rusty_cli::command_handler::CommandHandlerArguments;
use rusty_cli::runner::Runner;

mod command_list;
mod commands;

fn main() {

    let commands = command_list::get_commands();

    let mut runner = Runner::new();
    runner.enable_command_handler(CommandHandlerArguments {
        commands: commands,
        default_no_argument_callback: None,
        flags: vec![]
    });
    runner.run();
}
