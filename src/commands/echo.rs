use crate::shell::{CommandInput, CommandOutput};

pub fn echo(input: CommandInput) -> CommandOutput {
    let args = input.command_arguments;

    if args.first().is_some_and(|a| a == "-e") {
        let text = args[1..].join(" ");
        CommandOutput::success(interpret_escapes(&text))
    } else {
        CommandOutput::success(args.join(" "))
    }
}

fn interpret_escapes(input: &str) -> String {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut result = String::with_capacity(len);
    let mut i = 0;

    while i < len {
        if bytes[i] == b'\\' && i + 1 < len {
            match bytes[i + 1] {
                b'n' => { result.push('\n'); i += 2; }
                b't' => { result.push('\t'); i += 2; }
                b'\\' => { result.push('\\'); i += 2; }
                b'a' => { result.push('\x07'); i += 2; }
                b'b' => { result.push('\x08'); i += 2; }
                b'r' => { result.push('\r'); i += 2; }
                b'c' => break, // \c stops all output
                b'0' => {
                    // octal: \0nnn (up to 3 octal digits after the 0)
                    i += 2;
                    let mut octal = String::new();
                    for _ in 0..3 {
                        if i < len && bytes[i] >= b'0' && bytes[i] <= b'7' {
                            octal.push(bytes[i] as char);
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    let value = u8::from_str_radix(&octal, 8).unwrap_or(0);
                    result.push(value as char);
                }
                b'x' => {
                    // hex: \xHH (up to 2 hex digits)
                    i += 2;
                    let mut hex = String::new();
                    for _ in 0..2 {
                        if i < len && (bytes[i] as char).is_ascii_hexdigit() {
                            hex.push(bytes[i] as char);
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    if let Ok(value) = u8::from_str_radix(&hex, 16) {
                        result.push(value as char);
                    } else {
                        result.push_str("\\x");
                        result.push_str(&hex);
                    }
                }
                other => {
                    result.push('\\');
                    result.push(other as char);
                    i += 2;
                }
            }
        } else {
            result.push(bytes[i] as char);
            i += 1;
        }
    }

    result
}