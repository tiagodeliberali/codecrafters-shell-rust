mod commands;
mod parser;
mod shell;

// use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;
use std::{env, fs};

use crate::shell::{CommandInput, CommandOutput};

enum OutputProcessor {
    Console,
    StdoutToFile(PathBuf, bool),
    StderrToFile(PathBuf, bool),
}

fn main() {
    let mut output_processor;
    let mut current_dir: PathBuf = env::current_dir().unwrap_or_default();

    let mut commands: HashMap<&str, fn(CommandInput) -> CommandOutput> = HashMap::new();
    commands.insert("echo", commands::echo);
    commands.insert("exit", commands::exit);
    commands.insert("pwd", commands::pwd);
    commands.insert("cd", commands::cd);
    commands.insert("dir", commands::ls);
    commands.insert("type", commands::type_fn);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Invalid input");

        let command = user_input.trim();
        let words = parser::parse_input(command);

        // redirect operation
        let find_position = command.find('>');
        if let Some(position) = find_position {
            // chgeck byte after the '>' position to see if it is an append operation, with another '>'
            let append_operation = if let Some(&byte_value) = position
                .checked_add(1)
                .and_then(|i| command.as_bytes().get(i))
            {
                byte_value == b'>'
            } else {
                false
            };

            let inital_argument_position = if append_operation {
                position + 2
            } else {
                position + 1
            };

            match parser::parse_path(&command[inital_argument_position..].trim(), &current_dir) {
                Ok(path) => {
                    if let Some(&byte_value) = position
                        .checked_sub(1)
                        .and_then(|i| command.as_bytes().get(i))
                    {
                        if byte_value == b'2' {
                            output_processor =
                                OutputProcessor::StderrToFile(path, append_operation);
                        } else {
                            output_processor =
                                OutputProcessor::StdoutToFile(path, append_operation);
                        }
                    } else {
                        output_processor = OutputProcessor::StdoutToFile(path, append_operation);
                    }
                }
                Err(message) => {
                    println!("Invalid redirect output operation: {message}");
                    continue;
                }
            }
        } else {
            output_processor = OutputProcessor::Console;
        }

        if let Some(command_name) = words.first() {
            let action_requested = commands.get(&command_name.as_str());

            let input = CommandInput {
                command_name: command_name.as_str(),
                command_arguments: &words[1..],
                current_dir: &current_dir,
            };

            let result = if let Some(action) = action_requested {
                action(input)
            } else {
                commands::run_program(input)
            };

            // process results
            if let Some(path) = result.updated_dir {
                current_dir = path;
            }

            match output_processor {
                OutputProcessor::Console => {
                    if let Some(msg) = result.std_output {
                        println!("{msg}");
                    }

                    if let Some(msg) = result.std_error {
                        println!("{}", msg);
                    }
                }
                OutputProcessor::StdoutToFile(ref output_path, append) => {
                    write_output_to_file(output_path, result.std_output, append);

                    if let Some(msg) = result.std_error {
                        println!("{}", msg);
                    }
                }
                OutputProcessor::StderrToFile(ref output_path, append) => {
                    if let Some(msg) = result.std_output {
                        println!("{msg}");
                    }

                    write_output_to_file(output_path, result.std_error, append);
                }
            }
        }
    }
}

fn write_output_to_file(output_path: &PathBuf, content: Option<String>, append: bool) {
    let result = if append {
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_path)
        {
            Ok(file) => {
                let content = content.unwrap_or_default();
                if file.metadata().map(|m| m.len() > 0).unwrap_or(false) {
                    write!(&file, "\n{}", content)
                } else {
                    write!(&file, "{}", content)
                }
            },
            Err(error) => Err(error),
        }
    } else {
        fs::write(output_path, content.unwrap_or_default())
    };

    if let Err(error) = result {
        println!("Failed to write output file: {error}");
    }
}
