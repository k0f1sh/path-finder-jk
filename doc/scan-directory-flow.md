# scan-directory 処理フロー

このドキュメントでは、path-finderの`scan-directory`コマンドの詳細な処理フローを説明します。

## 概要フロー

```mermaid
flowchart TD
    A[CLI Command: scan-directory] --> B{JSON出力フラグ?}
    B -->|Yes| C[scan_directory_json]
    B -->|No| D[scan_directory]
    C --> E[scan_directory_internal with json=true]
    D --> F[scan_directory_internal with json=false]
    E --> G[処理結果をJSON形式で返却]
    F --> H[処理結果をEndpoint配列で返却]
    G --> I[JSON文字列を出力]
    H --> J[フォーマット済みテキストを出力]
```

## 詳細処理フロー

```mermaid
flowchart TD
    Start([scan_directory開始]) --> Init[all_endpoints = Vec::new<br/>初期化]
    Init --> WalkDir[WalkDir::new<br/>ディレクトリ走査開始]
    
    WalkDir --> FileCheck{ファイル?}
    FileCheck -->|No| WalkDir
    FileCheck -->|Yes| ExtCheck{拡張子判定}
    
    ExtCheck -->|.java| JavaPath[Javaファイル処理パス]
    ExtCheck -->|.kt| KotlinPath[Kotlinファイル処理パス]
    ExtCheck -->|Other| WalkDir
    
    %% Java処理パス
    JavaPath --> JavaHasRM{java::has_request_mapping<br/>@RequestMapping有り?}
    JavaHasRM -->|No| WalkDir
    JavaHasRM -->|Yes| JavaExtract[java::extract_request_mapping<br/>_with_inheritance]
    
    %% Kotlin処理パス
    KotlinPath --> KotlinHasRM{kotlin::has_request_mapping<br/>@RequestMapping有り?}
    KotlinHasRM -->|No| WalkDir
    KotlinHasRM -->|Yes| KotlinExtract[kotlin::extract_request_mapping<br/>_with_inheritance]
    
    JavaExtract --> JavaInherit[Java継承処理]
    KotlinExtract --> KotlinInherit[Kotlin継承処理]
    
    JavaInherit --> AddEndpoints[all_endpoints.extend<br/>エンドポイント追加]
    KotlinInherit --> AddEndpoints
    
    AddEndpoints --> MoreFiles{他にファイル有り?}
    MoreFiles -->|Yes| WalkDir
    MoreFiles -->|No| OutputCheck{JSON出力?}
    
    OutputCheck -->|Yes| JsonOutput[serde_json::to_string_pretty<br/>JSON変換]
    OutputCheck -->|No| VecOutput[Endpoint配列を返却]
    
    JsonOutput --> End([完了])
    VecOutput --> End
```

## 継承処理の詳細フロー

```mermaid
flowchart TD
    Start([extract_request_mapping_with_inheritance開始]) --> ExtractOwn[自クラスのエンドポイント抽出]
    ExtractOwn --> CheckInherit{継承有り?}
    
    CheckInherit -->|No| Return[エンドポイント配列を返却]
    CheckInherit -->|Yes| CreateTasks[InheritanceTask作成]
    
    CreateTasks --> ProcessQueue[process_inheritance_queue<br/>継承キュー処理]
    
    ProcessQueue --> QueueEmpty{キューが空?}
    QueueEmpty -->|Yes| Combine[継承エンドポイントをマージ]
    QueueEmpty -->|No| PopTask[タスクをキューから取得]
    
    PopTask --> AlreadyProcessed{処理済み?<br/>無限ループ防止}
    AlreadyProcessed -->|Yes| QueueEmpty
    AlreadyProcessed -->|No| FindParent[find_parent_class_file<br/>親クラスファイル検索]
    
    FindParent --> ParentFound{親クラス<br/>見つかった?}
    ParentFound -->|No| ShowWarning[Warning出力<br/>Spring標準クラスは除外]
    ParentFound -->|Yes| ExtensionCheck{ファイル拡張子?}
    
    ShowWarning --> QueueEmpty
    
    ExtensionCheck -->|.java| JavaParent[Javaパーサーで<br/>親クラス処理]
    ExtensionCheck -->|.kt| KotlinParent[Kotlinパーサーで<br/>親クラス処理]
    
    JavaParent --> ExtractParentMethods[親クラスメソッド抽出<br/>パス結合処理]
    KotlinParent --> ExtractParentMethods
    
    ExtractParentMethods --> AddInherited[継承エンドポイント追加]
    AddInherited --> CheckGrandParent[さらに上位の継承確認]
    
    CheckGrandParent --> HasGrandParent{祖先クラス有り?}
    HasGrandParent -->|Yes| AddGrandTask[祖先クラスタスクを<br/>キューに追加]
    HasGrandParent -->|No| QueueEmpty
    
    AddGrandTask --> QueueEmpty
    
    Combine --> Return
```

