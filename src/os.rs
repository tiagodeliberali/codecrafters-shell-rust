use is_executable::IsExecutable;

use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

pub struct OSInstance {
    path_commands: HashMap<OsString, PathBuf>,
}

impl OSInstance {
    pub fn new() -> OSInstance {
        OSInstance {
            path_commands: load_path_commands(),
        }
    }

    pub fn find_executable(&self, name: &str, current_dir: &Path) -> Option<PathBuf> {
        // search current folder
        if let Some(value) = find_executable_folder(name, current_dir) {
            return Some(value);
        }

        // search path
        if let Some(item) = self.path_commands.get(&OsString::from(name)) {
            Some(item.clone())
        } else {
            None
        }
    }
    
    pub(crate) fn get_know_commands(&self) -> Vec<String> {
        self.path_commands.keys().map(|i| i.display().to_string()).collect()
    }
}

fn load_path_commands() -> HashMap<OsString, PathBuf> {
    let mut commands: HashMap<OsString, PathBuf> = HashMap::new();

    if let Some(path) = std::env::var_os("PATH") {
        for path_item in env::split_paths(&path) {
            let Ok(read_dir_value) = fs::read_dir(path_item) else {
                continue;
            };

            for entry in read_dir_value {
                let Ok(entry_result) = entry else {
                    continue;
                };

                let file_path = entry_result.path();

                if file_path.is_executable() {
                    if let Some(name) = file_path.file_name() {
                        commands.insert(name.to_os_string(), file_path);
                    }
                }
            }
        }
    }

    commands
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
