mod commands;
mod parser;
mod shell;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::terminal::ClearType;
// use owo_colors::OwoColorize;
use crossterm::{cursor, event, execute, terminal};
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::shell::{CommandInput, CommandOutput, processor};

enum OutputProcessor {
    Console,
    StdoutToFile(PathBuf, bool),
    StderrToFile(PathBuf, bool),
}

fn main() {
    let mut output_processor;
    let mut current_dir: PathBuf = env::current_dir().unwrap_or_default();

    let mut commands: HashMap<&str, fn(CommandInput) -> CommandOutput> = HashMap::new();
    commands.insert("echo", commands::echo);
    commands.insert("exit", commands::exit);
    commands.insert("pwd", commands::pwd);
    commands.insert("cd", commands::cd);
    commands.insert("dir", commands::ls);
    commands.insert("type", commands::type_fn);

    let know_commands = commands.keys().map(|i| i.to_string()).collect();

    loop {
        let user_input = retrieve_user_input(&know_commands);
        let command = user_input.trim();
        let words = parser::parse_input(command);

        match processor::define_output_processor(&command, &current_dir) {
            Ok(processor) => output_processor = processor,
            Err(message) => {
                println!("{}", message);
                continue;
            }
        };

        if let Some(command_name) = words.first() {
            let action_requested = commands.get(&command_name.as_str());

            let input = CommandInput {
                command_name: command_name.as_str(),
                command_arguments: &words[1..],
                current_dir: &current_dir,
            };

            let result = if let Some(action) = action_requested {
                action(input)
            } else {
                commands::run_program(input)
            };

            // process results
            if let Some(path) = result.updated_dir {
                current_dir = path;
            }

            processor::process_output(&output_processor, result.std_output, result.std_error);
        }
    }
}

fn retrieve_user_input(know_commands: &Vec<String>) -> String {
    let prompt = "$ ";
    print!("{prompt}");
    io::stdout().flush().unwrap();

    terminal::enable_raw_mode().unwrap();
    let mut user_input = String::new(); // what the user has typed so far
    let mut cursor_pos: usize = 0; // cursor position in the string

    loop {
        let event = event::read().unwrap();

        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            ..
        }) = event
        {
            match code {
                KeyCode::Char(c) => {
                    user_input.insert(cursor_pos, c);
                    cursor_pos += 1;
                    redraw_line(prompt, &user_input, cursor_pos);
                }
                KeyCode::Left => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                    }
                    redraw_line(prompt, &user_input, cursor_pos);
                }
                KeyCode::Right => {
                    if cursor_pos < user_input.len() {
                        cursor_pos += 1;
                    }
                    redraw_line(prompt, &user_input, cursor_pos);
                }
                KeyCode::Backspace => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        user_input.remove(cursor_pos);
                        redraw_line(prompt, &user_input, cursor_pos);
                    }
                }
                KeyCode::Tab => {
                    let found_command: Vec<&String> = know_commands.iter().filter(|i| i.starts_with(&user_input)).collect();
                    let found_command = found_command.first();

                    if let Some(command_name) = found_command {
                        user_input = format!("{command_name} ");
                        cursor_pos = user_input.len();
                        redraw_line(prompt, &user_input, cursor_pos);
                    }
                }
                KeyCode::Enter => {
                    print!("\r\n");
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
    }

    terminal::disable_raw_mode().unwrap();

    user_input
}

fn redraw_line(prompt: &str, input: &str, cursor_pos: usize) {
    // print content
    let mut stdout = io::stdout();

    print!("\r");
    execute!(stdout, terminal::Clear(ClearType::CurrentLine)).unwrap();
    print!("{prompt}{input}");

    // position cursos
    print!("\r");
    let target_col = prompt.len() + cursor_pos;
    if target_col > 0 {
        execute!(stdout, cursor::MoveRight(target_col as u16)).unwrap();
    }

    stdout.flush().unwrap();
}
