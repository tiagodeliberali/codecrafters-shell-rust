use crate::shell::{CommandInput, CommandOutput};

pub fn history(input: CommandInput) -> CommandOutput {
    let mut output = String::new();
    for (position, command) in input.command_history.iter().enumerate() {
        output += format!("{} {}\n", (position + 1), command).as_str();
    }
    CommandOutput::success(output)
}