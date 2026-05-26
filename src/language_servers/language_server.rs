#[cfg(test)]
use std::collections::HashMap;

#[cfg(feature = "command_api")]
use crate::{
    bundler::Bundler,
    command_executor::RealCommandExecutor,
    gemset::{versioned_gem_home, Gemset},
};
#[cfg(feature = "command_api")]
use std::path::PathBuf;
use zed_extension_api::{self as zed};

#[derive(Clone, Debug)]
pub struct LanguageServerBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<(String, String)>>,
}

#[derive(Clone, Debug, Default)]
pub struct LspBinarySettings {
    #[allow(dead_code)]
    pub path: Option<String>,
    pub arguments: Option<Vec<String>>,
}

pub trait WorktreeLike {
    #[allow(dead_code)]
    fn root_path(&self) -> String;
    #[allow(dead_code)]
    fn shell_env(&self) -> Vec<(String, String)>;
    fn read_text_file(&self, path: &str) -> Result<String, String>;
    fn lsp_binary_settings(&self, server_id: &str) -> Result<Option<LspBinarySettings>, String>;
    #[cfg(any(test, not(feature = "command_api")))]
    fn use_bundler(&self, server_id: &str) -> Result<Option<bool>, String>;
    fn which(&self, name: &str) -> Option<String>;
}

impl WorktreeLike for zed::Worktree {
    fn root_path(&self) -> String {
        zed::Worktree::root_path(self)
    }

    fn shell_env(&self) -> Vec<(String, String)> {
        zed::Worktree::shell_env(self)
    }

    fn read_text_file(&self, path: &str) -> Result<String, String> {
        zed::Worktree::read_text_file(self, path)
    }

    fn lsp_binary_settings(&self, server_id: &str) -> Result<Option<LspBinarySettings>, String> {
        match zed::settings::LspSettings::for_worktree(server_id, self) {
            Ok(lsp_settings) => Ok(lsp_settings.binary.map(|b| LspBinarySettings {
                path: b.path,
                arguments: b.arguments,
            })),
            Err(e) => Err(e),
        }
    }

    #[cfg(any(test, not(feature = "command_api")))]
    fn use_bundler(&self, server_id: &str) -> Result<Option<bool>, String> {
        zed::settings::LspSettings::for_worktree(server_id, self).map(|lsp_settings| {
            lsp_settings
                .settings
                .as_ref()
                .and_then(|settings| settings["use_bundler"].as_bool())
        })
    }

    fn which(&self, name: &str) -> Option<String> {
        zed::Worktree::which(self, name)
    }
}

#[cfg(test)]
pub struct FakeWorktree {
    root_path: String,
    shell_env: Vec<(String, String)>,
    files: HashMap<String, Result<String, String>>,
    lsp_binary_settings_map: HashMap<String, Result<Option<LspBinarySettings>, String>>,
    use_bundler_map: HashMap<String, Result<Option<bool>, String>>,
    which_map: HashMap<String, Option<String>>,
}

#[cfg(test)]
impl FakeWorktree {
    pub fn new(root_path: String) -> Self {
        FakeWorktree {
            root_path,
            shell_env: Vec::new(),
            files: HashMap::new(),
            lsp_binary_settings_map: HashMap::new(),
            use_bundler_map: HashMap::new(),
            which_map: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, path: String, content: Result<String, String>) {
        self.files.insert(path, content);
    }

    pub fn add_lsp_binary_setting(
        &mut self,
        server_id: String,
        settings: Result<Option<LspBinarySettings>, String>,
    ) {
        self.lsp_binary_settings_map.insert(server_id, settings);
    }

    pub fn set_use_bundler(&mut self, server_id: String, value: Result<Option<bool>, String>) {
        self.use_bundler_map.insert(server_id, value);
    }

    pub fn set_which(&mut self, name: String, result: Option<String>) {
        self.which_map.insert(name, result);
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
        self.files
            .get(path)
            .cloned()
            .unwrap_or_else(|| Err(format!("File not found in mock: {path}")))
    }

    fn lsp_binary_settings(&self, server_id: &str) -> Result<Option<LspBinarySettings>, String> {
        self.lsp_binary_settings_map
            .get(server_id)
            .cloned()
            .unwrap_or(Ok(None))
    }

    fn use_bundler(&self, server_id: &str) -> Result<Option<bool>, String> {
        self.use_bundler_map
            .get(server_id)
            .cloned()
            .unwrap_or(Ok(None))
    }

    fn which(&self, name: &str) -> Option<String> {
        self.which_map.get(name).cloned().flatten()
    }
}

pub trait LanguageServer {
    const SERVER_ID: &str;
    const EXECUTABLE_NAME: &str;
    #[allow(dead_code)]
    const GEM_NAME: &str;

    fn default_use_bundler() -> bool {
        true
    }

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
        #[cfg(not(feature = "command_api"))]
        {
            return self.command_free_language_server_binary(language_server_id.as_ref(), worktree);
        }

