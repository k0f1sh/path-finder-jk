use path_finder::{scan_directory, Endpoint};
use std::collections::HashMap;

#[test]
fn test_empty_mapping_java() {
    let result = scan_directory("tests/resources");
    assert!(result.is_ok());
    
    let endpoints = result.unwrap();
    let empty_mapping_endpoints: Vec<&Endpoint> = endpoints
        .iter()
        .filter(|e| e.class_name == "EmptyMappingController" && e.file_path.ends_with(".java"))
        .collect();
    
    assert!(!empty_mapping_endpoints.is_empty(), "EmptyMappingController のエンドポイントが見つかりません");
    
    // エンドポイントをメソッド名でマッピング
    let endpoint_map: HashMap<&str, &Endpoint> = empty_mapping_endpoints
        .iter()
        .map(|e| (e.method_name.as_str(), *e))
        .collect();
    
    // 空文字列mappingの場合、クラス側のpathが使われるべき
    if let Some(get_endpoint) = endpoint_map.get("getUsersWithEmptyMapping") {
        assert_eq!(get_endpoint.path, "/api/users", 
            "空文字列mappingでクラス側のpathが使われていない: {}", get_endpoint.path);
        assert_eq!(get_endpoint.http_method, "GET");
    } else {
        panic!("getUsersWithEmptyMapping メソッドが見つかりません");
    }
    
    if let Some(post_endpoint) = endpoint_map.get("createUserWithEmptyMapping") {
        assert_eq!(post_endpoint.path, "/api/users", 
            "空文字列mappingでクラス側のpathが使われていない: {}", post_endpoint.path);
        assert_eq!(post_endpoint.http_method, "POST");
    } else {
        panic!("createUserWithEmptyMapping メソッドが見つかりません");
    }
    
    // 通常のmappingは結合されるべき
    if let Some(details_endpoint) = endpoint_map.get("getUserDetails") {
        assert_eq!(details_endpoint.path, "/api/users/details", 
            "通常のpath結合が正しく動作していない: {}", details_endpoint.path);
        assert_eq!(details_endpoint.http_method, "GET");
    } else {
        panic!("getUserDetails メソッドが見つかりません");
    }
}

#[test]
fn test_empty_mapping_kotlin() {
    let result = scan_directory("tests/resources");
    assert!(result.is_ok());
    
    let endpoints = result.unwrap();
    let empty_mapping_endpoints: Vec<&Endpoint> = endpoints
        .iter()
        .filter(|e| e.class_name == "EmptyMappingController" && e.file_path.ends_with(".kt"))
        .collect();
    
    assert!(!empty_mapping_endpoints.is_empty(), "Kotlin EmptyMappingController のエンドポイントが見つかりません");
    
    // エンドポイントをメソッド名でマッピング
    let endpoint_map: HashMap<&str, &Endpoint> = empty_mapping_endpoints
        .iter()
        .map(|e| (e.method_name.as_str(), *e))
        .collect();
    
    // 空文字列mappingの場合、クラス側のpathが使われるべき
    if let Some(get_endpoint) = endpoint_map.get("getProductsWithEmptyMapping") {
        assert_eq!(get_endpoint.path, "/api/products", 
            "Kotlin: 空文字列mappingでクラス側のpathが使われていない: {}", get_endpoint.path);
        assert_eq!(get_endpoint.http_method, "GET");
    } else {
        panic!("getProductsWithEmptyMapping メソッドが見つかりません");
    }
    
    if let Some(post_endpoint) = endpoint_map.get("createProductWithEmptyMapping") {
        assert_eq!(post_endpoint.path, "/api/products", 
            "Kotlin: 空文字列mappingでクラス側のpathが使われていない: {}", post_endpoint.path);
        assert_eq!(post_endpoint.http_method, "POST");
    } else {
        panic!("createProductWithEmptyMapping メソッドが見つかりません");
    }
    
    // 通常のmappingは結合されるべき
    if let Some(details_endpoint) = endpoint_map.get("getProductDetails") {
        assert_eq!(details_endpoint.path, "/api/products/details", 
            "Kotlin: 通常のpath結合が正しく動作していない: {}", details_endpoint.path);
        assert_eq!(details_endpoint.http_method, "GET");
    } else {
        panic!("getProductDetails メソッドが見つかりません");
    }
}