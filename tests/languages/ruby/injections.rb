# Inline RBS examples
#: (String) -> void
# @rbs (Integer) -> String
# | arg: String
# | (Integer) -> String

def example(name)
  name
end

pattern = /[a-z]+/i

sql = <<~SQL
  SELECT * FROM users
SQL
