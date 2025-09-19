use anyhow::{Context, Result};
use std::fs;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};
use walkdir::WalkDir;

use crate::{Endpoint, Parameter};

// 警告を出さない親クラス名のリスト
fn should_warn_about_missing_parent(parent_class_name: &str) -> bool {
    !matches!(
        parent_class_name,
        // Java標準クラス
        "Object" | "Exception" | "RuntimeException" | "Throwable" |
        "Enum" | "Record" | "Number" | "String" |
        
        // Spring Boot / Spring Framework標準クラス
        "BaseEntity" | "AbstractEntity" | "AbstractAggregateRoot" |
        "JpaRepository" | "CrudRepository" | "Repository" | "PagingAndSortingRepository" |
        "Controller" | "RestController" | "Component" | "Service" |
        "Configuration" | "ConfigurationProperties" |
        
        // JPA / Hibernate標準クラス
        "EntityListener" | "AbstractEntityListener" | "Auditable" |
        "Persistable" | "AbstractAuditable" | "AbstractPersistable" |
        
        // 一般的なライブラリクラス
        "ResponseEntity" | "HttpEntity" | "RequestEntity" |
        "Page" | "Pageable" | "Sort" | "Slice" |
        
        // よくあるベースクラス名
        "BaseController" | "AbstractController" | "BaseService" | "AbstractService" |
        "BaseRepository" | "AbstractRepository" | "BaseDomain" | "AbstractDomain" |
        "BaseDto" | "AbstractDto"
    )
}

// 継承処理用の構造体
#[derive(Debug)]
struct InheritanceTask {
    #[allow(dead_code)]
    child_file_path: String,
    child_class_name: String,
    child_base_path: Option<String>,
    parent_class_name: String,
}

fn create_parser() -> Parser {
    let mut parser = Parser::new();
    let language = tree_sitter_kotlin_sg::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Kotlin parser");
    parser
}

fn create_query(query_source: &str) -> Query {
    let language = tree_sitter_kotlin_sg::LANGUAGE;
    Query::new(&language.into(), query_source).expect("Invalid query")
}

