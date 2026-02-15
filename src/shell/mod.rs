pub mod processor;

use std::path::{Path, PathBuf};

pub struct CommandInput<'a> {
    pub command_name: &'a str,
    pub command_arguments: &'a [String],
    pub current_dir: &'a Path,
}

#[derive(Default)]
pub struct CommandOutput {
    pub updated_dir: Option<PathBuf>,
    pub std_output: Option<String>,
    pub std_error: Option<String>,
}

impl CommandOutput {
    pub fn success(msg: String) -> Self {
        Self {
            std_output: Some(msg),
            ..Default::default()
        }
    }

    pub fn failure(msg: String) -> Self {
        Self {
            std_error: Some(msg),
            ..Default::default()
        }
    }

    pub fn empty() -> Self {
        Default::default()
    }

    pub fn path_update(path: PathBuf) -> Self {
        Self {
            updated_dir: Some(path),
            ..Default::default()
        }
    }
}