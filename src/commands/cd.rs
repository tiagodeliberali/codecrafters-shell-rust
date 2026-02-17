use crate::{
    parser,
    shell::{CommandInput, CommandOutput},
};

pub fn cd(input: CommandInput) -> CommandOutput {
    let Some(path) = input.command_arguments.first() else {
        return CommandOutput::empty();
    };

    match parser::parse_path(path, input.current_dir) {
        Ok(target_dir) => {
            if target_dir.exists() {
                CommandOutput::path_update(target_dir)
            } else {
                CommandOutput::failure(format!("cd: {path}: No such file or directory"))
            }
        }
        Err(message) => CommandOutput::failure(message),
    }
}
