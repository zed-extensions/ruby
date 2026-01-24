# Comment line for textobjects

class Widget
  def initialize
    # instance method
  end

  def self.build
    new
  end

  class << self
    def singleton_block
      # singleton class body
    end
  end
end

module Tools
  # module comment
  def self.helper
    # helper
  end
end
