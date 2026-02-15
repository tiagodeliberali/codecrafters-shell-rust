use std::{
    env,
    path::{Component, Path, PathBuf},
};

enum Parser {
    SingleQuote,
    DoubleQuote,
    Escape,
    EscapeInDoubleQuote,
    None,
}

pub fn parse_input(argument: &str) -> Vec<String> {
    let mut arguments: Vec<String> = Vec::new();
    let mut current_argument = String::new();

    let mut current_parser = Parser::None;

    for character in argument.chars() {
        match current_parser {
            Parser::None => {
                if character == '\'' {
                    current_parser = Parser::SingleQuote;
                } else if character == '"' {
                    current_parser = Parser::DoubleQuote;
                } else if character == '\\' {
                    current_parser = Parser::Escape;
                } else if character == ' ' {
                    if !current_argument.is_empty() {
                        if matches!(current_argument.as_str(), ">" | "1>" | "2>") {
                            current_argument.clear();
                            break; // redirect stout argument
                        }
                        arguments.push(current_argument.clone());
                        current_argument.clear();
                    }
                } else {
                    current_argument.push(character);
                }
            }
            Parser::DoubleQuote => {
                if character == '"' {
                    current_parser = Parser::None;
                } else if character == '\\' {
                    current_parser = Parser::EscapeInDoubleQuote;
                } else {
                    current_argument.push(character);
                }
            }
            Parser::SingleQuote => {
                if character == '\'' {
                    current_parser = Parser::None;
                } else {
                    current_argument.push(character);
                }
            }
            Parser::Escape => {
                current_argument.push(character);
                current_parser = Parser::None;
            }
            Parser::EscapeInDoubleQuote => {
                if matches!(character, '"' | '\\') {
                    current_argument.push(character);
                } else {
                    current_argument.push('\\');
                    current_argument.push(character);
                }

                current_parser = Parser::DoubleQuote;
            }
        }
    }

    if !current_argument.is_empty() {
        arguments.push(current_argument);
    }

    arguments
}

pub fn parse_path(path: &str, current_dir: &Path) -> Result<PathBuf, String> {
    let path = if path.starts_with("~") {
        let Some(home_dir) = env::var("HOME").ok().map(PathBuf::from) else {
            return Err(String::from("HOME directory not defined."));
        };
        path.replacen("~", &home_dir.display().to_string(), 1)
    } else {
        path.to_string()
    };

    let mut target_dir = PathBuf::from(current_dir);
    let pathbuf_dir = PathBuf::from(&path);

    for path_component in pathbuf_dir.components() {
        match path_component {
            Component::RootDir | Component::Prefix(_) => {
                target_dir = PathBuf::from(&path);
                break;
            }
            Component::ParentDir => {
                target_dir.pop();
            }
            Component::Normal(value) => {
                target_dir.push(value);
            }
            Component::CurDir => continue,
        }
    }

    Ok(target_dir)
}
