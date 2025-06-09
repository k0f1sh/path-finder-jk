import org.springframework.web.bind.annotation.*;

// 正しく検索される親クラスを持つコントローラー
@RestController
@RequestMapping("/api/child")
public class ValidParentController extends SomeParentClass {
    
    @GetMapping("/child-method")
    public String childMethod() {
        return "child";
    }
    
    @DeleteMapping("/{id}")
    public String delete() {
        return "deleted";
    }
}