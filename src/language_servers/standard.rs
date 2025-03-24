use super::LanguageServer;

pub struct Standard {}

impl LanguageServer for Standard {
    const SERVER_ID: &str = "standard";
    const EXECUTABLE_NAME: &str = "standardrb";

    fn get_executable_args() -> Vec<String> {
        vec!["--lsp".to_string()]
    }
}

impl Standard {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::{LanguageServer, Standard};

    #[test]
    fn test_server_id() {
        assert_eq!(Standard::SERVER_ID, "standard");
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
