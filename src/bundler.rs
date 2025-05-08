use std::path::Path;
use zed_extension_api::{process::Output, Command, Result};

pub trait CommandExecutor {
    fn execute_bundle(
        &self,
        sub_command: String,
        args: Vec<String>,
        envs: Vec<(String, String)>,
        bundle_gemfile_path: &str,
    ) -> Result<Output>;
}

pub struct RealCommandExecutor;

impl CommandExecutor for RealCommandExecutor {
    fn execute_bundle(
        &self,
        sub_command: String,
        args: Vec<String>,
        envs: Vec<(String, String)>,
        bundle_gemfile_path: &str,
    ) -> Result<Output> {
        Command::new("bundle")
            .arg(sub_command)
            .args(args)
            .envs(envs)
            .env("BUNDLE_GEMFILE", bundle_gemfile_path)
            .output()
    }
}

/// A simple wrapper around the `bundle` command.
pub struct Bundler {
    pub working_dir: String,
    envs: Vec<(String, String)>,
    command_executor: Box<dyn CommandExecutor>,
}

impl Bundler {
    /// Creates a new `Bundler` instance.
    ///
    /// # Arguments
    /// * `working_dir` - The working directory where `bundle` commands should be executed.
    /// * `command_executor` - An executor for `bundle` commands.
    pub fn new(
        working_dir: String,
        envs: Vec<(String, String)>,
        command_executor: Box<dyn CommandExecutor>,
    ) -> Self {
        Bundler {
            working_dir,
            envs,
            command_executor,
        }
    }

    /// Retrieves the installed version of a gem using `bundle info --version <name>`.
    ///
    /// # Arguments
    /// * `name` - The name of the gem.
    ///
    /// # Returns
    /// A `Result` containing the version string if successful, or an error message.
    pub fn installed_gem_version(&self, name: &str) -> Result<String> {
        let args = vec!["--version".into(), name.into()];

        self.execute_bundle_command("info".into(), args)
    }

    fn execute_bundle_command(&self, cmd: String, args: Vec<String>) -> Result<String> {
        let bundle_gemfile_path = Path::new(&self.working_dir).join("Gemfile");
        let bundle_gemfile = bundle_gemfile_path
            .to_str()
            .ok_or_else(|| "Invalid path to Gemfile".to_string())?;

        self.command_executor
            .execute_bundle(cmd, args, self.envs.clone(), bundle_gemfile)
            .and_then(|output| match output.status {
                Some(0) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
                Some(status) => {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(format!(
                        "'bundle' command failed (status: {})\nError: {}",
                        status, stderr
                    ))
                }
                None => {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(format!("Failed to execute 'bundle' command: {}", stderr))
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockExecutorConfig {
        output_to_return: Option<Result<Output>>,
        expected_sub_command: Option<String>,
        expected_args: Option<Vec<String>>,
        expected_envs: Option<Vec<(String, String)>>,
        expected_bundle_gemfile_path: Option<String>,
    }

    struct MockCommandExecutor {
        config: RefCell<MockExecutorConfig>,
    }

    impl MockCommandExecutor {
        fn new() -> Self {
            MockCommandExecutor {
                config: RefCell::new(MockExecutorConfig {
                    output_to_return: None,
                    expected_sub_command: None,
                    expected_args: None,
                    expected_envs: None,
                    expected_bundle_gemfile_path: None,
                }),
            }
        }

        fn expect(
            &self,
            sub_command: &str,
            args: &[&str],
            envs: &[(&str, &str)],
            bundle_gemfile_path: &str,
            output: super::Result<Output>,
        ) {
            let mut config = self.config.borrow_mut();
            config.expected_sub_command = Some(sub_command.to_string());
            config.expected_args = Some(args.iter().map(|s| s.to_string()).collect());
            config.expected_envs = Some(
                envs.iter()
                    .map(|&(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            );
            config.expected_bundle_gemfile_path = Some(bundle_gemfile_path.to_string());
            config.output_to_return = Some(output);
        }
    }

    impl CommandExecutor for MockCommandExecutor {
        fn execute_bundle(
            &self,
            sub_command: String,
            args: Vec<String>,
            envs: Vec<(String, String)>,
            bundle_gemfile_path: &str,
        ) -> super::Result<Output> {
            let mut config = self.config.borrow_mut();

            if let Some(expected_cmd) = &config.expected_sub_command {
                assert_eq!(&sub_command, expected_cmd, "Mock: Sub-command mismatch");
            }
            if let Some(expected_args) = &config.expected_args {
                assert_eq!(&args, expected_args, "Mock: Args mismatch");
            }
            if let Some(expected_envs) = &config.expected_envs {
                assert_eq!(&envs, expected_envs, "Mock: Env mismatch");
            }
            if let Some(expected_path) = &config.expected_bundle_gemfile_path {
                assert_eq!(
                    bundle_gemfile_path, expected_path,
                    "Mock: Gemfile path mismatch"
                );
            }

            config.output_to_return.take().expect(
                   "MockCommandExecutor: output_to_return was not set or already consumed for the test",
               )
        }
    }

    fn create_mock_executor_for_success(
        version: &str,
        dir: &str,
        gem: &str,
    ) -> MockCommandExecutor {
        let mock = MockCommandExecutor::new();
        mock.expect(
            "info",
            &["--version", gem],
            &[],
            &format!("{}/Gemfile", dir),
            Ok(Output {
                status: Some(0),
                stdout: version.as_bytes().to_vec(),
                stderr: Vec::new(),
            }),
        );
        mock
    }

    #[test]
    fn test_installed_gem_version_success() {
        let mock_executor = create_mock_executor_for_success("8.0.0", "test_dir", "rails");
        let bundler = Bundler::new("test_dir".into(), vec![], Box::new(mock_executor));
        let version = bundler
            .installed_gem_version("rails")
            .expect("Expected successful version");
        assert_eq!(version, "8.0.0", "Installed gem version should match");
    }

    #[test]
    fn test_installed_gem_version_command_error() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "unknown_gem";
        let error_output = "Could not find gem 'unknown_gem'.";

        mock_executor.expect(
            "info",
            &["--version", gem_name],
            &[],
            "test_dir/Gemfile",
            Ok(Output {
                status: Some(1),
                stdout: Vec::new(),
                stderr: error_output.as_bytes().to_vec(),
            }),
        );

        let bundler = Bundler::new("test_dir".into(), vec![], Box::new(mock_executor));
        let result = bundler.installed_gem_version(gem_name);

        assert!(
            result.is_err(),
            "Expected error for failed gem version check"
        );
        let err_msg = result.unwrap_err();
        assert!(
            err_msg.contains("'bundle' command failed (status: 1)"),
            "Error message should contain status"
        );
        assert!(
            err_msg.contains(error_output),
            "Error message should contain stderr output"
        );
    }

    #[test]
    fn test_installed_gem_version_execution_failure_from_executor() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "critical_gem";
        let specific_error_msg = "Mocked execution failure";

        mock_executor.expect(
            "info",
            &["--version", gem_name],
            &[],
            "test_dir/Gemfile",
            Err(specific_error_msg.to_string()),
        );

        let bundler = Bundler::new("test_dir".into(), vec![], Box::new(mock_executor));
        let result = bundler.installed_gem_version(gem_name);

        assert!(result.is_err(), "Expected error from executor failure");
        assert_eq!(
            result.unwrap_err(),
            specific_error_msg,
            "Error message should match executor error"
        );
    }
}
