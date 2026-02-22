# Rust Shell (CodeCrafters Challenge)

This repository contains my implementation of the
["Build Your Own Shell" challenge on CodeCrafters](https://app.codecrafters.io/courses/shell/overview),
written in **Rust**.

The goal of the challenge is to build a shell that can:
- read and parse user commands,
- execute builtin and external programs,
- support piping and output redirection,
- maintain command history,
- and behave like a small POSIX-style interactive shell.

## Project Overview

The shell runs as a REPL loop from `src/main.rs`:
1. Read input from the terminal.
2. Parse commands (including quotes/escapes).
3. Resolve builtins or external executables.
4. Execute command pipelines.
5. Process stdout/stderr and optional redirection.

### Builtins implemented

- `echo` (including `-e` escape interpretation)
- `cd`
- `pwd`
- `exit`
- `type`
- `history` (print/read/write/append modes)
- `dir` (directory listing)

### Shell features

- Interactive prompt with line editing (left/right/backspace)
- Command history navigation with Up/Down arrows
- Tab completion for known commands
- Command parsing with support for:
   - single quotes `'...'`
   - double quotes `"..."`
   - escape sequences with `\`
- Pipelines with `|`
- Output redirection:
   - stdout: `>` and `>>`
   - stderr: `2>` and `2>>`

## Repository Structure

- `src/main.rs`: main REPL and command dispatch
- `src/parser.rs`: input and path parsing
- `src/os.rs`: executable discovery and OS helpers
- `src/commands/`: builtin commands + external run helper
- `src/shell/input.rs`: interactive terminal input behavior
- `src/shell/output.rs`: stdout/stderr processing and file redirection

## Running Locally

### Requirements

- Rust toolchain (Cargo)

### Run the shell

Use the CodeCrafters wrapper script:

```sh
./your_program.sh
```

Or run directly with Cargo:

```sh
cargo run
```

## Submitting to CodeCrafters

After making changes:

```sh
git add .
git commit -m "your message"
git push origin master
```

CodeCrafters will run the stage tests automatically and stream the output.

## Notes

This project started from the official Rust starter template provided by
CodeCrafters and was incrementally extended while progressing through the
challenge stages.
