#[cfg(test)]
mod tests {
    use anyhow::Result;
    use path_finder::scan_directory;

    #[test]
    fn test_inheritance_support() -> Result<()> {
        // 継承のテストケースをスキャン
        let endpoints = scan_directory("tests/resources_inherit")?;

        // デバッグ出力
        println!("継承テストで検出されたエンドポイント:");
        for endpoint in &endpoints {
            println!("{:?}", endpoint);
        }

        // 期待されるエンドポイント数: 10個
        // Java: 子クラス2個 + 親クラス3個 = 5個
        // Kotlin: 子クラス2個 + 親クラス3個 = 5個
        assert_eq!(
            endpoints.len(),
            10,
            "継承対応後は親クラスのメソッドも含めて10個検出されるべき"
        );

        // Java ChildController の子クラスメソッド
        let java_child_specific = endpoints.iter().find(|e| {
            e.class_name == "ChildController"
                && e.method_name == "specificMethod"
                && e.path == "/api/child/specific"
                && e.http_method == "GET"
                && e.file_path.contains("ChildController.java")
        });
        assert!(
            java_child_specific.is_some(),
            "Java子クラスのspecificMethodが検出されませんでした"
        );

        let java_child_create = endpoints.iter().find(|e| {
            e.class_name == "ChildController"
                && e.method_name == "createSomething"
                && e.path == "/api/child/create/{name}"
                && e.http_method == "POST"
                && e.file_path.contains("ChildController.java")
        });
        assert!(
            java_child_create.is_some(),
            "Java子クラスのcreateSomethingが検出されませんでした"
        );

        // Java ChildController の継承されたメソッド（親クラスから）
        let java_inherited_health = endpoints.iter().find(|e| {
            e.class_name == "ChildController"
                && e.method_name == "health"
                && e.path == "/api/child/health"
                && e.http_method == "GET"
                && e.file_path.contains("ChildController.java")
        });
        assert!(
            java_inherited_health.is_some(),
            "Java継承されたhealthメソッドが検出されませんでした"
        );

        let java_inherited_status = endpoints.iter().find(|e| {
            e.class_name == "ChildController"
                && e.method_name == "getStatus"
                && e.path == "/api/child/status/{id}"
                && e.http_method == "GET"
                && e.file_path.contains("ChildController.java")
        });
        assert!(
            java_inherited_status.is_some(),
            "Java継承されたgetStatusメソッドが検出されませんでした"
        );

        let java_inherited_common = endpoints.iter().find(|e| {
            e.class_name == "ChildController"
                && e.method_name == "commonAction"
                && e.path == "/api/child/common"
                && e.http_method == "POST"
                && e.file_path.contains("ChildController.java")
        });
        assert!(
            java_inherited_common.is_some(),
            "Java継承されたcommonActionメソッドが検出されませんでした"
        );

        // Kotlin ChildController の子クラスメソッド
        let kotlin_child_specific = endpoints.iter().find(|e| {
            e.class_name == "ChildController"
                && e.method_name == "specificMethod"
                && e.path == "/api/kotlin/child/specific"
                && e.http_method == "GET"
                && e.file_path.contains("ChildController.kt")
        });
        assert!(
            kotlin_child_specific.is_some(),
            "Kotlin子クラスのspecificMethodが検出されませんでした"
        );

        let kotlin_child_create = endpoints.iter().find(|e| {
            e.class_name == "ChildController"
                && e.method_name == "createSomething"
                && e.path == "/api/kotlin/child/create/{name}"
                && e.http_method == "POST"
                && e.file_path.contains("ChildController.kt")
        });
        assert!(
            kotlin_child_create.is_some(),
            "Kotlin子クラスのcreateSomethingが検出されませんでした"
        );

        // Kotlin ChildController の継承されたメソッド（親クラスから）
        let kotlin_inherited_health = endpoints.iter().find(|e| {
            e.class_name == "ChildController"
                && e.method_name == "health"
                && e.path == "/api/kotlin/child/health"
                && e.http_method == "GET"
                && e.file_path.contains("ChildController.kt")
        });
        assert!(
            kotlin_inherited_health.is_some(),
            "Kotlin継承されたhealthメソッドが検出されませんでした"
        );

        let kotlin_inherited_status = endpoints.iter().find(|e| {
            e.class_name == "ChildController"
                && e.method_name == "getStatus"
                && e.path == "/api/kotlin/child/status/{id}"
                && e.http_method == "GET"
                && e.file_path.contains("ChildController.kt")
        });
        assert!(
            kotlin_inherited_status.is_some(),
            "Kotlin継承されたgetStatusメソッドが検出されませんでした"
        );

        let kotlin_inherited_common = endpoints.iter().find(|e| {
            e.class_name == "ChildController"
                && e.method_name == "commonAction"
                && e.path == "/api/kotlin/child/common"
                && e.http_method == "POST"
                && e.file_path.contains("ChildController.kt")
        });
        assert!(
            kotlin_inherited_common.is_some(),
            "Kotlin継承されたcommonActionメソッドが検出されませんでした"
        );

        // パラメータの検証も追加
        if let Some(endpoint) = java_inherited_status {
            assert_eq!(
                endpoint.parameters.len(),
                1,
                "getStatusメソッドはパラメータを1つ持つべき"
            );
            assert_eq!(
                endpoint.parameters[0].name, "id",
                "パラメータ名はidであるべき"
            );
            assert_eq!(
                endpoint.parameters[0].param_type, "Long",
                "パラメータ型はLongであるべき"
            );
            assert_eq!(
                endpoint.parameters[0].annotation, "PathVariable",
                "パラメータアノテーションはPathVariableであるべき"
            );
        }

        if let Some(endpoint) = java_inherited_common {
            assert_eq!(
                endpoint.parameters.len(),
                1,
                "commonActionメソッドはパラメータを1つ持つべき"
            );
            assert_eq!(
                endpoint.parameters[0].name, "data",
                "パラメータ名はdataであるべき"
            );
            assert_eq!(
                endpoint.parameters[0].param_type, "String",
                "パラメータ型はStringであるべき"
            );
            assert_eq!(
                endpoint.parameters[0].annotation, "RequestBody",
                "パラメータアノテーションはRequestBodyであるべき"
            );
        }

        println!("継承対応が正しく実装されています！");

        Ok(())
    }

