use is_executable::IsExecutable;
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let mut commands: HashMap<&str, fn(&Vec<&str>) -> ()> = HashMap::new();
    commands.insert("echo", echo);
    commands.insert("exit", exit);
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
                action(&words);
            } else {
                run_program(&words);
            }
        }
    }
}

fn exit(_: &Vec<&str>) {
    std::process::exit(0);
}

fn echo(words: &Vec<&str>) {
    println!("{}", words[1..].join(" "));
}

fn type_fn(words: &Vec<&str>) {
    let keywords: HashSet<&str> = HashSet::from(["echo", "exit", "type"]);

    let Some(name) = words.get(1) else {
        println!(": not found");
        return;
    };

    if keywords.contains(name) {
        println!("{name} is a shell builtin");
    } else {
        match find_executable(name) {
            Some(path) => println!("{name} is {path}"),
            None => println!("{name}: not found"),
        };
    }
}

fn run_program(words: &Vec<&str>) {
    let Some(name) = words.first() else {
        println!(": not found");
        return;
    };

    let Some(path) = find_executable(name) else {
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

fn find_executable(name: &str) -> Option<String> {
    let Some(path) = std::env::var_os("PATH") else {
        return None;
    };

    let path_list: Vec<PathBuf> = std::env::split_paths(&path).collect();

    for path_item in &path_list {
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
    }

    return None;
}
