import org.springframework.web.bind.annotation.*;

// Spring標準クラスを継承するコントローラー（BaseEntityは存在しない）
@RestController
@RequestMapping("/api/spring")
public class SpringStandardParentController extends BaseEntity {
    
    @GetMapping("/standard")
    public String getStandard() {
        return "standard";
    }
    
    @PostMapping("/create")
    public String create() {
        return "created";
    }
}