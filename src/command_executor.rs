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

    fn execute_in_dir(
        &self,
        cmd: &str,
        args: &[&str],
        envs: &[(&str, &str)],
        working_dir: &str,
    ) -> zed::Result<zed::process::Output> {
        let _ = working_dir;
        self.execute(cmd, args, envs)
    }
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

    fn execute_in_dir(
        &self,
        cmd: &str,
        args: &[&str],
        envs: &[(&str, &str)],
        working_dir: &str,
    ) -> zed::Result<zed::process::Output> {
        let script = sh_command_in_dir(working_dir, cmd, args);

        eprintln!("Executing in dir via sh: {script}");

        zed::Command::new("sh")
            .args(["-c", script.as_str()])
            .envs(envs.iter().copied())
            .output()
    }
}

fn sh_command_in_dir(working_dir: &str, cmd: &str, args: &[&str]) -> String {
    format!(
        "cd -- {} && exec {}{}",
        sh_quote(working_dir),
        sh_quote(cmd),
        args.iter()
            .map(|arg| format!(" {}", sh_quote(arg)))
            .collect::<String>()
    )
}

fn sh_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\"'\"'"))
}
