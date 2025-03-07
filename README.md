# path-finder

- Rust製のCLIツール
- tree-sitterを使った構文解析
- Java, KotlinのファイルからSpringのコントローラークラスを抽出
- さらにそこで定義されているRequstMappingアノテーションからエンドポイントとそのソースコードのgithubのリンクを生成する

## Usage

```bash
# ディレクトリ内のJavaファイルからエンドポイントを抽出
path-finder scan-directory path/to/your/java/sources

# 例：
path-finder scan-directory src/main/java
```

出力例：
```
GET /api/users (UserController#getAllUsers) [src/main/java/com/example/UserController.java:24]
GET /api/users/{id} (UserController#getUserById) [src/main/java/com/example/UserController.java:29]
POST /api/users (UserController#createUser) [src/main/java/com/example/UserController.java:34]
```

## TODO

- [x] Javaファイルのパース
- [x] Spring FrameworkのRequestMappingアノテーションの解析
- [x] エンドポイント情報の抽出
- [ ] Kotlinファイルのサポート
  - [ ] Kotlinパーサーの追加
  - [ ] Spring FrameworkアノテーションのKotlin構文対応