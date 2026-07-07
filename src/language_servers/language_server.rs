#[cfg(test)]
use std::collections::HashMap;

use crate::{
    bundler::Bundler,
    command_executor::RealCommandExecutor,
    gemset::{versioned_gem_home, Gemset},
};
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
            Ok(_version) => bundle_exec_binary(
                matches!(zed::current_platform().0, zed::Os::Mac),
                worktree.which("bundle"),
                Self::EXECUTABLE_NAME,
                self.get_executable_args(worktree),
                shell_env,
            ),
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
        let base_dir = std::env::current_dir()
            .map_err(|e| format!("Failed to get extension directory: {e:#}"))?;

        let worktree_shell_env = worktree.shell_env();
        let worktree_shell_env_vars: Vec<(&str, &str)> = worktree_shell_env
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();

        let gem_home = versioned_gem_home(
            &base_dir,
            &worktree.root_path(),
            &worktree_shell_env_vars,
            &RealCommandExecutor,
        )
        .map_err(|e| format!("{:#}", e))?;

        let gemset = Gemset::new(
            gem_home,
            worktree.root_path(),
            Some(&worktree_shell_env_vars),
            Box::new(RealCommandExecutor),
        );
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let executable_path = match gemset.installed_gem_version(Self::GEM_NAME) {
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

                    if let Err(e) = gemset.uninstall_gem(Self::GEM_NAME, &version) {
                        eprintln!(
                            "Warning: Failed to uninstall old version {} of {}: {:#}",
                            version,
                            Self::GEM_NAME,
                            e
                        );
                    }
                }

                gemset
                    .gem_bin_path(Self::EXECUTABLE_NAME)
                    .map_err(|e| format!("{:#}", e))?
            }
            Ok(None) => {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &zed::LanguageServerInstallationStatus::Downloading,
                );

                gemset
                    .install_gem(Self::GEM_NAME)
                    .map_err(|e| format!("{:#}", e))?;

                gemset
                    .gem_bin_path(Self::EXECUTABLE_NAME)
                    .map_err(|e| format!("{:#}", e))?
            }
            Err(e) => return Err(format!("{:#}", e)),
        };

        gemset_binary(
            matches!(zed::current_platform().0, zed::Os::Mac),
            worktree.root_path(),
            executable_path,
            self.get_executable_args(worktree),
            gemset.env().to_vec(),
        )
    }
}

fn gemset_binary(
    is_macos: bool,
    worktree_root: String,
    executable_path: String,
    extra_args: Vec<String>,
    env: Vec<(String, String)>,
) -> zed::Result<LanguageServerBinary> {
    if is_macos {
        Ok(LanguageServerBinary {
            path: "/bin/sh".to_string(),
            args: Some(
                vec![
                    "-c".to_string(),
                    "cd \"$0\" && command=\"$1\" && shift && exec \"$command\" \"$@\"".to_string(),
                    worktree_root,
                    "ruby".to_string(),
                    executable_path,
                ]
                .into_iter()
                .chain(extra_args)
                .collect(),
            ),
            env: Some(env),
        })
    } else {
        Ok(LanguageServerBinary {
            path: executable_path,
            args: Some(extra_args),
            env: Some(env),
        })
    }
}

