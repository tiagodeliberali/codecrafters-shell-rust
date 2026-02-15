mod commands;
mod parser;
mod shell;

use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::{env, fs};

use crate::shell::{CommandInput, CommandOutput};

enum OutputProcessor {
    Console,
    File(PathBuf),
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
            match parser::parse_path(&command[position+1..].trim(), &current_dir) {
                Ok(path) => {
                    output_processor = OutputProcessor::File(path);
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
                        println!("{}", msg.red());
                    }
                }
                OutputProcessor::File(ref output_path) => {
                    if let Some(msg) = result.std_output {
                        if let Err(error) = fs::write(output_path, msg) {
                            println!("Failed to write output file: {error}");
                        }
                    }

                    if let Some(msg) = result.std_error {
                        println!("{}", msg);
                    }
                }
            }
        }
    }
}
