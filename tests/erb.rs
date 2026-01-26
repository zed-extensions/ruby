mod support;

#[test]
fn injections() {
    support::assert_query_snapshot(
        "injections",
        "tests/languages/erb/injections.erb",
        "languages/erb/injections.scm",
    );
}
