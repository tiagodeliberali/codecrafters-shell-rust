use crate::shell::{CommandInput, CommandOutput};

pub fn echo(input: CommandInput) -> CommandOutput {
    CommandOutput::success(input.command_arguments.join(" "))
}