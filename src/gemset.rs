use crate::command_executor::CommandExecutor;
use regex::Regex;
use std::path::PathBuf;

/// A simple wrapper around the `gem` command.
pub struct Gemset {
    gem_home: PathBuf,
    working_dir: PathBuf,
    envs: Vec<(String, String)>,
    command_executor: Box<dyn CommandExecutor>,
}

impl Gemset {
    pub fn new(
        gem_home: PathBuf,
        working_dir: PathBuf,
        envs: Option<&[(&str, &str)]>,
        command_executor: Box<dyn CommandExecutor>,
    ) -> Self {
        Self {
            gem_home,
            working_dir,
            envs: envs.map_or(Vec::new(), |envs| {
                envs.iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect()
            }),
            command_executor,
        }
    }

    /// Returns the full path to a gem binary executable.
    pub fn gem_bin_path(&self, bin_name: &str) -> Result<String, String> {
        let path = self.gem_home.join("bin").join(bin_name);

        path.to_str()
            .map(ToString::to_string)
            .ok_or_else(|| format!("Failed to convert path for '{bin_name}'"))
    }

    pub fn env(&self, envs: Option<&[(&str, &str)]>) -> Vec<(String, String)> {
        let gem_bin_path = self.gem_home.join("bin");
        let gem_bin_str = gem_bin_path.display().to_string();
        let gem_path = (
            "GEM_PATH".to_string(),
            format!("{}:$GEM_PATH", self.gem_home.display()),
        );

        let path_override = {
            let existing_path = envs
                .unwrap_or(&[])
                .iter()
                .find(|(k, _)| *k == "PATH")
                .map(|(_, v)| v)
                .map_or("$PATH", |v| v);

            (
                "PATH".to_string(),
                format!("{}:{}", gem_bin_str, existing_path),
            )
        };

        envs.unwrap_or(&[])
            .iter()
            .filter(|(k, _)| *k != "GEM_PATH" || *k != "PATH")
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .chain([gem_path, path_override])
            .collect()
    }

    pub fn install_gem(&self, name: &str) -> Result<(), String> {
        let args = &[
            "--no-user-install",
            "--no-format-executable",
            "--no-document",
            name,
        ];

        self.execute_gem_command("install", args)
            .map_err(|e| format!("Failed to install gem '{name}': {e}"))?;

        Ok(())
    }

    pub fn update_gem(&self, name: &str) -> Result<(), String> {
        self.execute_gem_command("update", &[name])
            .map_err(|e| format!("Failed to update gem '{name}': {e}"))?;
        Ok(())
    }

    pub fn uninstall_gem(&self, name: &str, version: &str) -> Result<(), String> {
        let args = &[name, "--version", version];
        self.execute_gem_command("uninstall", args)
            .map_err(|e| format!("Failed to uninstall gem '{name}': {e}"))?;

        Ok(())
    }

    pub fn installed_gem_version(&self, name: &str) -> Result<Option<String>, String> {
        let re =
            Regex::new(r"^(\S+) \((.+)\)$").map_err(|e| format!("Failed to compile regex: {e}"))?;

        let args = &["--exact", name];
        let output_str = self.execute_gem_command("list", args)?;

        for line in output_str.lines() {
            let captures = match re.captures(line) {
                Some(c) => c,
                None => continue,
            };

            let gem_package = captures.get(1).map(|m| m.as_str());
            let version = captures.get(2).map(|m| m.as_str());

            if gem_package == Some(name) {
                return Ok(version.map(|v| v.to_owned()));
            }
        }
        Ok(None)
    }

    pub fn is_outdated_gem(&self, name: &str) -> Result<bool, String> {
        self.execute_gem_command("outdated", &[]).map(|output| {
            output
                .lines()
                .any(|line| line.split_whitespace().next().is_some_and(|n| n == name))
        })
    }