        #[cfg(feature = "command_api")]
        {
            let lsp_settings =
                zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

            if let Some(binary_settings) = &lsp_settings.binary {
                if let Some(path) = &binary_settings.path {
                    return Ok(LanguageServerBinary {
                        path: path.clone(),
                        args: binary_settings.arguments.clone(),
                        env: Some(worktree.shell_env()),
                    });
                }
            }

            let use_bundler = lsp_settings
                .settings
                .as_ref()
                .and_then(|settings| settings["use_bundler"].as_bool())
                .unwrap_or_else(Self::default_use_bundler);

            if !use_bundler {
                return self.try_find_on_path_or_extension_gemset(language_server_id, worktree);
            }

            let bundler = Bundler::new(PathBuf::from(worktree.root_path()), RealCommandExecutor);
            let shell_env = worktree.shell_env();
            let env_vars: Vec<(&str, &str)> = shell_env
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str()))
                .collect();

            match bundler.installed_gem_version(Self::GEM_NAME, &env_vars) {
                Ok(_version) => {
                    let bundle_path = worktree
                        .which("bundle")
                        .ok_or_else(|| "Unable to find 'bundle' command".to_string())?;

                    Ok(LanguageServerBinary {
                        path: bundle_path,
                        args: Some(
                            vec!["exec".into(), Self::EXECUTABLE_NAME.into()]
                                .into_iter()
                                .chain(self.get_executable_args(worktree))
                                .collect(),
                        ),
                        env: Some(shell_env),
                    })
                }
                Err(_e) => self.try_find_on_path_or_extension_gemset(language_server_id, worktree),
            }
        }
    }

    #[cfg(any(test, not(feature = "command_api")))]
    fn command_free_language_server_binary<T: WorktreeLike>(
        &self,
        server_id: &str,
        worktree: &T,
    ) -> zed::Result<LanguageServerBinary> {
        if let Some(binary_settings) = worktree.lsp_binary_settings(server_id)? {
            if let Some(path) = binary_settings.path {
                return Ok(LanguageServerBinary {
                    path,
                    args: binary_settings.arguments,
                    env: Some(worktree.shell_env()),
                });
            }
        }

        let use_bundler = worktree
            .use_bundler(server_id)?
            .unwrap_or_else(Self::default_use_bundler);

        if use_bundler {
            if let Some(bundle_path) = worktree.which("bundle") {
                return Ok(LanguageServerBinary {
                    path: bundle_path,
                    args: Some(
                        vec!["exec".into(), Self::EXECUTABLE_NAME.into()]
                            .into_iter()
                            .chain(self.get_executable_args(worktree))
                            .collect(),
                    ),
                    env: Some(worktree.shell_env()),
                });
            }
        }

        if let Some(path) = worktree.which(Self::EXECUTABLE_NAME) {
            return Ok(LanguageServerBinary {
                path,
                args: Some(self.get_executable_args(worktree)),
                env: Some(worktree.shell_env()),
            });
        }

        Err(format!(
            "Unable to find 'bundle' or '{}' command for {server_id}. Install one in the project environment or configure lsp.{server_id}.binary.path.",
            Self::EXECUTABLE_NAME
        ))
    }

    #[cfg(feature = "command_api")]
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

    #[cfg(feature = "command_api")]
    fn extension_gemset_language_server_binary(
        &self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<LanguageServerBinary> {
        let base_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get extension directory: {e:#}"))?;

        let worktree_shell_env = worktree.shell_env();
        let worktree_shell_env_vars: Vec<(&str, &str)> = worktree_shell_env
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();

        let gem_home =
            versioned_gem_home(&base_dir, &worktree_shell_env_vars, &RealCommandExecutor)
                .map_err(|e| format!("{:#}", e))?;

        let gemset = Gemset::new(
            gem_home,
            Some(&worktree_shell_env_vars),
            Box::new(RealCommandExecutor),
        );
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        match gemset.installed_gem_version(Self::GEM_NAME) {
            Ok(Some(version)) => {
                if gemset
                    .is_outdated_gem(Self::GEM_NAME)
                    .map_err(|e| format!("{:#}", e))?
                {
                    zed::set_language_server_installation_status(
                        language_server_id,
                        &zed::LanguageServerInstallationStatus::Downloading,
                    );

                    gemset
                        .update_gem(Self::GEM_NAME)
                        .map_err(|e| format!("{:#}", e))?;

                    // Try to uninstall old version, but don't fail if it errors
                    // The new version is already installed and working
                    if let Err(e) = gemset.uninstall_gem(Self::GEM_NAME, &version) {
                        eprintln!(
                            "Warning: Failed to uninstall old version {} of {}: {:#}",
                            version,
                            Self::GEM_NAME,
                            e
                        );
                    }
                }

                let executable_path = gemset
                    .gem_bin_path(Self::EXECUTABLE_NAME)
                    .map_err(|e| format!("{:#}", e))?;

                Ok(LanguageServerBinary {
                    path: executable_path,
                    args: Some(self.get_executable_args(worktree)),
                    env: Some(gemset.env().to_vec()),
                })
            }
            Ok(None) => {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &zed::LanguageServerInstallationStatus::Downloading,
                );

                gemset
                    .install_gem(Self::GEM_NAME)
                    .map_err(|e| format!("{:#}", e))?;

                let executable_path = gemset
                    .gem_bin_path(Self::EXECUTABLE_NAME)
                    .map_err(|e| format!("{:#}", e))?;

                Ok(LanguageServerBinary {
                    path: executable_path,
                    args: Some(self.get_executable_args(worktree)),
                    env: Some(gemset.env().to_vec()),
                })
            }
            Err(e) => Err(format!("{:#}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FakeWorktree, LanguageServer, WorktreeLike};

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

    #[test]
    fn test_fake_worktree_root_path() {
        let mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        assert_eq!(mock_worktree.root_path(), "/path/to/project");
    }

    #[test]
    fn test_fake_worktree_shell_env() {
        let mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        assert_eq!(mock_worktree.shell_env(), Vec::<(String, String)>::new());
    }

    #[test]
    fn test_command_free_uses_bundle_exec_when_use_bundler_enabled() {
        let test_server = TestServer::new();
        let mut mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        mock_worktree.set_use_bundler(TestServer::SERVER_ID.to_string(), Ok(Some(true)));
        mock_worktree.set_which("bundle".to_string(), Some("/bin/bundle".to_string()));

        let binary = test_server
            .command_free_language_server_binary(TestServer::SERVER_ID, &mock_worktree)
            .expect("command-free resolver should find bundle");

        assert_eq!(binary.path, "/bin/bundle");
        assert_eq!(
            binary.args,
            Some(vec![
                "exec".to_string(),
                "test-exe".to_string(),
                "--test-arg".to_string()
            ])
        );
    }

    #[test]
    fn test_command_free_falls_back_to_executable_when_bundle_missing() {
        let test_server = TestServer::new();
        let mut mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        mock_worktree.set_use_bundler(TestServer::SERVER_ID.to_string(), Ok(Some(true)));
        mock_worktree.set_which("bundle".to_string(), None);
        mock_worktree.set_which("test-exe".to_string(), Some("/bin/test-exe".to_string()));

        let binary = test_server
            .command_free_language_server_binary(TestServer::SERVER_ID, &mock_worktree)
            .expect("command-free resolver should fall back to executable");

        assert_eq!(binary.path, "/bin/test-exe");
        assert_eq!(binary.args, Some(vec!["--test-arg".to_string()]));
    }

    #[test]
    fn test_command_free_uses_configured_binary_before_bundler() {
        let test_server = TestServer::new();
        let mut mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        mock_worktree.set_use_bundler(TestServer::SERVER_ID.to_string(), Ok(Some(true)));
        mock_worktree.add_lsp_binary_setting(
            TestServer::SERVER_ID.to_string(),
            Ok(Some(super::LspBinarySettings {
                path: Some("/custom/test-exe".to_string()),
                arguments: Some(vec!["--custom".to_string()]),
            })),
        );

        let binary = test_server
            .command_free_language_server_binary(TestServer::SERVER_ID, &mock_worktree)
            .expect("command-free resolver should use configured binary");

        assert_eq!(binary.path, "/custom/test-exe");
        assert_eq!(binary.args, Some(vec!["--custom".to_string()]));
    }

    #[test]
    fn test_command_free_uses_path_lookup_when_use_bundler_disabled() {
        let test_server = TestServer::new();
        let mut mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        mock_worktree.set_use_bundler(TestServer::SERVER_ID.to_string(), Ok(Some(false)));
        mock_worktree.set_which("test-exe".to_string(), Some("/bin/test-exe".to_string()));

        let binary = test_server
            .command_free_language_server_binary(TestServer::SERVER_ID, &mock_worktree)
            .expect("command-free resolver should find server executable");

        assert_eq!(binary.path, "/bin/test-exe");
        assert_eq!(binary.args, Some(vec!["--test-arg".to_string()]));
    }

    #[test]
    fn test_command_free_missing_executable_errors() {
        let test_server = TestServer::new();
        let mut mock_worktree = FakeWorktree::new("/path/to/project".to_string());
        mock_worktree.set_use_bundler(TestServer::SERVER_ID.to_string(), Ok(Some(true)));
        mock_worktree.set_which("bundle".to_string(), None);
        mock_worktree.set_which("test-exe".to_string(), None);

        let error = test_server
            .command_free_language_server_binary(TestServer::SERVER_ID, &mock_worktree)
            .expect_err("command-free resolver should fail when executable is missing");

        assert!(error.contains("Unable to find 'bundle' or 'test-exe' command"));
        assert!(error.contains("lsp.test-server.binary.path"));
    }
}
