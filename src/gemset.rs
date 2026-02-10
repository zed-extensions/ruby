use crate::command_executor::CommandExecutor;
use anyhow::{anyhow, bail, Context, Result};
use regex::Regex;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::{LazyLock, OnceLock},
};

pub fn versioned_gem_home(
    base_dir: &Path,
    envs: &[(&str, &str)],
    executor: &dyn CommandExecutor,
) -> Result<PathBuf> {
    let output = executor
        .execute("ruby", &["--version"], envs)
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to detect Ruby version")?;

    match output.status {
        Some(0) => {
            let version_string = String::from_utf8_lossy(&output.stdout);
            let mut hasher = DefaultHasher::new();
            version_string.trim().hash(&mut hasher);
            let version_hash = format!("{:x}", hasher.finish());
            Ok(base_dir.join("gems").join(version_hash))
        }
        Some(status) => bail!("Ruby version check failed with status {status}"),
        None => bail!("Failed to execute ruby --version"),
    }
}

/// A simple wrapper around the `gem` command.
pub struct Gemset {
    gem_home: PathBuf,
    envs: Vec<(String, String)>,
    cached_env: OnceLock<Vec<(String, String)>>,
    command_executor: Box<dyn CommandExecutor>,
}

impl Gemset {
    pub fn new(
        gem_home: PathBuf,
        envs: Option<&[(&str, &str)]>,
        command_executor: Box<dyn CommandExecutor>,
    ) -> Self {
        Self {
            gem_home,
            envs: envs.map_or(Vec::new(), |envs| {
                envs.iter()
                    .map(|&(k, v)| (k.to_string(), v.to_string()))
                    .collect()
            }),
            cached_env: OnceLock::new(),
            command_executor,
        }
    }

    /// Returns the full path to a gem binary executable.
    pub fn gem_bin_path(&self, bin_name: &str) -> Result<String> {
        let path = self.gem_home.join("bin").join(bin_name);

        path.to_str()
            .map(ToString::to_string)
            .with_context(|| format!("Failed to convert path for '{bin_name}'"))
    }

    pub fn env(&self) -> &[(String, String)] {
        self.cached_env.get_or_init(|| {
            let mut env_map: std::collections::HashMap<String, String> = self
                .envs
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            let gem_path = self.gem_home.display().to_string();

            // If the GEM_PATH env variable is already set,
            // prepend our gem home directory to it to ensure
            // that our gems are prioritized over system/user gems.
            env_map
                .entry("GEM_PATH".to_string())
                .and_modify(|existing_gem_path| {
                    let paths: Vec<_> = std::env::split_paths(existing_gem_path).collect();
                    let gem_home_path = std::path::Path::new(&gem_path);

                    if !paths.iter().any(|p| p == gem_home_path) {
                        *existing_gem_path = format!("{gem_path}:{existing_gem_path}");
                    }
                })
                .or_insert(gem_path);

            // Do the same for the PATH env variable for binaries
            env_map
                .entry("PATH".to_string())
                .and_modify(|path| {
                    *path = format!("{}:{}", path, self.gem_home.join("bin").display())
                })
                .or_insert(self.gem_home.join("bin").display().to_string());

            env_map.into_iter().collect()
        })
    }

    pub fn install_gem(&self, name: &str) -> Result<()> {
        let args = &[
            "--no-user-install",
            "--no-format-executable",
            "--no-document",
            name,
        ];

        self.execute_gem_command("install", args)
            .with_context(|| format!("Failed to install gem '{name}'"))?;

        Ok(())
    }

    pub fn update_gem(&self, name: &str) -> Result<()> {
        self.execute_gem_command("update", &[name])
            .with_context(|| format!("Failed to update gem '{name}'"))?;
        Ok(())
    }

    pub fn uninstall_gem(&self, name: &str, version: &str) -> Result<()> {
        let args = &[name, "--version", version];
        self.execute_gem_command("uninstall", args)
            .with_context(|| format!("Failed to uninstall gem '{name}' version {version}"))?;

        Ok(())
    }

