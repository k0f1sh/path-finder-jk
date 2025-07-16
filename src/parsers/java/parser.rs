use tree_sitter::{Parser, Query};

/// Javaパーサーを作成する関数
pub fn create_parser() -> Parser {
    let mut parser = Parser::new();
    let language = tree_sitter_java::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Java parser");
    parser
}

/// Java用のクエリを作成する関数
pub fn create_query(query_source: &str) -> Query {
    let language = tree_sitter_java::LANGUAGE;
    Query::new(&language.into(), query_source).expect("Invalid query")
}