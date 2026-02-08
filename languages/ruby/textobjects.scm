; Adapted from https://github.com/helix-editor/helix/blob/master/runtime/queries/ruby/textobjects.scm
; Class and Modules
(class
  body: (_)? @class.inside) @class.around

(singleton_class
  value: (_)
  (_)+ @class.inside) @class.around

(call
  receiver: (constant) @_class_const
  method: (identifier) @_class_method
  (#match? @_class_const "Class")
  (#match? @_class_method "new")
  (do_block
    (_)+ @class.inside)) @class.around

(module
  body: (_)? @class.inside) @class.around

; Functions and Blocks
(singleton_method
  body: (_)? @function.inside) @function.around

(method
  body: (_)? @function.inside) @function.around

(do_block
  body: (_)? @function.inside) @function.around

(block
  body: (_)? @function.inside) @function.around

; Comments
(comment) @comment.inside

(comment)+ @comment.around
