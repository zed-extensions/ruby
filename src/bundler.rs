use crate::command_executor::CommandExecutor;
use std::path::Path;

/// A simple wrapper around the `bundle` command.
pub struct Bundler {
    pub working_dir: String,
    command_executor: Box<dyn CommandExecutor>,
}

impl Bundler {
    /// Creates a new `Bundler` instance.
    ///
    /// # Arguments
    /// * `working_dir` - The working directory where `bundle` commands should be executed.
    /// * `command_executor` - An executor for `bundle` commands.
    pub fn new(working_dir: String, command_executor: Box<dyn CommandExecutor>) -> Self {
        Bundler {
            working_dir,
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
    pub fn installed_gem_version(
        &self,
        name: &str,
        envs: &[(&str, &str)],
    ) -> Result<String, String> {
        let args = &["--version", name];

        self.execute_bundle_command("info", args, envs)
    }

    fn execute_bundle_command(
        &self,
        cmd: &str,
        args: &[&str],
        envs: &[(&str, &str)],
    ) -> Result<String, String> {
        let bundle_gemfile_path = Path::new(&self.working_dir).join("Gemfile");
        let bundle_gemfile = bundle_gemfile_path
            .to_str()
            .ok_or_else(|| "Invalid path to Gemfile".to_string())?;

        let full_args: Vec<&str> = std::iter::once(cmd).chain(args.iter().copied()).collect();
        let command_envs: Vec<(&str, &str)> = envs
            .iter()
            .cloned()
            .chain(std::iter::once(("BUNDLE_GEMFILE", bundle_gemfile)))
            .collect();

        self.command_executor
            .execute("bundle", &full_args, &command_envs)
            .and_then(|output| match output.status {
                Some(0) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
                Some(status) => {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(format!(
                        "'bundle' command failed (status: {status})\nError: {stderr}",
                    ))
                }
                None => {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(format!("Failed to execute 'bundle' command: {stderr}"))
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
        output_to_return: Option<Result<Output, String>>,
        expected_command_name: Option<String>,
        expected_args: Option<Vec<String>>,
        expected_envs: Option<Vec<(String, String)>>,
    }

    struct MockCommandExecutor {
        config: RefCell<MockExecutorConfig>,
    }

    impl MockCommandExecutor {
        fn new() -> Self {
            MockCommandExecutor {
                config: RefCell::new(MockExecutorConfig {
                    output_to_return: None,
                    expected_command_name: None,
                    expected_args: None,
                    expected_envs: None,
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
        let gemfile_path = format!("{dir}/Gemfile");
        mock.expect(
            "bundle",
            &["info", "--version", gem],
            &[("BUNDLE_GEMFILE", &gemfile_path)],
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
        let bundler = Bundler::new("test_dir".into(), Box::new(mock_executor));
        let version = bundler
            .installed_gem_version("rails", &[])
            .expect("Expected successful version");
        assert_eq!(version, "8.0.0", "Installed gem version should match");
    }

    #[test]
    fn test_installed_gem_version_command_error() {
        let mock_executor = MockCommandExecutor::new();
        let gem_name = "unknown_gem";
        let error_output = "Could not find gem 'unknown_gem'.";
        let gemfile_path = "test_dir/Gemfile";

        mock_executor.expect(
            "bundle",
            &["info", "--version", gem_name],
            &[("BUNDLE_GEMFILE", gemfile_path)],
            Ok(Output {
                status: Some(1),
                stdout: Vec::new(),
                stderr: error_output.as_bytes().to_vec(),
            }),
        );

        let bundler = Bundler::new("test_dir".into(), Box::new(mock_executor));
        let result = bundler.installed_gem_version(gem_name, &[]);

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
        let gemfile_path = "test_dir/Gemfile";

        mock_executor.expect(
            "bundle",
            &["info", "--version", gem_name],
            &[("BUNDLE_GEMFILE", gemfile_path)],
            Err(specific_error_msg.to_string()),
        );

        let bundler = Bundler::new("test_dir".into(), Box::new(mock_executor));
        let result = bundler.installed_gem_version(gem_name, &[]);

        assert!(result.is_err(), "Expected error from executor failure");
        assert_eq!(
            result.unwrap_err(),
            specific_error_msg,
            "Error message should match executor error"
        );
    }
}
