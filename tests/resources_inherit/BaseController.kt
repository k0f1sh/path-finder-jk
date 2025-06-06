package com.example.demo.controller

import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.http.ResponseEntity

// 親クラス - @RequestMappingアノテーションなし
open class BaseController {

    @GetMapping("/health")
    fun health(): ResponseEntity<String> {
        return ResponseEntity.ok("OK")
    }

    @GetMapping("/status/{id}")
    fun getStatus(@PathVariable id: Long): ResponseEntity<String> {
        return ResponseEntity.ok("Status for $id")
    }

    @PostMapping("/common")
    fun commonAction(@RequestBody data: String): ResponseEntity<String> {
        return ResponseEntity.ok("Common action: $data")
    }
} 