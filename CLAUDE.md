# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Conversation Guidelines

- 常に日本語で会話する


## Project Overview

path-finder is a Rust-based CLI tool that extracts Spring Framework endpoint information from Java and Kotlin source files using tree-sitter for syntax parsing. It supports complex inheritance chains and outputs results in both human-readable and JSON formats.

## Common Commands

### Build and Run
```bash
cargo build --release
cargo run -- scan-directory <directory_path>
cargo run -- scan-directory <directory_path> --json
```

### Testing
```bash
cargo test
cargo test -- --nocapture  # Run tests with output
```

### Development
```bash
cargo check  # Quick syntax check
cargo clippy  # Linting
cargo fmt     # Format code
```

## Architecture

### Core Components
- **lib.rs**: Main library interface with `scan_directory()` and `scan_directory_json()` functions
- **java.rs**: Java-specific parsing logic using tree-sitter-java
- **kotlin.rs**: Kotlin-specific parsing logic using tree-sitter-kotlin-sg  
- **main.rs**: CLI interface using clap for argument parsing

### Key Features
- **Inheritance Support**: Handles single and multi-level inheritance chains with loop prevention
- **Dual Language Support**: Parses both Java and Kotlin Spring controllers
- **Tree-sitter Integration**: Uses tree-sitter parsers for robust syntax analysis
- **Output Formats**: Supports both colored terminal output and JSON for tooling integration

### Data Structures
- `Endpoint`: Core struct containing HTTP method, path, class/method names, parameters, and file location
- `Parameter`: Represents method parameters with type and annotation information

### Inheritance Processing
The tool implements queue-based inheritance traversal to handle complex inheritance chains:
- Processes all parent classes recursively
- Combines `@RequestMapping` paths from child classes with parent method paths
- Maintains visited class tracking to prevent infinite loops
- Works identically for both Java and Kotlin codebases

## Testing Strategy
Tests are located in `tests/` with sample Java/Kotlin files in `tests/resources/` and `tests/resources_inherit/` for inheritance scenarios.