use zed_extension_api::{
    lsp::{Completion, CompletionKind, Symbol, SymbolKind},
    CodeLabel, CodeLabelSpan,
};

use super::LanguageServer;

pub struct RubyLsp {}

impl LanguageServer for RubyLsp {
    const SERVER_ID: &str = "ruby-lsp";
    const EXECUTABLE_NAME: &str = "ruby-lsp";
    const GEM_NAME: &str = "ruby-lsp";

    fn default_use_bundler() -> bool {
        false
    }
}

impl RubyLsp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        let highlight_name = match completion.kind? {
            CompletionKind::Class | CompletionKind::Module => "type",
            CompletionKind::Constant => "constant",
            CompletionKind::Method => "function.method",
            CompletionKind::Reference => "function.method",
            CompletionKind::Keyword => "keyword",
            _ => return None,
        };

        let len = completion.label.len();
        let mut spans = vec![CodeLabelSpan::literal(
            completion.label,
            Some(highlight_name.to_string()),
        )];

        if let Some(detail) = completion
            .label_details
            .and_then(|label_details| label_details.detail)
        {
            spans.push(CodeLabelSpan::literal(" ", None));
            spans.push(CodeLabelSpan::literal(detail, None));
        }

        Some(CodeLabel {
            code: Default::default(),
            spans,
            filter_range: (0..len).into(),
        })
    }

    pub fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        let name = &symbol.name;

        match symbol.kind {
            SymbolKind::Method => {
                let code = format!("def {name}; end");
                let filter_range = 0..name.len();
                let display_range = 4..4 + name.len();

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(display_range)],
                    filter_range: filter_range.into(),
                })
            }
            SymbolKind::Class | SymbolKind::Module => {
                let code = format!("class {name}; end");
                let filter_range = 0..name.len();
                let display_range = 6..6 + name.len();

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(display_range)],
                    filter_range: filter_range.into(),
                })
            }
            SymbolKind::Constant => {
                let code = name.to_uppercase().to_string();
                let filter_range = 0..name.len();
                let display_range = 0..name.len();

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(display_range)],
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

    #[test]
    fn test_default_use_bundler() {
        assert!(!RubyLsp::default_use_bundler());
    }
}
