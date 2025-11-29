(assignment
  left: (identifier) @debug-variable)

(assignment
  left: (call
    receiver: (identifier) @debug-variable))

(assignment
  left: (call
    method: (identifier) @debug-variable))

(call
  (argument_list
    (call
      receiver: (identifier) @debug-variable)))

(call
  (argument_list
    (call
      method: (identifier) @debug-variable)))

(call
  (argument_list
    (identifier) @debug-variable))

(method
  (method_parameters) @debug-variable)

(body_statement
  (assignment
    (instance_variable) @debug-variable))

(program) @debug-scope

(body_statement) @debug-scope
