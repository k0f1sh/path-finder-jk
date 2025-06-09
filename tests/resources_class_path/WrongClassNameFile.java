import org.springframework.web.bind.annotation.*;

// ファイル名とクラス名が異なるケース（ファイル名: WrongClassNameFile.java, クラス名: ActualClassName）
@RestController
@RequestMapping("/api/wrong")
public class ActualClassName extends SomeParentClass {
    
    @GetMapping("/actual")
    public String getActual() {
        return "actual";
    }
}