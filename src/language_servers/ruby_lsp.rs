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
        let zed::lsp::Completion {
            label,
            kind,
            label_details,
            ..
        } = completion;
        let kind = kind?;

        let highlight_scope = match kind {
            zed::lsp::CompletionKind::Class | zed::lsp::CompletionKind::Module => "type",
            zed::lsp::CompletionKind::Constant if label == "nil" => "constant.builtin",
            zed::lsp::CompletionKind::Constant
                if label.starts_with("__") && label.ends_with("__") =>
            {
                "constant.builtin"
            }
            zed::lsp::CompletionKind::Constant => "constant",
            zed::lsp::CompletionKind::Method
            | zed::lsp::CompletionKind::Reference
            | zed::lsp::CompletionKind::Function => "function.method",
            zed::lsp::CompletionKind::Constructor => "function.method",
            zed::lsp::CompletionKind::Keyword => "keyword",
            zed::lsp::CompletionKind::Field if label.starts_with("@@") => "variable.special",
            zed::lsp::CompletionKind::Field if label.starts_with('@') => "variable.special",
            zed::lsp::CompletionKind::Field if label == "self" || label == "super" => {
                "variable.special"
            }
            zed::lsp::CompletionKind::Variable => "variable",
            zed::lsp::CompletionKind::Property => "property",
            _ => return None,
        };

        let label_len = label.len();
        let mut spans = vec![zed::CodeLabelSpan::literal(
            label,
            Some(highlight_scope.to_string()),
        )];

        if let Some(label_details) = label_details {
            if let Some(detail) = label_details.detail {
                spans.push(zed::CodeLabelSpan::literal(detail, None));
            }

            if let Some(description) = label_details.description {
                spans.push(zed::CodeLabelSpan::literal(" ", None));
                spans.push(zed::CodeLabelSpan::literal(description, None));
            }
        }

        Some(zed::CodeLabel {
            code: Default::default(),
            spans,
            filter_range: (0..label_len).into(),
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

    pub fn language_server_initialization_options(
        &self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let mut initialization_options =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                .ok()
                .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
                .unwrap_or_else(|| zed::serde_json::json!({}));

        let options_obj = initialization_options
            .as_object_mut()
            .expect("initialization_options must be an object");

        let enabled_features = options_obj
            .entry("enabledFeatures")
            .or_insert_with(|| zed::serde_json::json!({}));

        // Workaround ruby-lsp upstream issue
        // https://github.com/zed-extensions/ruby/issues/38
        if let Some(features_obj) = enabled_features.as_object_mut() {
            features_obj
                .entry("onTypeFormatting")
                .or_insert(zed::serde_json::Value::Bool(false));
        }

        Ok(Some(initialization_options))
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::{language_server::FakeWorktree, LanguageServer, RubyLsp};

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
        let ruby_lsp = RubyLsp::new();
        let mock_worktree = FakeWorktree::new("/path/to/project".to_string());

        assert_eq!(
            ruby_lsp.get_executable_args(&mock_worktree),
            vec![] as Vec<String>
        );
    }
}