    fn execute_gem_command(&self, cmd: &str, args: &[&str]) -> Result<String, String> {
        let full_args: Vec<&str> = std::iter::once(cmd)
            .chain(std::iter::once("--norc"))
            .chain(args.iter().copied())
            .collect();
        let gem_home_str = self
            .gem_home
            .to_str()
            .ok_or("Failed to convert gem_home path to string")?;
        let working_dir_str = self
            .working_dir
            .to_str()
            .ok_or("Failed to convert working_dir path to string")?;

        let command_envs = vec![
            ("GEM_HOME", gem_home_str),
            ("GEM_PATH", gem_home_str),
            ("RBENV_DIR", working_dir_str),
        ];

        let merged_envs: Vec<(&str, &str)> = command_envs
            .into_iter()
            .chain(self.envs.iter().map(|(k, v)| (k.as_str(), v.as_str())))
            .collect();

        self.command_executor
            .execute("gem", &full_args, &merged_envs)
            .and_then(|output| match output.status {
                Some(0) => Ok(String::from_utf8_lossy(&output.stdout).into_owned()),
                Some(status) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(format!(
                        "Gem command failed (status: {status})\nError: {stderr}",
                    ))
                }
                None => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(format!("Failed to execute gem command: {stderr}"))
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_executor::CommandExecutor;
    use std::cell::RefCell;
    use zed_extension_api::process::Output;

    struct MockExecutorConfig {
        expected_command_name: Option<String>,
        expected_args: Option<Vec<String>>,
        expected_envs: Option<Vec<(String, String)>>,
        output_to_return: Option<Result<Output, String>>,
    }

    struct MockGemCommandExecutor {
        config: RefCell<MockExecutorConfig>,
    }

    impl MockGemCommandExecutor {
        fn new() -> Self {
            MockGemCommandExecutor {
                config: RefCell::new(MockExecutorConfig {
                    expected_command_name: None,
                    expected_args: None,
                    expected_envs: None,
                    output_to_return: None,
                }),
            }
        }

        fn expect(
            &self,
            command_name: &str,
            full_args: &[&str],
            final_envs: &[(&str, &str)],
            output: Result<Output, String>,
        ) {
            let mut config = self.config.borrow_mut();
            config.expected_command_name = Some(command_name.to_string());
            config.expected_args = Some(full_args.iter().map(|s| s.to_string()).collect());
            config.expected_envs = Some(
                final_envs
                    .iter()
                    .map(|&(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            );
            config.output_to_return = Some(output);
        }
    }

    impl CommandExecutor for MockGemCommandExecutor {
        fn execute(
            &self,
            command_name: &str,
            args: &[&str],
            envs: &[(&str, &str)],
        ) -> Result<Output, String> {
            let mut config = self.config.borrow_mut();

            if let Some(expected_name) = &config.expected_command_name {
                assert_eq!(command_name, expected_name, "Mock: Command name mismatch");
            }
            if let Some(expected_args) = &config.expected_args {
                assert_eq!(&args, expected_args, "Mock: Args mismatch");
            }
            if let Some(expected_envs) = &config.expected_envs {
                let envs: Vec<(String, String)> = envs
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();
                assert_eq!(&envs, expected_envs, "Mock: Env mismatch");
            }

            config
                .output_to_return
                .take()
                .expect("MockGemCommandExecutor: output_to_return was not set or already consumed")
        }
    }

    const TEST_GEM_HOME: &str = "/test/gem_home";
    const TEST_WORKING_DIR: &str = "/test/my_project";

    fn create_gemset(mock_executor: MockGemCommandExecutor) -> Gemset {
        Gemset::new(
            TEST_GEM_HOME.into(),
            TEST_WORKING_DIR.into(),
            None,
            Box::new(mock_executor),
        )
    }

    #[test]
    fn test_gem_bin_path() {
        let gemset = Gemset::new(
            TEST_GEM_HOME.into(),
            TEST_WORKING_DIR.into(),
            None,
            Box::new(MockGemCommandExecutor::new()),
        );
        let path = gemset.gem_bin_path("ruby-lsp").unwrap();
        assert_eq!(path, "/test/gem_home/bin/ruby-lsp");
    }

    #[test]
    fn test_gem_env() {
        let gemset = Gemset::new(
            TEST_GEM_HOME.into(),
            TEST_WORKING_DIR.into(),
            None,
            Box::new(MockGemCommandExecutor::new()),
        );
        let env = gemset.env(None);
        assert_eq!(env.len(), 2);
        assert_eq!(env[0].0, "GEM_PATH");
        assert_eq!(env[0].1, "/test/gem_home:$GEM_PATH");
    }

    #[test]
    fn test_gem_env_with_env_vars() {
        let gemset = Gemset::new(
            TEST_GEM_HOME.into(),
            TEST_WORKING_DIR.into(),
            None,
            Box::new(MockGemCommandExecutor::new()),
        );
        let env = gemset.env(Some(&[("GEM_HOME", "/home/user/.gem")]));
        assert_eq!(env.len(), 3);

        let env_map: std::collections::HashMap<String, String> = env.into_iter().collect();
        assert_eq!(env_map.get("GEM_HOME").unwrap(), "/home/user/.gem");
        assert_eq!(env_map.get("GEM_PATH").unwrap(), "/test/gem_home:$GEM_PATH");
        assert_eq!(env_map.get("PATH").unwrap(), "/test/gem_home/bin:$PATH");
    }

    #[test]
    fn test_gem_env_with_env_vars_overwrite() {
        let gemset = Gemset::new(
            TEST_GEM_HOME.into(),
            TEST_WORKING_DIR.into(),
            None,
            Box::new(MockGemCommandExecutor::new()),
        );
        let env = gemset.env(Some(&[("GEM_PATH", "/home/user/.gem")]));
        assert_eq!(env.len(), 3);

        // GEM_PATH should be overwritten with our value
        let env_map: std::collections::HashMap<String, String> = env.into_iter().collect();
        assert_eq!(env_map.get("GEM_PATH").unwrap(), "/test/gem_home:$GEM_PATH");
        assert_eq!(env_map.get("PATH").unwrap(), "/test/gem_home/bin:$PATH");
    }

    #[test]
    fn test_install_gem_success() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &[
                "install",
                "--norc",
                "--no-user-install",
                "--no-format-executable",
                "--no-document",
                gem_name,
            ],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(0),
                stdout: "Successfully installed ruby-lsp-1.0.0".as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        assert!(gemset.install_gem(gem_name).is_ok());
    }

    #[test]
    fn test_install_gem_with_custom_env() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &[
                "install",
                "--norc",
                "--no-user-install",
                "--no-format-executable",
                "--no-document",
                gem_name,
            ],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
                ("CUSTOM_VAR", "custom_value"),
            ],
            Ok(Output {
                status: Some(0),
                stdout: "Successfully installed ruby-lsp-1.0.0".as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = Gemset::new(
            TEST_GEM_HOME.into(),
            TEST_WORKING_DIR.into(),
            Some(&[("CUSTOM_VAR", "custom_value")]),
            Box::new(mock_executor),
        );
        assert!(gemset.install_gem(gem_name).is_ok());
    }

    #[test]
    fn test_install_gem_failure() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &[
                "install",
                "--norc",
                "--no-user-install",
                "--no-format-executable",
                "--no-document",
                gem_name,
            ],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(1),
                stdout: Vec::new(),
                stderr: "Installation error".as_bytes().to_vec(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        let result = gemset.install_gem(gem_name);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Failed to install gem 'ruby-lsp'"));
    }

    #[test]
    fn test_update_gem_success() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &["update", "--norc", gem_name],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(0),
                stdout: "Gems updated: ruby-lsp".as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        assert!(gemset.update_gem(gem_name).is_ok());
    }

    #[test]
    fn test_update_gem_failure() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &["update", "--norc", gem_name],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(1),
                stdout: Vec::new(),
                stderr: "Update error".as_bytes().to_vec(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        let result = gemset.update_gem(gem_name);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Failed to update gem 'ruby-lsp'"));
    }

    #[test]
    fn test_installed_gem_version_found() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        let expected_version = "1.2.3";
        let gem_list_output = format!(
            "{}\n{} ({})\n{}",
            "ignore this", gem_name, expected_version, "other_gem (3.2.1)"
        );

        mock_executor.expect(
            "gem",
            &["list", "--norc", "--exact", gem_name],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(0),
                stdout: gem_list_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        let version = gemset.installed_gem_version(gem_name).unwrap();
        assert_eq!(version, Some(expected_version.to_string()));
    }

    #[test]
    fn test_installed_gem_version_found_with_default() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "prism";
        let version_in_output = "default: 1.2.0";
        let gem_list_output = format!(
            "{}\n{} ({})\n{}",
            "*** LOCAL GEMS ***", gem_name, version_in_output, "abbrev (0.1.2)"
        );

        mock_executor.expect(
            "gem",
            &["list", "--norc", "--exact", gem_name],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(0),
                stdout: gem_list_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        let version = gemset.installed_gem_version(gem_name).unwrap();
        assert_eq!(version, Some(version_in_output.to_string()));
    }

    #[test]
    fn test_installed_gem_version_not_found() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "non_existent_gem";
        let gem_list_output = "other_gem (1.0.0)\nanother_gem (2.0.0)";

        mock_executor.expect(
            "gem",
            &["list", "--norc", "--exact", gem_name],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(0),
                stdout: gem_list_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        let version = gemset.installed_gem_version(gem_name).unwrap();
        assert_eq!(version, None);
    }

    #[test]
    fn test_installed_gem_version_command_failure() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &["list", "--norc", "--exact", gem_name],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(127),
                stdout: Vec::new(),
                stderr: "gem list error".as_bytes().to_vec(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        let result = gemset.installed_gem_version(gem_name);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Gem command failed (status: 127)"));
    }

    #[test]
    fn test_is_outdated_gem_true() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        let outdated_output = format!(
            "{} (3.3.2 < 3.3.4)\n{} (2.9.1 < 2.11.3)\n{} (0.5.6 < 0.5.8)",
            "csv", gem_name, "net-imap"
        );

        mock_executor.expect(
            "gem",
            &["outdated", "--norc"],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(0),
                stdout: outdated_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        let is_outdated = gemset.is_outdated_gem(gem_name).unwrap();
        assert!(is_outdated);
    }

    #[test]
    fn test_is_outdated_gem_false() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        let outdated_output = "csv (3.3.2 < 3.3.4)";

        mock_executor.expect(
            "gem",
            &["outdated", "--norc"],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(0),
                stdout: outdated_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        let is_outdated = gemset.is_outdated_gem(gem_name).unwrap();
        assert!(!is_outdated);
    }

    #[test]
    fn test_is_outdated_gem_command_failure() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &["outdated", "--norc"],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(1),
                stdout: Vec::new(),
                stderr: "outdated command error".as_bytes().to_vec(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        let result = gemset.is_outdated_gem(gem_name);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Gem command failed (status: 1)"));
    }

    #[test]
    fn test_uninstall_gem_success() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "solargraph";
        let gem_version = "0.55.1";

        mock_executor.expect(
            "gem",
            &["uninstall", "--norc", gem_name, "--version", gem_version],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Ok(Output {
                status: Some(0),
                stdout: format!("Successfully uninstalled {gem_name}-{gem_version}")
                    .as_bytes()
                    .to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        assert!(gemset.uninstall_gem(gem_name, gem_version).is_ok());
    }

    #[test]
    fn test_uninstall_gem_failure() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "solargraph";
        let gem_version = "0.55.1";

        mock_executor.expect(
            "gem",
            &["uninstall", "--norc", gem_name, "--version", gem_version],
            &[("GEM_HOME", TEST_GEM_HOME), ("GEM_PATH", TEST_GEM_HOME), ("RBENV_DIR", TEST_WORKING_DIR)],
            Ok(Output {
                status: Some(1),
                stdout: Vec::new(),
                stderr: format!("ERROR: While executing gem ... (Gem::InstallError)\n    gem \"{gem_name}\" is not installed")
                    .as_bytes()
                    .to_vec(),
            }),
        );
        let gemset = create_gemset(mock_executor);
        let result = gemset.uninstall_gem(gem_name, gem_version);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Failed to uninstall gem 'solargraph'"));
    }

    #[test]
    fn test_uninstall_gem_command_execution_error() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "solargraph";
        let gem_version = "0.55.1";

        mock_executor.expect(
            "gem",
            &["uninstall", "--norc", gem_name, "--version", gem_version],
            &[
                ("GEM_HOME", TEST_GEM_HOME),
                ("GEM_PATH", TEST_GEM_HOME),
                ("RBENV_DIR", TEST_WORKING_DIR),
            ],
            Err("Command not found: gem".to_string()),
        );
        let gemset = create_gemset(mock_executor);
        let result = gemset.uninstall_gem(gem_name, gem_version);
        assert!(result.is_err());
        let error_message = result.unwrap_err();
        assert!(error_message.contains("Failed to uninstall gem 'solargraph'"));
        assert!(error_message.contains("Command not found: gem"));
    }
}
