import org.springframework.web.bind.annotation.*;

// 外部ライブラリのクラスを継承（JpaRepositoryは存在しない）
@RestController
@RequestMapping("/api/external")
public class ExternalLibraryController extends JpaRepository {
    
    @GetMapping("/list")
    public String getList() {
        return "list";
    }
    
    @DeleteMapping("/{id}")
    public String delete() {
        return "deleted";
    }
}