    pub fn installed_gem_version(&self, name: &str) -> Result<Option<String>> {
        static GEM_VERSION_REGEX: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"^(\S+) \((.+)\)$").unwrap());

        let args = &["--exact", name];
        let output_str = self.execute_gem_command("list", args)?;

        for line in output_str.lines() {
            let captures = match GEM_VERSION_REGEX.captures(line) {
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

    pub fn is_outdated_gem(&self, name: &str) -> Result<bool> {
        self.execute_gem_command("outdated", &[]).map(|output| {
            output
                .lines()
                .any(|line| line.split_whitespace().next().is_some_and(|n| n == name))
        })
    }

    fn execute_gem_command(&self, cmd: &str, args: &[&str]) -> Result<String> {
        let full_args: Vec<&str> = std::iter::once(cmd)
            .chain(std::iter::once("--norc"))
            .chain(args.iter().copied())
            .collect();
        let gem_home_str = self
            .gem_home
            .to_str()
            .context("Failed to convert gem_home path to string")?;

        let command_envs = vec![("GEM_HOME", gem_home_str)];

        let merged_envs: Vec<(&str, &str)> = command_envs
            .into_iter()
            .chain(self.envs.iter().map(|(k, v)| (k.as_str(), v.as_str())))
            .collect();

        let output = self
            .command_executor
            .execute("gem", &full_args, &merged_envs)
            .map_err(|e| anyhow!(e))?;

        match output.status {
            Some(0) => Ok(String::from_utf8_lossy(&output.stdout).into_owned()),
            Some(status) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                bail!("Gem command failed (status: {status})\nError: {stderr}")
            }
            None => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                bail!("Failed to execute gem command: {stderr}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command_executor::CommandExecutor;
    use std::cell::RefCell;
    use std::path::Path;
    use zed_extension_api::process::Output;

    struct MockExecutorConfig {
        expected_command_name: Option<String>,
        expected_args: Option<Vec<String>>,
        expected_envs: Option<Vec<(String, String)>>,
        output_to_return: Option<Result<Output, String>>,
    }

    struct MockCommandExecutor {
        config: RefCell<MockExecutorConfig>,
    }

    impl MockCommandExecutor {
        fn new() -> Self {
            MockCommandExecutor {
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

    impl CommandExecutor for MockCommandExecutor {
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
                .expect("MockCommandExecutor: output_to_return was not set or already consumed")
        }
    }

    const TEST_GEM_HOME: &str = "/test/gem_home";
    const TEST_GEM_PATH: &str = "/test/gem_path";

    fn create_gemset(envs: Option<&[(&str, &str)]>, mock_executor: MockCommandExecutor) -> Gemset {
        Gemset::new(TEST_GEM_HOME.into(), envs, Box::new(mock_executor))
    }

    #[test]
    fn test_versioned_gem_home_success() {
        let executor = MockCommandExecutor::new();
        executor.expect(
            "ruby",
            &["--version"],
            &[],
            Ok(Output {
                status: Some(0),
                stdout: "ruby 3.3.0 (2023-12-25 revision 5124f9ac75) [arm64-darwin23]\n"
                    .as_bytes()
                    .to_vec(),
                stderr: Vec::new(),
            }),
        );

        let result = versioned_gem_home(Path::new("/extension"), &[], &executor);
        assert!(result.is_ok());
        let path = result.expect("should return path");
        assert!(path.starts_with("/extension/gems/"));
        assert_eq!(path.components().count(), 4);
    }

    #[test]
    fn test_versioned_gem_home_different_versions_produce_different_hashes() {
        let executor1 = MockCommandExecutor::new();
        executor1.expect(
            "ruby",
            &["--version"],
            &[],
            Ok(Output {
                status: Some(0),
                stdout: "ruby 3.3.0 (2023-12-25 revision 5124f9ac75) [arm64-darwin23]\n"
                    .as_bytes()
                    .to_vec(),
                stderr: Vec::new(),
            }),
        );

        let executor2 = MockCommandExecutor::new();
        executor2.expect(
            "ruby",
            &["--version"],
            &[],
            Ok(Output {
                status: Some(0),
                stdout: "ruby 3.2.2 (2023-03-30 revision e51014f9c0) [arm64-darwin23]\n"
                    .as_bytes()
                    .to_vec(),
                stderr: Vec::new(),
            }),
        );

        let path1 = versioned_gem_home(Path::new("/extension"), &[], &executor1)
            .expect("should return path");
        let path2 = versioned_gem_home(Path::new("/extension"), &[], &executor2)
            .expect("should return path");

        assert_ne!(path1, path2);
    }

    #[test]
    fn test_versioned_gem_home_same_version_produces_same_hash() {
        let version_output = "ruby 3.3.0 (2023-12-25 revision 5124f9ac75) [arm64-darwin23]\n";

        let executor1 = MockCommandExecutor::new();
        executor1.expect(
            "ruby",
            &["--version"],
            &[],
            Ok(Output {
                status: Some(0),
                stdout: version_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );

        let executor2 = MockCommandExecutor::new();
        executor2.expect(
            "ruby",
            &["--version"],
            &[],
            Ok(Output {
                status: Some(0),
                stdout: version_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );

        let path1 = versioned_gem_home(Path::new("/extension"), &[], &executor1)
            .expect("should return path");
        let path2 = versioned_gem_home(Path::new("/extension"), &[], &executor2)
            .expect("should return path");

        assert_eq!(path1, path2);
    }

    #[test]
    fn test_versioned_gem_home_command_failure() {
        let executor = MockCommandExecutor::new();
        executor.expect(
            "ruby",
            &["--version"],
            &[],
            Ok(Output {
                status: Some(127),
                stdout: Vec::new(),
                stderr: "ruby: command not found".as_bytes().to_vec(),
            }),
        );

        let result = versioned_gem_home(Path::new("/extension"), &[], &executor);
        assert!(result.is_err());
        let error_message = format!("{:#}", result.expect_err("should return error"));
        assert!(error_message.contains("Ruby version check failed with status 127"));
    }

    #[test]
    fn test_versioned_gem_home_execution_error() {
        let executor = MockCommandExecutor::new();
        executor.expect(
            "ruby",
            &["--version"],
            &[],
            Err("Failed to spawn process".to_string()),
        );

        let result = versioned_gem_home(Path::new("/extension"), &[], &executor);
        assert!(result.is_err());
        let error_message = format!("{:#}", result.expect_err("should return error"));
        assert!(error_message.contains("Failed to detect Ruby version"));
    }

    #[test]
    fn test_gem_bin_path() {
        let gemset = Gemset::new(
            TEST_GEM_HOME.into(),
            None,
            Box::new(MockCommandExecutor::new()),
        );
        let path = gemset.gem_bin_path("ruby-lsp").unwrap();
        let expected = Path::new(TEST_GEM_HOME)
            .join("bin")
            .join("ruby-lsp")
            .to_string_lossy()
            .into_owned();
        assert_eq!(path, expected);
    }

    #[test]
    fn test_gem_env() {
        let gemset = Gemset::new(
            TEST_GEM_HOME.into(),
            Some(&[("GEM_PATH", TEST_GEM_PATH), ("PATH", "/usr/bin")]),
            Box::new(MockCommandExecutor::new()),
        );
        let env: std::collections::HashMap<String, String> = gemset.env().iter().cloned().collect();

        let gem_home = Path::new(TEST_GEM_HOME).display().to_string();
        let gem_bin = Path::new(TEST_GEM_HOME).join("bin").display().to_string();

        assert_eq!(env.len(), 2);
        assert_eq!(
            env.get("GEM_PATH").unwrap(),
            &format!("{gem_home}:{TEST_GEM_PATH}")
        );
        assert_eq!(env.get("PATH").unwrap(), &format!("/usr/bin:{gem_bin}"));
    }

    #[test]
    fn test_install_gem_success() {
        let mock_executor = MockCommandExecutor::new();
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
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(0),
                stdout: "Successfully installed ruby-lsp-1.0.0".as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        assert!(gemset.install_gem(gem_name).is_ok());
    }

    #[test]
    fn test_install_gem_with_custom_env() {
        let mock_executor = MockCommandExecutor::new();
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
            &[("GEM_HOME", TEST_GEM_HOME), ("CUSTOM_VAR", "custom_value")],
            Ok(Output {
                status: Some(0),
                stdout: "Successfully installed ruby-lsp-1.0.0".as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = Gemset::new(
            TEST_GEM_HOME.into(),
            Some(&[("CUSTOM_VAR", "custom_value")]),
            Box::new(mock_executor),
        );
        assert!(gemset.install_gem(gem_name).is_ok());
    }

    #[test]
    fn test_install_gem_failure() {
        let mock_executor = MockCommandExecutor::new();
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
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(1),
                stdout: Vec::new(),
                stderr: "Installation error".as_bytes().to_vec(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        let result = gemset.install_gem(gem_name);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to install gem 'ruby-lsp'"));
    }

    #[test]
    fn test_update_gem_success() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &["update", "--norc", gem_name],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(0),
                stdout: "Gems updated: ruby-lsp".as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        assert!(gemset.update_gem(gem_name).is_ok());
    }

    #[test]
    fn test_update_gem_failure() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &["update", "--norc", gem_name],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(1),
                stdout: Vec::new(),
                stderr: "Update error".as_bytes().to_vec(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        let result = gemset.update_gem(gem_name);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to update gem 'ruby-lsp'"));
    }

    #[test]
    fn test_installed_gem_version_found() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "ruby-lsp";
        let expected_version = "1.2.3";
        let gem_list_output = format!(
            "{}\n{} ({})\n{}",
            "ignore this", gem_name, expected_version, "other_gem (3.2.1)"
        );

        mock_executor.expect(
            "gem",
            &["list", "--norc", "--exact", gem_name],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(0),
                stdout: gem_list_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        let version = gemset.installed_gem_version(gem_name).unwrap();
        assert_eq!(version, Some(expected_version.to_string()));
    }

    #[test]
    fn test_installed_gem_version_found_with_default() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "prism";
        let version_in_output = "default: 1.2.0";
        let gem_list_output = format!(
            "{}\n{} ({})\n{}",
            "*** LOCAL GEMS ***", gem_name, version_in_output, "abbrev (0.1.2)"
        );

        mock_executor.expect(
            "gem",
            &["list", "--norc", "--exact", gem_name],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(0),
                stdout: gem_list_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        let version = gemset.installed_gem_version(gem_name).unwrap();
        assert_eq!(version, Some(version_in_output.to_string()));
    }

    #[test]
    fn test_installed_gem_version_not_found() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "non_existent_gem";
        let gem_list_output = "other_gem (1.0.0)\nanother_gem (2.0.0)";

        mock_executor.expect(
            "gem",
            &["list", "--norc", "--exact", gem_name],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(0),
                stdout: gem_list_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        let version = gemset.installed_gem_version(gem_name).unwrap();
        assert_eq!(version, None);
    }

    #[test]
    fn test_installed_gem_version_command_failure() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &["list", "--norc", "--exact", gem_name],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(127),
                stdout: Vec::new(),
                stderr: "gem list error".as_bytes().to_vec(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        let result = gemset.installed_gem_version(gem_name);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Gem command failed (status: 127)"));
    }

    #[test]
    fn test_is_outdated_gem_true() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "ruby-lsp";
        let outdated_output = format!(
            "{} (3.3.2 < 3.3.4)\n{} (2.9.1 < 2.11.3)\n{} (0.5.6 < 0.5.8)",
            "csv", gem_name, "net-imap"
        );

        mock_executor.expect(
            "gem",
            &["outdated", "--norc"],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(0),
                stdout: outdated_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        let is_outdated = gemset.is_outdated_gem(gem_name).unwrap();
        assert!(is_outdated);
    }

    #[test]
    fn test_is_outdated_gem_false() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "ruby-lsp";
        let outdated_output = "csv (3.3.2 < 3.3.4)";

        mock_executor.expect(
            "gem",
            &["outdated", "--norc"],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(0),
                stdout: outdated_output.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        let is_outdated = gemset.is_outdated_gem(gem_name).unwrap();
        assert!(!is_outdated);
    }

    #[test]
    fn test_is_outdated_gem_command_failure() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            "gem",
            &["outdated", "--norc"],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(1),
                stdout: Vec::new(),
                stderr: "outdated command error".as_bytes().to_vec(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        let result = gemset.is_outdated_gem(gem_name);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Gem command failed (status: 1)"));
    }

    #[test]
    fn test_uninstall_gem_success() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "solargraph";
        let gem_version = "0.55.1";

        mock_executor.expect(
            "gem",
            &["uninstall", "--norc", gem_name, "--version", gem_version],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(0),
                stdout: format!("Successfully uninstalled {gem_name}-{gem_version}")
                    .as_bytes()
                    .to_vec(),
                stderr: Vec::new(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        assert!(gemset.uninstall_gem(gem_name, gem_version).is_ok());
    }

    #[test]
    fn test_uninstall_gem_failure() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "solargraph";
        let gem_version = "0.55.1";

        mock_executor.expect(
            "gem",
            &["uninstall", "--norc", gem_name, "--version", gem_version],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Ok(Output {
                status: Some(1),
                stdout: Vec::new(),
                stderr: format!("ERROR: While executing gem ... (Gem::InstallError)\n    gem \"{gem_name}\" is not installed")
                    .as_bytes()
                    .to_vec(),
            }),
        );
        let gemset = create_gemset(None, mock_executor);
        let result = gemset.uninstall_gem(gem_name, gem_version);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Failed to uninstall gem 'solargraph'"));
    }

    #[test]
    fn test_uninstall_gem_command_execution_error() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "solargraph";
        let gem_version = "0.55.1";

        mock_executor.expect(
            "gem",
            &["uninstall", "--norc", gem_name, "--version", gem_version],
            &[("GEM_HOME", TEST_GEM_HOME)],
            Err("Command not found: gem".to_string()),
        );
        let gemset = create_gemset(None, mock_executor);
        let result = gemset.uninstall_gem(gem_name, gem_version);
        assert!(result.is_err());
        let error_message = format!("{:#}", result.unwrap_err());
        assert!(error_message.contains("Failed to uninstall gem 'solargraph'"));
        assert!(error_message.contains("Command not found: gem"));
    }
}
