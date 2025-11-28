# Agent Guidelines for Standby Project

## Build/Test Commands
- **Build**: `cargo build` or `cargo b`
- **Check**: `cargo check` or `cargo c` (fast compilation check)
- **Test all**: `cargo test` or `cargo t`
- **Single test**: `cargo test <test_name>` (matches test function names containing the string)
- **Format**: `cargo fmt` (auto-formats code)
- **Lint**: `cargo clippy` (catches common mistakes)

## Code Style Guidelines

### Imports & Structure
- Group imports by crate, then std, then external dependencies
- Use explicit imports over glob imports (`use std::io;` not `use std::*;`)
- Sort imports alphabetically within groups

### Naming Conventions
- **Functions/Methods**: snake_case (`create_gradient_bar`)
- **Variables**: snake_case (`device_name`, `current_db`)
- **Structs/Enums**: PascalCase (`AppState`, `Args`)
- **Constants**: SCREAMING_SNAKE_CASE (not used in current codebase)

### Types & Error Handling
- Use `Result<T, Box<dyn std::error::Error>>` for main functions
- Prefer explicit types over inference for public APIs
- Use `?` operator for error propagation
- Use `Arc<Mutex<T>>` for shared mutable state in async contexts

### Async Programming
- Use `#[tokio::main]` for main function
- Use `tokio::select!` for concurrent operations
- Use `tokio::time::interval` for periodic tasks

### Testing
- Place unit tests in `#[cfg(test)]` modules at file end
- Use descriptive test function names (`test_parse_args_no_device`)
- Test both success and error cases

### Dependencies
- CLI: clap with derive feature
- Audio: cpal
- Async: tokio with full features
- TUI: ratatui + crossterm