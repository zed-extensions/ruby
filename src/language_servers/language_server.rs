use crate::{bundler::Bundler, command_executor::RealCommandExecutor, gemset::Gemset};
use zed_extension_api::{self as zed};

#[derive(Clone, Debug)]
pub struct LanguageServerBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<(String, String)>>,
}

pub trait WorktreeLike {
    #[allow(dead_code)]
    fn root_path(&self) -> String;

    #[allow(dead_code)]
    fn shell_env(&self) -> Vec<(String, String)>;

    #[allow(dead_code)]
    fn read_text_file(&self, path: &str) -> Result<String, String>;
}

impl WorktreeLike for zed::Worktree {
    fn root_path(&self) -> String {
        self.root_path()
    }

    fn shell_env(&self) -> Vec<(String, String)> {
        self.shell_env()
    }

    fn read_text_file(&self, path: &str) -> Result<String, String> {
        self.read_text_file(path)
    }
}

#[cfg(test)]
pub struct FakeWorktree {
    root_path: String,
    shell_env: Vec<(String, String)>,
}

#[cfg(test)]
impl FakeWorktree {
    pub fn new(root_path: String) -> Self {
        FakeWorktree {
            root_path,
            shell_env: Vec::new(),
        }
    }

    fn read_text_file(&self, _path: &str) -> Result<String, String> {
        Ok(String::new())
    }
}

#[cfg(test)]
impl WorktreeLike for FakeWorktree {
    fn root_path(&self) -> String {
        self.root_path.clone()
    }

    fn shell_env(&self) -> Vec<(String, String)> {
        self.shell_env.clone()
    }

    fn read_text_file(&self, path: &str) -> Result<String, String> {
        self.read_text_file(path)
    }
}

pub trait LanguageServer {
    const SERVER_ID: &str;
    const EXECUTABLE_NAME: &str;
    const GEM_NAME: &str;

    fn get_executable_args<T: WorktreeLike>(&self, _worktree: &T) -> Vec<String> {
        Vec::new()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
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

    fn language_server_binary(
        &self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<LanguageServerBinary> {
        let lsp_settings =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

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
                            .chain(self.get_executable_args(worktree))
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
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<LanguageServerBinary> {
        if let Some(path) = worktree.which(Self::EXECUTABLE_NAME) {
            Ok(LanguageServerBinary {
                path,
                args: Some(self.get_executable_args(worktree)),
                env: Some(worktree.shell_env()),
            })
        } else {
            self.extension_gemset_language_server_binary(language_server_id, worktree)
        }
    }

    fn extension_gemset_language_server_binary(
        &self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<LanguageServerBinary> {
        let gem_home = std::env::current_dir()
            .map_err(|e| format!("Failed to get extension directory: {}", e))?
            .to_string_lossy()
            .to_string();

        let gemset = Gemset::new(gem_home.clone(), Box::new(RealCommandExecutor));

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        match gemset.installed_gem_version(Self::GEM_NAME) {
            Ok(Some(_version)) => {
                if gemset
                    .is_outdated_gem(Self::GEM_NAME)
                    .map_err(|e| e.to_string())?
                {
                    zed::set_language_server_installation_status(
                        language_server_id,
                        &zed::LanguageServerInstallationStatus::Downloading,
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
                    args: Some(self.get_executable_args(worktree)),
                    env: Some(gemset.gem_path_env()),
                })
            }
            Ok(None) => {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &zed::LanguageServerInstallationStatus::Downloading,
                );

                gemset
                    .install_gem(Self::GEM_NAME)
                    .map_err(|e| e.to_string())?;

                let executable_path = gemset
                    .gem_bin_path(Self::EXECUTABLE_NAME)
                    .map_err(|e| e.to_string())?;

                Ok(LanguageServerBinary {
                    path: executable_path,
                    args: Some(self.get_executable_args(worktree)),
                    env: Some(gemset.gem_path_env()),
                })
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::language_servers::language_server::FakeWorktree;

    use super::{LanguageServer, WorktreeLike};

    struct TestServer {}

    impl TestServer {
        fn new() -> Self {
            Self {}
        }
    }

    impl LanguageServer for TestServer {
        const SERVER_ID: &'static str = "test-server";
        const EXECUTABLE_NAME: &'static str = "test-exe";
        const GEM_NAME: &'static str = "test";

        fn get_executable_args<T: WorktreeLike>(&self, _worktree: &T) -> Vec<String> {
            vec!["--test-arg".into()]
        }
    }

    #[test]
    fn test_default_executable_args() {
        let test_server = TestServer::new();
        let mock_worktree = FakeWorktree::new("/path/to/project".to_string());

        assert_eq!(
            test_server.get_executable_args(&mock_worktree),
            vec!["--test-arg"],
            "Default executable args should match expected vector"
        );
    }
}
