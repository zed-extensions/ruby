use super::LanguageServer;
use zed_extension_api::{self as zed};

pub struct Rubocop {}

impl LanguageServer for Rubocop {
    const SERVER_ID: &str = "rubocop";
    const EXECUTABLE_NAME: &str = "rubocop";
    const GEM_NAME: &str = "rubocop";

    fn get_executable_args(&self, _worktree: &zed::Worktree) -> Vec<String> {
        vec!["--lsp".to_string()]
    }
}

impl Rubocop {
    pub fn new() -> Self {
        Self {}
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

    // #[test]
    // fn test_executable_args() {
    //     assert_eq!(Rubocop::get_executable_args(), vec!["--lsp"]);
    // }
}