pub fn has_request_mapping(file_path: &str) -> Result<bool> {
    let source_code = fs::read_to_string(file_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", file_path))?;

    // Simple string search for quick check before parsing
    if !source_code.contains("@RequestMapping") {
        return Ok(false);
    }

    let mut parser = create_parser();

    let tree = parser
        .parse(&source_code, None)
        .expect("パースに失敗しました");

    let query_source = r#"
        (class_declaration
            (modifiers
                (annotation (_) @annotation_name
                    (#match? @annotation_name "RequestMapping"))))
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    Ok(matches.count() > 0)
}

fn extract_request_mapping_with_endpoints(
    file_path: &str,
) -> Result<(Vec<Endpoint>, Vec<InheritanceTask>)> {
    // setup parser
    let mut parser = create_parser();

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
                (annotation (_) @annotation_name
                    (#match? @annotation_name "RequestMapping")))
            (type_identifier) @class_name) @class
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    let mut endpoints = Vec::new();
    let mut inheritance_tasks = Vec::new();

    while let Some(m) = matches.next() {
        let mut class_name = "";

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            if *capture_name == "class_name" {
                class_name = node_text;
            }
        }

        // Get the class node to extract the full class definition
        for capture in m.captures {
            if query.capture_names()[capture.index as usize] == "class" {
                let class_node = capture.node;
                // Extract the path from the annotation if available
                let base_path = extract_request_mapping_path(&source_code, class_node);

                // Extract method-level mappings
                let method_endpoints = extract_method_mappings_with_endpoints(
                    &source_code,
                    class_node,
                    base_path.as_deref(),
                    class_name,
                    file_path,
                );
                endpoints.extend(method_endpoints);

                // Check for inheritance and create tasks
                let tasks = check_inheritance_and_create_tasks(
                    &source_code,
                    class_node,
                    class_name,
                    base_path,
                    file_path,
                );
                inheritance_tasks.extend(tasks);

                break;
            }
        }
    }

    Ok((endpoints, inheritance_tasks))
}

fn extract_request_mapping_path(
    source_code: &str,
    class_node: tree_sitter::Node,
) -> Option<String> {
    // Create a query to find RequestMapping annotation with path
    let query_source = r#"
(class_declaration
  (modifiers
    (annotation
      (constructor_invocation
        (user_type) @annotation_name (#match? @annotation_name "RequestMapping")
        (value_arguments (value_argument (string_literal (string_content) @path)))
      ))))
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, class_node, source_code.as_bytes());

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            if capture_name == &"path" {
                let path_text = &source_code[capture.node.byte_range()];
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
    file_path: &str,
) -> Vec<Endpoint> {
    // Create a query to find method-level mapping annotations
    let query_source = r#"
        (function_declaration
            (modifiers
                (annotation
                    (user_type (type_identifier) @mapping_type
                    (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping"))))
             (simple_identifier) @method_name) @method
            
        (function_declaration
            (modifiers
                (annotation
                  (constructor_invocation
                    (user_type (type_identifier) @mapping_type
                      (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping"))
                    (value_arguments (value_argument (string_literal (string_content) @path)))
                    )))
             (simple_identifier) @method_name) @method

        (function_declaration
            (modifiers
                (annotation
                  (constructor_invocation
                    (user_type (type_identifier) @mapping_type
                      (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping|RequestMapping"))
                    (value_arguments (value_argument (simple_identifier) @key
                      (#match? @key "value")
                      (collection_literal (string_literal (string_content) @path))
                    ))
                    )))
             (simple_identifier) @method_name) @method

        (function_declaration
            (modifiers
                (annotation
                  (constructor_invocation
                    (user_type (type_identifier) @mapping_type
                      (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping"))
                    (value_arguments (value_argument (simple_identifier) @key
                      (#match? @key "params")
                      (collection_literal (_))
                    ))
                    )))
             (simple_identifier) @method_name) @method
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, class_node, source_code.as_bytes());

    let mut endpoints = Vec::new();
    let mut processed_methods = std::collections::HashSet::new();

    while let Some(m) = matches.next() {
        let mut method_name = "";
        let mut mapping_type = "";
        let mut path = "";
        let mut method_node = None;

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            match *capture_name {
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
            
            // 重複チェック: 同じメソッド（行番号で判定）が既に処理済みかチェック
            let method_key = format!("{}:{}", method_name, start_line);
            if processed_methods.contains(&method_key) {
                continue;
            }
            processed_methods.insert(method_key);

            // RequestMappingの場合はmethod属性を調べる
            let http_method = if mapping_type == "RequestMapping" {
                extract_request_mapping_method(source_code, node)
            } else {
                // 他のマッピングタイプはそのままHTTPメソッドに変換
                mapping_type_to_http_method(mapping_type)
            };

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

            let parameters = extract_method_parameters_with_data(source_code, node);
            let headers = extract_method_headers_with_data(source_code, node);
            let params = extract_method_params_with_data(source_code, node);

            endpoints.push(Endpoint {
                class_name: class_name.to_string(),
                method_name: method_name.to_string(),
                http_method,
                path: full_path.trim_matches('"').to_string(),
                parameters,
                line_range: (start_line, end_line),
                file_path: file_path.to_string(),
                headers: headers.to_string(),
                params: params.to_string(),
            });
        }
    }

    endpoints
}

fn mapping_type_to_http_method(mapping_type: &str) -> String {
    match mapping_type {
        "GetMapping" => "GET".to_string(),
        "PostMapping" => "POST".to_string(),
        "PutMapping" => "PUT".to_string(),
        "DeleteMapping" => "DELETE".to_string(),
        "PatchMapping" => "PATCH".to_string(),
        _ => "ANY".to_string(),
    }
}

fn extract_request_mapping_method(source_code: &str, method_node: tree_sitter::Node) -> String {
    // Create a query to find RequestMapping annotation with method attribute
    let query_source = r#"
                (annotation
                  (constructor_invocation
                    (user_type (type_identifier) @mapping_type
                      (#match? @mapping_type "RequestMapping"))
                    (value_arguments
                      (value_argument (simple_identifier) @key (#match? @key "method")
                        (collection_literal
                         (navigation_expression (navigation_suffix (simple_identifier) @http_method)))
                    ))
                    ))
                (annotation
                  (constructor_invocation
                    (user_type (type_identifier) @mapping_type
                      (#match? @mapping_type "RequestMapping"))
                    (value_arguments
                      (value_argument (simple_identifier) @key (#match? @key "method")
                        (collection_literal
                         (simple_identifier) @http_method)
                    ))
                    ))
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, method_node, source_code.as_bytes());

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            if capture_name == &"http_method" {
                let method_text = &source_code[capture.node.byte_range()];
                return method_text.to_string();
            }
        }
    }

    // Default to ANY if method is not specified
    "ANY".to_string()
}

fn extract_method_parameters_with_data(
    source_code: &str,
    method_node: tree_sitter::Node,
) -> Vec<Parameter> {
    // Create a query to find method parameters with annotations
    let query_source = r#"
(function_declaration
  (function_value_parameters
    (parameter_modifiers (annotation (user_type (type_identifier) @param_annotation)))
    (parameter
      (simple_identifier) @param_name
      (user_type (type_identifier) @param_type)))) @param
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, method_node, source_code.as_bytes());

    let mut parameters = Vec::new();

    while let Some(m) = matches.next() {
        let mut param_name = "";
        let mut param_type = "";
        let mut param_annotation = "";

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            match *capture_name {
                "param_name" => param_name = node_text,
                "param_type" => param_type = node_text,
                "param_annotation" => param_annotation = node_text,
                _ => {}
            }
        }

        if !param_name.is_empty() && !param_type.is_empty() {
            parameters.push(Parameter {
                name: param_name.to_string(),
                param_type: param_type.to_string(),
                annotation: param_annotation.to_string(),
            });
        }
    }

    parameters
}

fn extract_method_headers_with_data(source_code: &str, method_node: tree_sitter::Node) -> String {
    // Create a more flexible query to find headers attribute regardless of order
    let query_source = r#"
        (annotation
          (constructor_invocation
            (user_type (type_identifier) @mapping_type
              (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping|RequestMapping"))
            (value_arguments (value_argument (simple_identifier) @key
              (#match? @key "headers")
              (collection_literal (_) @headers)
            ))
            ))
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, method_node, source_code.as_bytes());

    let mut headers = "";
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            if capture_name == &"headers" { 
                headers = node_text;
                break; // 最初に見つかったheadersを使用
            }
        }
        if !headers.is_empty() {
            break; // headers が見つかったらループを抜ける
        }
    }

    headers.to_string()
}

fn extract_method_params_with_data(source_code: &str, method_node: tree_sitter::Node) -> String {
    // Create a query to find method parameters with annotations
    let query_source = r#"
        (function_declaration
            (modifiers
                (annotation
                  (constructor_invocation
                    (user_type (type_identifier) @mapping_type
                      (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping|RequestMapping"))
                    (value_arguments (value_argument (simple_identifier) @key
                      (#match? @key "params")
                      (collection_literal (string_literal) @params)
                    ))
                    )))
             (simple_identifier) @method_name) @method
             
        (function_declaration
            (modifiers
                (annotation
                  (constructor_invocation
                    (user_type (type_identifier) @mapping_type
                      (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping|RequestMapping"))
                    (value_arguments (value_argument (simple_identifier) @key
                      (#match? @key "params")
                      (collection_literal (_) @params)
                    ))
                    )))
             (simple_identifier) @method_name) @method
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, method_node, source_code.as_bytes());

    let mut params = "";
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            if capture_name == &"params" { params = node_text }
        }
    }

    params.to_string()
}

// 継承情報を抽出する関数（Kotlin用）
fn extract_inheritance_info(source_code: &str, class_node: tree_sitter::Node) -> Option<String> {
    // Get the class declaration text
    let class_text = &source_code[class_node.byte_range()];

    // Simple regex-based approach to find inheritance
    // Look for pattern: class ClassName : ParentClass
    if let Some(colon_pos) = class_text.find(" : ") {
        let after_colon = &class_text[colon_pos + 3..];

        // Find the parent class name (until '(' or '{' or whitespace)
        let parent_class_end = after_colon
            .find('(')
            .or_else(|| after_colon.find('{'))
            .or_else(|| after_colon.find(' '))
            .unwrap_or(after_colon.len());

        let parent_class_name = after_colon[..parent_class_end].trim();

        if !parent_class_name.is_empty() {
            return Some(parent_class_name.to_string());
        }
    }

    None
}

// 親クラスファイルを探索する関数（Java/Kotlin両方対応）
fn find_parent_class_file(scan_root_dir: &str, parent_class_name: &str) -> Option<String> {
    // 複数の拡張子を試す（Kotlin -> Java継承も考慮）
    let target_extensions = [".kt", ".java"];

    for extension in &target_extensions {
        let target_filename = format!("{}{}", parent_class_name, extension);

        for entry in WalkDir::new(scan_root_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(filename) = entry.path().file_name() {
                    if filename == target_filename.as_str() {
                        // ファイル名が一致した場合、クラス名も確認
                        let file_path = entry.path().to_string_lossy().to_string();
                        if verify_class_name_in_file(&file_path, parent_class_name).unwrap_or(false)
                        {
                            return Some(file_path);
                        }
                    }
                }
            }
        }
    }

    None
}

// ファイル内に指定されたクラス名があるかを確認する関数（Java/Kotlin両方対応）
fn verify_class_name_in_file(file_path: &str, expected_class_name: &str) -> Result<bool> {
    let source_code = fs::read_to_string(file_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", file_path))?;

    // ファイル拡張子によってパーサーを選択
    if file_path.ends_with(".kt") {
        // Kotlinファイルの場合
        let mut parser = create_parser();
        let tree = parser
            .parse(&source_code, None)
            .expect("パースに失敗しました");

        let query_source = r#"
            (class_declaration
                (type_identifier) @class_name)
        "#;

        let query = create_query(query_source);
        let mut query_cursor = QueryCursor::new();
        let mut matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let capture_name = &query.capture_names()[capture.index as usize];
                if capture_name == &"class_name" {
                    let class_name = &source_code[capture.node.byte_range()];
                    if class_name == expected_class_name {
                        return Ok(true);
                    }
                }
            }
        }
    } else if file_path.ends_with(".java") {
        // Javaファイルの場合は、javaモジュールの関数を使用
        return crate::java::verify_class_name_in_java_file(file_path, expected_class_name);
    }

    Ok(false)
}

// Javaモジュールから呼び出すための公開関数
pub fn verify_class_name_in_kotlin_file(
    file_path: &str,
    expected_class_name: &str,
) -> Result<bool> {
    if !file_path.ends_with(".kt") {
        return Ok(false);
    }

    let source_code = fs::read_to_string(file_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", file_path))?;

    let mut parser = create_parser();
    let tree = parser
        .parse(&source_code, None)
        .expect("パースに失敗しました");

    let query_source = r#"
        (class_declaration
            (type_identifier) @class_name)
    "#;

    let query = create_query(query_source);
    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            if capture_name == &"class_name" {
                let class_name = &source_code[capture.node.byte_range()];
                if class_name == expected_class_name {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

// 親クラスのメソッドを継承用に抽出する関数（Java/Kotlin両方対応）
fn extract_parent_methods_for_inheritance(
    parent_file_path: &str,
    task: &InheritanceTask,
) -> Result<Vec<Endpoint>> {
    // ファイル拡張子によって処理を分岐
    if parent_file_path.ends_with(".java") {
        // Javaファイルの場合は、javaモジュールの関数を使用
        return crate::java::extract_parent_methods_for_inheritance_from_kotlin(
            parent_file_path,
            task.child_base_path.as_deref(),
            &task.parent_class_name,
        );
    }

    // Kotlinファイルの場合は従来通りの処理
    let source_code = fs::read_to_string(parent_file_path).with_context(|| {
        format!(
            "親クラスファイルの読み込みに失敗しました: {}",
            parent_file_path
        )
    })?;

    let mut parser = create_parser();
    let tree = parser
        .parse(&source_code, None)
        .expect("親クラスのパースに失敗しました");

    // 親クラスを見つける
    let query_source = r#"
        (class_declaration
            (type_identifier) @class_name) @class
    "#;

    let query = create_query(query_source);
    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    let mut endpoints = Vec::new();

    while let Some(m) = matches.next() {
        let mut found_target_class = false;
        let mut class_node = None;

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            if capture_name == &"class_name" && node_text == task.parent_class_name {
                found_target_class = true;
            } else if capture_name == &"class" {
                class_node = Some(capture.node);
            }
        }

        if found_target_class {
            if let Some(class_node) = class_node {
                // 親クラスのメソッドを抽出（親クラス名と親ファイルパスを正しく使用）
                let parent_endpoints = extract_method_mappings_with_endpoints(
                    &source_code,
                    class_node,
                    task.child_base_path.as_deref(), // 子クラスのbase_pathを使用
                    &task.parent_class_name,         // 親クラス名を使用
                    parent_file_path,                // 親クラスのファイルパスを使用（修正）
                );
                endpoints.extend(parent_endpoints);
                break;
            }
        }
    }

    Ok(endpoints)
}

// 継承タスクを作成する関数（Kotlin用）
fn check_inheritance_and_create_tasks(
    source_code: &str,
    class_node: tree_sitter::Node,
    class_name: &str,
    base_path: Option<String>,
    file_path: &str,
) -> Vec<InheritanceTask> {
    if let Some(parent_class_name) = extract_inheritance_info(source_code, class_node) {
        vec![InheritanceTask {
            child_file_path: file_path.to_string(),
            child_class_name: class_name.to_string(),
            child_base_path: base_path,
            parent_class_name,
        }]
    } else {
        vec![]
    }
}

// 親クラスからさらなる継承タスクを抽出する関数（Kotlin用）
fn extract_further_inheritance_tasks(
    parent_file_path: &str,
    current_task: &InheritanceTask,
) -> Result<Vec<InheritanceTask>> {
    let source_code = fs::read_to_string(parent_file_path).with_context(|| {
        format!(
            "親クラスファイルの読み込みに失敗しました: {}",
            parent_file_path
        )
    })?;

    let mut parser = create_parser();
    let tree = parser
        .parse(&source_code, None)
        .expect("親クラスのパースに失敗しました");

    // 親クラスを見つける
    let query_source = r#"
        (class_declaration
            (type_identifier) @class_name) @class
    "#;

    let query = create_query(query_source);
    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    let mut tasks = Vec::new();

    while let Some(m) = matches.next() {
        let mut found_target_class = false;
        let mut class_node = None;

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            if capture_name == &"class_name" && node_text == current_task.parent_class_name {
                found_target_class = true;
            } else if capture_name == &"class" {
                class_node = Some(capture.node);
            }
        }

        if found_target_class {
            if let Some(class_node) = class_node {
                // 親クラスがさらに継承している場合、新しいタスクを作成
                if let Some(grandparent_class_name) =
                    extract_inheritance_info(&source_code, class_node)
                {
                    tasks.push(InheritanceTask {
                        child_file_path: current_task.child_file_path.clone(),
                        child_class_name: current_task.child_class_name.clone(),
                        child_base_path: current_task.child_base_path.clone(),
                        parent_class_name: grandparent_class_name,
                    });
                }
                break;
            }
        }
    }

    Ok(tasks)
}

// 継承キューを処理する関数（Kotlin用・多重継承対応）
fn process_inheritance_queue(
    queue: Vec<InheritanceTask>,
    scan_root_dir: &str,
) -> Result<Vec<Endpoint>> {
    let mut inherited_endpoints = Vec::new();
    let mut processed_classes = std::collections::HashSet::new();
    let mut task_queue = std::collections::VecDeque::from(queue);

    while let Some(task) = task_queue.pop_front() {
        // 無限ループ防止：既に処理済みのクラスはスキップ
        let class_key = format!("{}:{}", task.parent_class_name, task.child_class_name);
        if processed_classes.contains(&class_key) {
            continue;
        }
        processed_classes.insert(class_key);

        if let Some(parent_file_path) =
            find_parent_class_file(scan_root_dir, &task.parent_class_name)
        {
            match extract_parent_methods_for_inheritance(&parent_file_path, &task) {
                Ok(endpoints) => {
                    inherited_endpoints.extend(endpoints);
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to extract methods from parent class {}: {}",
                        task.parent_class_name, e
                    );
                    continue;
                }
            }

            // 親クラスがさらに継承している場合、新しいタスクをキューに追加
            match extract_further_inheritance_tasks(&parent_file_path, &task) {
                Ok(new_tasks) => {
                    for new_task in new_tasks {
                        task_queue.push_back(new_task);
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to extract further inheritance from {}: {}",
                        task.parent_class_name, e
                    );
                }
            }
        } else {
            // Spring標準クラスや一般的なJavaクラスの場合は警告を出さない
            if should_warn_about_missing_parent(&task.parent_class_name) {
                eprintln!(
                    "Warning: Parent class {} not found for {}",
                    task.parent_class_name, task.child_class_name
                );
            }
        }
    }

    Ok(inherited_endpoints)
}

// 継承対応版のエンドポイント抽出関数（公開用・Kotlin）
pub fn extract_request_mapping_with_inheritance(
    file_path: &str,
    scan_root_dir: &str,
) -> Result<Vec<Endpoint>> {
    let (mut endpoints, inheritance_tasks) = extract_request_mapping_with_endpoints(file_path)?;

    // 継承処理
    let inherited_endpoints = process_inheritance_queue(inheritance_tasks, scan_root_dir)?;
    endpoints.extend(inherited_endpoints);

    Ok(endpoints)
}
