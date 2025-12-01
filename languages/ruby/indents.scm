(method
  "end" @end) @indent

(class
  "end" @end) @indent

(module
  "end" @end) @indent

(begin
  "end" @end) @indent

(singleton_method
  "end" @end) @indent

(do_block
  "end" @end) @indent

[
  (then)
  (call)
] @indent

[
  (ensure)
  (rescue)
] @outdent

(_
  "["
  "]" @end) @indent

(_
  "{"
  "}" @end) @indent

(_
  "("
  ")" @end) @indent
