; Empirical-specific syntax highlighting
; NOTE: Ruby base syntax is handled by the Ruby grammar
; This file ONLY highlights Empirical extensions

; EMPIRICAL: 'fun' keyword
"fun" @keyword.function

; EMPIRICAL: Function name in empirical_fun_method
(empirical_fun_method
  name: [(identifier) (constant)] @function.method)

; EMPIRICAL: Type annotations in parameters
(empirical_parameter
  name: (identifier) @variable.parameter
  type: (empirical_type) @type)

; EMPIRICAL: Return type annotations
(empirical_return_type
  type: (empirical_type) @type)

; EMPIRICAL: Generic type names (e.g., _Hash, _Array)
(empirical_generic_type
  name: (constant) @type.builtin)
