use anyhow::Result;
use walkdir::WalkDir;
use serde_json;

use crate::common::types::{Endpoint, ScanResult};
use crate::parsers::{java, kotlin};

/// ディレクトリ内のJavaとKotlinファイルをスキャンしてエンドポイントを抽出する
pub fn scan_directory(dir_path: &str) -> Result<Vec<Endpoint>> {
    scan_directory_internal(dir_path, false).map(|result| match result {
        ScanResult::Endpoints(endpoints) => endpoints,
        _ => unreachable!(),
    })
}

/// ディレクトリ内のJavaとKotlinファイルをスキャンしてエンドポイントをJSON形式で返す
pub fn scan_directory_json(dir_path: &str) -> Result<String> {
    scan_directory_internal(dir_path, true).map(|result| match result {
        ScanResult::Json(json) => json,
        _ => unreachable!(),
    })
}

/// 内部的なスキャン処理
fn scan_directory_internal(dir_path: &str, json_output: bool) -> Result<ScanResult> {
    let mut all_endpoints = Vec::new();

    for entry in WalkDir::new(dir_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let file_path = entry.path().to_string_lossy().to_string();
            
            if entry.path().extension().is_some_and(|ext| ext == "java") {
                if java::has_request_mapping(&file_path)? {
                    let endpoints =
                        java::extract_request_mapping_with_inheritance(&file_path, dir_path)?;
                    all_endpoints.extend(endpoints);
                }
            } else if entry.path().extension().is_some_and(|ext| ext == "kt") && kotlin::has_request_mapping(&file_path)? {
                let endpoints =
                    kotlin::extract_request_mapping_with_inheritance(&file_path, dir_path)?;
                all_endpoints.extend(endpoints);
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