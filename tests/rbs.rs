mod support;

// ============================================================================
// Highlights Tests
// ============================================================================

#[test]
fn highlights() {
    support::assert_query_snapshot(
        "highlights",
        "tests/languages/rbs/highlights.rbs",
        "languages/rbs/highlights.scm",
    );
}
