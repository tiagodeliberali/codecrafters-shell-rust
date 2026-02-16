use std::process::Command;

use crate::shell::{CommandInput, CommandOutput};

pub fn run_program(input: CommandInput) -> CommandOutput {
    if input.os.find_executable(input.command_name, input.current_dir).is_none() {
        return CommandOutput::failure(format!("{}: not found", input.command_name));
    };

    let output = Command::new(input.command_name)
        .args(input.command_arguments)
        .current_dir(input.current_dir)
        .output()
        .expect("failed to execute process");

    CommandOutput {
        std_output: parse_output_std(output.stdout),
        std_error: parse_output_std(output.stderr),
        ..Default::default()
    }
}

fn parse_output_std(std_out: Vec<u8>) -> Option<String> {
    let std_out = String::from_utf8(std_out);
    match std_out {
        Err(_) => None,
        Ok(value) => {
            let output = value.trim_end_matches('\n').to_string();
            if output.is_empty() {
                None
            } else {
                Some(output)
            }
        }
    }
}
