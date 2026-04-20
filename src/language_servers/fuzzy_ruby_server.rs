use super::language_server::LanguageServerBinary;
use super::LanguageServer;
use zed_extension_api::{self as zed};

pub struct FuzzyRubyServer {}

impl LanguageServer for FuzzyRubyServer {
    const SERVER_ID: &str = "fuzzy-ruby-server";
    const EXECUTABLE_NAME: &str = "fuzzy";
    const GEM_NAME: &str = "";

    fn language_server_binary(
        &self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<LanguageServerBinary> {
        let lsp_settings =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        if let Some(binary) = &lsp_settings.binary {
            if let Some(path) = &binary.path {
                return Ok(LanguageServerBinary {
                    path: path.clone(),
                    args: binary.arguments.clone(),
                    env: Some(worktree.shell_env()),
                });
            }
        }

        if let Some(path) = worktree.which(Self::EXECUTABLE_NAME) {
            return Ok(LanguageServerBinary {
                path,
                args: Some(self.get_executable_args(worktree)),
                env: Some(worktree.shell_env()),
            });
        }

        Err("fuzzy not found. Install with: cargo install --git https://github.com/doompling/fuzzy_ruby_server".into())
    }
}

impl FuzzyRubyServer {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::{language_server::FakeWorktree, FuzzyRubyServer, LanguageServer};

    #[test]
    fn test_server_id() {
        assert_eq!(FuzzyRubyServer::SERVER_ID, "fuzzy-ruby-server");
    }

    #[test]
    fn test_executable_name() {
        assert_eq!(FuzzyRubyServer::EXECUTABLE_NAME, "fuzzy");
    }

    #[test]
    fn test_executable_args() {
        let server = FuzzyRubyServer::new();
        let mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        assert_eq!(server.get_executable_args(&mock_worktree), Vec::<String>::new());
    }
}
