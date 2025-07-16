pub mod parser;
pub mod annotations;
pub mod methods;

use anyhow::{Context, Result};
use std::fs;
use tree_sitter::{QueryCursor, StreamingIterator};

use crate::common::types::{Endpoint, InheritanceTask};
use crate::common::inheritance::create_inheritance_task;
use parser::{create_parser, create_query};

/// Javaファイルに@RequestMappingアノテーションがあるかチェックする
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

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    Ok(matches.count() > 0)
}

/// 継承対応版のエンドポイント抽出関数（公開用）
pub fn extract_request_mapping_with_inheritance(
    file_path: &str,
    scan_root_dir: &str,
) -> Result<Vec<Endpoint>> {
    let (mut endpoints, inheritance_tasks) = extract_request_mapping_with_endpoints(file_path)?;

    // 継承処理 - 共通ロジックを使用
    let inherited_endpoints = crate::common::inheritance::process_inheritance_queue(
        inheritance_tasks,
        scan_root_dir,
        |parent_file_path, task| {
            methods::extract_parent_methods_for_inheritance(parent_file_path, task)
        },
    )?;
    endpoints.extend(inherited_endpoints);

    Ok(endpoints)
}

/// RequestMappingアノテーションを持つクラスのエンドポイントと継承タスクを抽出
fn extract_request_mapping_with_endpoints(
    file_path: &str,
) -> Result<(Vec<Endpoint>, Vec<InheritanceTask>)> {
    let mut parser = create_parser();

    let source_code = fs::read_to_string(file_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", file_path))?;
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
                let base_path = annotations::extract_request_mapping_path(&source_code, class_node);

                // Extract method-level mappings
                let method_endpoints = methods::extract_method_mappings_with_endpoints(
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

/// 継承情報を抽出する関数
fn extract_inheritance_info(source_code: &str, class_node: tree_sitter::Node) -> Option<String> {
    let query_source = r#"
        (class_declaration
            (superclass (type_identifier) @parent_class_name))
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, class_node, source_code.as_bytes());

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            if capture_name == &"parent_class_name" {
                let parent_class_name = &source_code[capture.node.byte_range()];
                return Some(parent_class_name.to_string());
            }
        }
    }

    None
}

/// 継承タスクを作成する関数
fn check_inheritance_and_create_tasks(
    source_code: &str,
    class_node: tree_sitter::Node,
    class_name: &str,
    base_path: Option<String>,
    file_path: &str,
) -> Vec<InheritanceTask> {
    if let Some(parent_class_name) = extract_inheritance_info(source_code, class_node) {
        vec![create_inheritance_task(
            file_path,
            class_name,
            base_path,
            parent_class_name,
        )]
    } else {
        vec![]
    }
}

/// ファイル内に指定されたクラス名があるかを確認する関数（Java用）
pub fn verify_class_name_in_java_file(file_path: &str, expected_class_name: &str) -> Result<bool> {
    if !file_path.ends_with(".java") {
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
            name: (identifier) @class_name)
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