; Adapted from https://github.com/helix-editor/helix/blob/master/runtime/queries/ruby/textobjects.scm

; Class and Modules
(class
  body: (_)? @class.inside) @class.around

(singleton_class
  value: (_)
  (_)+ @class.inside) @class.around

(module
  body: (_)? @class.inside) @class.around

; Functions and Blocks
(singleton_method
  body: (_)? @function.inside) @function.around

(method
  body: (_)? @function.inside) @function.around

(call
  method: (identifier) @methodName (#eq? @methodName "fun")
  block: (do_block
    body: (_)? @function.inside
  )) @function.around

; Comments
(comment) @comment.inside
