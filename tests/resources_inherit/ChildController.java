package com.example.demo.controller;

import org.springframework.web.bind.annotation.RestController;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.http.ResponseEntity;

// 子クラス - @RequestMappingアノテーションあり、BaseControllerを継承
@RestController
@RequestMapping("/api/child")
public class ChildController extends BaseController {

    @GetMapping("/specific")
    public ResponseEntity<String> specificMethod() {
        return ResponseEntity.ok("Child specific method");
    }

    @PostMapping("/create/{name}")
    public ResponseEntity<String> createSomething(@PathVariable String name, @RequestBody String data) {
        return ResponseEntity.ok("Created " + name + " with " + data);
    }
} 