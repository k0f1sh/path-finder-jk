import org.springframework.web.bind.annotation.*;

// Java親クラス（Kotlinの子クラスから継承される）
public class JavaParentClass {
    
    @GetMapping("/java-method")
    public String javaParentMethod() {
        return "java parent";
    }
    
    @PostMapping("/java-create")
    public String createFromJava() {
        return "java created";
    }
}