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
    let first_space = command.find(' ');

    let command_name: &str = if let Some(i) = first_space {
        &command[0..i].trim()
    }
    else {
        &command[0..].trim()
    };

    let command_argument: &str = if let Some(i) = first_space {
        &command[i..].trim()
    }
    else {
        &""
    };

    match command_name {
        "echo" => println!("{command_argument}"),
        _ => println!("{command}: command not found")
    };
}
