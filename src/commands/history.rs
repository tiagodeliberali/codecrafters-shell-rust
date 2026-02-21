use std::error::Error;
use std::io::Write;
use std::{
    fs::{self, OpenOptions},
    num::ParseIntError,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::shell::{CommandInput, CommandOutput};

static LAST_APPENDED_INDEX: AtomicUsize = AtomicUsize::new(0);

pub fn load_history() -> Vec<String> {
    if let Some(path) = std::env::var_os("HISTFILE") {
        match read_path_file(path.to_str().unwrap_or_default()) {
            Ok(result) => return result,
            Err(_) => Vec::new()
        }
    } else {
        Vec::new()
    }
}

pub fn history(input: CommandInput) -> CommandOutput {
    if let Some(arg) = input.command_arguments.first()
        && matches!(arg.as_str(), "-r" | "-w" | "-a")
    {
        let path = if let Some(value) = input.command_arguments.get(1) {
            value
        } else {
            return CommandOutput::failure("missing path".to_string());
        };

        let result = if arg == "-r" {
            read_file_to_output(path)
        } else if arg == "-w" {
            write_lines_to_file(path, input.command_history)
        } else if arg == "-a" {
            append_content(path, input.command_history)
        } else {
            Ok(CommandOutput::empty()) // need to return Err instead of ok. Unexpected situation.
        };

        match result {
            Ok(output) => return output,
            Err(error) => return CommandOutput::failure(error.to_string()),
        }
    }

    let size: Option<Result<usize, ParseIntError>> =
        input.command_arguments.first().map(|s| s.as_str().parse());

    let (initial_value, enumeration) = match size {
        None => (1, input.command_history.iter().enumerate()),
        Some(value) => {
            let value = match value {
                Err(error) => {
                    return CommandOutput::failure(format!(
                        "Failed to parse history argument: {}",
                        error
                    ));
                }
                Ok(v) => v,
            };

            let start_position: usize = input.command_history.len().saturating_sub(value);
            (
                start_position + 1,
                input.command_history[start_position..].iter().enumerate(),
            )
        }
    };

    let mut output = String::new();
    for (position, command) in enumeration {
        output += format!("{} {}\n", (position + initial_value), command).as_str();
    }
    CommandOutput::success(output)
}

fn read_file_to_output(path: &str) -> Result<CommandOutput, Box<dyn Error>> {
    let paths = read_path_file(path)?;
    Ok(CommandOutput::history_update(paths))
}

fn read_path_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let result = fs::read_to_string(path)?;

    let mut paths: Vec<String> = Vec::new();

    for line in result.lines() {
        if !line.is_empty() {
            paths.push(line.to_string());
        }
    }

    Ok(paths)
}

fn write_lines_to_file(path: &str, content: &[String]) -> Result<CommandOutput, Box<dyn Error>> {
    fs::write(path, format!("{}\n", content.join("\n")))?;
    Ok(CommandOutput::empty())
}

fn append_content(path: &str, content: &[String]) -> Result<CommandOutput, Box<dyn Error>> {
    if LAST_APPENDED_INDEX.load(Ordering::Relaxed) >= content.len() {
        return Ok(CommandOutput::empty());
    }

    let append_content = &content[LAST_APPENDED_INDEX.load(Ordering::Relaxed)..];
    LAST_APPENDED_INDEX.store(content.len(), Ordering::Relaxed);

    if !fs::exists(path)? {
        write_lines_to_file(path, append_content)
    } else {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(&file, "{}", append_content.join("\n"))?;
        Ok(CommandOutput::empty())
    }
}
