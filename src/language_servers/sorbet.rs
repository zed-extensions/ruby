use super::LanguageServer;

pub struct Sorbet {}

impl LanguageServer for Sorbet {
    const SERVER_ID: &str = "sorbet";
    const EXECUTABLE_NAME: &str = "srb";
    const GEM_NAME: &str = "sorbet";

    fn get_executable_args() -> Vec<String> {
        [
            "tc",
            "--lsp",
            "--enable-experimental-lsp-document-highlight",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
}

impl Sorbet {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::{LanguageServer, Sorbet};

    #[test]
    fn test_server_id() {
        assert_eq!(Sorbet::SERVER_ID, "sorbet");
    }

    #[test]
    fn test_executable_name() {
        assert_eq!(Sorbet::EXECUTABLE_NAME, "srb");
    }

    #[test]
    fn test_executable_args() {
        assert_eq!(
            Sorbet::get_executable_args(),
            vec![
                "tc",
                "--lsp",
                "--enable-experimental-lsp-document-highlight"
            ]
        );
    }
}
