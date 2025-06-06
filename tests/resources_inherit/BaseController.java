package com.example.demo.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.http.ResponseEntity;

// 親クラス - @RequestMappingアノテーションなし、GrandParentControllerを継承
public class BaseController extends GrandParentController {

    @GetMapping("/health")
    public ResponseEntity<String> health() {
        return ResponseEntity.ok("OK");
    }

    @GetMapping("/status/{id}")
    public ResponseEntity<String> getStatus(@PathVariable Long id) {
        return ResponseEntity.ok("Status for " + id);
    }

    @PostMapping("/common")
    public ResponseEntity<String> commonAction(@RequestBody String data) {
        return ResponseEntity.ok("Common action: " + data);
    }
} 