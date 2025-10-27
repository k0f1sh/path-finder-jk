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

        // 期待されるエンドポイントの数を22に修正（EmptyMappingControllerで6つ追加）
        assert_eq!(endpoints.len(), 22, "エンドポイントの数が一致しません");

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
                headers: "".to_string(),
                params: "".to_string(),
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
                headers: "".to_string(),
                params: "".to_string(),
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
                headers: "".to_string(),
                params: "".to_string(),
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
                headers: "".to_string(),
                params: "".to_string(),
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
                headers: "XCustomHeader".to_string(),
                params: "".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "getAllUsers".to_string(),
                http_method: "GET".to_string(),
                path: "/api/kotlin/users".to_string(),
                parameters: vec![],
                line_range: (19, 22),
                file_path: "tests/resources/UserController.kt".to_string(),
                headers: "".to_string(),
                params: "".to_string(),
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
                headers: "".to_string(),
                params: "".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "getUserById2".to_string(),
                http_method: "GET".to_string(),
                path: "/api/kotlin/users/{id}".to_string(),
                parameters: vec![Parameter {
                    name: "id".to_string(),
                    param_type: "Long".to_string(),
                    annotation: "PathVariable".to_string(),
                }],
                line_range: (29, 32),
                file_path: "tests/resources/UserController.kt".to_string(),
                headers: "".to_string(),
                params: "".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "getUserById3".to_string(),
                http_method: "GET".to_string(),
                path: "/api/kotlin/users/{id}".to_string(),
                parameters: vec![Parameter {
                    name: "id".to_string(),
                    param_type: "Long".to_string(),
                    annotation: "PathVariable".to_string(),
                }],
                line_range: (34, 37),
                file_path: "tests/resources/UserController.kt".to_string(),
                headers: "XCustomHeader".to_string(),
                params: "".to_string(),
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
                line_range: (39, 42),
                file_path: "tests/resources/UserController.kt".to_string(),
                headers: "".to_string(),
                params: "".to_string(),
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
                line_range: (44, 47),
                file_path: "tests/resources/UserController.kt".to_string(),
                headers: "".to_string(),
                params: "".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "updateNameId".to_string(),
                http_method: "PUT".to_string(),
                path: "/api/kotlin/users/{id}/name-id".to_string(),
                parameters: vec![
                    Parameter {
                        name: "id".to_string(),
                        param_type: "Long".to_string(),
                        annotation: "PathVariable".to_string(),
                    },
                    Parameter {
                        name: "params".to_string(),
                        param_type: "UpdateNameIdRequestParams".to_string(),
                        annotation: "PathVariable".to_string(),
                    },
                    Parameter {
                        name: "params".to_string(),
                        param_type: "UpdateNameIdRequestParams".to_string(),
                        annotation: "RequestBody".to_string(),
                    },
                ],
                line_range: (49, 52),
                file_path: "tests/resources/UserController.kt".to_string(),
                headers: "".to_string(),
                params: "".to_string(),
            },
            // Java params付きエンドポイント
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "getUsersV1".to_string(),
                http_method: "GET".to_string(),
                path: "/api/users".to_string(),
                parameters: vec![],
                line_range: (49, 52),
                file_path: "tests/resources/UserController.java".to_string(),
                headers: "".to_string(),
                params: "\"version=1\"".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "searchUsers".to_string(),
                http_method: "GET".to_string(),
                path: "/api/users/search".to_string(),
                parameters: vec![],
                line_range: (54, 57),
                file_path: "tests/resources/UserController.java".to_string(),
                headers: "".to_string(),
                params: "{\"q\", \"type=advanced\"}".to_string(),
            },
            // Kotlin params付きエンドポイント
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "getUsersV2".to_string(),
                http_method: "GET".to_string(),
                path: "/api/kotlin/users".to_string(),
                parameters: vec![],
                line_range: (54, 57),
                file_path: "tests/resources/UserController.kt".to_string(),
                headers: "".to_string(),
                params: "\"version=2\"".to_string(),
            },
            Endpoint {
                class_name: "UserController".to_string(),
                method_name: "searchKotlinUsers".to_string(),
                http_method: "GET".to_string(),
                path: "/api/kotlin/users/search".to_string(),
                parameters: vec![],
                line_range: (59, 62),
                file_path: "tests/resources/UserController.kt".to_string(),
                headers: "".to_string(),
                params: "\"type=kotlin\"".to_string(),
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
            || a.headers != b.headers
            || a.params != b.params
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

    #[test]
    fn test_params_extraction() -> Result<()> {
        let endpoints = scan_directory("tests/resources")?;

        // デバッグ出力
        println!("params抽出テストで検出されたエンドポイント:");
        for endpoint in &endpoints {
            if !endpoint.params.is_empty() {
                println!(
                    "  {} {} - params: {}",
                    endpoint.http_method, endpoint.path, endpoint.params
                );
            }
        }

        // Java params付きエンドポイントのテスト
        let java_v1_endpoint = endpoints
            .iter()
            .find(|e| e.method_name == "getUsersV1" && e.file_path.contains("UserController.java"));
        assert!(
            java_v1_endpoint.is_some(),
            "Java getUsersV1エンドポイントが見つかりません"
        );
        if let Some(endpoint) = java_v1_endpoint {
            assert_eq!(
                endpoint.params, "\"version=1\"",
                "Java getUsersV1のparams値が正しくありません"
            );
        }

        let java_search_endpoint = endpoints.iter().find(|e| {
            e.method_name == "searchUsers" && e.file_path.contains("UserController.java")
        });
        assert!(
            java_search_endpoint.is_some(),
            "Java searchUsersエンドポイントが見つかりません"
        );
        if let Some(endpoint) = java_search_endpoint {
            assert_eq!(
                endpoint.params, "{\"q\", \"type=advanced\"}",
                "Java searchUsersのparams値が正しくありません"
            );
        }

        // Kotlin params付きエンドポイントのテスト
        let kotlin_v2_endpoint = endpoints
            .iter()
            .find(|e| e.method_name == "getUsersV2" && e.file_path.contains("UserController.kt"));
        assert!(
            kotlin_v2_endpoint.is_some(),
            "Kotlin getUsersV2エンドポイントが見つかりません"
        );
        if let Some(endpoint) = kotlin_v2_endpoint {
            assert_eq!(
                endpoint.params, "\"version=2\"",
                "Kotlin getUsersV2のparams値が正しくありません"
            );
        }

        let kotlin_search_endpoint = endpoints.iter().find(|e| {
            e.method_name == "searchKotlinUsers" && e.file_path.contains("UserController.kt")
        });
        assert!(
            kotlin_search_endpoint.is_some(),
            "Kotlin searchKotlinUsersエンドポイントが見つかりません"
        );
        if let Some(endpoint) = kotlin_search_endpoint {
            assert_eq!(
                endpoint.params, "\"type=kotlin\"",
                "Kotlin searchKotlinUsersのparams値が正しくありません"
            );
        }

        // params無しのエンドポイントもテスト
        let no_params_endpoint = endpoints.iter().find(|e| {
            e.method_name == "getAllUsers" && e.file_path.contains("UserController.java")
        });
        assert!(
            no_params_endpoint.is_some(),
            "params無しエンドポイントが見つかりません"
        );
        if let Some(endpoint) = no_params_endpoint {
            assert_eq!(
                endpoint.params, "",
                "params無しエンドポイントのparams値は空文字列であるべきです"
            );
        }

        Ok(())
    }
}
