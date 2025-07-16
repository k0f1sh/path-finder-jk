use tree_sitter::{QueryCursor, StreamingIterator};
use super::parser::create_query;

/// RequestMappingアノテーションからパスを抽出する
pub fn extract_request_mapping_path(
    source_code: &str,
    class_node: tree_sitter::Node,
) -> Option<String> {
    let query_source = r#"
        (annotation
            (user_type
                (type_identifier) @annotation_name
                (#match? @annotation_name "RequestMapping"))
            (value_arguments
                (value_argument
                    (string_literal) @path)))
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

/// RequestMappingアノテーションのmethod属性を抽出する
pub fn extract_request_mapping_method(source_code: &str, method_node: tree_sitter::Node) -> String {
    let query_source = r#"
        (annotation
            (user_type
                (type_identifier) @annotation_name
                (#match? @annotation_name "RequestMapping"))
            (value_arguments
                (value_argument
                    (call_expression
                        (navigation_expression
                            (simple_identifier) @method_class
                            (simple_identifier) @method_value)))))
    "#;

    let query = create_query(query_source);

    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, method_node, source_code.as_bytes());

    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            if capture_name == &"method_value" {
                let method_value = &source_code[capture.node.byte_range()];
                return method_value.to_string();
            }
        }
    }

    // メソッドが指定されていない場合はデフォルトでANY
    "ANY".to_string()
}

/// アノテーションタイプからHTTPメソッドへの変換
pub fn mapping_type_to_http_method(mapping_type: &str) -> String {
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