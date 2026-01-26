# Debugger fixtures for tree-sitter queries

# Assignments
class Example
  def process
    user = User.new
    email = user.email
    result = calculate_sum(1, 2)
    @total = result
  end

  def calculate_sum(a, b)
    sum = a + b
    sum
  end
end

# Method parameters
class Calculator
  def add(x, y)
    x + y
  end

  def multiply(a, b, c = 1)
    a * b * c
  end

  def process(*args, **kwargs)
    args.sum + kwargs.values.sum
  end
end

# Instance variables
class User
  def initialize(name)
    @name = name
    @created_at = Time.now
    @active = true
  end

  def update(email)
    @email = email
    @updated_at = Time.now
  end
end

# Scopes
global_var = "test"

class ScopeExample
  def method_scope
    local_var = 1
    another_var = 2

    if local_var > 0
      scoped_var = 3
    end
  end
end

# Complex scenario
class ComplexExample
  def process_data(input, options = {})
    @input = input
    result = transform(input)
    output = result.map { |item| item.value }
    logger.info(output)
    @processed = true
    output
  end

  def transform(data)
    temp = data.dup
    temp.filter { |x| x > 0 }
  end
end

# Call arguments
class ArgumentCalls
  def audit(user, value)
    log(user.name)
    log(fetch())
    log(value)
  end

  def fetch
    "ok"
  end

  def log(message)
    message
  end
end

# Assignment calls
class AssignmentCalls
  def update(user)
    user.email = "test@example.com"
    user.profile.name = "Tester"
  end
end
