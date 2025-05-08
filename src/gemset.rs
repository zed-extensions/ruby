use regex::Regex;
use zed_extension_api::{process::Output, Command, Result};

pub trait GemCommandExecutor {
    fn execute_gem(&self, gem_home: &str, sub_command: String, args: Vec<String>)
        -> Result<Output>;
}

pub struct RealGemCommandExecutor;

impl GemCommandExecutor for RealGemCommandExecutor {
    fn execute_gem(
        &self,
        gem_home: &str,
        sub_command: String,
        args: Vec<String>,
    ) -> Result<Output> {
        Command::new("gem")
            .env("GEM_HOME", gem_home)
            .arg(sub_command)
            .arg("--norc")
            .args(args)
            .output()
    }
}

/// A simple wrapper around the `gem` command.
pub struct Gemset {
    pub gem_home: String,
    command_executor: Box<dyn GemCommandExecutor>,
}

impl Gemset {
    pub fn new(gem_home: String, command_executor: Box<dyn GemCommandExecutor>) -> Self {
        Self {
            gem_home,
            command_executor,
        }
    }

    /// Returns the full path to a gem binary executable.
    pub fn gem_bin_path(&self, bin_name: impl Into<String>) -> Result<String> {
        let bin_name = bin_name.into();
        let path = std::path::Path::new(&self.gem_home)
            .join("bin")
            .join(&bin_name);

        path.to_str()
            .map(ToString::to_string)
            .ok_or_else(|| format!("Failed to convert path for '{}'", bin_name))
    }

    pub fn gem_path_env(&self) -> Vec<(String, String)> {
        vec![(
            "GEM_PATH".to_string(),
            format!("{}:$GEM_PATH", self.gem_home),
        )]
    }

    pub fn install_gem(&self, name: &str) -> Result<()> {
        let args = vec![
            "--no-user-install".to_string(),
            "--no-format-executable".to_string(),
            "--no-document".to_string(),
            name.into(),
        ];

        self.execute_gem_command("install".into(), args)
            .map_err(|e| format!("Failed to install gem '{}': {}", name, e))?;

        Ok(())
    }

    pub fn update_gem(&self, name: &str) -> Result<()> {
        self.execute_gem_command("update".into(), vec![name.into()])
            .map_err(|e| format!("Failed to update gem '{}': {}", name, e))?;
        Ok(())
    }

    pub fn installed_gem_version(&self, name: &str) -> Result<Option<String>> {
        let re = Regex::new(r"^(\S+) \((.+)\)$")
            .map_err(|e| format!("Failed to compile regex: {}", e))?;

        let args = vec!["--exact".to_string(), name.into()];
        let output_str = self.execute_gem_command("list".into(), args)?;

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

    pub fn is_outdated_gem(&self, name: &str) -> Result<bool> {
        self.execute_gem_command("outdated".into(), vec![])
            .map(|output| {
                output
                    .lines()
                    .any(|line| line.split_whitespace().next().is_some_and(|n| n == name))
            })
    }

    fn execute_gem_command(&self, command: String, args: Vec<String>) -> Result<String> {
        self.command_executor
            .execute_gem(&self.gem_home, command, args)
            .and_then(|output| match output.status {
                Some(0) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
                Some(status) => {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(format!(
                        "Gem command failed (status: {})\nError: {}",
                        status, stderr
                    ))
                }
                None => {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(format!("Failed to execute gem command: {}", stderr))
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockExecutorConfig {
        expected_gem_home: Option<String>,
        expected_sub_command: Option<String>,
        expected_args: Option<Vec<String>>,
        output_to_return: Option<Result<Output>>,
    }

    struct MockGemCommandExecutor {
        config: RefCell<MockExecutorConfig>,
    }

    impl MockGemCommandExecutor {
        fn new() -> Self {
            MockGemCommandExecutor {
                config: RefCell::new(MockExecutorConfig {
                    expected_gem_home: None,
                    expected_sub_command: None,
                    expected_args: None,
                    output_to_return: None,
                }),
            }
        }

        fn expect(&self, gem_home: &str, sub_command: &str, args: &[&str], output: Result<Output>) {
            let mut config = self.config.borrow_mut();
            config.expected_gem_home = Some(gem_home.to_string());
            config.expected_sub_command = Some(sub_command.to_string());
            config.expected_args = Some(args.iter().map(|s| s.to_string()).collect());
            config.output_to_return = Some(output);
        }
    }

    impl GemCommandExecutor for MockGemCommandExecutor {
        fn execute_gem(
            &self,
            gem_home: &str,
            sub_command: String,
            args: Vec<String>,
        ) -> Result<Output> {
            let mut config = self.config.borrow_mut();

            if let Some(expected_home) = &config.expected_gem_home {
                assert_eq!(gem_home, expected_home, "Mock: GEM_HOME mismatch");
            }
            if let Some(expected_cmd) = &config.expected_sub_command {
                assert_eq!(&sub_command, expected_cmd, "Mock: Sub-command mismatch");
            }
            if let Some(expected_args) = &config.expected_args {
                assert_eq!(&args, expected_args, "Mock: Args mismatch");
            }

            config
                .output_to_return
                .take()
                .expect("MockGemCommandExecutor: output_to_return was not set or already consumed")
        }
    }

    const TEST_GEM_HOME: &str = "/test/gem_home";

    fn create_gemset(mock_executor: MockGemCommandExecutor) -> Gemset {
        Gemset::new(TEST_GEM_HOME.to_string(), Box::new(mock_executor))
    }

    #[test]
    fn test_gem_bin_path() {
        let gemset = Gemset::new(
            TEST_GEM_HOME.to_string(),
            Box::new(MockGemCommandExecutor::new()),
        );
        let path = gemset.gem_bin_path("ruby-lsp").unwrap();
        assert_eq!(path, "/test/gem_home/bin/ruby-lsp");
    }

    #[test]
    fn test_gem_path_env() {
        let gemset = Gemset::new(
            TEST_GEM_HOME.to_string(),
            Box::new(MockGemCommandExecutor::new()),
        );
        let env = gemset.gem_path_env();
        assert_eq!(env.len(), 1);
        assert_eq!(env[0].0, "GEM_PATH");
        assert_eq!(env[0].1, "/test/gem_home:$GEM_PATH");
    }

    #[test]
    fn test_install_gem_success() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            TEST_GEM_HOME,
            "install",
            &[
                "--no-user-install",
                "--no-format-executable",
                "--no-document",
                gem_name,
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
    fn test_install_gem_failure() {
        let mock_executor = MockGemCommandExecutor::new();
        let gem_name = "ruby-lsp";
        mock_executor.expect(
            TEST_GEM_HOME,
            "install",
            &[
                "--no-user-install",
                "--no-format-executable",
                "--no-document",
                gem_name,
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
            TEST_GEM_HOME,
            "update",
            &[gem_name],
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
            TEST_GEM_HOME,
            "update",
            &[gem_name],
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
            TEST_GEM_HOME,
            "list",
            &["--exact", gem_name],
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
            TEST_GEM_HOME,
            "list",
            &["--exact", gem_name],
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
            TEST_GEM_HOME,
            "list",
            &["--exact", gem_name],
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
            TEST_GEM_HOME,
            "list",
            &["--exact", gem_name],
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
            TEST_GEM_HOME,
            "outdated",
            &[],
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
            TEST_GEM_HOME,
            "outdated",
            &[],
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
            TEST_GEM_HOME,
            "outdated",
            &[],
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
}
