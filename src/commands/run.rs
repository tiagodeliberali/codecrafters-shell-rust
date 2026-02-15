use std::process::Command;

use crate::{
    commands::find_executable,
    shell::{CommandInput, CommandOutput},
};

pub fn run_program(input: CommandInput) -> CommandOutput {
    if find_executable(input.command_name, input.current_dir).is_none() {
        return CommandOutput::failure(format!("{}: not found", input.command_name));
    };

    let output = Command::new(input.command_name)
        .args(input.command_arguments)
        .current_dir(input.current_dir)
        .output()
        .expect("failed to execute process");

    let std_out = if let Ok(message) = str::from_utf8(&output.stdout) {
        Some(message.trim_end_matches('\n').to_string())
    } else {
        None
    };

    let std_err = if let Ok(message) = str::from_utf8(&output.stderr) {
        Some(message.trim_end_matches('\n').to_string())
    } else {
        None
    };

    CommandOutput {
        std_output: std_out,
        std_error: std_err,
        ..Default::default()
    }
}
