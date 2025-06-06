package com.example.demo.controller

import org.springframework.web.bind.annotation.RestController
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.http.ResponseEntity

// 子クラス - @RequestMappingアノテーションあり、BaseControllerを継承
@RestController
@RequestMapping("/api/kotlin/child")
class ChildController : BaseController() {

    @GetMapping("/specific")
    fun specificMethod(): ResponseEntity<String> {
        return ResponseEntity.ok("Child specific method")
    }

    @PostMapping("/create/{name}")
    fun createSomething(@PathVariable name: String, @RequestBody data: String): ResponseEntity<String> {
        return ResponseEntity.ok("Created $name with $data")
    }
} 