pub mod parser;
pub mod annotations;
pub mod methods;

use anyhow::{Context, Result};
use std::fs;
use tree_sitter::{QueryCursor, StreamingIterator};

use crate::common::types::{Endpoint, InheritanceTask};
use parser::{create_parser, create_query};

/// Kotlinファイルに@RequestMappingアノテーションがあるかチェックする
pub fn has_request_mapping(file_path: &str) -> Result<bool> {
    let source_code = fs::read_to_string(file_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", file_path))?;

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

/// 継承対応版のエンドポイント抽出関数（公開用）
pub fn extract_request_mapping_with_inheritance(
    file_path: &str,
    scan_root_dir: &str,
) -> Result<Vec<Endpoint>> {
    let (mut endpoints, inheritance_tasks) = extract_request_mapping_with_endpoints(file_path)?;

    let inherited_endpoints = crate::common::inheritance::process_inheritance_queue(
        inheritance_tasks,
        scan_root_dir,
        |parent_file_path, task| {
            // ファイル拡張子によって適切なパーサーを選択
            if parent_file_path.ends_with(".java") {
                crate::parsers::java::methods::extract_parent_methods_for_inheritance_from_kotlin(
                    parent_file_path,
                    task.child_base_path.as_deref(),
                    &task.parent_class_name,
                )
            } else {
                methods::extract_parent_methods_for_inheritance(parent_file_path, task)
            }
        },
    )?;
    endpoints.extend(inherited_endpoints);

    Ok(endpoints)
}

/// RequestMappingアノテーションを持つクラスのエンドポイントと継承タスクを抽出（簡略化版）
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
                (annotation (_) @annotation_name
                    (#match? @annotation_name "RequestMapping"))))
    "#;

    let query = create_query(query_source);
    let mut query_cursor = QueryCursor::new();
    let mut matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());

    let mut endpoints = Vec::new();
    let inheritance_tasks = Vec::new();

    while let Some(m) = matches.next() {
        let mut class_name = "";

        for capture in m.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node_text = &source_code[capture.node.byte_range()];

            if *capture_name == "class_name" {
                class_name = node_text;
            }
        }

        for capture in m.captures {
            if query.capture_names()[capture.index as usize] == "class" {
                let class_node = capture.node;
                let base_path = annotations::extract_request_mapping_path(&source_code, class_node);

                let method_endpoints = methods::extract_method_mappings_with_endpoints(
                    &source_code,
                    class_node,
                    base_path.as_deref(),
                    class_name,
                    file_path,
                );
                endpoints.extend(method_endpoints);

                // 継承処理は簡略化のためスキップ
                break;
            }
        }
    }

    Ok((endpoints, inheritance_tasks))
}

/// ファイル内に指定されたクラス名があるかを確認する関数（Kotlin用）
pub fn verify_class_name_in_kotlin_file(file_path: &str, expected_class_name: &str) -> Result<bool> {
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
            (simple_identifier) @class_name)
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