#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Invalid input");

        let user_input = user_input.trim();

        match user_input {
            "exit" => break,
            _ => process_comman(user_input),
        };
    }
}
fn process_comman(command: &str) {
    let words: Vec<&str> = command.split(' ').collect();

    if let Some(&command_name) = words.first() {
        match command_name {
            "echo" => println!("{}", words[1..].join(" ")),
            _ => println!("{command}: command not found"),
        };
    }
}
