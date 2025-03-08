# path-finder

- Rust製のCLIツール
- tree-sitterを使った構文解析
- Java, KotlinのファイルからSpringのコントローラークラスを抽出
- さらにそこで定義されているRequstMappingアノテーションからエンドポイントとそのソースコードのgithubのリンクを生成する

## Usage

```bash
# ディレクトリ内のJavaファイルからエンドポイントを抽出
path-finder scan-directory path/to/your/java/sources

# JSON形式で出力する場合
path-finder scan-directory path/to/your/java/sources --json

# 例：
path-finder scan-directory src/main/java
path-finder scan-directory src/main/java --json
```

出力例：
```
GET /api/users (UserController#getAllUsers) [src/main/java/com/example/UserController.java:24]
GET /api/users/{id} (UserController#getUserById) [src/main/java/com/example/UserController.java:29]
POST /api/users (UserController#createUser) [src/main/java/com/example/UserController.java:34]
```

JSON出力例：
```json
[
  {
    "class_name": "UserController",
    "method_name": "getAllUsers",
    "http_method": "GET",
    "path": "/api/users",
    "parameters": [],
    "line_range": [24, 27],
    "file_path": "src/main/java/com/example/UserController.java"
  },
  {
    "class_name": "UserController",
    "method_name": "getUserById",
    "http_method": "GET",
    "path": "/api/users/{id}",
    "parameters": [
      {
        "name": "id",
        "param_type": "Long",
        "annotation": "PathVariable"
      }
    ],
    "line_range": [29, 32],
    "file_path": "src/main/java/com/example/UserController.java"
  }
]
```

## TODO

- [x] Javaファイルのパース
- [x] Spring FrameworkのRequestMappingアノテーションの解析
- [x] エンドポイント情報の抽出
- [x] Kotlinファイルのサポート
  - [x] Kotlinパーサーの追加
  - [x] Spring FrameworkアノテーションのKotlin構文対応
  - [ ] GetMappingなどでconstructor_invocationじゃないパターンの対応
