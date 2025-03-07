use anyhow::Result;
use walkdir::WalkDir;

pub mod java;

// エンドポイント情報を格納する構造体
#[derive(Debug)]
pub struct Endpoint {
    pub class_name: String,
    pub method_name: String,
    pub http_method: String,
    pub path: String,
    pub parameters: Vec<Parameter>,
    pub line_range: (usize, usize),
    pub file_path: String,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub annotation: String,
}

pub fn scan_directory(dir_path: &str) -> Result<Vec<Endpoint>> {
    let mut all_endpoints = Vec::new();

    for entry in WalkDir::new(dir_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path().to_string_lossy().to_string();
            if entry.path().extension().map_or(false, |ext| ext == "java") {
                if java::has_request_mapping(&file_path)? {
                    let endpoints = java::extract_request_mapping_with_endpoints(&file_path)?;
                    all_endpoints.extend(endpoints);
                }
            }
            // TODO: Kotlin support will be added here
        }
    }

    Ok(all_endpoints)
}
