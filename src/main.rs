use is_executable::IsExecutable;
use std::collections::{HashMap, HashSet};
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::PathBuf;

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
                println!("{command}: command not found")
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

    if let Some(name) = words.get(1) {
        if keywords.contains(name) {
            println!("{name} is a shell builtin");
        } else {
            let system_path = std::env::var_os("PATH");

            if let Some(path) = system_path {
                let path_list: Vec<PathBuf> = std::env::split_paths(&path).collect();

                for path_item in &path_list {
                    let read_dir_result = fs::read_dir(path_item);

                    if let Ok(read_dir_value) = read_dir_result {
                        for entry in read_dir_value {
                            if let Ok(entry_result) = entry {
                                let file_path = entry_result.path();
                                if file_path.ends_with(name) && file_path.is_executable() {
                                    println!("{name} is {:?}", &file_path);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
            println!("{name}: not found")
        }
    } else {
        println!(": not found")
    }
}
