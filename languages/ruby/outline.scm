(class
    "class" @context
    name: (_) @name) @item

(singleton_class
    "class" @context
    "<<" @context
    value: (self) @context
) @item

(body_statement
    ((identifier) @context
    (#match? @context "^(private|protected|public)$")) @item
)

(method
    "def" @context
    name: (_) @name) @item

(singleton_method
    "def" @context
    object: (_) @context
    "." @context
    name: (_) @name) @item

(module
    "module" @context
    name: (_) @name) @item

(assignment left: (constant) @name) @item

; Support Minitest/RSpec symbols
;
; Note that `(_)+` is used to capture one more child nodes, meaning it will also include any modifier symbols, like
; :focus, so that we can easily jump to focused tests
(call
    method: (identifier) @run (#any-of? @run "describe" "context" "test" "it")
    arguments: (argument_list . (_)+) @name
) @item
