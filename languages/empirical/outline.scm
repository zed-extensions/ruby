; Empirical outline - show fun methods in document outline

; EMPIRICAL: fun method definitions
(empirical_fun_method
  name: (identifier) @name) @item

; EMPIRICAL: fun method with "fun" keyword as context
(empirical_fun_method
  "fun" @context
  name: (identifier) @name) @item

; EMPIRICAL: fun method in class/module context
(class
  (body_statement
    (empirical_fun_method
      "fun" @context
      name: (identifier) @name) @item))

(module
  (body_statement
    (empirical_fun_method
      "fun" @context
      name: (identifier) @name) @item))
