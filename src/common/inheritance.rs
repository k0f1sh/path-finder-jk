use anyhow::Result;
use std::collections::{HashSet, VecDeque};

use crate::common::file_utils::find_parent_class_file;
use crate::common::warnings::should_warn_about_missing_parent;
use crate::common::types::{Endpoint, InheritanceTask};

/// 継承キューを処理する関数（多重継承対応）
/// 
/// この関数は言語に依存しない継承処理のコアロジックを担当する
/// 各言語固有の親クラスメソッド抽出は、渡されたクロージャーで処理される
pub fn process_inheritance_queue<F>(
    queue: Vec<InheritanceTask>,
    scan_root_dir: &str,
    mut extract_parent_methods: F,
) -> Result<Vec<Endpoint>>
where
    F: FnMut(&str, &InheritanceTask) -> Result<Vec<Endpoint>>,
{
    let mut inherited_endpoints = Vec::new();
    let mut processed_classes = HashSet::new();
    let mut task_queue = VecDeque::from(queue);

    while let Some(task) = task_queue.pop_front() {
        // 無限ループ防止：既に処理済みのクラスはスキップ
        let class_key = format!("{}:{}", task.parent_class_name, task.child_class_name);
        if processed_classes.contains(&class_key) {
            continue;
        }
        processed_classes.insert(class_key);

        if let Some(parent_file_path) =
            find_parent_class_file(scan_root_dir, &task.parent_class_name)
        {
            match extract_parent_methods(&parent_file_path, &task) {
                Ok(endpoints) => {
                    inherited_endpoints.extend(endpoints);
                }
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to extract methods from parent class {}: {}",
                        task.parent_class_name, e
                    );
                    continue;
                }
            }

            // 親クラスがさらに継承している場合、新しいタスクをキューに追加
            // この部分は各言語パーサーで実装する必要があります
            // 今回は簡単化のため、この処理は各言語固有の実装に委ねます
        } else {
            // Spring標準クラスや一般的なJavaクラスの場合は警告を出さない
            if should_warn_about_missing_parent(&task.parent_class_name) {
                eprintln!(
                    "Warning: Parent class {} not found for {}",
                    task.parent_class_name, task.child_class_name
                );
            }
        }
    }

    Ok(inherited_endpoints)
}

/// 継承タスクを作成するヘルパー関数
pub fn create_inheritance_task(
    child_file_path: &str,
    child_class_name: &str,
    child_base_path: Option<String>,
    parent_class_name: String,
) -> InheritanceTask {
    InheritanceTask {
        child_file_path: child_file_path.to_string(),
        child_class_name: child_class_name.to_string(),
        child_base_path,
        parent_class_name,
    }
}