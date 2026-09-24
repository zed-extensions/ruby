# Highlights fixtures for tree-sitter queries

# Variables
variable
$global_variable
@instance_variable
@@class_variable
Constant
CONSTANT_ALL_CAPS
__FILE__
__LINE__
__ENCODING__
self
super

# comment

# Keywords — definition
module TestModule
  class TestClass < ::TestSuperClass::Scoped
  end

  class ::TestClass
  end

  class ::TestClass::Scoped
  end

  class << TestSingleton::Class
  end
end

# Keywords — conditional
case expr
in 5
in 5,
in ^foo
in ^(1+1)
in 1, 2
in 1, 2,
in 1, 2, 3
in 1, 2, 3,
in 1, 2, 3, *
in 1, *x, 3
in *
in *, 3, 4
in *, 3, *
in *a, 3, *b
in a:
in a: 5
in a: 5,
in a: 5, b:, **
in a: 5, b:, **map
in a: 5, "b":, **nil
in **nil
in [5]
in [5,]
in [1, 2]
in [1, 2,]
in [1, 2, 3]
in [1, 2, 3,]
in [1, 2, 3, *]
in [1, *x, 3]
in [*]
in [*, 3, 4]
in [*, 3, *]
in [*a, 3, *b]
in {a:}
in {a: 5}
in {a: 5,}
in {a: 5, b:, **}
in {a: 5, b:, **map}
in {a: 5, b:, **nil}
in {**nil}
in {}
in []
in -5 | +10
in (nil | self | true | false | __LINE__ | __FILE__ | __ENCODING__)
end

case fun(..., &block, **hash_splat)
when *splat then true
end

true ? false : true

if foo
  bar
elsif qux
  baz
else
  bat
end

unless true
end

# Keywords — loop
while true do
  redo
  break
end

until false do
end

for x, *y in z do
  next
end

# Keywords — exception handling
begin
  foo
rescue x
  retry
else
  qux
ensure
  baz
end

# Keywords — exception raising
raise
fail
catch
throw

# Keywords — flow return/jump
break
next
retry
return
yield

# Keywords — method scopes
private
protected
public

# Keywords — imports
require
require_relative
load

# Method/function definitions
undef +
alias foo -
def method;
  super
end
def METHOD;
end
def self.singleton_method = nil
def setter=
end
def method_params(var, optional = nil, &block, ..., kw:, *opts, **hash, **nil)
end
fun { || nil }
fun do ||
  nil
end
lambda = -> () { nil }

# Method/function calls
method var
METHOD var

# Operator overloading definitions
def +(a)
end
def -(a)
end
def *(a)
end
def **(a)
end
def /(a)
end
def %(a)
end
def <<(a)
end
def >>(a)
end
def ~(a)
end
def &(a)
end
def |(a)
end
def ^(a)
end
def <(a)
end
def >(a)
end
def <=(a)
end
def >=(a)
end
def !(a)
end
def ==(a)
end
def !=(a)
end
def ===(a)
end
def =~(a)
end
def !~(a)
end
def <=>(a)
end
def ..(a)
end
def +@(a)
end
def -@(a)
end
def ~@(a)
end
def [](a)
end
def []=(a)
end
def `(a)
end

# Builtin functions/methods
defined? foo
include foo
extend foo
prepend foo
refine foo
using foo

# BEGIN/END code blocks
BEGIN {}
END {}

# String literals
"string"
'string'
%[string]
%q[string]
%Q[string]
%w[string string string]
%W[string string string]

"\n\xA\000\u0af2"
?a
?\n
?0

# String interpolation
"inter#{polation}"

# Subshell literals
`subshell`
%x(subshell)

# Heredoc literal
<<~EOS
  heredoc
EOS

# Symbol literals
:symbol
:symbol!
:symbol?
:"delimited-symbol"
%s[delimited-symbol]
%s[delimited-symbol]
%i[symbol symbol! symbol?]
%I[symbol symbol! symbol?]

# Regex literals
/regex/
%r{regex}

# Number literals
1
1.0
12-34.2ri

# Boolean/nil literals
true
false
nil

# Hash
{ s1: nil, s1!: nil, s1?: nil, "?s1": nil, :s2 => nil, :"?s2" => nil, "string" => nil }

# Unary operators
not true
!true
+1
-1
~var

# Binary operators
true and true
true or false
true && true
true || false
true == true
true != false
1 + 2
2 - 1
3 * 2
2 ** 2
2 / 3
0 % 2
1 << 2
2 >> 1
1 & 2
2 | 1
2 ^ 2
2 < 1
1 > 2
2 <= 1
1 >= 2
1 === 1.0
"string" =~ /reg/
"string" !~ /reg/
foo <=> bar
1..2
1...2
foo&.bar
foo&.bar(var)
foo &.- bar

# Assignment operators
var = "foo"
var += 1
var -= 1
var *= 2
var **= 2
var /= 2
var %= 2
var <<= 2
var >>= 2
var &= 2
var |= 2
var ^= 2
var &&= nil
var ||= "string"
