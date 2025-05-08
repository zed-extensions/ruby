use zed_extension_api::{
    self as zed, set_language_server_installation_status, settings::LspSettings, LanguageServerId,
    LanguageServerInstallationStatus, Result,
};

use crate::{
    bundler::{Bundler, RealCommandExecutor},
    gemset::{Gemset, RealGemCommandExecutor},
};

#[derive(Clone, Debug)]
pub struct LanguageServerBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<(String, String)>>,
}

pub trait LanguageServer {
    const SERVER_ID: &str;
    const EXECUTABLE_NAME: &str;
    const GEM_NAME: &str;

    fn get_executable_args() -> Vec<String> {
        Vec::new()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;

        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::None,
        );

        Ok(zed::Command {
            command: binary.path,
            args: binary.args.unwrap_or(Self::get_executable_args()),
            env: binary.env.unwrap_or_default(),
        })
    }

    fn language_server_binary(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<LanguageServerBinary> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        if let Some(binary_settings) = lsp_settings.binary {
            if let Some(path) = binary_settings.path {
                return Ok(LanguageServerBinary {
                    path,
                    args: binary_settings.arguments,
                    env: Some(worktree.shell_env()),
                });
            }
        }

        let use_bundler = lsp_settings
            .settings
            .as_ref()
            .and_then(|settings| settings["use_bundler"].as_bool())
            .unwrap_or(true);

        if !use_bundler {
            return self.try_find_on_path_or_extension_gemset(language_server_id, worktree);
        }

        let bundler = Bundler::new(
            worktree.root_path(),
            worktree.shell_env(),
            Box::new(RealCommandExecutor),
        );
        match bundler.installed_gem_version(Self::GEM_NAME) {
            Ok(_version) => {
                let bundle_path = worktree
                    .which("bundle")
                    .ok_or("Unable to find 'bundle' command: e")?;

                Ok(LanguageServerBinary {
                    path: bundle_path,
                    args: Some(
                        vec!["exec".into(), Self::EXECUTABLE_NAME.into()]
                            .into_iter()
                            .chain(Self::get_executable_args())
                            .collect(),
                    ),
                    env: Some(worktree.shell_env()),
                })
            }
            Err(_e) => self.try_find_on_path_or_extension_gemset(language_server_id, worktree),
        }
    }

    fn try_find_on_path_or_extension_gemset(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<LanguageServerBinary> {
        if let Some(path) = worktree.which(Self::EXECUTABLE_NAME) {
            Ok(LanguageServerBinary {
                path,
                args: Some(Self::get_executable_args()),
                env: Some(worktree.shell_env()),
            })
        } else {
            self.extension_gemset_language_server_binary(language_server_id)
        }
    }

    fn extension_gemset_language_server_binary(
        &self,
        language_server_id: &LanguageServerId,
    ) -> Result<LanguageServerBinary> {
        let gem_home = std::env::current_dir()
            .map_err(|e| format!("Failed to get extension directory: {}", e))?
            .to_string_lossy()
            .to_string();

        let gemset = Gemset::new(gem_home.clone(), Box::new(RealGemCommandExecutor));

        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        match gemset.installed_gem_version(Self::GEM_NAME) {
            Ok(Some(_version)) => {
                if gemset
                    .is_outdated_gem(Self::GEM_NAME)
                    .map_err(|e| e.to_string())?
                {
                    set_language_server_installation_status(
                        language_server_id,
                        &LanguageServerInstallationStatus::Downloading,
                    );

                    gemset
                        .update_gem(Self::GEM_NAME)
                        .map_err(|e| e.to_string())?;
                }

                let executable_path = gemset
                    .gem_bin_path(Self::EXECUTABLE_NAME)
                    .map_err(|e| e.to_string())?;

                Ok(LanguageServerBinary {
                    path: executable_path,
                    args: Some(Self::get_executable_args()),
                    env: Some(gemset.gem_path_env()),
                })
            }
            Ok(None) => {
                set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::Downloading,
                );

                gemset
                    .install_gem(Self::GEM_NAME)
                    .map_err(|e| e.to_string())?;

                let executable_path = gemset
                    .gem_bin_path(Self::EXECUTABLE_NAME)
                    .map_err(|e| e.to_string())?;

                Ok(LanguageServerBinary {
                    path: executable_path,
                    args: Some(Self::get_executable_args()),
                    env: Some(gemset.gem_path_env()),
                })
            }
            Err(e) => Err(e),
        }
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
    fn test_default_executable_args() {
        assert_eq!(
            TestServer::get_executable_args(),
            vec!["--test-arg"],
            "Default executable args should match expected vector"
        );
    }
}
