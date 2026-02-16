use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};

pub fn retrieve_user_input(know_commands: &Vec<String>) -> String {
    let prompt = "$ ";
    print!("{prompt}");
    io::stdout().flush().unwrap();

    terminal::enable_raw_mode().unwrap();
    let mut user_input = String::new(); // what the user has typed so far
    let mut cursor_pos: usize = 0; // cursor position in the string
    let mut one_tab_pressed = false;

    loop {
        let event = event::read().unwrap();

        if let Event::Key(KeyEvent {
            code,
            kind: KeyEventKind::Press,
            modifiers,
            ..
        }) = event
        {
            match code {
                // In raw mode, \n (0x0A) is mapped to Ctrl+J instead of Enter
                KeyCode::Char('j') if modifiers.contains(KeyModifiers::CONTROL) => {
                    print!("\r\n");
                    break;
                }
                KeyCode::Char(c) => {
                    one_tab_pressed = false;
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
                    one_tab_pressed = false;
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        user_input.remove(cursor_pos);
                        redraw_line(prompt, &user_input, cursor_pos);
                    }
                }
                KeyCode::Tab => {
                    let found_command: Vec<&String> = know_commands
                        .iter()
                        .filter(|i| i.starts_with(&user_input))
                        .collect();

                    if found_command.is_empty() {
                        redraw_line(prompt, &format!("{}\x07", user_input), cursor_pos); // beep!
                    } else if found_command.len() == 1 {
                        let command_name = found_command.first().unwrap();
                        user_input = format!("{command_name} ");
                        cursor_pos = user_input.len();
                        redraw_line(prompt, &user_input, cursor_pos);
                    } else {
                        if !one_tab_pressed {
                            one_tab_pressed = true;
                            redraw_line(prompt, &format!("{}\x07", user_input), cursor_pos); // beep!
                        } else {
                            one_tab_pressed = false;
                            redraw_line(prompt, &format!("{}", user_input), cursor_pos);
                            let names: Vec<&str> = found_command.iter().map(|s| s.as_str()).collect();
                            println!("\r\n{}\r\n", names.join("  "));
                            redraw_line(prompt, &format!("{}", user_input), cursor_pos);
                        }
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
