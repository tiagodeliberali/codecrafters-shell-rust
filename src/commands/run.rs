use std::{
    io::Write,
    process::{Command, Stdio},
};

use crate::shell::{CommandInput, CommandOutput};

pub fn run_program(input: CommandInput) -> CommandOutput {
    if input
        .os
        .find_executable(input.command_name, input.current_dir)
        .is_none()
    {
        return CommandOutput::failure(format!("{}: not found", input.command_name));
    };

    let mut child = Command::new(input.command_name)
        .args(input.command_arguments)
        .current_dir(input.current_dir)
        .stdin(if input.std_input.is_some() {
            Stdio::piped()
        } else {
            Stdio::inherit()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    if let Some(piped_input) = &input.std_input
        && let Some(mut stdin) = child.stdin.take()
    {
        let _ = stdin.write_all(piped_input.as_bytes());
    }

    match child.wait_with_output() {
        Ok(output) => CommandOutput {
            std_output: parse_output_std(output.stdout),
            std_error: parse_output_std(output.stderr),
            ..Default::default()
        },
        Err(error) => CommandOutput::failure(error.to_string()),
    }
}

fn parse_output_std(std_out: Vec<u8>) -> Option<String> {
    let std_out = String::from_utf8(std_out);
    match std_out {
        Err(_) => None,
        Ok(value) => {
            let output = value.to_string();
            if output.is_empty() {
                None
            } else {
                Some(output)
            }
        }
    }
}
