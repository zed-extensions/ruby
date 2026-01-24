# Outline fixtures for classes, modules, methods, tasks, and tests

API_VERSION = "v1"

class User
  VERSION = "1.0"

  include Comparable
  include Foo::Bar
  include Baz
  attr_reader :name, :email
  alias_method "login", "sign_in"

  private
  self.private

  def initialize(name)
    @name = name
  end

  private def secret
    # hidden
  end

  def public_method
    # visible
  end

  def self.build
    new("default")
  end

  private_class_method def self.build_private
    new("private")
  end

  class << self
    def singleton_block_method
      # singleton class method
    end
  end
end

module Billing
  include Payments
  include Payments::Gateway
  alias_method :charge, :bill

  private
  self.private
end

private def root_private
  # root private
end

def root_public
  # root public
end

def self.root_singleton
  # root singleton
end

# Root test methods

describe "root describe" do
end

context "root context" do
end

test "root test" do
end

it "root it" do
end

its "root its" do
end

specify "root specify" do
end

example "root example" do
end

feature "root feature" do
end

scenario "root scenario" do
end

shared_examples "root shared_examples" do
end

fdescribe "root fdescribe" do
end

fcontext "root fcontext" do
end

fit "root fit" do
end

fexample "root fexample" do
end

focus "root focus" do
end

xdescribe "root xdescribe" do
end

xcontext "root xcontext" do
end

xit "root xit" do
end

xexample "root xexample" do
end

xspecify "root xspecify" do
end

skip "root skip" do
end

pending "root pending" do
end

it_behaves_like "root it_behaves_like"

it_should_behave_like "root it_should_behave_like"

include_context "root include_context"

include_examples "root include_examples"

# Nested test methods and one-liners

describe "outer" do
  context "inner" do
    it "nested it" do
    end

    specify "nested specify" do
    end

    example "nested example" do
    end

    feature "nested feature" do
    end

    scenario "nested scenario" do
    end

    shared_examples "nested shared_examples" do
    end

    fdescribe "nested fdescribe" do
    end

    fcontext "nested fcontext" do
    end

    fit "nested fit" do
    end

    fexample "nested fexample" do
    end

    focus "nested focus" do
    end

    xdescribe "nested xdescribe" do
    end

    xcontext "nested xcontext" do
    end

    xit "nested xit" do
    end

    xexample "nested xexample" do
    end

    xspecify "nested xspecify" do
    end

    skip "nested skip" do
    end

    pending "nested pending" do
    end

    it_behaves_like "nested it_behaves_like"

    it_should_behave_like "nested it_should_behave_like"

    include_context "nested include_context"

    include_examples "nested include_examples"

    it { is_expected.to be_truthy }
    it { is_expected.not_to be_nil }
    its { is_expected.to be_empty }
    specify { is_expected.to be_truthy }
    example { is_expected.to be_truthy }
    fit { is_expected.to be_truthy }
    fexample { is_expected.to be_truthy }
    focus { is_expected.to be_truthy }
    xit { is_expected.to be_truthy }
    xexample { is_expected.to be_truthy }
    xspecify { is_expected.to be_truthy }
    skip { is_expected.to be_truthy }
    pending { is_expected.to be_truthy }
  end
end

# Rake namespaces and tasks

namespace :db do
  namespace :migrate do
    task :up do
      # migrate
    end
  end
end

task :version do
  # version
end

task default: :test do
  # default
end

# Schema.rb style

ActiveRecord::Schema.define(version: 2024_01_01) do
  create_table "users"
  create_enum "status"
  create_schema "audit"
  create_virtual_table "search"
  enable_extension "pgcrypto"
  add_foreign_key "orders", "users"
end
