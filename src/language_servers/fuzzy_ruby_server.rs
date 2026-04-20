use super::language_server::LanguageServerBinary;
use super::LanguageServer;
use zed_extension_api::{self as zed};

pub struct FuzzyRubyServer {}

impl LanguageServer for FuzzyRubyServer {
    const SERVER_ID: &str = "fuzzy-ruby-server";
    const EXECUTABLE_NAME: &str = "fuzzy";
    const GEM_NAME: &str = "fuzzy-ruby-server--not-a-gem";

    fn language_server_binary(
        &self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<LanguageServerBinary> {
        self.resolve_binary(language_server_id.as_ref(), worktree)
    }
}

impl FuzzyRubyServer {
    pub fn new() -> Self {
        Self {}
    }

    fn resolve_binary<T: super::language_server::WorktreeLike>(
        &self,
        server_id: &str,
        worktree: &T,
    ) -> zed::Result<LanguageServerBinary> {
        let lsp_settings = worktree.lsp_binary_settings(server_id)?;

        if let Some(binary_settings) = lsp_settings {
            if let Some(path) = binary_settings.path {
                if !std::path::Path::new(&path).is_file() {
                    return Err(format!(
                        "fuzzy-ruby-server: configured binary path '{}' does not exist or is not a file. Update lsp.fuzzy-ruby-server.binary.path in your Zed settings.",
                        path
                    )
                    .into());
                }
                return Ok(LanguageServerBinary {
                    path,
                    args: binary_settings.arguments,
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

#[cfg(test)]
mod tests {
    use crate::language_servers::{
        language_server::{FakeWorktree, LspBinarySettings},
        FuzzyRubyServer, LanguageServer,
    };

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

    #[test]
    fn test_language_server_binary_custom_path() {
        let server = FuzzyRubyServer::new();
        let mut mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        // Use a path that actually exists on the system for the custom-path test
        let real_path = std::env::current_exe()
            .expect("could not get test binary path")
            .to_string_lossy()
            .to_string();
        mock_worktree.add_lsp_binary_setting(
            FuzzyRubyServer::SERVER_ID.to_string(),
            Ok(Some(LspBinarySettings {
                path: Some(real_path.clone()),
                arguments: None,
            })),
        );
        let result = server.resolve_binary(FuzzyRubyServer::SERVER_ID, &mock_worktree);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().path, real_path);
    }

    #[test]
    fn test_language_server_binary_custom_path_missing() {
        let server = FuzzyRubyServer::new();
        let mut mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        mock_worktree.add_lsp_binary_setting(
            FuzzyRubyServer::SERVER_ID.to_string(),
            Ok(Some(LspBinarySettings {
                path: Some("/nonexistent/fuzzy".to_string()),
                arguments: None,
            })),
        );
        let result = server.resolve_binary(FuzzyRubyServer::SERVER_ID, &mock_worktree);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("does not exist or is not a file"),
            "Error was: {err}"
        );
    }

    #[test]
    fn test_language_server_binary_path_lookup() {
        let server = FuzzyRubyServer::new();
        let mut mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        mock_worktree.add_lsp_binary_setting(FuzzyRubyServer::SERVER_ID.to_string(), Ok(None));
        mock_worktree.set_which("fuzzy".to_string(), Some("/usr/local/bin/fuzzy".to_string()));
        let result = server.resolve_binary(FuzzyRubyServer::SERVER_ID, &mock_worktree);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().path, "/usr/local/bin/fuzzy");
    }

    #[test]
    fn test_language_server_binary_not_found() {
        let server = FuzzyRubyServer::new();
        let mut mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        mock_worktree.add_lsp_binary_setting(FuzzyRubyServer::SERVER_ID.to_string(), Ok(None));
        mock_worktree.set_which("fuzzy".to_string(), None);
        let result = server.resolve_binary(FuzzyRubyServer::SERVER_ID, &mock_worktree);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("fuzzy not found"), "Error was: {err}");
        assert!(err.contains("cargo install"), "Error was: {err}");
    }
}
