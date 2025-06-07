use super::{language_server::WorktreeLike, LanguageServer};

pub struct Sorbet {}

impl LanguageServer for Sorbet {
    const SERVER_ID: &str = "sorbet";
    const EXECUTABLE_NAME: &str = "srb";
    const GEM_NAME: &str = "sorbet";

    fn get_executable_args<T: WorktreeLike>(&self, worktree: &T) -> Vec<String> {
        let binary_settings = worktree
            .lsp_binary_settings(Self::SERVER_ID)
            .unwrap_or_default();

        let default_args = vec![
            "tc".to_string(),
            "--lsp".to_string(),
            "--enable-experimental-lsp-document-highlight".to_string(),
        ];

        // test if sorbet/config is present
        match worktree.read_text_file("sorbet/config") {
            Ok(_) => {
                // Config file exists, prefer custom arguments if available.
                binary_settings
                    .and_then(|bs| bs.arguments)
                    .unwrap_or(default_args)
            }
            Err(_) => {
                // gross, but avoid sorbet errors in a non-sorbet
                // environment by using an empty config
                vec![
                    "tc".to_string(),
                    "--lsp".to_string(),
                    "--dir".to_string(),
                    "./".to_string(),
                ]
            }
        }
    }
}

impl Sorbet {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::{
        language_server::{FakeWorktree, LspBinarySettings},
        LanguageServer, Sorbet,
    };

    #[test]
    fn test_server_id() {
        assert_eq!(Sorbet::SERVER_ID, "sorbet");
    }

    #[test]
    fn test_executable_name() {
        assert_eq!(Sorbet::EXECUTABLE_NAME, "srb");
    }

    #[test]
    fn test_executable_args_no_config_file() {
        let sorbet = Sorbet::new();
        let mut fake_worktree = FakeWorktree::new("/path/to/project".to_string());

        fake_worktree.add_file(
            "sorbet/config".to_string(),
            Err("File not found".to_string()),
        );
        fake_worktree.add_lsp_binary_setting(Sorbet::SERVER_ID.to_string(), Ok(None));

        let expected_args_no_config = vec![
            "tc".to_string(),
            "--lsp".to_string(),
            "--dir".to_string(),
            "./".to_string(),
        ];
        assert_eq!(
            sorbet.get_executable_args(&fake_worktree),
            expected_args_no_config,
            "Should use fallback arguments when sorbet/config is not found"
        );
    }

    #[test]
    fn test_executable_args_with_config_and_custom_settings() {
        let sorbet = Sorbet::new();
        let mut fake_worktree = FakeWorktree::new("/path/to/project".to_string());

        fake_worktree.add_file("sorbet/config".to_string(), Ok("--dir\n.".to_string()));

        let custom_args = vec!["--custom-arg1".to_string(), "value1".to_string()];
        fake_worktree.add_lsp_binary_setting(
            Sorbet::SERVER_ID.to_string(),
            Ok(Some(LspBinarySettings {
                path: None,
                arguments: Some(custom_args.clone()),
            })),
        );

        assert_eq!(
            sorbet.get_executable_args(&fake_worktree),
            custom_args,
            "Should use custom arguments when config and settings are present"
        );
    }

    #[test]
    fn test_executable_args_with_config_no_custom_settings() {
        let sorbet = Sorbet::new();
        let mut fake_worktree = FakeWorktree::new("/path/to/project".to_string());

        fake_worktree.add_file("sorbet/config".to_string(), Ok("--dir\n.".to_string()));
        fake_worktree.add_lsp_binary_setting(Sorbet::SERVER_ID.to_string(), Ok(None));

        let expected_default_args = vec![
            "tc".to_string(),
            "--lsp".to_string(),
            "--enable-experimental-lsp-document-highlight".to_string(),
        ];
        assert_eq!(
            sorbet.get_executable_args(&fake_worktree),
            expected_default_args,
            "Should use default arguments when config is present but no custom settings"
        );
    }

    #[test]
    fn test_executable_args_with_config_lsp_settings_is_empty_struct() {
        let sorbet = Sorbet::new();
        let mut fake_worktree = FakeWorktree::new("/path/to/project".to_string());

        fake_worktree.add_file("sorbet/config".to_string(), Ok("--dir\n.".to_string()));
        fake_worktree.add_lsp_binary_setting(
            Sorbet::SERVER_ID.to_string(),
            Ok(Some(LspBinarySettings::default())),
        );

        let expected_default_args = vec![
            "tc".to_string(),
            "--lsp".to_string(),
            "--enable-experimental-lsp-document-highlight".to_string(),
        ];
        assert_eq!(
                sorbet.get_executable_args(&fake_worktree),
                expected_default_args,
                "Should use default arguments when config is present and LSP settings have no arguments"
            );
    }
}
