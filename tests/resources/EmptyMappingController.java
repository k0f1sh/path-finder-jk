package com.example.controller;

import org.springframework.web.bind.annotation.*;

@RestController
@RequestMapping("/api/users")
public class EmptyMappingController {

    @GetMapping("")
    public String getUsersWithEmptyMapping() {
        return "users";
    }

    @PostMapping("")
    public String createUserWithEmptyMapping() {
        return "created";
    }

    @GetMapping("/details")
    public String getUserDetails() {
        return "details";
    }
}