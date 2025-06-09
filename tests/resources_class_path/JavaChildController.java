import org.springframework.web.bind.annotation.*;

// Java子クラス（Kotlin親クラスを継承）
@RestController
@RequestMapping("/api/java-child")
public class JavaChildController extends KotlinParentClass {
    
    @GetMapping("/java-child-method")
    public String javaChildMethod() {
        return "java child";
    }
    
    @PutMapping("/{id}")
    public String updateJava() {
        return "java updated";
    }
}