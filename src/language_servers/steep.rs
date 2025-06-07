use super::LanguageServer;
use zed_extension_api::{self as zed};

pub struct Steep {}

impl LanguageServer for Steep {
    const SERVER_ID: &str = "steep";
    const EXECUTABLE_NAME: &str = "steep";
    const GEM_NAME: &str = "steep";

    fn get_executable_args(&self, _worktree: &zed::Worktree) -> Vec<String> {
        vec!["langserver".to_string()]
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let lsp_settings =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let require_root_steepfile = lsp_settings
            .settings
            .as_ref()
            .and_then(|settings| settings["require_root_steepfile"].as_bool())
            .unwrap_or(true);

        if require_root_steepfile && worktree.read_text_file("Steepfile").is_err() {
            return Err("Steep language server requires a Steepfile in the project root. You can disable this requirement by setting 'require_root_steepfile': false in your LSP settings.".to_string());
        }

        let binary = self.language_server_binary(language_server_id, worktree)?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(zed::Command {
            command: binary.path,
            args: binary.args.unwrap_or(self.get_executable_args(worktree)),
            env: binary.env.unwrap_or_default(),
        })
    }
}

impl Steep {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::{LanguageServer, Steep};

    #[test]
    fn test_server_id() {
        assert_eq!(Steep::SERVER_ID, "steep");
    }

    #[test]
    fn test_executable_name() {
        assert_eq!(Steep::EXECUTABLE_NAME, "steep");
    }

    // #[test]
    // fn test_executable_args() {
    //     let test_server = Steep::new();
    //     assert_eq!(test_server.get_executable_args(), vec!["langserver"]);
    // }
}