    #[test]
    fn test_inheritance_path_combination() -> Result<()> {
        // パスの組み合わせが正しく行われることを確認
        let endpoints = scan_directory("tests/resources_inherit")?;

        // 子クラスの@RequestMappingパス + 親クラスのメソッドパスが正しく結合されることを確認
        let java_health_path = endpoints
            .iter()
            .find(|e| e.method_name == "health" && e.file_path.contains("ChildController.java"))
            .map(|e| &e.path);

        let kotlin_health_path = endpoints
            .iter()
            .find(|e| e.method_name == "health" && e.file_path.contains("ChildController.kt"))
            .map(|e| &e.path);

        assert_eq!(
            java_health_path,
            Some(&"/api/child/health".to_string()),
            "Javaの継承されたhealthメソッドのパスが正しく結合されていません"
        );

        assert_eq!(
            kotlin_health_path,
            Some(&"/api/kotlin/child/health".to_string()),
            "Kotlinの継承されたhealthメソッドのパスが正しく結合されていません"
        );

        Ok(())
    }

    #[test]
    fn test_no_duplicate_methods() -> Result<()> {
        // 同じメソッドが重複して検出されないことを確認
        let endpoints = scan_directory("tests/resources_inherit")?;

        // 各ファイル内で同じメソッド名が重複していないことを確認
        let java_methods: Vec<_> = endpoints
            .iter()
            .filter(|e| e.file_path.contains("ChildController.java"))
            .map(|e| &e.method_name)
            .collect();

        let kotlin_methods: Vec<_> = endpoints
            .iter()
            .filter(|e| e.file_path.contains("ChildController.kt"))
            .map(|e| &e.method_name)
            .collect();

        // 各ファイルで5つのメソッドが検出されるべき
        assert_eq!(
            java_methods.len(),
            5,
            "Javaファイルで5つのメソッドが検出されるべき"
        );
        assert_eq!(
            kotlin_methods.len(),
            5,
            "Kotlinファイルで5つのメソッドが検出されるべき"
        );

        // メソッド名の重複がないことを確認
        let mut java_unique_methods = java_methods.clone();
        java_unique_methods.sort();
        java_unique_methods.dedup();
        assert_eq!(
            java_methods.len(),
            java_unique_methods.len(),
            "Javaメソッドに重複があります"
        );

        let mut kotlin_unique_methods = kotlin_methods.clone();
        kotlin_unique_methods.sort();
        kotlin_unique_methods.dedup();
        assert_eq!(
            kotlin_methods.len(),
            kotlin_unique_methods.len(),
            "Kotlinメソッドに重複があります"
        );

        Ok(())
    }
}
