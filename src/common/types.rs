use serde::Serialize;

/// エンドポイント情報を格納する構造体
#[derive(Debug, Serialize)]
pub struct Endpoint {
    pub class_name: String,
    pub method_name: String,
    pub http_method: String,
    pub path: String,
    pub parameters: Vec<Parameter>,
    pub line_range: (usize, usize),
    pub file_path: String,
    pub headers: String,
}

/// メソッドパラメータ情報を格納する構造体
#[derive(Debug, Serialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub annotation: String,
}

/// 継承処理用のタスク構造体
#[derive(Debug, Clone)]
pub struct InheritanceTask {
    pub child_file_path: String,
    pub child_class_name: String,
    pub child_base_path: Option<String>,
    pub parent_class_name: String,
}

/// スキャン結果の種類を表すenum
pub enum ScanResult {
    Endpoints(Vec<Endpoint>),
    Json(String),
}