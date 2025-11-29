; Class definitions, e.g. `class Foo`
(class
  "class" @context
  name: (_) @name) @item

; Singleton class definitions `class << self`
(singleton_class
  "class" @context
  "<<" @context
  value: (self) @context) @item

; Method definition with a modifier, e.g. `private def foo`
(body_statement
  (call
    method: (identifier) @context
    arguments: (argument_list
      (method
        "def" @context
        name: (_) @name))) @item)

; Method definition without modifier, e.g. `def foo`
(body_statement
  (method
    "def" @context
    name: (_) @name) @item)

; Root method definition with modifier, e.g. `private def foo`
(program
  (call
    method: (identifier) @context
    arguments: (argument_list
      (method
        "def" @context
        name: (_) @name))) @item)

; Root method definition without modifier, e.g. `def foo`
(program
  (method
    "def" @context
    name: (_) @name) @item)

; Root singleton method definition, e.g. `def self.foo`
(program
  (singleton_method
    "def" @context
    object: (_) @context
    "." @context
    name: (_) @name) @item)

; Singleton method definition without modifier, e.g. `def self.foo`
(body_statement
  (singleton_method
    "def" @context
    object: (_) @context
    "." @context
    name: (_) @name) @item)

; Singleton method definition with modifier, e.g. `private_class_method def self.foo`
(body_statement
  (call
    method: (identifier) @context
    arguments: (argument_list
      (singleton_method
        "def" @context
        object: (_) @context
        "." @context
        name: (_) @name) @item)) @item)

; Module definition, e.g. `module Foo`
(module
  "module" @context
  name: (_) @name) @item

; Constant assignment
(assignment
  left: (constant) @name) @item

; Class macros such as `alias_method`, `include`, `belongs_to`, `has_many`, `attr_reader`
(class
  (body_statement
    (call
      method: (identifier) @name
      arguments: (argument_list
        .
        [
          (string) @name
          (simple_symbol) @name
          (scope_resolution) @name
          (constant) @name
          "," @context
        ]*
        [
          (string) @name
          (simple_symbol) @name
          (scope_resolution) @name
          (constant) @name
        ])) @item))

; Module macros such as `alias_method`, `include`
(module
  (body_statement
    (call
      method: (identifier) @name
      arguments: (argument_list
        .
        [
          (string) @name
          (simple_symbol) @name
          (scope_resolution) @name
          (constant) @name
          "," @context
        ]*
        [
          (string) @name
          (simple_symbol) @name
          (scope_resolution) @name
          (constant) @name
        ])) @item))

; Class macros without arguments, such as `private`
(class
  (body_statement
    (identifier) @name @item))

(class
  (body_statement
    (call
      method: (identifier) @name
      !arguments) @item))

; Module macros without arguments, such as `private`
(module
  (body_statement
    (identifier) @name @item))

(module
  (body_statement
    (call
      method: (identifier) @name
      !arguments) @item))

; Root test methods
(program
  (call
    method: (identifier) @_run @name
    (#any-of? @_run "describe" "context" "test" "it" "shared_examples")
    arguments: (argument_list
      .
      [
        (string) @name
        (simple_symbol) @name
        (scope_resolution) @name
        (constant) @name
        "," @context
      ]*
      [
        (string) @name
        (simple_symbol) @name
        (scope_resolution) @name
        (constant) @name
      ])) @item)

; Nested test methods
(call
  method: (identifier) @_ctx
  (#any-of? @_ctx "describe" "context" "shared_examples")
  arguments: (argument_list
    .
    [
      (string)
      (simple_symbol)
      (scope_resolution)
      (constant)
    ]+)
  block: (_
    (_
      (call
        method: (identifier) @_run @name
        (#any-of? @_run "describe" "context" "test" "it" "shared_examples")
        arguments: (argument_list
          .
          [
            (string) @name
            (simple_symbol) @name
            (scope_resolution) @name
            (constant) @name
            "," @context
          ]*
          [
            (string) @name
            (simple_symbol) @name
            (scope_resolution) @name
            (constant) @name
          ])) @item)))

; RSpec one-liners
(call
  method: (identifier) @_ctx
  (#any-of? @_ctx "describe" "context" "shared_examples")
  arguments: (argument_list
    .
    [
      (string)
      (simple_symbol)
      (scope_resolution)
      (constant)
    ]+)
  block: (_
    (_
      (call
        method: (identifier) @_run @name
        (#any-of? @_run "it")
        block: (block
          body: (block_body
            (call
              receiver: (identifier) @_expectation
              (#any-of? @_expectation "is_expected")
              method: (identifier) @_negation
              (#any-of? @_negation "to" "not_to" "to_not")))) @name) @item)))

; Root rake namespace
(program
  (call
    method: (identifier) @_namespace @name
    (#any-of? @_namespace "namespace")
    arguments: (argument_list
      .
      [
        (string) @name
        (simple_symbol) @name
      ])) @item)

; Nested rake namespace
(call
  method: (identifier) @_parent_namespace
  (#any-of? @_parent_namespace "namespace")
  arguments: (argument_list
    .
    [
      (string)
      (simple_symbol)
    ]+)
  block: (_
    (_
      (call
        method: (identifier) @_namespace @name
        (#any-of? @_namespace "namespace")
        arguments: (argument_list
          .
          [
            (string) @name
            (simple_symbol) @name
          ])) @item)))

; Root rake task
(program
  (call
    method: (identifier) @_task @name
    (#any-of? @_task "task")
    arguments: (argument_list
      .
      [
        (string) @name
        (simple_symbol) @name
        (pair
          key: (hash_key_symbol) @name)
      ])) @item)

; Nested rake task
(call
  method: (identifier) @_namespace
  (#any-of? @_namespace "namespace")
  arguments: (argument_list
    .
    [
      (string)
      (simple_symbol)
    ]+)
  block: (_
    (_
      (call
        method: (identifier) @_task @name
        (#any-of? @_task "task")
        arguments: (argument_list
          .
          [
            (string) @name
            (simple_symbol) @name
            (pair
              key: (hash_key_symbol) @name)
          ])) @item)))

; Single argument methods in schema.rb
(call
  receiver: (_
    (scope_resolution) @_receiver
    (#eq? @_receiver "ActiveRecord::Schema"))
  block: (_
    (_
      (call
        method: (identifier) @context
        (#any-of? @context
          "create_enum" "create_schema" "create_table" "create_virtual_table" "enable_extension")
        arguments: (argument_list
          .
          (string) @name)) @item)))

; Double argument methods in schema.rb
(call
  receiver: (_
    (scope_resolution) @_receiver
    (#eq? @_receiver "ActiveRecord::Schema"))
  block: (_
    (_
      (call
        method: (identifier) @context
        (#eq? @context "add_foreign_key")
        arguments: (argument_list
          .
          (string) @name
          ","
          (string) @name)) @item)))
