use anyhow::{Context, Result};
use clap::{self, Parser, Subcommand};
use colored::Colorize;
use std::fs;
use tree_sitter::{Parser as TreeSitterParser, Query, QueryCursor};

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
    /// RequestMappingアノテーションがついているクラスを抽出する
    ExtractRequestMapping {
        /// パースするファイルパス
        #[arg(default_value = "samples/UserController.java")]
        file_path: String,
    },
    /// ディレクトリ内のJavaファイルからRequestMappingアノテーションがついているクラスを抽出する
    ScanDirectory {
        /// スキャンするディレクトリパス
        #[arg(default_value = "samples")]
        dir_path: String,
    },
}

// エンドポイント情報を格納する構造体
#[derive(Debug)]
struct Endpoint {
    class_name: String,
    method_name: String,
    http_method: String,
    path: String,
    parameters: Vec<Parameter>,
    line_range: (usize, usize),
}

#[derive(Debug)]
struct Parameter {
    name: String,
    param_type: String,
    annotation: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ParseJava { file_path }) => {
            parse_java_file(&file_path)?;
        }
        Some(Commands::ExtractRequestMapping { file_path }) => {
            extract_request_mapping(&file_path)?;
        }
        Some(Commands::ScanDirectory { dir_path }) => {
            scan_directory(&dir_path)?;
        }
        None => {
            println!(
                "サブコマンドが指定されていません。`parse-java`、`extract-request-mapping`、または`scan-directory`サブコマンドを試してください。"
            );
        }
    }

    Ok(())
}

fn scan_directory(dir_path: &str) -> Result<()> {
    println!(
        "{}ディレクトリ内のJavaファイルをスキャンします...",
        dir_path.blue()
    );

    let entries = fs::read_dir(dir_path)
        .with_context(|| format!("ディレクトリの読み込みに失敗しました: {}", dir_path))?;

    let mut found_controllers = false;
    let mut all_endpoints = Vec::new();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "java") {
            let file_path = path.to_string_lossy().to_string();
            println!("\n{}: {}", "ファイル".bold(), file_path);

            if has_request_mapping(&file_path)? {
                found_controllers = true;
                let endpoints = extract_request_mapping_with_endpoints(&file_path)?;
                all_endpoints.extend(endpoints);
            } else {
                println!(
                    "  RequestMappingアノテーションがついているクラスは見つかりませんでした。"
                );
            }
        }
    }

    if !found_controllers {
        println!("\n{}ディレクトリ内にRequestMappingアノテーションがついているクラスは見つかりませんでした。", dir_path);
    } else {
        // エンドポイントの概要を表示
        print_endpoints_summary(&all_endpoints);
    }

    Ok(())
}

fn print_endpoints_summary(endpoints: &[Endpoint]) {
    println!("\n{}", "=== エンドポイント概要 ===".bold());

    for endpoint in endpoints {
        let http_method = match endpoint.http_method.as_str() {
            "GetMapping" => "GET".green(),
            "PostMapping" => "POST".yellow(),
            "PutMapping" => "PUT".blue(),
            "DeleteMapping" => "DELETE".red(),
            "PatchMapping" => "PATCH".cyan(),
            _ => endpoint.http_method.normal(),
        };

        println!(
            "{} {} ({}#{}:{})",
            http_method,
            endpoint.path.magenta(),
            endpoint.class_name,
            endpoint.method_name,
            endpoint.line_range.0
        );

        // パラメータがあれば表示
        if !endpoint.parameters.is_empty() {
            print!("  パラメータ: ");
            for (i, param) in endpoint.parameters.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}:{} ({})", param.name, param.param_type, param.annotation);
            }
            println!();
        }
    }
}

