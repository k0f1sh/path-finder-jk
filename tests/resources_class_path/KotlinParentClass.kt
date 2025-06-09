import org.springframework.web.bind.annotation.*

// Kotlin親クラス（Javaの子クラスから継承される）
class KotlinParentClass {
    
    @GetMapping("/kotlin-method")
    fun kotlinParentMethod(): String {
        return "kotlin parent"
    }
    
    @PostMapping("/kotlin-create")
    fun createFromKotlin(): String {
        return "kotlin created"
    }
}