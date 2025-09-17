# path-finder
- Java, KotlinのファイルからSpringのパス情報を出力
- tree-sitterを使った構文解析
- Rust製


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
    "file_path": "src/main/java/com/example/UserController.java",
    "headers": "",
    "params": ""
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
    "file_path": "src/main/java/com/example/UserController.java",
    "headers": "",
    "params": ""
  },
  {
    "class_name": "UserController",
    "method_name": "getUsersV1",
    "http_method": "GET",
    "path": "/api/users",
    "parameters": [],
    "line_range": [49, 52],
    "file_path": "src/main/java/com/example/UserController.java",
    "headers": "",
    "params": "version=1"
  }
]
```

## 機能

### 基本機能
- Java/Kotlinファイルの構文解析（tree-sitter使用）
- Spring Framework の RequestMapping アノテーション解析
- エンドポイント情報の抽出（HTTP メソッド、パス、パラメータ、headers、params）
- JSON/テキスト形式での出力
- アノテーション属性の詳細抽出
  - `headers`: リクエストヘッダー条件の抽出
  - `params`: リクエストパラメータ条件の抽出

### 継承対応
本ツールは、Spring Controller クラスの継承関係を完全にサポートしています。

#### 単一継承
```java
// 親クラス（@RequestMapping なし）
public class BaseController {
    @GetMapping("/health")
    public String health() { return "OK"; }
}

// 子クラス（@RequestMapping あり）
@RestController
@RequestMapping("/api/users")
public class UserController extends BaseController {
    @PostMapping("/create")
    public String create() { return "Created"; }
}
```

**検出結果:**
- `POST /api/users/create` (UserController#create)
- `GET /api/users/health` (BaseController#health) ← 継承されたメソッド

#### 多重継承チェーン
```java
// 祖父クラス
public class GrandParentController {
    @GetMapping("/legacy")
    public String legacy() { return "Legacy"; }
}

// 親クラス
public class BaseController extends GrandParentController {
    @GetMapping("/health")
    public String health() { return "OK"; }
}

// 子クラス
@RestController
@RequestMapping("/api/users")
public class UserController extends BaseController {
    @PostMapping("/create")
    public String create() { return "Created"; }
}
```

**検出結果:**
- `POST /api/users/create` (UserController#create)
- `GET /api/users/health` (BaseController#health)
- `GET /api/users/legacy` (GrandParentController#legacy) ← 多重継承で検出

#### 技術的詳細
- **キューベースの継承処理**: 継承チェーンを再帰的に辿り、すべての祖先クラスのメソッドを検出
- **無限ループ防止**: 処理済みクラスを記録し、循環継承を安全に処理
- **パス結合**: 子クラスの `@RequestMapping` パスと親クラスのメソッドパスを適切に結合
- **Java/Kotlin両対応**: 両言語で同等の継承処理を実装
- **クロス言語継承**: JavaからKotlin、KotlinからJavaの継承関係もサポート

### アノテーション属性サポート
本ツールは、Spring Framework の様々なアノテーション属性を抽出できます。

#### Headers 抽出
```java
@RequestMapping(method = RequestMethod.POST, value = "/{id}", headers = "X-Custom-Header")
public ResponseEntity<?> createUser2(@RequestBody User user) {
    return ResponseEntity.ok(userService.save(user));
}
```

#### Params 抽出
```java
// 単一パラメータ条件
@GetMapping(params = "version=1")
public ResponseEntity<?> getUsersV1() {
    return ResponseEntity.ok(userService.findAll());
}

// 複数パラメータ条件
@RequestMapping(method = RequestMethod.GET, value = "/search", params = {"q", "type=advanced"})
public ResponseEntity<?> searchUsers() {
    return ResponseEntity.ok(userService.findAll());
}
```

**Kotlin対応:**
```kotlin
// 単一パラメータ条件
@GetMapping(params = ["version=2"])
fun getUsersV2(): ResponseEntity<*> {
    return ResponseEntity.ok(userService.findAll())
}

// 複数パラメータ条件
@RequestMapping(method = [RequestMethod.GET], value = ["/search"], params = ["q", "type=kotlin"])
fun searchKotlinUsers(): ResponseEntity<*> {
    return ResponseEntity.ok(userService.findAll())
}
```

## 対応アノテーション

### マッピングアノテーション
- `@RequestMapping`
- `@GetMapping`
- `@PostMapping`
- `@PutMapping`
- `@DeleteMapping`
- `@PatchMapping`

### パラメータアノテーション
- `@PathVariable`
- `@RequestBody`
- `@RequestParam`

### 属性抽出
- `value` / `path`: エンドポイントパス
- `method`: HTTPメソッド
- `headers`: リクエストヘッダー条件
- `params`: リクエストパラメータ条件

## TODO

- [x] Javaファイルのパース
- [x] Spring FrameworkのRequestMappingアノテーションの解析
- [x] エンドポイント情報の抽出
- [x] Kotlinファイルのサポート
  - [x] Kotlinパーサーの追加
  - [x] Spring FrameworkアノテーションのKotlin構文対応
- [x] 継承対応
  - [x] 単一継承のサポート
  - [x] 多重継承チェーンのサポート
  - [x] 無限ループ防止機能
  - [x] クロス言語継承サポート（Java↔Kotlin）
- [x] アノテーション属性サポート
  - [x] headers属性の抽出
  - [x] params属性の抽出
  - [x] 複数パラメータ条件の対応
