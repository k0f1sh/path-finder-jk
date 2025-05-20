package com.example.demo.controller

import org.springframework.web.bind.annotation.RestController
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestBody
import org.springframework.web.bind.annotation.RequestMethod
import org.springframework.http.ResponseEntity

import com.example.demo.model.User
import com.example.demo.service.UserService

@RestController
@RequestMapping("/api/kotlin/users")
class UserController(private val userService: UserService) {

    @GetMapping
    fun getAllUsers(): ResponseEntity<*> {
        return ResponseEntity.ok(userService.findAll())
    }

    @GetMapping("/{id}")
    fun getUserById(@PathVariable id: Long): ResponseEntity<*> {
        return ResponseEntity.ok(userService.findById(id))
    }

    @GetMapping(value = ["/{id}"])
    fun getUserById2(@PathVariable id: Long): ResponseEntity<*> {
        return ResponseEntity.ok(userService.findById(id))
    }

    @GetMapping(value = ["/{id}"], headers = [XCustomHeader])
    fun getUserById3(@PathVariable id: Long): ResponseEntity<*> {
        return ResponseEntity.ok(userService.findById(id))
    }

    @PostMapping
    fun createUser(@RequestBody user: User): ResponseEntity<*> {
        return ResponseEntity.ok(userService.save(user))
    }

    @RequestMapping(method = [RequestMethod.POST], value = ["/{id}"])
    fun createUser2(@RequestBody user: User): ResponseEntity<*> {
        return ResponseEntity.ok(userService.save(user))
    }

    @RequestMapping(value = ["/{id}/name-id"], method = [PUT], produces = ["application/json"])
    fun updateNameId(@PathVariable id: Long, @RequestBody params: UpdateNameIdRequestParams): ResponseEntity<*> {
        return ResponseEntity.ok().build()
    }
}