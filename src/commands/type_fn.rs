use crate::{
    commands::find_executable,
    shell::{CommandInput, CommandOutput},
};

pub fn type_fn(input: CommandInput) -> CommandOutput {
    let Some(name) = input.command_arguments.first() else {
        return CommandOutput::failure(": not found".to_string());
    };

    if matches!(
        name.as_str(),
        "echo" | "exit" | "type" | "pwd" | "cd" | "dir"
    ) {
        CommandOutput::success(format!("{name} is a shell builtin"))
    } else {
        match find_executable(name, input.current_dir) {
            Some(path) => CommandOutput::success(format!("{name} is {}", path.display())),
            None => CommandOutput::failure(format!("{name}: not found")),
        }
    }
}
