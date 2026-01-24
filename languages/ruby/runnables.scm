; Adapted from the following sources:
; Minitest: https://github.com/zidhuss/neotest-minitest/blob/main/lua/neotest-minitest/init.lua
; RSpec: https://github.com/olimorris/neotest-rspec/blob/main/lua/neotest-rspec/init.lua
; Tests that inherit from a specific class
((class
  name: [
    (constant) @run @name @RUBY_TEST_NAME
    (scope_resolution
      scope: (constant)
      name: (constant) @run)
  ]
  (superclass
    (scope_resolution) @superclass
    (#match? @superclass "(::IntegrationTest|::TestCase|::SystemTestCase|Minitest::Test|TLDR)$"))) @_ruby-test
  (#set! tag ruby-test))

((call
  method: (identifier) @run
  (#eq? @run "test")
  arguments: (argument_list
    (string
      (string_content) @name @RUBY_TEST_NAME))) @_ruby-test
  (#set! tag ruby-test))

; Methods that begin with test_
((method
  name: (identifier) @run @name @RUBY_TEST_NAME
  (#match? @run "^test_")) @_ruby-test
  (#set! tag ruby-test))

; System tests that inherit from ApplicationSystemTestCase
((class
  name: (constant) @run @name @RUBY_TEST_NAME
  (superclass) @superclass
  (#match? @superclass "(ApplicationSystemTestCase)$")) @_ruby-test
  (#set! tag ruby-test))

; Examples
((call
  method: (identifier) @run
  (#any-of? @run
    "describe" "context" "it" "its" "specify" "example" "feature" "scenario" "fdescribe" "fcontext"
    "fit" "fexample" "focus" "it_behaves_like" "it_should_behave_like" "include_context"
    "include_examples")
  arguments: (argument_list
    .
    [
      (_
        (string_content) @name @RUBY_TEST_NAME)
      (constant) @name @RUBY_TEST_NAME
      (scope_resolution) @name @RUBY_TEST_NAME
      (simple_symbol) @name @RUBY_TEST_NAME
      ; Catch-all to make sure we don't miss any cases (numbers, arrays, dynamically generated names, etc)
      (_) @name @RUBY_TEST_NAME
    ])) @_ruby-test
  (#set! tag ruby-test))

; Examples (one-liner syntax)
((call
  method: (identifier) @run
  (#any-of? @run "it" "its" "specify" "example" "fit" "fexample" "focus")
  block: (_) @name @RUBY_TEST_NAME
  !arguments) @_ruby-test
  (#set! tag ruby-test))
