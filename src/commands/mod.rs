mod cd;
mod echo;
mod exit;
mod ls;
mod pwd;
mod run;
mod type_fn;

pub use cd::cd;
pub use echo::echo;
pub use exit::exit;
pub use ls::ls;
pub use pwd::pwd;
pub use run::run_program;
pub use type_fn::type_fn;

use is_executable::IsExecutable;

use std::{
    env, fs,
    path::{Path, PathBuf},
};

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
