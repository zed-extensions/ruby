use streaming_iterator::StreamingIterator;
use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor};

/// Represents a single capture from a query match
#[derive(Debug, serde::Serialize)]
pub struct Capture {
    pub name: String,
    pub line: usize,
    pub column: usize,
    pub text: String,
}

fn normalize_line_endings(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

/// Run a tree-sitter query on source code and return captures
fn language_for_id(language_id: &str) -> tree_sitter::Language {
    match language_id {
        "ruby" => tree_sitter_ruby::LANGUAGE.into(),
        "erb" => tree_sitter_embedded_template::LANGUAGE.into(),
        _ => panic!("Unsupported language id for query tests: {language_id}"),
    }
}

fn extract_language_id(path: &str, root: &str) -> Option<String> {
    let normalized = path.replace('\\', "/");
    let mut iter = Path::new(&normalized)
        .components()
        .map(|component| component.as_os_str());
    while let Some(component) = iter.next() {
        if component == root {
            return iter
                .next()
                .map(|segment| segment.to_string_lossy().into_owned());
        }
    }
    None
}

pub fn detect_language_id(fixture_path: &str, query_path: &str) -> String {
    let fixture_lang = extract_language_id(fixture_path, "languages");
    let query_lang = extract_language_id(query_path, "languages");

    match (fixture_lang, query_lang) {
        (Some(fixture_lang), Some(query_lang)) if fixture_lang == query_lang => fixture_lang,
        (Some(fixture_lang), Some(query_lang)) => {
            panic!("Mismatched language ids: {fixture_lang} vs {query_lang}")
        }
        (Some(fixture_lang), None) => fixture_lang,
        (None, Some(query_lang)) => query_lang,
        (None, None) => {
            panic!("Unable to detect language id from paths: {fixture_path}, {query_path}")
        }
    }
}

pub fn run_query(source: &str, query_source: &str, language_id: &str) -> Vec<Capture> {
    let mut parser = Parser::new();
    let language = language_for_id(language_id);
    parser
        .set_language(&language)
        .unwrap_or_else(|_| panic!("Error loading {language_id} parser"));

    let tree = parser.parse(source, None).expect("Failed to parse source");
    let query = Query::new(&language, query_source).expect("Failed to create query");

    let mut cursor = QueryCursor::new();
    let mut captures = Vec::new();
    let source_bytes = source.as_bytes();

    let mut matches = cursor.matches(&query, tree.root_node(), source_bytes);
    while let Some(match_) = matches.next() {
        for capture in match_.captures {
            let capture_name = &query.capture_names()[capture.index as usize];
            let node = capture.node;
            let start = node.start_position();
            let text = node
                .utf8_text(source_bytes)
                .expect("Failed to extract capture text");

            captures.push(Capture {
                name: capture_name.to_string(),
                line: start.row + 1,
                column: start.column + 1,
                text: text.to_string(),
            });
        }
    }

    captures
}

pub fn snapshot_settings(language_id: &str) -> insta::Settings {
    let snapshot_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("languages")
        .join(language_id)
        .join("snapshots");
    std::fs::create_dir_all(&snapshot_dir).expect("Failed to create snapshot directory");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(snapshot_dir);
    settings.set_prepend_module_to_snapshot(false);
    settings
}

pub fn assert_query_snapshot(snapshot_name: &str, fixture_path: &str, query_path: &str) {
    let fixture_abs = Path::new(env!("CARGO_MANIFEST_DIR")).join(fixture_path);
    let query_abs = Path::new(env!("CARGO_MANIFEST_DIR")).join(query_path);
    let source = std::fs::read_to_string(&fixture_abs).unwrap_or_else(|error| {
        panic!(
            "Failed to read query fixture file {}: {error}",
            fixture_abs.display()
        )
    });
    let query_source = std::fs::read_to_string(&query_abs).unwrap_or_else(|error| {
        panic!("Failed to read query file {}: {error}", query_abs.display())
    });
    let language_id = detect_language_id(fixture_path, query_path);
    let captures = run_query(
        &normalize_line_endings(&source),
        &normalize_line_endings(&query_source),
        &language_id,
    );
    let settings = snapshot_settings(&language_id);
    settings.bind(|| insta::assert_yaml_snapshot!(snapshot_name, captures));
}

#[cfg(test)]
mod tests {
    use super::detect_language_id;

    #[test]
    fn detects_language_from_fixture_path() {
        let language = detect_language_id(
            "tests/languages/ruby/runnables.rb",
            "languages/ruby/runnables.scm",
        );
        assert_eq!(language, "ruby");
    }

    #[test]
    fn detects_language_with_windows_separators() {
        let language = detect_language_id(
            "tests\\languages\\erb\\fixtures.rb",
            "languages\\erb\\runnables.scm",
        );
        assert_eq!(language, "erb");
    }
}