fn bundle_exec_binary(
    is_macos: bool,
    bundle_path: Option<String>,
    executable_name: &str,
    extra_args: Vec<String>,
    shell_env: Vec<(String, String)>,
) -> zed::Result<LanguageServerBinary> {
    if is_macos {
        Ok(LanguageServerBinary {
            path: "/bin/sh".to_string(),
            args: Some(
                vec![
                    "-c".to_string(),
                    "exec \"$0\" \"$@\"".to_string(),
                    "bundle".to_string(),
                    "exec".to_string(),
                    executable_name.to_string(),
                ]
                .into_iter()
                .chain(extra_args)
                .collect(),
            ),
            env: Some(shell_env),
        })
    } else {
        let path = bundle_path.ok_or_else(|| "Unable to find 'bundle' command".to_string())?;
        Ok(LanguageServerBinary {
            path,
            args: Some(
                vec!["exec".to_string(), executable_name.to_string()]
                    .into_iter()
                    .chain(extra_args)
                    .collect(),
            ),
            env: Some(shell_env),
        })
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

    mod gemset_binary_tests {
        use super::super::gemset_binary;

        #[test]
        fn test_macos_runs_project_ruby_from_worktree_root() {
            let result = gemset_binary(
                true,
                "/project".to_string(),
                "/zed/gems/bin/ruby-lsp".to_string(),
                vec!["--stdio".to_string()],
                vec![("PATH".to_string(), "/project/bin".to_string())],
            )
            .unwrap();

            assert_eq!(result.path, "/bin/sh");
            assert_eq!(
                result.args.unwrap(),
                vec![
                    "-c",
                    "cd \"$0\" && command=\"$1\" && shift && exec \"$command\" \"$@\"",
                    "/project",
                    "ruby",
                    "/zed/gems/bin/ruby-lsp",
                    "--stdio",
                ]
            );
        }

        #[test]
        fn test_non_macos_runs_gemset_binstub_directly() {
            let result = gemset_binary(
                false,
                "/project".to_string(),
                "/zed/gems/bin/ruby-lsp".to_string(),
                vec!["--stdio".to_string()],
                vec![],
            )
            .unwrap();

            assert_eq!(result.path, "/zed/gems/bin/ruby-lsp");
            assert_eq!(result.args.unwrap(), vec!["--stdio"]);
        }
    }

    mod bundle_exec_binary_tests {
        use super::super::bundle_exec_binary;

        #[test]
        fn test_macos_uses_sh_as_path() {
            let result = bundle_exec_binary(true, None, "ruby-lsp", vec![], vec![]).unwrap();
            assert_eq!(result.path, "/bin/sh");
        }

        #[test]
        fn test_macos_args_shape() {
            let result = bundle_exec_binary(true, None, "ruby-lsp", vec![], vec![]).unwrap();
            let args = result.args.unwrap();
            assert_eq!(
                args,
                vec!["-c", "exec \"$0\" \"$@\"", "bundle", "exec", "ruby-lsp"]
            );
        }

        #[test]
        fn test_macos_with_extra_args() {
            let result =
                bundle_exec_binary(true, None, "ruby-lsp", vec!["--stdio".to_string()], vec![])
                    .unwrap();
            let args = result.args.unwrap();
            assert_eq!(
                args,
                vec![
                    "-c",
                    "exec \"$0\" \"$@\"",
                    "bundle",
                    "exec",
                    "ruby-lsp",
                    "--stdio"
                ]
            );
        }

        #[test]
        fn test_non_macos_uses_bundle_path() {
            let result = bundle_exec_binary(
                false,
                Some("/usr/local/bin/bundle".to_string()),
                "ruby-lsp",
                vec![],
                vec![],
            )
            .unwrap();
            assert_eq!(result.path, "/usr/local/bin/bundle");
            assert_eq!(result.args.unwrap(), vec!["exec", "ruby-lsp"]);
        }

        #[test]
        fn test_non_macos_no_bundle_path_errors() {
            let result = bundle_exec_binary(false, None, "ruby-lsp", vec![], vec![]);
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .contains("Unable to find 'bundle' command"));
        }

        #[test]
        fn test_non_macos_with_extra_args() {
            let result = bundle_exec_binary(
                false,
                Some("/usr/local/bin/bundle".to_string()),
                "ruby-lsp",
                vec!["--stdio".to_string()],
                vec![],
            )
            .unwrap();
            assert_eq!(result.args.unwrap(), vec!["exec", "ruby-lsp", "--stdio"]);
        }
    }
}
