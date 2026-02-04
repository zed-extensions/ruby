(class
  "end" @end) @indent

(module
  "end" @end) @indent

(method
  "end" @end) @indent

(singleton_method
  "end" @end) @indent

(if
  "end" @end) @indent

(unless
  "end" @end) @indent

(case
  "end" @end) @indent

(begin
  "end" @end) @indent

(do_block
  "end" @end) @indent

(do
  "end" @end) @indent

[
  (else)
  (elsif)
  (when)
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
