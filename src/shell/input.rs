use std::{
    collections::HashSet,
    io::{self, Write},
};

use crossterm::{
    cursor,
    event::{
        self, DisableBracketedPaste, EnableBracketedPaste, Event, KeyCode, KeyEvent, KeyEventKind,
        KeyModifiers,
    },
    execute,
    terminal::{self, ClearType},
};

pub fn retrieve_user_input(know_commands: &HashSet<String>, command_history: &Vec<String>) -> String {
    let prompt = "$ ";
    print!("{prompt}");
    io::stdout().flush().unwrap();

    terminal::enable_raw_mode().unwrap();
    execute!(io::stdout(), EnableBracketedPaste).unwrap();

    let mut user_input = String::new(); // what the user has typed so far
    let mut cursor_pos: usize = 0; // cursor position in the string
    let mut one_tab_pressed = false;
    let mut current_history_position = command_history.len().saturating_sub(1);

    loop {
        let event = event::read().unwrap();

        if let Event::Paste(text) = &event {
            for c in text.chars() {
                if c == '\n' || c == '\r' {
                    continue;
                } // skip newlines in pasted text
                user_input.insert(cursor_pos, c);
                cursor_pos += 1;
            }
            redraw_line(prompt, &user_input, cursor_pos);
            continue;
        }

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
                KeyCode::Up => {
                    if command_history.len() > 0 {
                        user_input = command_history.get(current_history_position).unwrap().clone();
                        cursor_pos = user_input.len();
                        redraw_line(prompt, &user_input, cursor_pos);

                        if  current_history_position == 0 {
                            current_history_position = command_history.len().saturating_sub(1);
                        } else {
                            current_history_position -= 1;
                        }
                    }
                }
                KeyCode::Char(c) => {
                    one_tab_pressed = false;
                    user_input.insert(cursor_pos, c);
                    cursor_pos += 1;
                    redraw_line(prompt, &user_input, cursor_pos);
                }
                KeyCode::Left => {
                    cursor_pos = cursor_pos.saturating_sub(1);
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
                    let found_commands: Vec<&String> = know_commands
                        .iter()
                        .filter(|i| i.starts_with(&user_input))
                        .collect();

                    if found_commands.is_empty() {
                        redraw_line(prompt, &format!("{}\x07", user_input), cursor_pos); // beep!
                    } else if found_commands.len() == 1 {
                        let command_name = found_commands.first().unwrap();
                        user_input = format!("{command_name} ");
                        cursor_pos = user_input.len();
                        redraw_line(prompt, &user_input, cursor_pos);
                    } else {
                        let mut names: Vec<&str> =
                            found_commands.iter().map(|s| s.as_str()).collect();
                        names.sort();

                        if !one_tab_pressed {
                            one_tab_pressed = true;

                            user_input = build_lcp(&names, &user_input);
                            cursor_pos = user_input.len();

                            redraw_line(prompt, &format!("{}\x07", user_input), cursor_pos); // beep!
                        } else {
                            one_tab_pressed = false;
                            redraw_line(prompt, &user_input.to_string(), cursor_pos);
                            print!("\r\n{}\r\n", names.join("  "));
                            redraw_line(prompt, &user_input.to_string(), cursor_pos);
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

    execute!(io::stdout(), DisableBracketedPaste).unwrap();
    terminal::disable_raw_mode().unwrap();

    user_input
}

fn build_lcp(names: &Vec<&str>, user_input: &str) -> String {
    let mut lcp = names.first().unwrap().to_string();

    for i in 1..(names.len() - 1) {
        let word = names.get(i).unwrap();

        let len = lcp.len().min(word.len());

        if len == user_input.len() {
            lcp = user_input.to_string();
            break;
        }

        let mut last_equal = user_input.len();

        for j in user_input.len()..len {
            if lcp.chars().nth(j) == word.chars().nth(j) {
                last_equal += 1;
            } else {
                break;
            }
        }
        lcp = lcp[..last_equal].to_string();
    }

    lcp
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
