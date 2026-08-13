((code) @content
  (#set! "language" "ruby")
  (#set! "combined"))

((content) @content
  (#set! "language" "javascript")
  (#set! "combined"))

((comment) @content
  (#match? @content "^\\s*locals:\\s+\\(")
  (#set! "language" "ruby"))

((comment) @content
  (#not-match? @content "^\\s*locals:\\s+\\(")
  (#set! injection.language "comment")
  (#set! "combined"))
