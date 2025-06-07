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
        args: Vec<String>,
        envs: Vec<(String, String)>,
    ) -> zed::Result<zed::process::Output>;
}

/// An implementation of `CommandExecutor` that executes commands
/// using the `zed_extension_api::Command`.
pub struct RealCommandExecutor;

impl CommandExecutor for RealCommandExecutor {
    fn execute(
        &self,
        cmd: &str,
        args: Vec<String>,
        envs: Vec<(String, String)>,
    ) -> zed::Result<zed::process::Output> {
        zed::Command::new(cmd).args(args).envs(envs).output()
    }
}
