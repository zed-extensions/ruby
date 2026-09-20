# Overrides fixtures for tree-sitter queries

# A Ruby comment with punctuation: ready? save!
plain_string = "plain text with ready? and save!"
interpolated_string = "hello #{user.name}, ready? save!"
single_quoted_string = 'single quoted ready? save!'
regexp = /ready\?|save!/
symbol = :valid?
defined? function!
{ symbol?: :"!delimited", "key-hash": "value", "string" => nil }
ready?
save!
receiver.ready?
receiver.save!
options = { ready?: true, save!: true }
availability = defined?(ready?)
predicate_symbol = :ready?
bang_symbol = :save!
quoted_predicate_symbol = :"ready?"
quoted_bang_symbol = :"save!"
symbol_array = %i[ready? save!]
interpolated_symbol_array = %I[ready? save!]
ternary = condition ? ready : fallback
negated = !condition
different = left != right
