use anyhow::{Context, Result};
use std::fs;
use tree_sitter::{QueryCursor, StreamingIterator};

use crate::common::types::{Endpoint, Parameter, InheritanceTask};
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
            
        (method_declaration
            (modifiers
                (annotation
                    name: (identifier) @mapping_type
                    (#match? @mapping_type "GetMapping|PostMapping|PutMapping|DeleteMapping|PatchMapping")
                    arguments: (annotation_argument_list
                        (element_value_pair
                            key: (identifier) @key
                            (#match? @key "value")
                            value: (string_literal) @path))))
            name: (identifier) @method_name) @method

        (method_declaration
            (modifiers
                (annotation
                    name: (identifier) @mapping_type
                    (#match? @mapping_type "RequestMapping")
                    arguments: (annotation_argument_list
                        (element_value_pair
                            key: (identifier) @key
                            (#match? @key "value")
                            value: (string_literal) @path))))
            name: (identifier) @method_name) @method
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, class_node, source_code.as_bytes());

    let mut endpoints = Vec::new();

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

            // RequestMappingの場合はmethod属性を調べる
            let http_method = if mapping_type == "RequestMapping" {
                extract_request_mapping_method(source_code, node)
            } else {
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

            // パラメータを抽出
            let parameters = extract_method_parameters_with_data(source_code, node);

            // headerを抽出
            let headers = extract_method_headers_with_data(source_code, node);

            // エンドポイントを作成
            let endpoint = Endpoint {
                class_name: class_name.to_string(),
                method_name: method_name.to_string(),
                http_method,
                path: full_path.trim_matches('"').to_string(),
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

/// メソッドパラメータを抽出する
fn extract_method_parameters_with_data(
    source_code: &str,
    method_node: tree_sitter::Node,
) -> Vec<Parameter> {
    let query_source = r#"
        (formal_parameter
            (modifiers
                (marker_annotation
                    name: (identifier) @param_annotation
                    (#match? @param_annotation "PathVariable|RequestBody|RequestParam")))
            type: (_) @param_type
            name: (identifier) @param_name) @param
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

        parameters.push(Parameter {
            name: param_name.to_string(),
            param_type: param_type.to_string(),
            annotation: param_annotation.to_string(),
        });
    }

    parameters
}

/// メソッドヘッダーを抽出する
fn extract_method_headers_with_data(source_code: &str, method_node: tree_sitter::Node) -> String {
    let query_source = r#"
        (method_declaration
            (modifiers
                (annotation
                    name: (identifier) @mapping_type
                    (#match? @mapping_type "RequestMapping")
                    arguments: (annotation_argument_list
                        (element_value_pair
                            key: (identifier) @key
                            (#match? @key "headers")
                            value: (_) @headers))))
            name: (identifier) @method_name)
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, method_node, source_code.as_bytes());

    let mut headers = "";
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            if capture_name == &"headers" { headers = node_text }
        }
    }

    headers.to_string()
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

    // 親クラスを見つける
    let query_source = r#"
        (class_declaration
            name: (identifier) @class_name) @class
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
                    parent_file_path,                // 親クラスのファイルパスを使用
                );
                endpoints.extend(parent_endpoints);
                break;
            }
        }
    }

    Ok(endpoints)
}

/// Kotlinから呼び出すための公開関数（Javaファイルの親クラスメソッド抽出）
pub fn extract_parent_methods_for_inheritance_from_kotlin(
    parent_file_path: &str,
    child_base_path: Option<&str>,
    parent_class_name: &str,
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
            name: (identifier) @class_name) @class
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

            if capture_name == &"class_name" && node_text == parent_class_name {
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
                    child_base_path,   // 子クラスのbase_pathを使用
                    parent_class_name, // 親クラス名を使用
                    parent_file_path,  // 親クラスのファイルパスを使用
                );
                endpoints.extend(parent_endpoints);
                break;
            }
        }
    }

    Ok(endpoints)
}