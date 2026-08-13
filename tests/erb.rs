mod support;

#[test]
fn injections() {
    support::assert_query_snapshot(
        "injections",
        "tests/languages/erb/injections.erb",
        "languages/erb/injections.scm",
    );
}

#[test]
fn strict_locals_are_injected_as_ruby() {
    let source = "<%# locals: (title:, subtitle: nil, **options) %>";
    let query = std::fs::read_to_string("languages/erb/injections.scm").unwrap();
    let captures = support::run_query(source, &query, "erb");

    assert_eq!(captures.len(), 1);
    assert_eq!(captures[0].name, "content");
    assert_eq!(
        captures[0].text,
        " locals: (title:, subtitle: nil, **options) "
    );
}

#[test]
fn strict_locals_require_whitespace_before_parameters() {
    let source = "<%# locals:(title:) %>";
    let query = std::fs::read_to_string("languages/erb/injections.scm").unwrap();
    let captures = support::run_query(source, &query, "erb");

    assert_eq!(captures.len(), 1);
    assert_eq!(captures[0].name, "content");
    assert_eq!(captures[0].text, " locals:(title:) ");
}

#[test]
fn strict_local_names_share_parameter_highlighting() {
    let source = "locals: (title:, talks:, view_all_path: root_path, subtitle: nil)";
    let query = std::fs::read_to_string("languages/ruby/highlights.scm").unwrap();
    let captures = support::run_query(source, &query, "ruby");
    let parameters: Vec<_> = captures
        .iter()
        .filter(|capture| capture.name == "variable.parameter.keyword")
        .map(|capture| capture.text.as_str())
        .collect();

    assert_eq!(parameters, ["title", "talks", "view_all_path", "subtitle"]);
}
