use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use path_finder::{scan_directory, Endpoint, Parameter};

    #[test]
    fn test_scan_directory() -> Result<()> {
        let endpoints = scan_directory("tests/resources")?;

        // デバッグ出力を追加
        println!("検出されたエンドポイント:");
        for endpoint in &endpoints {
            println!("{:?}", endpoint);
        }

        // 期待されるエンドポイントの数をチェック
        assert_eq!(endpoints.len(), 9, "エンドポイントの数が一致しません");

        // エンドポイントの内容を検証
        let expected_endpoints = vec![
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "getAllUsers".to_string(),
                http_method: "GET".to_string(),
                path: "/api/users".to_string(),
                parameters: vec![],
                line_range: (24, 27),
                file_path: "tests/resources/UserController.java".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "getUserById".to_string(),
                http_method: "GET".to_string(),
                path: "/api/users/{id}".to_string(),
                parameters: vec![Parameter {
                    name: "id".to_string(),
                    param_type: "Long".to_string(),
                    annotation: "PathVariable".to_string(),
                }],
                line_range: (29, 32),
                file_path: "tests/resources/UserController.java".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "getUserById2".to_string(),
                http_method: "GET".to_string(),
                path: "/api/users/{id}".to_string(),
                parameters: vec![Parameter {
                    name: "id".to_string(),
                    param_type: "Long".to_string(),
                    annotation: "PathVariable".to_string(),
                }],
                line_range: (34, 37),
                file_path: "tests/resources/UserController.java".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "createUser".to_string(),
                http_method: "POST".to_string(),
                path: "/api/users".to_string(),
                parameters: vec![Parameter {
                    name: "user".to_string(),
                    param_type: "User".to_string(),
                    annotation: "RequestBody".to_string(),
                }],
                line_range: (39, 42),
                file_path: "tests/resources/UserController.java".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "createUser2".to_string(),
                http_method: "POST".to_string(),
                path: "/api/users/{id}".to_string(),
                parameters: vec![Parameter {
                    name: "user".to_string(),
                    param_type: "User".to_string(),
                    annotation: "RequestBody".to_string(),
                }],
                line_range: (44, 47),
                file_path: "tests/resources/UserController.java".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "getAllUsers".to_string(),
                http_method: "GET".to_string(),
                path: "/api/kotlin/users".to_string(),
                parameters: vec![],
                line_range: (19, 22),
                file_path: "tests/resources/UserController.kt".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "getUserById".to_string(),
                http_method: "GET".to_string(),
                path: "/api/kotlin/users/{id}".to_string(),
                parameters: vec![Parameter {
                    name: "id".to_string(),
                    param_type: "Long".to_string(),
                    annotation: "PathVariable".to_string(),
                }],
                line_range: (24, 27),
                file_path: "tests/resources/UserController.kt".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "createUser".to_string(),
                http_method: "POST".to_string(),
                path: "/api/kotlin/users".to_string(),
                parameters: vec![Parameter {
                    name: "user".to_string(),
                    param_type: "User".to_string(),
                    annotation: "RequestBody".to_string(),
                }],
                line_range: (29, 32),
                file_path: "tests/resources/UserController.kt".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "createUser2".to_string(),
                http_method: "POST".to_string(),
                path: "/api/kotlin/users/{id}".to_string(),
                parameters: vec![Parameter {
                    name: "user".to_string(),
                    param_type: "User".to_string(),
                    annotation: "RequestBody".to_string(),
                }],
                line_range: (34, 37),
                file_path: "tests/resources/UserController.kt".to_string(),
            },
        ];

        // エンドポイントの内容を比較
        for expected in expected_endpoints {
            assert!(
                endpoints.iter().any(|e| endpoints_match(e, &expected)),
                "期待されるエンドポイントが見つかりません: {:?}",
                expected
            );
        }

        Ok(())
    }

    // エンドポイントの比較関数
    fn endpoints_match(a: &Endpoint, b: &Endpoint) -> bool {
        if a.class_name != b.class_name
            || a.method_name != b.method_name
            || a.http_method != b.http_method
            || a.path != b.path
            || a.line_range != b.line_range
            || a.file_path != b.file_path
            || a.parameters.len() != b.parameters.len()
        {
            return false;
        }

        // パラメータの比較
        let a_params: HashSet<_> = a
            .parameters
            .iter()
            .map(|p| {
                (
                    p.name.as_str(),
                    p.param_type.as_str(),
                    p.annotation.as_str(),
                )
            })
            .collect();

        let b_params: HashSet<_> = b
            .parameters
            .iter()
            .map(|p| {
                (
                    p.name.as_str(),
                    p.param_type.as_str(),
                    p.annotation.as_str(),
                )
            })
            .collect();

        a_params == b_params
    }
}
