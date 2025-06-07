use zed_extension_api::{self as zed};

use super::{language_server::WorktreeLike, LanguageServer};

pub struct Solargraph {}

impl LanguageServer for Solargraph {
    const SERVER_ID: &str = "solargraph";
    const EXECUTABLE_NAME: &str = "solargraph";
    const GEM_NAME: &str = "solargraph";

    fn get_executable_args<T: WorktreeLike>(&self, _worktree: &T) -> Vec<String> {
        vec!["stdio".to_string()]
    }
}

impl Solargraph {
    pub fn new() -> Self {
        Self {}
    }

    pub fn label_for_completion(&self, completion: zed::lsp::Completion) -> Option<zed::CodeLabel> {
        let highlight_name = match completion.kind? {
            zed::lsp::CompletionKind::Class | zed::lsp::CompletionKind::Module => "type",
            zed::lsp::CompletionKind::Constant => "constant",
            zed::lsp::CompletionKind::Method => "function.method",
            zed::lsp::CompletionKind::Keyword => {
                if completion.label.starts_with(':') {
                    "string.special.symbol"
                } else {
                    "keyword"
                }
            }
            zed::lsp::CompletionKind::Variable => {
                if completion.label.starts_with('@') {
                    "property"
                } else {
                    return None;
                }
            }
            _ => return None,
        };

        let len = completion.label.len();
        let name_span =
            zed::CodeLabelSpan::literal(completion.label, Some(highlight_name.to_string()));

        Some(zed::CodeLabel {
            code: Default::default(),
            spans: if let Some(detail) = completion.detail {
                vec![
                    name_span,
                    zed::CodeLabelSpan::literal(" ", None),
                    zed::CodeLabelSpan::literal(detail, None),
                ]
            } else {
                vec![name_span]
            },
            filter_range: (0..len).into(),
        })
    }

    pub fn label_for_symbol(&self, symbol: zed::lsp::Symbol) -> Option<zed::CodeLabel> {
        let name = &symbol.name;

        match symbol.kind {
            zed::lsp::SymbolKind::Method => {
                let mut parts = name.split('#');
                let container_name = parts.next()?;
                let method_name = parts.next()?;

                if parts.next().is_some() {
                    return None;
                }

                let filter_range = 0..name.len();

                let spans = vec![
                    zed::CodeLabelSpan::literal(container_name, Some("type".to_string())),
                    zed::CodeLabelSpan::literal("#", None),
                    zed::CodeLabelSpan::literal(method_name, Some("function.method".to_string())),
                ];

                Some(zed::CodeLabel {
                    code: name.to_string(),
                    spans,
                    filter_range: filter_range.into(),
                })
            }
            zed::lsp::SymbolKind::Class | zed::lsp::SymbolKind::Module => {
                let class = "class ";
                let code = format!("{class}{name}");
                let filter_range = 0..name.len();
                let display_range = class.len()..class.len() + name.len();

                Some(zed::CodeLabel {
                    code,
                    spans: vec![zed::CodeLabelSpan::code_range(display_range)],
                    filter_range: filter_range.into(),
                })
            }
            zed::lsp::SymbolKind::Constant => {
                let code = name.to_uppercase().to_string();
                let filter_range = 0..name.len();
                let display_range = 0..name.len();

                Some(zed::CodeLabel {
                    code,
                    spans: vec![zed::CodeLabelSpan::code_range(display_range)],
                    filter_range: filter_range.into(),
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::{language_server::MockWorktree, LanguageServer, Solargraph};

    #[test]
    fn test_server_id() {
        assert_eq!(Solargraph::SERVER_ID, "solargraph");
    }

    #[test]
    fn test_executable_name() {
        assert_eq!(Solargraph::EXECUTABLE_NAME, "solargraph");
    }

    #[test]
    fn test_executable_args() {
        let solargraph = Solargraph::new();
        let mock_worktree = MockWorktree::new("/path/to/project".to_string());

        assert_eq!(
            solargraph.get_executable_args(&mock_worktree),
            vec!["stdio"]
        );
    }
}
