# Textobjects fixtures

# ============================================================================
# 1. Regular Class Text Objects
# ============================================================================

class UserController
  def initialize(user)
    @user = user
  end

  def process
    @user.save!
  end
end

class EmptyClass
end

class SingleMethodClass
  def single_method
    42
  end
end

# ============================================================================
# 2. Singleton Class Text Objects
# ============================================================================

user = User.new

class << user
  def special_method
    puts "Only for this instance"
  end

  def another_special
    @value = 42
  end
end

class Helper
  class << self
    def helper_one
      perform_action
    end

    def helper_two
      another_action
    end
  end
end

# ============================================================================
# 3. Class.new Block Text Objects
# ============================================================================

DynamicClass = Class.new do
  def method_one
    puts "Method One"
  end

  def method_two
    puts "Method Two"
  end

  attr_reader :value
end

SimpleClass = Class.new do
  def single
    1
  end
end

# ============================================================================
# 4. Module Text Objects
# ============================================================================

module Authentication
  def login(credentials)
    verify(credentials)
  end

  def logout
    clear_session
  end

  def self.included(base)
    base.extend(ClassMethods)
  end
end

module EmptyModule
end

module SingleMethod
  def helper
    "help"
  end
end

# ============================================================================
# 5. Instance Method Text Objects
# ============================================================================

class Calculator
  def calculate_total(items)
    subtotal = items.sum(&:price)
    tax = subtotal * 0.08
    shipping = 10
    subtotal + tax + shipping
  end

  def single_line_method
    42
  end

  def empty_method
  end

  def method_with_blocks
    items.each do |item|
      item.process
    end
  end
end

# ============================================================================
# 6. Singleton Method Text Objects (Class Methods)
# ============================================================================

class User
  def self.find_active
    where(active: true)
  end

  def self.admin_count
    where(role: 'admin').count
  end

  def self.complex_query
    query = base_scope
    query = query.where(verified: true)
    query.order(:created_at)
  end

  def self.single_line
    true
  end
end

# ============================================================================
# 7. Do Block Text Objects
# ============================================================================

users.each do |user|
  user.send_welcome_email
  user.update(welcomed: true)
  log_action(:welcome, user)
end

results.map do |item|
  item.process do |data|
    data.transform
    data.validate
  end
end

items.select do |item|
  item.valid?
end

[1, 2, 3].each_with_index do |num, idx|
  puts "#{idx}: #{num}"
end

values.reduce(0) do |sum, val|
  sum + val
end

# ============================================================================
# 8. Curly Brace Block Text Objects
# ============================================================================

numbers = [1, 2, 3].map { |n| n * 2 }

data.select { |item|
  item.valid? && item.active?
}

items.each { |item|
  item.validate
  item.process
  item.save
}

result = values.map { |v|
  transformed = v * 2
  transformed + 1
}

nested = data.map { |item|
  item.transform { |val| val * 2 }
}

# ============================================================================
# 9. Comment Text Objects
# ============================================================================

# Single line comment
def simple_method
  # Comment inside method
  do_something
end

# This is a multi-line comment
# explaining what the function does
# and why it's important
def process_data
  data.transform
end

# Another multi-line block
# describing the class
# and its responsibilities
class DataProcessor
  # Inner comment block
  # with multiple lines
  def process
    # Single comment in method
    perform_action
  end
end

# ============================================================================
# 10. Nested Structures
# ============================================================================

class OuterClass
  class InnerClass
    def inner_method
      puts "nested method"
    end
  end

  def outer_method
    InnerClass.new.inner_method
  end

  class << self
    def outer_singleton
      "singleton in outer"
    end
  end
end

module OuterModule
  module InnerModule
    def nested_helper
      perform_action
    end
  end

  class InnerClass
    def module_class_method
      "in module class"
    end
  end
end

# ============================================================================
# 11. Edge Cases
# ============================================================================

# Method containing do block
def method_with_do_block
  items.each do |item|
    item.process
  end
end

# Method containing brace block
def method_with_brace_block
  items.map { |x| x * 2 }
end

# Method with multiple blocks
def method_with_multiple_blocks
  items.select { |x| x.valid? }.map do |item|
    item.transform
  end
end

# Class with mixed content
class MixedContent
  # Class comment
  CONSTANT = 42

  attr_reader :value

  def initialize
    @value = 0
  end

  def self.factory
    new
  end

  class << self
    def helper
      "help"
    end
  end
end

# Multiple comments with code in between
# First comment block
# continues here
def first_method
  1
end

# Second comment block
# also multi-line
def second_method
  2
end

# ============================================================================
# 12. Real-World Examples
# ============================================================================

class PostsController
  def index
    @posts = Post.all.map do |post|
      {
        id: post.id,
        title: post.title,
        author: post.author.name
      }
    end
  end

  def create
    @post = Post.new(post_params)

    if @post.save
      redirect_to @post
    else
      render :new
    end
  end

  private

  def post_params
    params.require(:post).permit(:title, :body)
  end
end

module ApiHelper
  def self.format_response(data)
    {
      status: :success,
      data: data,
      timestamp: Time.now
    }
  end

  def render_json(obj)
    JSON.generate(obj)
  end
end

# ============================================================================
# End of Text Objects Test Fixtures
# ============================================================================
