use std::{fs::{self, OpenOptions}, num::ParseIntError};
use std::io::Write;

use crate::shell::{CommandInput, CommandOutput};

pub fn history(input: CommandInput) -> CommandOutput {
    if let Some(arg) = input.command_arguments.first() {
        if arg == "-r" {
            let result =
                fs::read_to_string(input.command_arguments.get(1).unwrap_or(&String::new()));

            match result {
                Err(error) => {
                    return CommandOutput::failure(format!(
                        "Failed to read history file: {}",
                        error.to_string()
                    ));
                }
                Ok(result) => {
                    let mut paths: Vec<String> = Vec::new();

                    for line in result.lines() {
                        if !line.is_empty() {
                            paths.push(line.to_string());
                        }
                    }

                    return CommandOutput::history_update(paths);
                }
            }
        } else if arg == "-w" {
            return write_file(&input);
        } else if arg == "-a" {
            if let Some(path) = input.command_arguments.get(1) {
                match fs::exists(path) {
                    Err(error) => {
                        return CommandOutput::failure(error.to_string());
                    }
                    Ok(exists) => {
                        if !exists {
                            return write_file(&input);
                        } else {
                            match OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(path)
                            {
                                Ok(file) => {
                                    match write!(&file, "{}", input.command_history.join("\n")) {
                                        Err(error) => {
                                            return CommandOutput::failure(error.to_string());
                                        }
                                        Ok(_) => {
                                            return CommandOutput::empty();
                                        }
                                    }
                                }
                                Err(error) => {
                                    return CommandOutput::failure(error.to_string());
                                }
                            }
                        }
                    }
                }
            } else {
                return CommandOutput::failure("Missing history append path".to_string());
            }
        }
    }

    let size: Option<Result<usize, ParseIntError>> =
        input.command_arguments.first().map(|s| s.as_str().parse());

    let (initial_value, enumeration) = match size {
        None => (1, input.command_history.iter().enumerate()),
        Some(value) => {
            let value = match value {
                Err(error) => {
                    return CommandOutput::failure(format!(
                        "Failed to parse history argument: {}",
                        error.to_string()
                    ));
                }
                Ok(v) => v,
            };

            let start_position: usize = input.command_history.len().saturating_sub(value);
            (
                start_position + 1,
                input.command_history[start_position..].iter().enumerate(),
            )
        }
    };

    let mut output = String::new();
    for (position, command) in enumeration {
        output += format!("{} {}\n", (position + initial_value), command).as_str();
    }
    CommandOutput::success(output)
}

fn write_file(input: &CommandInput<'_>) -> CommandOutput {
    match fs::write(
        input.command_arguments.get(1).unwrap_or(&String::new()),
        format!("{}\n", input.command_history.join("\n")),
    ) {
        Err(error) => {
            return CommandOutput::failure(format!(
                "Failed to write history: {}",
                error.to_string()
            ));
        }
        Ok(_) => {
            return CommandOutput::empty();
        }
    }
}
