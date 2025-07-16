use anyhow::{Context, Result};
use std::fs;
use walkdir::WalkDir;

/// 親クラスファイルを探索する関数（Java/Kotlin両方対応）
pub fn find_parent_class_file(scan_root_dir: &str, parent_class_name: &str) -> Option<String> {
    // 複数の拡張子を試す（Java -> Kotlin継承も考慮）
    let target_extensions = [".java", ".kt"];

    for extension in &target_extensions {
        let target_filename = format!("{}{}", parent_class_name, extension);

        for entry in WalkDir::new(scan_root_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(filename) = entry.path().file_name() {
                    if filename == target_filename.as_str() {
                        // ファイル名が一致した場合、クラス名も確認
                        let file_path = entry.path().to_string_lossy().to_string();
                        if verify_class_name_in_file(&file_path, parent_class_name).unwrap_or(false)
                        {
                            return Some(file_path);
                        }
                    }
                }
            }
        }
    }

    None
}

/// ファイル内に指定されたクラス名があるかを確認する関数（Java/Kotlin両方対応）
pub fn verify_class_name_in_file(file_path: &str, expected_class_name: &str) -> Result<bool> {
    let _source_code = fs::read_to_string(file_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {}", file_path))?;

    // ファイル拡張子によってパーサーを選択
    if file_path.ends_with(".java") {
        // Javaファイルの場合
        crate::parsers::java::verify_class_name_in_java_file(file_path, expected_class_name)
    } else if file_path.ends_with(".kt") {
        // Kotlinファイルの場合
        crate::parsers::kotlin::verify_class_name_in_kotlin_file(file_path, expected_class_name)
    } else {
        Ok(false)
    }
}