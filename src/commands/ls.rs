use crate::shell::{CommandInput, CommandOutput};
use is_executable::IsExecutable;
use owo_colors::OwoColorize;
use std::fs;

pub fn ls(input: CommandInput) -> CommandOutput {
    let Ok(read_dir_value) = fs::read_dir(input.current_dir) else {
        return CommandOutput::empty();
    };

    let mut folders: Vec<String> = Vec::new();
    let mut executables: Vec<String> = Vec::new();
    let mut others: Vec<String> = Vec::new();

    for entry in read_dir_value {
        let Ok(entry_result) = entry else {
            continue;
        };

        let file_path = entry_result.path();
        let Some(file_name) = file_path.file_name() else {
            continue;
        };

        if file_path.is_dir() {
            folders.push(format!("[{}]", file_name.display()).yellow().to_string());
        } else if file_path.is_executable() {
            executables.push(format!("*{}", file_name.display()).green().to_string());
        } else {
            others.push(file_name.display().to_string());
        }
    }

    folders.sort();
    executables.sort();
    others.sort();

    folders.append(&mut executables);
    folders.append(&mut others);

    CommandOutput::success(folders.join("\n"))
}
