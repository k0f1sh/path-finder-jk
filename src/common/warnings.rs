/// 警告を出さない親クラス名のリスト
/// Spring標準クラスや一般的なJavaクラスの場合は警告を出さない
pub fn should_warn_about_missing_parent(parent_class_name: &str) -> bool {
    !matches!(
        parent_class_name,
        // Java標準クラス
        "Object" | "Exception" | "RuntimeException" | "Throwable" |
        "Enum" | "Record" | "Number" | "String" |
        
        // Spring Boot / Spring Framework標準クラス
        "BaseEntity" | "AbstractEntity" | "AbstractAggregateRoot" |
        "JpaRepository" | "CrudRepository" | "Repository" | "PagingAndSortingRepository" |
        "Controller" | "RestController" | "Component" | "Service" |
        "Configuration" | "ConfigurationProperties" |
        
        // JPA / Hibernate標準クラス
        "EntityListener" | "AbstractEntityListener" | "Auditable" |
        "Persistable" | "AbstractAuditable" | "AbstractPersistable" |
        
        // 一般的なライブラリクラス
        "ResponseEntity" | "HttpEntity" | "RequestEntity" |
        "Page" | "Pageable" | "Sort" | "Slice" |
        
        // よくあるベースクラス名
        "BaseController" | "AbstractController" | "BaseService" | "AbstractService" |
        "BaseRepository" | "AbstractRepository" | "BaseDomain" | "AbstractDomain" |
        "BaseDto" | "AbstractDto"
    )
}