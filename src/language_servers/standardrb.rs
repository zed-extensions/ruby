use super::LanguageServer;

pub struct Standardrb {}

impl LanguageServer for Standardrb {
    const SERVER_ID: &str = "standardrb";
    const EXECUTABLE_NAME: &str = "standardrb";

    fn get_executable_args() -> Vec<String> {
        vec!["--lsp".to_string()]
    }
}

impl Standardrb {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::{LanguageServer, Standardrb};

    #[test]
    fn test_server_id() {
        assert_eq!(Standardrb::SERVER_ID, "standardrb");
    }

    #[test]
    fn test_executable_name() {
        assert_eq!(Standardrb::EXECUTABLE_NAME, "standardrb");
    }

    #[test]
    fn test_executable_args() {
        assert_eq!(Standardrb::get_executable_args(), vec!["--lsp"]);
    }

    #[test]
    fn test_default_use_bundler() {
        assert!(Standardrb::default_use_bundler());
    }
}
