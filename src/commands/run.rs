use std::{
    io::Write,
    process::{Child, ChildStdout, Command, Stdio},
};

use crate::shell::{CommandInput};

pub fn run_program(
    input: CommandInput,
    previous_stdout: &mut Option<ChildStdout>,
    is_last: bool,
    has_redirect: bool,
) -> Result<Child, String> {
    if input
        .os
        .find_executable(input.command_name, input.current_dir)
        .is_none()
    {
        return Err(format!("{}: not found", input.command_name));
    };

    let stdin = match previous_stdout.take() {
        Some(prev_out) => Stdio::from(prev_out),    // pipe from previous
        None => Stdio::inherit(),                               // first command
    };

    let inherit_output = is_last && !has_redirect;
    let stdout = if inherit_output { Stdio::inherit() } else { Stdio::piped() };
    let stderr = if inherit_output { Stdio::inherit() } else { Stdio::piped() };

    let mut child = Command::new(input.command_name)
        .args(input.command_arguments)
        .current_dir(input.current_dir)
        .stdin(stdin)
        .stdout(stdout)
        .stderr(stderr)
        .spawn()
        .expect("failed to execute process");

    if let Some(piped_input) = &input.std_input
        && let Some(mut stdin) = child.stdin.take()
    {
        let _ = stdin.write_all(piped_input.as_bytes());
    }

    if !is_last {
        *previous_stdout = child.stdout.take();
    }

    Ok(child)
}
