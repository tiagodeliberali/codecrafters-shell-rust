use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{OutputProcessor, parser};

pub fn define_output_processor(command: &str, current_dir: &Path) -> Result<OutputProcessor, String> {
    let find_position = command.find('>');

    if let Some(position) = find_position {
        // chgeck byte after the '>' position to see if it is an append operation, with another '>'
        let append_operation = if let Some(&byte_value) = position
            .checked_add(1)
            .and_then(|i| command.as_bytes().get(i))
        {
            byte_value == b'>'
        } else {
            false
        };

        let inital_argument_position = if append_operation {
            position + 2
        } else {
            position + 1
        };

        match parser::parse_path(command[inital_argument_position..].trim(), current_dir) {
            Ok(path) => {
                if let Some(&byte_value) = position
                    .checked_sub(1)
                    .and_then(|i| command.as_bytes().get(i))
                {
                    if byte_value == b'2' {
                        Ok(OutputProcessor::StderrToFile(path, append_operation))
                    } else {
                        Ok(OutputProcessor::StdoutToFile(path, append_operation))
                    }
                } else {
                    Ok(OutputProcessor::StdoutToFile(path, append_operation))
                }
            }
            Err(message) => Err(format!("Invalid redirect output operation: {message}")),
        }
    } else {
        Ok(OutputProcessor::Console)
    }
}

pub fn process_output(
    output_processor: &OutputProcessor,
    std_output: Option<String>,
    std_error: Option<String>,
    last_piped_command: bool,
) {
    match *output_processor {
        OutputProcessor::Console => {
            if last_piped_command && let Some(msg) = std_output {
                println!("{}", msg.trim_end_matches('\n'));
            }

            if let Some(msg) = std_error {
                println!("{}", msg);
            }
        }
        OutputProcessor::StdoutToFile(ref output_path, append) => {
            write_output_to_file(output_path, std_output, append);

            if let Some(msg) = std_error {
                println!("{}", msg);
            }
        }
        OutputProcessor::StderrToFile(ref output_path, append) => {
            if let Some(msg) = std_output {
                println!("{msg}");
            }

            write_output_to_file(output_path, std_error, append);
        }
    }
}

fn write_output_to_file(output_path: &PathBuf, content: Option<String>, append: bool) {
    let content = ensure_trailing_newline(content);

    let result = if append {
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_path)
        {
            Ok(file) => {
                write!(&file, "{}", content)
            }
            Err(error) => Err(error),
        }
    } else {
        fs::write(output_path, content)
    };

    if let Err(error) = result {
        println!("Failed to write output file: {error}");
    }
}

fn ensure_trailing_newline(content: Option<String>) -> String {
    match content {
        None => String::new(),
        Some(s) if s.is_empty() => s,
        Some(s) if s.ends_with('\n') => s,
        Some(s) => format!("{s}\n"),
    }
}
