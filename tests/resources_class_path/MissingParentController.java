import org.springframework.web.bind.annotation.*;

// 存在しないカスタムクラスを継承
@RestController
@RequestMapping("/api/missing")
public class MissingParentController extends NonExistentParentClass {
    
    @GetMapping("/test")
    public String getTest() {
        return "test";
    }
    
    @PutMapping("/update")
    public String update() {
        return "updated";
    }
}