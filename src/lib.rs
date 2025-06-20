use anyhow::Result;
use serde::Serialize;
use serde_json;
use walkdir::WalkDir;

pub mod java;
pub mod kotlin;

// エンドポイント情報を格納する構造体
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

#[derive(Debug, Serialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: String,
    pub annotation: String,
}

pub fn scan_directory(dir_path: &str) -> Result<Vec<Endpoint>> {
    scan_directory_internal(dir_path, false).map(|result| match result {
        ScanResult::Endpoints(endpoints) => endpoints,
        _ => unreachable!(),
    })
}

/// Scans the directory for Java and Kotlin files with Spring RequestMapping annotations
/// and returns the endpoints in JSON format.
///
/// This is useful for integrating with other tools or scripts that can parse JSON.
///
/// # Arguments
///
/// * `dir_path` - The path to the directory to scan
///
/// # Returns
///
/// A JSON string containing an array of endpoint objects
pub fn scan_directory_json(dir_path: &str) -> Result<String> {
    scan_directory_internal(dir_path, true).map(|result| match result {
        ScanResult::Json(json) => json,
        _ => unreachable!(),
    })
}

enum ScanResult {
    Endpoints(Vec<Endpoint>),
    Json(String),
}

fn scan_directory_internal(dir_path: &str, json_output: bool) -> Result<ScanResult> {
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
                    let endpoints =
                        java::extract_request_mapping_with_inheritance(&file_path, dir_path)?;
                    all_endpoints.extend(endpoints);
                }
            } else if entry.path().extension().map_or(false, |ext| ext == "kt") {
                if kotlin::has_request_mapping(&file_path)? {
                    let endpoints =
                        kotlin::extract_request_mapping_with_inheritance(&file_path, dir_path)?;
                    all_endpoints.extend(endpoints);
                }
            }
        }
    }

    if json_output {
        Ok(ScanResult::Json(serde_json::to_string_pretty(
            &all_endpoints,
        )?))
    } else {
        Ok(ScanResult::Endpoints(all_endpoints))
    }
}
