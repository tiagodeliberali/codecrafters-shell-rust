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

    let Ok(message) = str::from_utf8(&output.stdout) else {
        return CommandOutput::empty();
    };

    CommandOutput::success(message.trim().to_string())
}
