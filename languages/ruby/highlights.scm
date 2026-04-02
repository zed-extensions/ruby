; Variables
[
  (identifier)
  (global_variable)
] @variable

; Keywords — definition
[
  "class"
  "def"
  "module"
] @keyword.function

; Keywords — conditional
[
  "case"
  "else"
  "elsif"
  "if"
  "then"
  "unless"
  "when"
] @keyword.control.conditional

; Keywords — loop
[
  "do"
  "for"
  "until"
  "while"
] @keyword.control.repeat

; Keywords — flow return/jump
[
  "break"
  "next"
  "retry"
  "return"
  "yield"
] @keyword.control.return

; Keywords — exception handling
[
  "begin"
  "ensure"
  "rescue"
] @keyword.exception

; Keywords — general
[
  "alias"
  "and"
  "end"
  "in"
  "or"
] @keyword

((identifier) @keyword
  (#match? @keyword "^(private|protected|public)$"))

; Function calls
(call
  method: [
    (identifier)
    (constant)
  ] @function.method)

((identifier) @keyword.import
  (#any-of? @keyword.import "require" "require_relative" "load"))

"defined?" @function.method.builtin

; Function definitions
(alias
  (identifier) @function.method)

(setter
  (identifier) @function.method)

(method
  name: [
    (identifier)
    (constant)
  ] @function.method)

(singleton_method
  name: [
    (identifier)
    (constant)
  ] @function.method)

(method_parameters
  [
    (identifier) @variable.parameter
    (optional_parameter
      name: (identifier) @variable.parameter)
    (keyword_parameter
      [
        name: (identifier)
        ":"
      ] @variable.parameter)
  ])

(block_parameters
  (identifier) @variable.parameter)

; Identifiers
((identifier) @constant.builtin
  (#match? @constant.builtin "^__(FILE|LINE|ENCODING)__$"))

(file) @constant.builtin

(line) @constant.builtin

(encoding) @constant.builtin

(hash_splat_nil
  "**" @operator) @constant.builtin

(constant) @type

((constant) @constant
  (#match? @constant "^[A-Z\\d_]+$"))

(superclass
  (constant) @type.super)

(superclass
  (scope_resolution
    (constant) @type.super))

(superclass
  (scope_resolution
    (scope_resolution
      (constant) @type.super)))

(self) @variable.special

(super) @variable.special

[
  (class_variable)
  (instance_variable)
] @variable.special

((call
  !receiver
  method: (identifier) @function.builtin)
  (#any-of? @function.builtin "include" "extend" "prepend" "refine" "using"))

((identifier) @keyword.exception
  (#any-of? @keyword.exception "raise" "fail" "catch" "throw"))

; Literals
[
  (string)
  (bare_string)
  (subshell)
  (heredoc_body)
  (heredoc_beginning)
] @string

[
  (simple_symbol)
  (delimited_symbol)
  (hash_key_symbol)
  (bare_symbol)
] @string.special.symbol

(regex) @string.regex

(escape_sequence) @string.escape

[
  (integer)
  (float)
] @number

[
  (true)
  (false)
] @boolean

(nil) @constant.builtin

; Regular comments (exclude RBS inline comments)
((comment) @comment
  (#not-match? @comment "^\\s*#[:|]")
  (#not-match? @comment "^\\s*#\\s*(@rbs|\\|)"))

; Operators
[
  "!"
  "~"
  "+"
  "-"
  "**"
  "*"
  "/"
  "%"
  "<<"
  ">>"
  "&"
  "|"
  "^"
  ">"
  "<"
  "<="
  ">="
  "=="
  "!="
  "=~"
  "!~"
  "<=>"
  "||"
  "&&"
  ".."
  "..."
  "="
  "**="
  "*="
  "/="
  "%="
  "+="
  "-="
  "<<="
  ">>="
  "&&="
  "&="
  "||="
  "|="
  "^="
  "=>"
  "->"
  (operator)
] @operator

[
  ","
  ";"
  "."
  "::"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "%w("
  "%i("
] @punctuation.bracket

(interpolation
  "#{" @punctuation.special
  "}" @punctuation.special) @embedded
