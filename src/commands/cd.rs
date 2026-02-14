use std::{
    env,
    path::{Component, PathBuf},
};

use crate::shell::{CommandInput, CommandOutput};

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
