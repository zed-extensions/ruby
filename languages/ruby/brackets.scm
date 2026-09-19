("(" @open
  ")" @close)

("[" @open
  "]" @close)

("{" @open
  "}" @close)

("do" @open
  "end" @close)

(string
  "\"" @open
  "\"" @close
  (#set! rainbow.exclude))

(string_array
  "%w(" @open
  ")" @close
  (#set! rainbow.exclude))

(delimited_symbol
  ":\"" @open
  "\"" @close
  (#set! rainbow.exclude))

(symbol_array
  "%i(" @open
  ")" @close
  (#set! rainbow.exclude))

(regex
  "/" @open
  "/" @close
  (#set! rainbow.exclude))

(subshell
  "`" @open
  "`" @close
  (#set! rainbow.exclude))

(interpolation
  "#{" @open
  "}" @close
  (#set! rainbow.exclude))

(block_parameters
  "|" @open
  "|" @close)

(if
  "if" @open
  "end" @close)

(unless
  "unless" @open
  "end" @close)

(begin
  "begin" @open
  "end" @close)

(module
  "module" @open
  "end" @close)

(_
  .
  "def" @open
  "end" @close)

(_
  .
  "class" @open
  "end" @close)
