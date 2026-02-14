use crate::shell::{CommandInput, CommandOutput};

pub fn exit(_: CommandInput) -> CommandOutput {
    std::process::exit(0);
}