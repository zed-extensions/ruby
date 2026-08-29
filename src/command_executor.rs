use zed_extension_api::{self as zed};

pub trait CommandExecutor {
    /// Executes a command with the given arguments and environment variables.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The name or path of the command to execute (e.g., "gem", "bundle").
    /// * `args` - A vector of string arguments to pass to the command.
    /// * `envs` - A vector of key-value pairs representing environment variables
    ///   to set for the command's execution context.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `Output` of the command if successful. The `Output`
    /// typically includes stdout, stderr, and the exit status. Returns an error
    /// if the command execution fails at a lower level (e.g., command not found,
    /// or if the `zed_extension_api::Command` itself returns an error).
    fn execute(
        &self,
        cmd: &str,
        args: &[&str],
        envs: &[(&str, &str)],
    ) -> zed::Result<zed::process::Output>;
}

/// An implementation of `CommandExecutor` that executes commands
/// using the `zed_extension_api::Command`.
#[derive(Clone)]
pub struct RealCommandExecutor;

impl CommandExecutor for RealCommandExecutor {
    fn execute(
        &self,
        cmd: &str,
        args: &[&str],
        envs: &[(&str, &str)],
    ) -> zed::Result<zed::process::Output> {
        zed::Command::new(cmd)
            .args(args.iter().copied())
            .envs(envs.iter().copied())
            .output()
    }
}

#[cfg(test)]
pub struct MockCommandExecutor {
    config: std::cell::RefCell<MockExecutorConfig>,
}

#[cfg(test)]
struct MockExecutorConfig {
    output_to_return: Option<zed::Result<zed::process::Output>>,
    expected_command_name: Option<String>,
    expected_args: Option<Vec<String>>,
    expected_envs: Option<Vec<(String, String)>>,
}

#[cfg(test)]
impl MockCommandExecutor {
    pub fn new() -> Self {
        Self {
            config: std::cell::RefCell::new(MockExecutorConfig {
                output_to_return: None,
                expected_command_name: None,
                expected_args: None,
                expected_envs: None,
            }),
        }
    }

    pub fn expect(
        &self,
        command_name: &str,
        args: &[&str],
        envs: &[(&str, &str)],
        output: zed::Result<zed::process::Output>,
    ) {
        let mut config = self.config.borrow_mut();
        config.expected_command_name = Some(command_name.to_string());
        config.expected_args = Some(args.iter().map(ToString::to_string).collect());
        config.expected_envs = Some(
            envs.iter()
                .map(|&(key, value)| (key.to_string(), value.to_string()))
                .collect(),
        );
        config.output_to_return = Some(output);
    }
}

#[cfg(test)]
impl CommandExecutor for MockCommandExecutor {
    fn execute(
        &self,
        command_name: &str,
        args: &[&str],
        envs: &[(&str, &str)],
    ) -> zed::Result<zed::process::Output> {
        let mut config = self.config.borrow_mut();

        if let Some(expected) = &config.expected_command_name {
            assert_eq!(command_name, expected, "Mock: command name mismatch");
        }
        if let Some(expected) = &config.expected_args {
            assert_eq!(
                args.iter().map(ToString::to_string).collect::<Vec<_>>(),
                *expected,
                "Mock: args mismatch"
            );
        }
        if let Some(expected) = &config.expected_envs {
            assert_eq!(
                envs.iter()
                    .map(|&(key, value)| (key.to_string(), value.to_string()))
                    .collect::<Vec<_>>(),
                *expected,
                "Mock: env mismatch"
            );
        }

        config
            .output_to_return
            .take()
            .expect("MockCommandExecutor: output was not set or was already consumed")
    }
}
