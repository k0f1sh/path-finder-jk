package com.example.demo.controller

import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.http.ResponseEntity

// 祖父クラス - @RequestMappingアノテーションなし
open class GrandParentController {

    @GetMapping("/grandparent")
    fun grandparentMethod(): ResponseEntity<String> {
        return ResponseEntity.ok("GrandParent method")
    }

    @PostMapping("/legacy/{id}")
    fun legacyAction(@PathVariable id: Long): ResponseEntity<String> {
        return ResponseEntity.ok("Legacy action for $id")
    }
} 