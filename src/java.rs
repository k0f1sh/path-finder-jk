use anyhow::{Context, Result};
use std::fs;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

use crate::{Endpoint, Parameter};

fn create_parser() -> Parser {
    let mut parser = Parser::new();
    let language = tree_sitter_java::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Java parser");
    parser
}

fn create_query(query_source: &str) -> Query {
    let language = tree_sitter_java::LANGUAGE;
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

pub fn extract_request_mapping_with_endpoints(file_path: &str) -> Result<Vec<Endpoint>> {
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
            if &query.capture_names()[capture.index as usize] == &"class" {
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

                break;
            }
        }
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

            match capture_name {
                &"method_name" => method_name = node_text,
                &"mapping_type" => mapping_type = node_text,
                &"path" => path = node_text,
                &"method" => method_node = Some(capture.node),
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

            // パラメータを抽出
            let parameters = extract_method_parameters_with_data(source_code, node);

            // headerを抽出
            let headers = extract_method_headers_with_data(source_code, node);

            // エンドポイントを作成
            let endpoint = Endpoint {
                class_name: class_name.to_string(),
                method_name: method_name.to_string(),
                http_method: http_method,
                path: full_path.trim_matches('"').to_string(),
                parameters,
                line_range: (start_line, end_line),
                file_path: file_path.to_string(),
                headers: headers,
            };

            endpoints.push(endpoint);
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
        "RequestMapping" => "ANY".to_string(), // デフォルト値
        _ => "UNKNOWN".to_string(),
    }
}

fn extract_request_mapping_method(source_code: &str, method_node: tree_sitter::Node) -> String {
    // RequestMappingのmethod属性を検索するクエリ
    let query_source = r#"
        (annotation
            name: (identifier) @annotation_name
            (#match? @annotation_name "RequestMapping")
            arguments: (annotation_argument_list
                (element_value_pair
                    key: (identifier) @key
                    (#match? @key "method")
                    value: (_) @method_value)))
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, method_node, source_code.as_bytes());

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            if capture_name == &"method_value" {
                let method_value = &source_code[capture.node.byte_range()];

                // RequestMethod.XXX 形式から XXX 部分を抽出
                if let Some(dot_pos) = method_value.rfind('.') {
                    if dot_pos + 1 < method_value.len() {
                        return method_value[dot_pos + 1..].to_string();
                    }
                }

                // 配列の場合（例：{RequestMethod.GET, RequestMethod.POST}）
                if method_value.contains("GET") {
                    return "GET".to_string();
                } else if method_value.contains("POST") {
                    return "POST".to_string();
                } else if method_value.contains("PUT") {
                    return "PUT".to_string();
                } else if method_value.contains("DELETE") {
                    return "DELETE".to_string();
                } else if method_value.contains("PATCH") {
                    return "PATCH".to_string();
                }

                return method_value.to_string();
            }
        }
    }

    // メソッドが指定されていない場合はデフォルトでANY
    "ANY".to_string()
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

            match capture_name {
                &"param_name" => param_name = node_text,
                &"param_type" => param_type = node_text,
                &"param_annotation" => param_annotation = node_text,
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
fn extract_method_headers_with_data(source_code: &str, method_node: tree_sitter::Node) -> String {
    // Create a query to find method parameters with annotations
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

            match capture_name {
                &"headers" => headers = node_text,
                _ => {}
            }
        }
    }

    headers.to_string()
}
