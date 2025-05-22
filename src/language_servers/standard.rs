use super::language_server::LanguageServer;
use zed_extension_api::{self as zed, LanguageServerId, Result};

pub struct Standard {}

impl LanguageServer for Standard {
    const SERVER_ID: &str = "standardrb";
    const EXECUTABLE_NAME: &str = "standardrb";

    fn get_executable_args() -> Vec<String> {
        vec!["--lsp".into()]
    }
}

impl Standard {
    pub fn new() -> Self {
        Self {}
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary.path,
            args: binary.args.unwrap_or(Self::get_executable_args()),
            env: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::{LanguageServer, Standard};

    #[test]
    fn test_server_id() {
        assert_eq!(Standard::SERVER_ID, "standardrb");
    }

    #[test]
    fn test_executable_name() {
        assert_eq!(Standard::EXECUTABLE_NAME, "standardrb");
    }

    #[test]
    fn test_executable_args() {
        assert_eq!(Standard::get_executable_args(), vec!["--lsp"]);
    }

    #[test]
    fn test_default_use_bundler() {
        assert!(Standard::default_use_bundler());
    }
}
