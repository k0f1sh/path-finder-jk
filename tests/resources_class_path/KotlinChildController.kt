import org.springframework.web.bind.annotation.*

// Kotlin子クラス（Java親クラスを継承）
@RestController
@RequestMapping("/api/kotlin-child")
class KotlinChildController : JavaParentClass() {
    
    @GetMapping("/kotlin-child-method")
    fun kotlinChildMethod(): String {
        return "kotlin child"
    }
    
    @DeleteMapping("/{id}")
    fun deleteKotlin(): String {
        return "kotlin deleted"
    }
}