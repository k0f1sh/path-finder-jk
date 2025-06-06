package com.example.demo.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.http.ResponseEntity;

// 祖父クラス - @RequestMappingアノテーションなし
public class GrandParentController {

    @GetMapping("/grandparent")
    public ResponseEntity<String> grandparentMethod() {
        return ResponseEntity.ok("GrandParent method");
    }

    @PostMapping("/legacy/{id}")
    public ResponseEntity<String> legacyAction(@PathVariable Long id) {
        return ResponseEntity.ok("Legacy action for " + id);
    }
} 