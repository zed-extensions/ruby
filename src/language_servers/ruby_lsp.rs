use zed_extension_api::{self as zed};

use super::LanguageServer;

pub struct RubyLsp {}

impl LanguageServer for RubyLsp {
    const SERVER_ID: &str = "ruby-lsp";
    const EXECUTABLE_NAME: &str = "ruby-lsp";
    const GEM_NAME: &str = "ruby-lsp";
}

impl RubyLsp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn label_for_completion(&self, completion: zed::lsp::Completion) -> Option<zed::CodeLabel> {
        let highlight_name = match completion.kind? {
            zed::lsp::CompletionKind::Class | zed::lsp::CompletionKind::Module => "type",
            zed::lsp::CompletionKind::Constant => "constant",
            zed::lsp::CompletionKind::Method => "function.method",
            zed::lsp::CompletionKind::Reference => "function.method",
            zed::lsp::CompletionKind::Keyword => "keyword",
            _ => return None,
        };

        let len = completion.label.len();
        let mut spans = vec![zed::CodeLabelSpan::literal(
            completion.label,
            Some(highlight_name.to_string()),
        )];

        if let Some(detail) = completion
            .label_details
            .and_then(|label_details| label_details.detail)
        {
            spans.push(zed::CodeLabelSpan::literal(" ", None));
            spans.push(zed::CodeLabelSpan::literal(detail, None));
        }

        Some(zed::CodeLabel {
            code: Default::default(),
            spans,
            filter_range: (0..len).into(),
        })
    }

    pub fn label_for_symbol(&self, symbol: zed::lsp::Symbol) -> Option<zed::CodeLabel> {
        let name = &symbol.name;

        match symbol.kind {
            zed::lsp::SymbolKind::Method => {
                let code = format!("def {name}; end");
                let filter_range = 0..name.len();
                let display_range = 4..4 + name.len();

                Some(zed::CodeLabel {
                    code,
                    spans: vec![zed::CodeLabelSpan::code_range(display_range)],
                    filter_range: filter_range.into(),
                })
            }
            zed::lsp::SymbolKind::Class | zed::lsp::SymbolKind::Module => {
                let code = format!("class {name}; end");
                let filter_range = 0..name.len();
                let display_range = 6..6 + name.len();

                Some(zed::CodeLabel {
                    code,
                    spans: vec![zed::CodeLabelSpan::code_range(display_range)],
                    filter_range: filter_range.into(),
                })
            }
            zed::lsp::SymbolKind::Constant => {
                let code = name.to_uppercase();
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
    use crate::language_servers::{LanguageServer, RubyLsp};

    #[test]
    fn test_server_id() {
        assert_eq!(RubyLsp::SERVER_ID, "ruby-lsp");
    }

    #[test]
    fn test_executable_name() {
        assert_eq!(RubyLsp::EXECUTABLE_NAME, "ruby-lsp");
    }

    #[test]
    fn test_executable_args() {
        assert_eq!(RubyLsp::get_executable_args(), vec![] as Vec<String>);
    }
}
