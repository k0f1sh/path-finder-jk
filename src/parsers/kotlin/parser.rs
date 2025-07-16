use tree_sitter::{Parser, Query};

/// Kotlinパーサーを作成する関数
pub fn create_parser() -> Parser {
    let mut parser = Parser::new();
    let language = tree_sitter_kotlin_sg::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Kotlin parser");
    parser
}

/// Kotlin用のクエリを作成する関数
pub fn create_query(query_source: &str) -> Query {
    let language = tree_sitter_kotlin_sg::LANGUAGE;
    Query::new(&language.into(), query_source).expect("Invalid query")
}