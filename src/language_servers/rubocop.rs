use zed_extension_api::{self as zed, LanguageServerId, Result};

use super::LanguageServer;

pub struct Rubocop {}

impl LanguageServer for Rubocop {
    const SERVER_ID: &str = "rubocop";
    const EXECUTABLE_NAME: &str = "rubocop";

    fn get_executable_args() -> Vec<String> {
        vec!["--lsp".to_string()]
    }
}

impl Rubocop {
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
    use crate::language_servers::{LanguageServer, Rubocop};

    #[test]
    fn test_server_id() {
        assert_eq!(Rubocop::SERVER_ID, "rubocop");
    }

    #[test]
    fn test_executable_name() {
        assert_eq!(Rubocop::EXECUTABLE_NAME, "rubocop");
    }

    #[test]
    fn test_executable_args() {
        assert_eq!(Rubocop::get_executable_args(), vec!["--lsp"]);
    }

    #[test]
    fn test_default_use_bundler() {
        assert!(Rubocop::default_use_bundler());
    }
}
