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