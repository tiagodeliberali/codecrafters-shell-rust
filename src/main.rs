mod commands;
mod parser;
mod shell;
mod os;

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use crate::{os::OSInstance, shell::{CommandInput, CommandOutput, output}};

enum OutputProcessor {
    Console,
    StdoutToFile(PathBuf, bool),
    StderrToFile(PathBuf, bool),
}

fn main() {
    let mut output_processor;
    let mut current_dir: PathBuf = env::current_dir().unwrap_or_default();

    let os_instance = OSInstance::new();

    let mut commands: HashMap<&str, fn(CommandInput) -> CommandOutput> = HashMap::new();
    commands.insert("echo", commands::echo);
    commands.insert("exit", commands::exit);
    commands.insert("pwd", commands::pwd);
    commands.insert("cd", commands::cd);
    commands.insert("dir", commands::ls);
    commands.insert("type", commands::type_fn);

    let know_commands = commands.keys().map(|i| i.to_string()).collect();

    loop {
        let user_input = shell::input::retrieve_user_input(&know_commands);
        let command = user_input.trim();
        let words = parser::parse_input(command);

        match output::define_output_processor(&command, &current_dir) {
            Ok(processor) => output_processor = processor,
            Err(message) => {
                println!("{}", message);
                continue;
            }
        };

        if let Some(command_name) = words.first() {
            let action_requested = commands.get(&command_name.as_str());

            let input = CommandInput {
                command_name: command_name.as_str(),
                command_arguments: &words[1..],
                current_dir: &current_dir,
                os: &os_instance,
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

            output::process_output(&output_processor, result.std_output, result.std_error);
        }
    }
}
