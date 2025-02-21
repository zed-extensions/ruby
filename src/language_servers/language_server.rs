use std::fs;

use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

#[derive(Clone, Debug)]
pub struct LanguageServerBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
}

pub trait LanguageServer {
    const SERVER_ID: &str;
    const EXECUTABLE_NAME: &str;
    const GEM_NAME: &str;

    fn default_use_bundler() -> bool {
        true // Default for most LSPs except Ruby LSP
    }

    fn get_executable_args() -> Vec<String> {
        Vec::new()
    }

    fn language_server_command(
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

    fn server_exists(&self) -> bool {
        fs::metadata(Self::EXECUTABLE_NAME).map_or(false, |stat| stat.is_file())
    }

    fn language_server_binary(
        &self,
        language_server_id: &LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<LanguageServerBinary> {
        let server_exists = self.server_exists();
        if server_exists {
            return Ok({
                LanguageServerBinary {
                    path: Self::EXECUTABLE_NAME.into(),
                    args: Some(Self::get_executable_args()),
                }
            });
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::gems_latest_version(Self::GEM_NAME)?;

        if !server_exists || zed::gems_installed_version(Self::GEM_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result =
                zed::gems_install_gem(Self::GEM_NAME, &version, &[Self::EXECUTABLE_NAME.into()]);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{}' did not contain expected path '{}'",
                            Self::GEM_NAME,
                            Self::EXECUTABLE_NAME
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        Ok(LanguageServerBinary {
            path: Self::EXECUTABLE_NAME.into(),
            args: Some(Self::get_executable_args()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestServer {}
    impl LanguageServer for TestServer {
        const SERVER_ID: &'static str = "test-server";
        const EXECUTABLE_NAME: &'static str = "test-exe";
        const GEM_NAME: &'static str = "test";

        fn get_executable_args() -> Vec<String> {
            vec!["--test-arg".into()]
        }
    }

    #[test]
    fn test_default_use_bundler() {
        assert!(TestServer::default_use_bundler());
    }

    #[test]
    fn test_default_executable_args() {
        assert!(TestServer::get_executable_args() == vec!["--test-arg"]);
    }
}
