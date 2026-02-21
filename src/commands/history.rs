use std::num::ParseIntError;

use crate::shell::{CommandInput, CommandOutput};

pub fn history(input: CommandInput) -> CommandOutput {
    let size: Option<Result<usize, ParseIntError>> = input.command_arguments.first().map(|s| s.as_str().parse());

    let (initial_value, enumeration) = match size {
        None => (1, input.command_history.iter().enumerate()),
        Some(value) => {
            let value = match value {
                Err(error) => {
                    return CommandOutput::failure(format!("Failed to parse history argument: {}", error.to_string()))
                }
                Ok(v) => v
            };

            let start_position: usize = input.command_history.len().saturating_sub(value);
            (start_position + 1, input.command_history[start_position..].iter().enumerate())
        }
    };

    let mut output = String::new();
    for (position, command) in enumeration {
        output += format!("{} {}\n", (position + initial_value), command).as_str();
    }
    CommandOutput::success(output)
}