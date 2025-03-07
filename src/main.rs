use anyhow::{Context, Result};
use clap::{self, Parser, Subcommand};
use colored::Colorize;
use std::fs;
use tree_sitter::{Language, Parser as TreeSitterParser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Javaファイルをパースする
    ParseJava {
        /// パースするファイルパス
        #[arg(default_value = "samples/UserController.java")]
        file_path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ParseJava { file_path }) => {
            parse_java_file(&file_path)?;
        }
        None => {
            println!(
                "サブコマンドが指定されていません。`parse-java`サブコマンドを試してください。"
            );
        }
    }

    Ok(())
}

fn parse_java_file(file_path: &str) -> Result<()> {
    println!("{}ファイルをパースします...", file_path.blue());

    // setup parser
    let mut parser = TreeSitterParser::new();

    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java parser");

    // parse file
    let source_code = fs::read_to_string(file_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", file_path))?;
    let tree = parser
        .parse(&source_code, None)
        .expect("パースに失敗しました");

    // analyze tree
    let root_node = tree.root_node();
    println!("\n{}:\n{}", "構文ツリー".green(), root_node.to_sexp());

    Ok(())
}
