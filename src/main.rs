mod commands;
mod os;
mod parser;
mod shell;

use std::collections::{HashMap, HashSet};
use std::env;
use std::path::PathBuf;
use std::process::ChildStdout;

use crossterm::cursor;

use crate::{
    os::OSInstance,
    shell::{CommandInput, CommandOutput, output},
};

enum OutputProcessor {
    Console,
    StdoutToFile(PathBuf, bool),
    StderrToFile(PathBuf, bool),
}

fn main() {
    let mut output_processor = OutputProcessor::Console;
    let mut current_dir: PathBuf = env::current_dir().unwrap_or_default();

    let os_instance = OSInstance::new();

    let mut commands: HashMap<&str, fn(CommandInput) -> CommandOutput> = HashMap::new();
    commands.insert("echo", commands::echo);
    commands.insert("exit", commands::exit);
    commands.insert("pwd", commands::pwd);
    commands.insert("cd", commands::cd);
    commands.insert("dir", commands::ls);
    commands.insert("type", commands::type_fn);

    let mut know_commands: HashSet<String> = HashSet::new();

    for c in commands.keys() {
        know_commands.insert(c.to_string());
    }

    for c in os_instance.get_know_commands() {
        know_commands.insert(c);
    }

    loop {
        let user_input = shell::input::retrieve_user_input(&know_commands);
        let command_input: Vec<&str> = user_input.trim().split('|').collect();
        let last_command_position = &command_input.len() - 1;
        let mut previous_result: Option<String> = None;

        let mut program_run_children = Vec::new();
        let mut previous_stdout: Option<ChildStdout> = None;

        for (position, command) in command_input.into_iter().enumerate() {
            let words = parser::parse_input(command);

            match output::define_output_processor(command, &current_dir) {
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
                    std_input: previous_result.clone(),
                };

                if let Some(action) = action_requested {
                    let result = action(input);

                    // process results
                    if let Some(path) = result.updated_dir {
                        current_dir = path;
                    }

                    output::process_output(
                        &output_processor,
                        result.std_output.clone(),
                        result.std_error,
                        position == last_command_position,
                    );
                    previous_result = result.std_output;
                } else {
                    let is_last = position == last_command_position;
                    let has_redirect = !matches!(output_processor, OutputProcessor::Console);
                    match commands::run_program(input, &mut previous_stdout, is_last, has_redirect)
                    {
                        Ok(result) => program_run_children.push((result, is_last && has_redirect)),
                        Err(error) => print!("{error}"),
                    }
                };
            }
        }

        // Wait for all children
        for (mut child, capture_output) in program_run_children {
            if capture_output {
                let result = child.wait_with_output().expect("failed to wait");
                let std_output = parse_child_output(result.stdout);
                let std_error = parse_child_output(result.stderr);
                output::process_output(&output_processor, std_output, std_error, true);
            } else {
                child.wait().expect("failed to wait");
            }
        }
    }
}

fn parse_child_output(raw: Vec<u8>) -> Option<String> {
    match String::from_utf8(raw) {
        Ok(s) if s.is_empty() => None,
        Ok(s) => Some(s.trim_end_matches('\n').to_string()),
        Err(_) => None,
    }
}
