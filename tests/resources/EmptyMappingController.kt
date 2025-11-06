package com.example.controller

import org.springframework.web.bind.annotation.*

@RestController
@RequestMapping("/api/products")
class EmptyMappingController {

    @GetMapping("")
    fun getProductsWithEmptyMapping(): String {
        return "products"
    }

    @PostMapping("")
    fun createProductWithEmptyMapping(): String {
        return "created"
    }

    @GetMapping("/details")
    fun getProductDetails(): String {
        return "details"
    }
}