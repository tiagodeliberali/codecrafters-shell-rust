use crate::{commands::save_history, shell::{CommandInput, CommandOutput}};

pub fn exit(input: CommandInput) -> CommandOutput {
    save_history(input.command_history);
    std::process::exit(0);
}