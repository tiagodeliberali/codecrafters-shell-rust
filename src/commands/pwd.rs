use crate::shell::{CommandInput, CommandOutput};

pub fn pwd(input: CommandInput) -> CommandOutput {
    CommandOutput::success(input.current_dir.display().to_string())
}