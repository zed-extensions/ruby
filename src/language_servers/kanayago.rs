use super::{language_server::WorktreeLike, LanguageServer};

pub struct Kanayago {}

impl LanguageServer for Kanayago {
    const SERVER_ID: &str = "kanayago";
    const EXECUTABLE_NAME: &str = "kanayago";
    const GEM_NAME: &str = "kanayago";

    fn get_executable_args<T: WorktreeLike>(&self, _worktree: &T) -> Vec<String> {
        vec!["--lsp".to_string()]
    }
}

impl Kanayago {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::{language_server::FakeWorktree, Kanayago, LanguageServer};

    #[test]
    fn test_server_id() {
        assert_eq!(Kanayago::SERVER_ID, "kanayago");
    }

    #[test]
    fn test_executable_name() {
        assert_eq!(Kanayago::EXECUTABLE_NAME, "kanayago");
    }

    #[test]
    fn test_executable_args() {
        let kanayago = Kanayago::new();
        let mock_worktree = FakeWorktree::new("/path/to/project".to_string());

        assert_eq!(kanayago.get_executable_args(&mock_worktree), vec!["--lsp"]);
    }
}
