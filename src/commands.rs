use crate::shell::{CommandInput, CommandOutput};
use is_executable::IsExecutable;
use owo_colors::OwoColorize;
use std::{
    env, fs,
    path::{Component, Path, PathBuf},
    process::Command,
};

pub fn exit(_: CommandInput) -> CommandOutput {
    std::process::exit(0);
}

pub fn pwd(input: CommandInput) -> CommandOutput {
    CommandOutput::success(input.current_dir.display().to_string())
}

pub fn echo(input: CommandInput) -> CommandOutput {
    CommandOutput::success(input.command_arguments.join(" "))
}

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

fn find_executable(name: &str, current_dir: &Path) -> Option<PathBuf> {
    // search current folder
    if let Some(value) = find_executable_folder(name, current_dir) {
        return Some(value);
    }

    // search path
    let path = std::env::var_os("PATH")?;

    env::split_paths(&path).find_map(|path_item| find_executable_folder(name, &path_item))
}

fn find_executable_folder(name: &str, path_item: &Path) -> Option<PathBuf> {
    let Ok(read_dir_value) = fs::read_dir(path_item) else {
        return None;
    };

    for entry in read_dir_value {
        let Ok(entry_result) = entry else {
            continue;
        };

        let file_path = entry_result.path();

        if file_path.ends_with(name) && file_path.is_executable() {
            return Some(file_path);
        }
    }

    None
}

pub fn cd(input: CommandInput) -> CommandOutput {
    let Some(path) = input.command_arguments.first() else {
        return CommandOutput::empty();
    };

    let path = if path.starts_with("~") {
        let Some(home_dir) = env::var("HOME").ok().map(PathBuf::from) else {
            return CommandOutput::failure("HOME directory is not available".to_string());
        };
        path.replacen("~", &home_dir.display().to_string(), 1)
    } else {
        path.to_string()
    };

    let mut target_dir = PathBuf::from(input.current_dir);
    let pathbuf_dir = PathBuf::from(&path);

    for path_component in pathbuf_dir.components() {
        match path_component {
            Component::RootDir | Component::Prefix(_) => {
                target_dir = PathBuf::from(&path);
                break;
            }
            Component::ParentDir => {
                target_dir.pop();
            }
            Component::Normal(value) => {
                target_dir.push(value);
            }
            Component::CurDir => continue,
        }
    }

    if target_dir.exists() {
        CommandOutput::path_update(target_dir)
    } else {
        CommandOutput::failure(format!("cd: {path}: No such file or directory"))
    }
}

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
