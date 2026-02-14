use is_executable::IsExecutable;
use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use owo_colors::OwoColorize;

struct CommandInput<'a> {
    command_name: &'a str,
    command_arguments: &'a [&'a str],
    current_dir: &'a Path
}

struct CommandOutput {
    updated_dir: Option<PathBuf>,
    std_output: Option<String>,
    std_error: Option<String>
}

impl CommandOutput {
    fn success(msg: String) -> CommandOutput {
        CommandOutput { 
            updated_dir: None, 
            std_output: Some(msg), 
            std_error: None }
    }

    fn failure(msg: String) -> CommandOutput {
        CommandOutput { 
            updated_dir: None, 
            std_output: None, 
            std_error: Some(msg), 
        }
    }

    fn empty() -> CommandOutput {
        CommandOutput { 
            updated_dir:None, 
            std_output: None, 
            std_error: None 
        }
    }

    fn path_update(path: PathBuf) -> CommandOutput {
        CommandOutput {
            updated_dir: Some(path),
            std_output: None,
            std_error: None
        }
    }
}

fn main() {
    let mut current_dir: PathBuf = PathBuf::new();

    if let Ok(cur_dir) = env::current_dir() {
        current_dir = cur_dir;
    };

    let mut commands: HashMap<&str, fn(CommandInput) -> CommandOutput> = HashMap::new();
    commands.insert("echo", echo);
    commands.insert("exit", exit);
    commands.insert("pwd", pwd);
    commands.insert("cd", cd);
    commands.insert("ls", ls);
    commands.insert("type", type_fn);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Invalid input");

        let command = user_input.trim();
        let words: Vec<&str> = command.split(' ').collect();

        if let Some(&command_name) = words.first() {
            let action_requested = commands.get(&command_name);

            let input = CommandInput {
                command_name,
                command_arguments: &words[1..],
                current_dir: &current_dir
            };

            let result = if let Some(action) = action_requested {
                action(input)
            } else {
                run_program(input)
            };

            // process results
            if let Some(msg) = result.std_output {
                println!("{msg}");
            }

            if let Some(msg) = result.std_error {
                println!("{}", msg.red());
            }

            if let Some(path) = result.updated_dir {
                current_dir = path;
            }
        }
    }
}

fn exit(_: CommandInput) -> CommandOutput {
    std::process::exit(0);
}

fn pwd(input: CommandInput) -> CommandOutput {
    CommandOutput::success(format!("{}", &input.current_dir.display()))
}

fn echo(input: CommandInput) -> CommandOutput {
    CommandOutput::success(format!("{}", input.command_arguments.join(" ")))
}

fn type_fn(input: CommandInput) -> CommandOutput {
    let keywords: HashSet<&str> = HashSet::from(["echo", "exit", "type", "pwd", "cd", "ls"]);

    let Some(name) = input.command_arguments.first() else {
        return CommandOutput::failure(format!(": not found"));
    };

    return if keywords.contains(name) {
        CommandOutput::success(format!("{name} is a shell builtin"))
    } else {
        match find_executable(&input) {
            Some(path) => CommandOutput::success(format!("{name} is {path}")),
            None => CommandOutput::failure(format!("{name}: not found")),
        }
    }
}

fn run_program(input: CommandInput) -> CommandOutput {
    let Some(_) = find_executable(&input) else {
        return CommandOutput::failure(format!("{}: not found", input.command_name));
    };

    let output = Command::new(input.command_name)
        .args(input.command_arguments.iter().map(|x| OsStr::new(x)))
        .current_dir(input.current_dir)
        .output()
        .expect("failed to execute process");

    let Ok(message) = str::from_utf8(&output.stdout) else {
        return CommandOutput::empty();
    };

    CommandOutput::success(format!("{message}"))
}

fn find_executable(input: &CommandInput) -> Option<String> {
    // search current folder
    if let Some(value) = find_executable_folder(input.command_name, input.current_dir) {
        return Some(value);
    }

    // search path
    let Some(path) = std::env::var_os("PATH") else {
        return None;
    };

    let path_list: Vec<PathBuf> = std::env::split_paths(&path).collect();

    for path_item in &path_list {
        if let Some(value) = find_executable_folder(input.command_name, path_item) {
            return Some(value);
        }
    }

    return None;
}

fn find_executable_folder(name: &str, path_item: &Path) -> Option<String> {
    let Ok(read_dir_value) = fs::read_dir(path_item) else {
        return None;
    };

    for entry in read_dir_value {
        let Ok(entry_result) = entry else {
            return None;
        };

        let file_path = entry_result.path();

        if file_path.ends_with(name) && file_path.is_executable() {
            return Some(String::from(file_path.to_str().unwrap_or_default()));
        }
    }

    return None;
}

fn cd(input: CommandInput) -> CommandOutput {
    let Some(path) = input.command_arguments.first() else {
        return CommandOutput::empty();
    };

    let Some(home_dir) = env::home_dir() else {
        return CommandOutput::failure(format!("HOME directory is not available"));
    };

    let mut target_dir = PathBuf::from(input.current_dir);
    let Ok(pathbuf_dir) = PathBuf::from_str(*path);

    for path_component in pathbuf_dir.components() {
        match path_component {
            Component::RootDir | Component::Prefix(_) => {
                let Ok(value) = PathBuf::from_str(*path);
                target_dir = value;
                break;
            }
            Component::ParentDir => {
                target_dir.pop();
            }
            Component::Normal(value) => {
                if value.eq("~") {
                    target_dir = home_dir.clone();
                } else {
                    target_dir.push(value);
                }
            }
            Component::CurDir => continue,
        }
    }

    let path_exists = match fs::exists(&target_dir) {
        Ok(value) => value,
        _ => false,
    };

    return if path_exists {
        CommandOutput::path_update(target_dir)
    } else {
        CommandOutput::failure(format!("cd: {path}: No such file or directory"))
    }
}

fn ls(input: CommandInput) -> CommandOutput {
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
            folders.push(format!("{}", format!("[{}]", file_name.display()).yellow()));
        }
        else if file_path.is_executable() {
            executables.push(format!("{}", format!("*{}", file_name.display()).green()));
        }
        else {
            others.push(format!("{}", file_name.display()));
        }
    }

    folders.sort();
    executables.sort();
    others.sort();

    folders.append(&mut executables);
    folders.append(&mut others);

    CommandOutput::success(folders.join("\n"))
}


