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

        // 期待されるエンドポイント数: 14個（多重継承対応後）
        // Java: 子クラス2個 + 親クラス3個 + 祖父クラス2個 = 7個
        // Kotlin: 子クラス2個 + 親クラス3個 + 祖父クラス2個 = 7個
        assert_eq!(
            endpoints.len(),
            14,
            "多重継承対応後は祖父クラスのメソッドも含めて14個検出されるべき"
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
            e.class_name == "BaseController"
                && e.method_name == "health"
                && e.path == "/api/child/health"
                && e.http_method == "GET"
                && e.file_path.contains("BaseController.java")
        });
        assert!(
            java_inherited_health.is_some(),
            "Java継承されたhealthメソッドが検出されませんでした"
        );

        let java_inherited_status = endpoints.iter().find(|e| {
            e.class_name == "BaseController"
                && e.method_name == "getStatus"
                && e.path == "/api/child/status/{id}"
                && e.http_method == "GET"
                && e.file_path.contains("BaseController.java")
        });
        assert!(
            java_inherited_status.is_some(),
            "Java継承されたgetStatusメソッドが検出されませんでした"
        );

        let java_inherited_common = endpoints.iter().find(|e| {
            e.class_name == "BaseController"
                && e.method_name == "commonAction"
                && e.path == "/api/child/common"
                && e.http_method == "POST"
                && e.file_path.contains("BaseController.java")
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
            e.class_name == "BaseController"
                && e.method_name == "health"
                && e.path == "/api/kotlin/child/health"
                && e.http_method == "GET"
                && e.file_path.contains("BaseController.kt")
        });
        assert!(
            kotlin_inherited_health.is_some(),
            "Kotlin継承されたhealthメソッドが検出されませんでした"
        );

        let kotlin_inherited_status = endpoints.iter().find(|e| {
            e.class_name == "BaseController"
                && e.method_name == "getStatus"
                && e.path == "/api/kotlin/child/status/{id}"
                && e.http_method == "GET"
                && e.file_path.contains("BaseController.kt")
        });
        assert!(
            kotlin_inherited_status.is_some(),
            "Kotlin継承されたgetStatusメソッドが検出されませんでした"
        );

        let kotlin_inherited_common = endpoints.iter().find(|e| {
            e.class_name == "BaseController"
                && e.method_name == "commonAction"
                && e.path == "/api/kotlin/child/common"
                && e.http_method == "POST"
                && e.file_path.contains("BaseController.kt")
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
            .find(|e| e.method_name == "health" && e.file_path.contains("BaseController.java"))
            .map(|e| &e.path);

        let kotlin_health_path = endpoints
            .iter()
            .find(|e| e.method_name == "health" && e.file_path.contains("BaseController.kt"))
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
        let java_child_methods: Vec<_> = endpoints
            .iter()
            .filter(|e| e.file_path.contains("ChildController.java"))
            .map(|e| &e.method_name)
            .collect();

        let java_base_methods: Vec<_> = endpoints
            .iter()
            .filter(|e| e.file_path.contains("BaseController.java"))
            .map(|e| &e.method_name)
            .collect();

        let kotlin_child_methods: Vec<_> = endpoints
            .iter()
            .filter(|e| e.file_path.contains("ChildController.kt"))
            .map(|e| &e.method_name)
            .collect();

        let kotlin_base_methods: Vec<_> = endpoints
            .iter()
            .filter(|e| e.file_path.contains("BaseController.kt"))
            .map(|e| &e.method_name)
            .collect();

        // 子クラスファイルで2つのメソッドが検出されるべき
        assert_eq!(
            java_child_methods.len(),
            2,
            "Java子クラスファイルで2つのメソッドが検出されるべき"
        );
        assert_eq!(
            kotlin_child_methods.len(),
            2,
            "Kotlin子クラスファイルで2つのメソッドが検出されるべき"
        );

        // 親クラスファイルで3つのメソッドが検出されるべき
        assert_eq!(
            java_base_methods.len(),
            3,
            "Java親クラスファイルで3つのメソッドが検出されるべき"
        );
        assert_eq!(
            kotlin_base_methods.len(),
            3,
            "Kotlin親クラスファイルで3つのメソッドが検出されるべき"
        );

        // 祖父クラスファイルで2つのメソッドが検出されるべき（多重継承対応後）
        let java_grandparent_methods: Vec<_> = endpoints
            .iter()
            .filter(|e| e.file_path.contains("GrandParentController.java"))
            .map(|e| &e.method_name)
            .collect();

        let kotlin_grandparent_methods: Vec<_> = endpoints
            .iter()
            .filter(|e| e.file_path.contains("GrandParentController.kt"))
            .map(|e| &e.method_name)
            .collect();

        assert_eq!(
            java_grandparent_methods.len(),
            2,
            "Java祖父クラスファイルで2つのメソッドが検出されるべき"
        );
        assert_eq!(
            kotlin_grandparent_methods.len(),
            2,
            "Kotlin祖父クラスファイルで2つのメソッドが検出されるべき"
        );

        // メソッド名の重複がないことを確認
        let mut java_child_unique_methods = java_child_methods.clone();
        java_child_unique_methods.sort();
        java_child_unique_methods.dedup();
        assert_eq!(
            java_child_methods.len(),
            java_child_unique_methods.len(),
            "Java子クラスメソッドに重複があります"
        );

        let mut java_base_unique_methods = java_base_methods.clone();
        java_base_unique_methods.sort();
        java_base_unique_methods.dedup();
        assert_eq!(
            java_base_methods.len(),
            java_base_unique_methods.len(),
            "Java親クラスメソッドに重複があります"
        );

        let mut kotlin_child_unique_methods = kotlin_child_methods.clone();
        kotlin_child_unique_methods.sort();
        kotlin_child_unique_methods.dedup();
        assert_eq!(
            kotlin_child_methods.len(),
            kotlin_child_unique_methods.len(),
            "Kotlin子クラスメソッドに重複があります"
        );

        let mut kotlin_base_unique_methods = kotlin_base_methods.clone();
        kotlin_base_unique_methods.sort();
        kotlin_base_unique_methods.dedup();
        assert_eq!(
            kotlin_base_methods.len(),
            kotlin_base_unique_methods.len(),
            "Kotlin親クラスメソッドに重複があります"
        );

        // 祖父クラスメソッドの重複チェック（多重継承対応後）
        let mut java_grandparent_unique_methods = java_grandparent_methods.clone();
        java_grandparent_unique_methods.sort();
        java_grandparent_unique_methods.dedup();
        assert_eq!(
            java_grandparent_methods.len(),
            java_grandparent_unique_methods.len(),
            "Java祖父クラスメソッドに重複があります"
        );

        let mut kotlin_grandparent_unique_methods = kotlin_grandparent_methods.clone();
        kotlin_grandparent_unique_methods.sort();
        kotlin_grandparent_unique_methods.dedup();
        assert_eq!(
            kotlin_grandparent_methods.len(),
            kotlin_grandparent_unique_methods.len(),
            "Kotlin祖父クラスメソッドに重複があります"
        );

        Ok(())
    }

    #[test]
    fn test_multiple_inheritance_chain() -> Result<()> {
        // 多重継承チェーンのテスト: GrandParent -> Base -> Child
        let endpoints = scan_directory("tests/resources_inherit")?;

        // デバッグ出力
        println!("多重継承チェーンテストで検出されたエンドポイント:");
        for endpoint in &endpoints {
            println!("{:?}", endpoint);
        }

        // 期待されるエンドポイント数: 14個
        // Java: 子クラス2個 + 親クラス3個 + 祖父クラス2個 = 7個
        // Kotlin: 子クラス2個 + 親クラス3個 + 祖父クラス2個 = 7個
        println!("検出されたエンドポイント数: {}", endpoints.len());

        // 現在は10個しか検出されない（祖父クラスのメソッドが検出されない）
        // 多重継承対応後は14個になるべき

        // 祖父クラスのメソッドが継承されているかチェック
        let java_grandparent_method = endpoints.iter().find(|e| {
            e.class_name == "GrandParentController"
                && e.method_name == "grandparentMethod"
                && e.path == "/api/child/grandparent"
                && e.http_method == "GET"
                && e.file_path.contains("GrandParentController.java")
        });

        let java_legacy_method = endpoints.iter().find(|e| {
            e.class_name == "GrandParentController"
                && e.method_name == "legacyAction"
                && e.path == "/api/child/legacy/{id}"
                && e.http_method == "POST"
                && e.file_path.contains("GrandParentController.java")
        });

        let kotlin_grandparent_method = endpoints.iter().find(|e| {
            e.class_name == "GrandParentController"
                && e.method_name == "grandparentMethod"
                && e.path == "/api/kotlin/child/grandparent"
                && e.http_method == "GET"
                && e.file_path.contains("GrandParentController.kt")
        });

        let kotlin_legacy_method = endpoints.iter().find(|e| {
            e.class_name == "GrandParentController"
                && e.method_name == "legacyAction"
                && e.path == "/api/kotlin/child/legacy/{id}"
                && e.http_method == "POST"
                && e.file_path.contains("GrandParentController.kt")
        });

        // 現在の実装では祖父クラスのメソッドは検出されない
        if java_grandparent_method.is_none() {
            println!(
                "❌ Java祖父クラスのgrandparentMethodが検出されませんでした（多重継承未対応）"
            );
        } else {
            println!("✅ Java祖父クラスのgrandparentMethodが検出されました");
        }

        if java_legacy_method.is_none() {
            println!("❌ Java祖父クラスのlegacyActionが検出されませんでした（多重継承未対応）");
        } else {
            println!("✅ Java祖父クラスのlegacyActionが検出されました");
        }

        if kotlin_grandparent_method.is_none() {
            println!(
                "❌ Kotlin祖父クラスのgrandparentMethodが検出されませんでした（多重継承未対応）"
            );
        } else {
            println!("✅ Kotlin祖父クラスのgrandparentMethodが検出されました");
        }

        if kotlin_legacy_method.is_none() {
            println!("❌ Kotlin祖父クラスのlegacyActionが検出されませんでした（多重継承未対応）");
        } else {
            println!("✅ Kotlin祖父クラスのlegacyActionが検出されました");
        }

        // 多重継承対応が実装されたので、アサーションを有効にする
        assert_eq!(
            endpoints.len(),
            14,
            "多重継承対応後は祖父クラスのメソッドも含めて14個検出されるべき"
        );

        assert!(
            java_grandparent_method.is_some(),
            "Java祖父クラスのgrandparentMethodが検出されませんでした"
        );

        assert!(
            java_legacy_method.is_some(),
            "Java祖父クラスのlegacyActionが検出されませんでした"
        );

        assert!(
            kotlin_grandparent_method.is_some(),
            "Kotlin祖父クラスのgrandparentMethodが検出されませんでした"
        );

        assert!(
            kotlin_legacy_method.is_some(),
            "Kotlin祖父クラスのlegacyActionが検出されませんでした"
        );

        println!("多重継承チェーンのテストが完了しました。多重継承が正しく実装されています！");

        Ok(())
    }
}
