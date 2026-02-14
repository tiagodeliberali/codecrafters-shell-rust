mod commands;
mod shell;

use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::commands::{cd, echo, exit, ls, pwd, run_program, type_fn};
use crate::shell::{CommandInput, CommandOutput};

fn main() {
    let mut current_dir: PathBuf = env::current_dir().unwrap_or_default();

    let mut commands: HashMap<&str, fn(CommandInput) -> CommandOutput> = HashMap::new();
    commands.insert("echo", echo);
    commands.insert("exit", exit);
    commands.insert("pwd", pwd);
    commands.insert("cd", cd);
    commands.insert("dir", ls);
    commands.insert("type", type_fn);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Invalid input");

        let command = user_input.trim();

        let words = parse_input(command);

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
                run_program(input)
            };

            // process results
            if let Some(msg) = result.std_output {
                println!("{msg}");
            }

            if let Some(msg) = result.std_error {
                println!("{}", msg.red());
            }

            if let Some(path) = result.updated_dir {
                current_dir = path;
            }
        }
    }
}

enum Parser {
    SingleQuote,
    DoubleQuote,
    Escape,
    EscapeInDoubleQuote,
    None,
}

fn parse_input(argument: &str) -> Vec<String> {
    let mut arguments: Vec<String> = Vec::new();
    let mut current_argument = String::new();

    let mut current_parser = Parser::None;

    for character in argument.chars() {
        match current_parser {
            Parser::None => {
                if character == '\'' {
                    current_parser = Parser::SingleQuote;
                } else if character == '"' {
                    current_parser = Parser::DoubleQuote;
                } else if character == '\\' {
                    current_parser = Parser::Escape;
                } else if character == ' ' {
                    if !current_argument.is_empty() {
                        arguments.push(current_argument.clone());
                        current_argument.clear();
                    }
                } else {
                    current_argument.push(character);
                }
            }
            Parser::DoubleQuote => {
                if character == '"' {
                    current_parser = Parser::None;
                } else if character == '\\' {
                    current_parser = Parser::EscapeInDoubleQuote;
                } else {
                    current_argument.push(character);
                }
            }
            Parser::SingleQuote => {
                if character == '\'' {
                    current_parser = Parser::None;
                } else {
                    current_argument.push(character);
                }
            }
            Parser::Escape => {
                current_argument.push(character);
                current_parser = Parser::None;
            }
            Parser::EscapeInDoubleQuote => {
                if matches!(character, '"' | '\\') {
                    current_argument.push(character);
                } else {
                    current_argument.push('\\');
                    current_argument.push(character);
                }

                current_parser = Parser::DoubleQuote;
            }
        }
    }

    if current_argument.len() > 0 {
        arguments.push(current_argument);
    }

    arguments
}
