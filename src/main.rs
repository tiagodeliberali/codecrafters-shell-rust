use is_executable::IsExecutable;
use std::collections::{HashMap, HashSet};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

fn main() {
    let mut current_dir: PathBuf = PathBuf::new();

    if let Ok(cur_dir) = env::current_dir() {
        current_dir = cur_dir.clone();
    };

    let mut commands: HashMap<&str, fn(&Vec<&str>, &mut PathBuf) -> ()> = HashMap::new();
    commands.insert("echo", echo);
    commands.insert("exit", exit);
    commands.insert("pwd", pwd);
    commands.insert("cd", cd);
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

            if let Some(action) = action_requested {
                action(&words, &mut current_dir);
            } else {
                run_program(&words, &mut current_dir);
            }
        }
    }
}

fn exit(_: &Vec<&str>, _: &mut PathBuf) {
    std::process::exit(0);
}

fn pwd(_: &Vec<&str>, current_dir: &mut PathBuf) {
    println!("{}", &current_dir.display());
}

fn echo(words: &Vec<&str>, _: &mut PathBuf) {
    println!("{}", words[1..].join(" "));
}

fn type_fn(words: &Vec<&str>, current_dir: &mut PathBuf) {
    let keywords: HashSet<&str> = HashSet::from(["echo", "exit", "type", "pwd", "cd"]);

    let Some(name) = words.get(1) else {
        println!(": not found");
        return;
    };

    if keywords.contains(name) {
        println!("{name} is a shell builtin");
    } else {
        match find_executable(name, current_dir) {
            Some(path) => println!("{name} is {path}"),
            None => println!("{name}: not found"),
        };
    }
}

fn run_program(words: &Vec<&str>, current_dir: &mut PathBuf) {
    let Some(name) = words.first() else {
        println!(": not found");
        return;
    };

    let Some(path) = find_executable(name, current_dir) else {
        println!("{name}: not found");
        return;
    };

    let output = Command::new(name)
        .args(words[1..].iter().map(|x| OsStr::new(x)))
        .output()
        .expect("failed to execute process");

    let Ok(message) = str::from_utf8(&output.stdout) else {
        return;
    };

    print!("{message}");
}

fn find_executable(name: &str, current_dir: &mut PathBuf) -> Option<String> {
    // search current folder
    if let Some(value) = find_executable_folder(name, current_dir) {
        return Some(value);
    }

    // search path
    let Some(path) = std::env::var_os("PATH") else {
        return None;
    };

    let path_list: Vec<PathBuf> = std::env::split_paths(&path).collect();

    for path_item in &path_list {
        if let Some(value) = find_executable_folder(name, path_item) {
            return Some(value);
        }
    }

    return None;
}

fn find_executable_folder(name: &str, path_item: &PathBuf) -> Option<String> {
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

fn cd(words: &Vec<&str>, current_dir: &mut PathBuf) {
    let Some(path) = words.get(1) else {
        return;
    };

    let mut target_dir = current_dir.clone();
    let Ok(pathbuf_dir) = PathBuf::from_str(*path);

    for path_component in pathbuf_dir.components() {
        match path_component {
            Component::RootDir | Component::Prefix(_) => {
                let Ok(value) = PathBuf::from_str(*path);
                target_dir = value;
                break;
            },
            Component::ParentDir => {
                target_dir.pop();
            },
            Component::Normal(value) => target_dir.push(value),
            Component::CurDir => continue,
        }
    }

    let path_exists = match fs::exists(&target_dir) {
        Ok(value) => value,
        _ => false,
    };

    if path_exists {
        *current_dir = target_dir.clone();
    } else {
        println!("cd: {path}: No such file or directory")
    }
}
