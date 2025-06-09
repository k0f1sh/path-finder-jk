import org.springframework.web.bind.annotation.*;

// ファイル名とクラス名の不一致をテストするための親クラス（@RequestMappingなし）
public class SomeParentClass {
    
    @GetMapping("/method")
    public String parentMethod() {
        return "parent";
    }
    
    @PostMapping("/create")
    public String createParent() {
        return "parent created";
    }
}