use super::{language_server::WorktreeLike, LanguageServer};

pub struct Rubocop {}

impl LanguageServer for Rubocop {
    const SERVER_ID: &str = "rubocop";
    const EXECUTABLE_NAME: &str = "rubocop";
    const GEM_NAME: &str = "rubocop";

    fn get_executable_args<T: WorktreeLike>(&self, _worktree: &T) -> Vec<String> {
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
    use crate::language_servers::{language_server::MockWorktree, LanguageServer, Rubocop};

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
        let rubocop = Rubocop::new();
        let mock_worktree = MockWorktree::new("/path/to/project".to_string());

        assert_eq!(rubocop.get_executable_args(&mock_worktree), vec!["--lsp"]);
    }
}
