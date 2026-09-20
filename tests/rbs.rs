mod support;

#[test]
fn parses_and_highlights_rbs() {
    let source = "module Billing\n  class Invoice\n    def total: (Integer quantity) -> Numeric\n  end\nend\n";
    let query = std::fs::read_to_string("languages/rbs/highlights.scm").unwrap();
    let captures = support::run_query(source, &query, "rbs");

    assert!(captures
        .iter()
        .any(|capture| capture.name == "type" && capture.text == "Billing"));
    assert!(captures
        .iter()
        .any(|capture| capture.name == "function.method" && capture.text == "total"));
    assert!(captures
        .iter()
        .any(|capture| capture.name == "variable.parameter" && capture.text == "quantity"));
}