fn has_request_mapping(file_path: &str) -> Result<bool> {
    let source_code = fs::read_to_string(file_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", file_path))?;

    // Simple string search for quick check before parsing
    if !source_code.contains("@RequestMapping") {
        return Ok(false);
    }

    let mut parser = TreeSitterParser::new();
    parser
        .set_language(tree_sitter_java::language())
        .expect("Error loading Java parser");

    let tree = parser
        .parse(&source_code, None)
        .expect("パースに失敗しました");

    let query_source = r#"
        (class_declaration
            (modifiers
                (annotation
                    name: (identifier) @annotation_name
                    (#match? @annotation_name "RequestMapping")))
            name: (identifier) @class_name) @class
        
        (class_declaration
            (modifiers
                (marker_annotation
                    name: (identifier) @annotation_name
                    (#match? @annotation_name "RequestMapping")))
            name: (identifier) @class_name) @class
    "#;

    let query = Query::new(tree_sitter_java::language(), query_source).expect("Invalid query");

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    Ok(matches.count() > 0)
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

fn extract_request_mapping(file_path: &str) -> Result<()> {
    let _endpoints = extract_request_mapping_with_endpoints(file_path)?;
    Ok(())
}

fn extract_request_mapping_with_endpoints(file_path: &str) -> Result<Vec<Endpoint>> {
    println!(
        "{}からRequestMappingアノテーションがついているクラスを抽出します...",
        file_path.blue()
    );

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

    // Create a query to find classes with RequestMapping annotations
    let query_source = r#"
        (class_declaration
            (modifiers
                (annotation
                    name: (identifier) @annotation_name
                    (#match? @annotation_name "RequestMapping")))
            name: (identifier) @class_name) @class
        
        (class_declaration
            (modifiers
                (marker_annotation
                    name: (identifier) @annotation_name
                    (#match? @annotation_name "RequestMapping")))
            name: (identifier) @class_name) @class
    "#;

    let query = Query::new(tree_sitter_java::language(), query_source).expect("Invalid query");

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    let mut found = false;
    let mut endpoints = Vec::new();

    for m in matches {
        found = true;
        let mut class_name = "";

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            if capture_name == "class_name" {
                class_name = node_text;
                println!("\n{}: {}", "クラス名".green(), node_text);
            } else if capture_name == "annotation_name" {
                println!("{}: {}", "アノテーション".yellow(), node_text);
            }
        }

        // Get the class node to extract the full class definition
        for capture in m.captures {
            if &query.capture_names()[capture.index as usize] == "class" {
                let class_node = capture.node;
                let start_line = class_node.start_position().row + 1;
                let end_line = class_node.end_position().row + 1;
                println!("{}: {}:{}", "行番号".cyan(), start_line, end_line);

                // Extract the path from the annotation if available
                let base_path = extract_request_mapping_path(&source_code, class_node);

                // Extract method-level mappings
                let method_endpoints = extract_method_mappings_with_endpoints(
                    &source_code,
                    class_node,
                    base_path.as_deref(),
                    class_name,
                );
                endpoints.extend(method_endpoints);

                break;
            }
        }
    }

    if !found {
        println!(
            "{}にRequestMappingアノテーションがついているクラスは見つかりませんでした。",
            file_path
        );
    }

    Ok(endpoints)
}

fn extract_request_mapping_path(
    source_code: &str,
    class_node: tree_sitter::Node,
) -> Option<String> {
    // Create a query to find RequestMapping annotation with path
    let query_source = r#"
        (annotation
            name: (identifier) @annotation_name
            (#match? @annotation_name "RequestMapping")
            arguments: (annotation_argument_list
                (string_literal) @path))
    "#;

    let query = Query::new(tree_sitter_java::language(), query_source).expect("Invalid query");

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, class_node, source_code.as_bytes());

    for m in matches {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            if capture_name == "path" {
                let path_text = &source_code[capture.node.byte_range()];
                println!("{}: {}", "エンドポイントパス".magenta(), path_text);
                return Some(path_text.to_string());
            }
        }
    }

    None
}

fn extract_method_mappings_with_endpoints(
    source_code: &str,
    class_node: tree_sitter::Node,
    base_path: Option<&str>,
    class_name: &str,
) -> Vec<Endpoint> {
    // Create a query to find method-level mapping annotations
    let query_source = r#"
        (method_declaration
            (modifiers
                (marker_annotation
                    name: (identifier) @mapping_type
                    (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping")))
            name: (identifier) @method_name) @method
            
        (method_declaration
            (modifiers
                (annotation
                    name: (identifier) @mapping_type
                    (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping")
                    arguments: (annotation_argument_list
                        (string_literal) @path)))
            name: (identifier) @method_name) @method
    "#;

    let query = Query::new(tree_sitter_java::language(), query_source).expect("Invalid query");

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, class_node, source_code.as_bytes());

    println!("\n{}", "メソッドレベルのマッピング:".bold());
    let mut found = false;
    let mut endpoints = Vec::new();

    for m in matches {
        found = true;
        let mut method_name = "";
        let mut mapping_type = "";
        let mut path = "";
        let mut method_node = None;

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            match capture_name.as_str() {
                "method_name" => method_name = node_text,
                "mapping_type" => mapping_type = node_text,
                "path" => path = node_text,
                "method" => method_node = Some(capture.node),
                _ => {}
            }
        }

        if let Some(node) = method_node {
            let start_line = node.start_position().row + 1;
            let end_line = node.end_position().row + 1;

            println!("\n  {}: {}", "メソッド名".green(), method_name);
            println!("  {}: {}", "マッピングタイプ".yellow(), mapping_type);

            let full_path = if !path.is_empty() {
                match base_path {
                    Some(base) => {
                        let base = base.trim_matches('"');
                        let path = path.trim_matches('"');
                        format!("\"{}{}\"", base, path)
                    }
                    None => path.to_string(),
                }
            } else if let Some(base) = base_path {
                base.to_string()
            } else {
                "".to_string()
            };

            if !full_path.is_empty() {
                println!("  {}: {}", "パス".magenta(), full_path);
            }

            println!("  {}: {}:{}", "行番号".cyan(), start_line, end_line);

            // Extract method parameters
            let parameters = extract_method_parameters_with_data(source_code, node);

            // Create endpoint
            let endpoint = Endpoint {
                class_name: class_name.to_string(),
                method_name: method_name.to_string(),
                http_method: mapping_type.to_string(),
                path: full_path.trim_matches('"').to_string(),
                parameters,
                line_range: (start_line, end_line),
            };

            endpoints.push(endpoint);
        }
    }

    if !found {
        println!("  メソッドレベルのマッピングは見つかりませんでした。");
    }

    endpoints
}

fn extract_method_parameters_with_data(
    source_code: &str,
    method_node: tree_sitter::Node,
) -> Vec<Parameter> {
    // Create a query to find method parameters with annotations
    let query_source = r#"
        (formal_parameter
            (modifiers
                (marker_annotation
                    name: (identifier) @param_annotation
                    (#match? @param_annotation "PathVariable|RequestBody|RequestParam")))
            type: (_) @param_type
            name: (identifier) @param_name) @param
    "#;

    let query = Query::new(tree_sitter_java::language(), query_source).expect("Invalid query");

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, method_node, source_code.as_bytes());

    let mut found = false;
    let mut parameters = Vec::new();

    for m in matches {
        if !found {
            println!("  {}:", "パラメータ".blue());
            found = true;
        }

        let mut param_name = "";
        let mut param_type = "";
        let mut param_annotation = "";

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            match capture_name.as_str() {
                "param_name" => param_name = node_text,
                "param_type" => param_type = node_text,
                "param_annotation" => param_annotation = node_text,
                _ => {}
            }
        }

        println!(
            "    - {}: {} ({})",
            param_annotation, param_name, param_type
        );

        parameters.push(Parameter {
            name: param_name.to_string(),
            param_type: param_type.to_string(),
            annotation: param_annotation.to_string(),
        });
    }

    parameters
}
