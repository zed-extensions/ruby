mod support;

// ============================================================================
// Runnables Tests
// ============================================================================

#[test]
fn runnables() {
    support::assert_query_snapshot(
        "runnables",
        "tests/languages/ruby/runnables.rb",
        "languages/ruby/runnables.scm",
    );
}

// ============================================================================
// Debugger Tests
// ============================================================================

#[test]
fn debugger() {
    support::assert_query_snapshot(
        "debugger",
        "tests/languages/ruby/debugger.rb",
        "languages/ruby/debugger.scm",
    );
}

// ============================================================================
// Outline Tests
// ============================================================================

#[test]
fn outline() {
    support::assert_query_snapshot(
        "outline",
        "tests/languages/ruby/outline.rb",
        "languages/ruby/outline.scm",
    );
}

// ============================================================================
// Textobjects Tests
// ============================================================================

#[test]
fn textobjects() {
    support::assert_query_snapshot(
        "textobjects",
        "tests/languages/ruby/textobjects.rb",
        "languages/ruby/textobjects.scm",
    );
}

// ============================================================================
// Injections Tests
// ============================================================================

#[test]
fn injections() {
    support::assert_query_snapshot(
        "injections",
        "tests/languages/ruby/injections.rb",
        "languages/ruby/injections.scm",
    );
}
