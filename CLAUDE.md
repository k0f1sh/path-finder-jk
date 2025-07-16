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

### 個別テストの実行
```bash
# 特定のテストファイルのみ実行
cargo test inheritance_test
cargo test class_path_test
cargo test scan_directory_test

# 特定のテスト関数のみ実行
cargo test test_single_inheritance
cargo test test_multi_level_inheritance
```

## Architecture

### Core Components
- **lib.rs**: Main library interface with `scan_directory()` and `scan_directory_json()` functions
- **java.rs**: Java-specific parsing logic using tree-sitter-java
- **kotlin.rs**: Kotlin-specific parsing logic using tree-sitter-kotlin-sg  
- **main.rs**: CLI interface using clap for argument parsing

### 重要な実装詳細
- **継承処理**: `InheritanceTask`構造体を使用したキューベースの継承追跡
- **クロス言語対応**: Java/Kotlin間の継承関係も正しく処理
- **エラーハンドリング**: Spring標準クラスの警告除外リスト（`should_warn_about_missing_parent`）
- **パフォーマンス**: tree-sitterパーサーの再利用とメモリ効率的な処理

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

### テストファイル構成
- `scan_directory_test.rs`: 基本的なディレクトリスキャン機能のテスト
- `inheritance_test.rs`: 継承関係の処理テスト（単一継承、多重継承、クロス言語継承）
- `class_path_test.rs`: クラス名とファイル名の不一致処理テスト
- `tests/resources/`: 基本的なテスト用Javaファイル
- `tests/resources_inherit/`: 継承関係のテスト用ファイル（Java/Kotlin）
- `tests/resources_class_path/`: クラス名検証のテスト用ファイル

### 主要なテストリソース
- 単一継承: `BaseController` → `ChildController`
- 多重継承: `GrandParentController` → `BaseController` → `ChildController`
- クロス言語継承: Java親クラス → Kotlin子クラス（またはその逆）
- エラーケース: 存在しない親クラス、ファイル名とクラス名の不一致