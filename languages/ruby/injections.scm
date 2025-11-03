(heredoc_body
  (heredoc_content) @content
  (heredoc_end) @language
  (#downcase! @language))

((regex
  (string_content) @content)
  (#set! "language" "regex"))

; Empirical type system support
(body_statement
  (call
    method: (identifier) @_name (#any-of? @_name "fun")
    arguments: (_)
  ) @content
  (#set! "language" "empirical")
)
