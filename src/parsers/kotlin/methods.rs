use anyhow::{Context, Result};
use std::fs;
use tree_sitter::{QueryCursor, StreamingIterator};

use crate::common::types::{Endpoint, InheritanceTask};
use super::parser::{create_parser, create_query};
use super::annotations::{extract_request_mapping_method, mapping_type_to_http_method};

/// メソッドレベルのマッピングアノテーションを抽出してエンドポイントを作成
pub fn extract_method_mappings_with_endpoints(
    source_code: &str,
    class_node: tree_sitter::Node,
    base_path: Option<&str>,
    class_name: &str,
    file_path: &str,
) -> Vec<Endpoint> {
    // Kotlinの場合のクエリ（簡略化版）
    let query_source = r#"
        (function_declaration
            (modifiers
                (annotation
                    (user_type
                        (type_identifier) @mapping_type
                        (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping|RequestMapping"))))
            (simple_identifier) @method_name) @method
    "#;

    let query = create_query(query_source);
    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, class_node, source_code.as_bytes());

    let mut endpoints = Vec::new();

    while let Some(m) = matches.next() {
        let mut method_name = "";
        let mut mapping_type = "";
        let mut method_node = None;

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            match *capture_name {
                "method_name" => method_name = node_text,
                "mapping_type" => mapping_type = node_text,
                "method" => method_node = Some(capture.node),
                _ => {}
            }
        }

        if let Some(node) = method_node {
            let start_line = node.start_position().row + 1;
            let end_line = node.end_position().row + 1;

            let http_method = if mapping_type == "RequestMapping" {
                extract_request_mapping_method(source_code, node)
            } else {
                mapping_type_to_http_method(mapping_type)
            };

            let full_path = match base_path {
                Some(base) => format!("{}/", base.trim_matches('"')),
                None => "".to_string(),
            };

            // 簡略化：パラメータとヘッダーは空で作成
            let parameters = Vec::new();
            let headers = String::new();

            let endpoint = Endpoint {
                class_name: class_name.to_string(),
                method_name: method_name.to_string(),
                http_method,
                path: full_path,
                parameters,
                line_range: (start_line, end_line),
                file_path: file_path.to_string(),
                headers,
            };

            endpoints.push(endpoint);
        }
    }

    endpoints
}

/// 親クラスのメソッドを継承用に抽出する関数
pub fn extract_parent_methods_for_inheritance(
    parent_file_path: &str,
    task: &InheritanceTask,
) -> Result<Vec<Endpoint>> {
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

    let query_source = r#"
        (class_declaration
            (simple_identifier) @class_name) @class
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
                let parent_endpoints = extract_method_mappings_with_endpoints(
                    &source_code,
                    class_node,
                    task.child_base_path.as_deref(),
                    &task.parent_class_name,
                    parent_file_path,
                );
                endpoints.extend(parent_endpoints);
                break;
            }
        }
    }

    Ok(endpoints)
}