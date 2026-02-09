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
            _ => println!("{user_input}: command not found")
        };
    }
}