## クロス言語継承の処理

```mermaid
flowchart TD
    Start([クロス言語継承開始]) --> ChildLang{子クラス言語}
    
    ChildLang -->|Java| JavaChild[Javaパーサーで解析]
    ChildLang -->|Kotlin| KotlinChild[Kotlinパーサーで解析]
    
    JavaChild --> JavaFindParent[find_parent_class_file<br/>.java, .kt両方検索]
    KotlinChild --> KotlinFindParent[find_parent_class_file<br/>.kt, .java両方検索]
    
    JavaFindParent --> ParentFileFound{親ファイル発見?}
    KotlinFindParent --> ParentFileFound
    
    ParentFileFound -->|No| Warning[Warning出力]
    ParentFileFound -->|Yes| ParentExt{親ファイル拡張子}
    
    Warning --> End([処理終了])
    
    ParentExt -->|.java| UseJavaParser[Javaパーサーで<br/>親クラス解析]
    ParentExt -->|.kt| UseKotlinParser[Kotlinパーサーで<br/>親クラス解析]
    
    UseJavaParser --> VerifyJava[verify_class_name_in_java_file<br/>クラス名検証]
    UseKotlinParser --> VerifyKotlin[verify_class_name_in_kotlin_file<br/>クラス名検証]
    
    VerifyJava --> ExtractMethods[適切なパーサーで<br/>メソッド抽出]
    VerifyKotlin --> ExtractMethods
    
    ExtractMethods --> PathCombine[子クラスbase_path +<br/>親クラスmethod_path]
    PathCombine --> End
```

## エラーハンドリング

```mermaid
flowchart TD
    Process[処理実行] --> Error{エラー発生?}
    
    Error -->|No| Success[正常終了]
    Error -->|Yes| ErrorType{エラー種別}
    
    ErrorType -->|ファイル読み込み| FileError[ファイルアクセスエラー<br/>Context付きで報告]
    ErrorType -->|パース失敗| ParseError[tree-sitterパースエラー<br/>panic!で終了]
    ErrorType -->|親クラス未発見| Warning[Warning出力<br/>処理は継続]
    ErrorType -->|JSON変換失敗| JsonError[serde_jsonエラー<br/>Result::Errで返却]
    
    FileError --> Propagate[エラーを上位に伝播]
    ParseError --> Crash[プロセス終了]
    Warning --> Continue[処理継続]
    JsonError --> Propagate
    
    Propagate --> End([エラー終了])
    Crash --> End
    Continue --> Success
    Success --> End([正常終了])
```

## 主要なデータ構造

### Endpoint
```rust
struct Endpoint {
    class_name: String,     // クラス名
    method_name: String,    // メソッド名
    http_method: String,    // HTTP動詞 (GET, POST, etc.)
    path: String,          // エンドポイントパス
    parameters: Vec<Parameter>, // パラメータ情報
    line_range: (usize, usize), // ファイル内行番号
    file_path: String,     // ファイルパス
    headers: String,       // ヘッダー情報
}
```

### InheritanceTask
```rust
struct InheritanceTask {
    child_file_path: String,    // 子クラスファイルパス
    child_class_name: String,   // 子クラス名
    child_base_path: Option<String>, // 子クラスのbase path
    parent_class_name: String,  // 親クラス名
}
```

## パフォーマンス特性

- **時間計算量**: O(n×m) (n=ファイル数, m=平均継承深度)
- **空間計算量**: O(k) (k=総エンドポイント数)
- **並列処理**: なし (シングルスレッド)
- **メモリ使用**: 全エンドポイントをメモリに保持

## 制限事項

- 循環継承の検出と防止
- Spring標準クラスの警告除外
- ファイル名とクラス名の不一致への対応
- tree-sitterパーサーの制約に依存