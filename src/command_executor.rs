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
        _cwd: &str,
    ) -> zed::Result<zed::process::Output> {
        self.execute(cmd, args, envs)
    }
}

/// An implementation of `CommandExecutor` that executes commands
/// using the `zed_extension_api::Command`.
#[derive(Clone)]
pub struct RealCommandExecutor;

const MACOS_SHELL_COMMAND: &str = "/bin/sh";

const MACOS_SHELL_SCRIPT: &str = "exec \"$0\" \"$@\"";

const MACOS_SHELL_SCRIPT_IN_DIR: &str =
    "cd \"$0\" && command=\"$1\" && shift && exec \"$command\" \"$@\"";

#[derive(Debug, PartialEq, Eq)]
struct CommandInvocation {
    command: String,
    args: Vec<String>,
}

fn command_invocation(
    is_macos: bool,
    cwd: Option<&str>,
    cmd: &str,
    args: &[&str],
) -> CommandInvocation {
    if !is_macos {
        return CommandInvocation {
            command: cmd.to_string(),
            args: args.iter().map(|arg| arg.to_string()).collect(),
        };
    }

    let mut wrapped_args = if let Some(cwd) = cwd {
        vec![
            "-c".to_string(),
            MACOS_SHELL_SCRIPT_IN_DIR.to_string(),
            cwd.to_string(),
            cmd.to_string(),
        ]
    } else {
        vec![
            "-c".to_string(),
            MACOS_SHELL_SCRIPT.to_string(),
            cmd.to_string(),
        ]
    };
    wrapped_args.extend(args.iter().map(|arg| arg.to_string()));

    CommandInvocation {
        command: MACOS_SHELL_COMMAND.to_string(),
        args: wrapped_args,
    }
}

impl CommandExecutor for RealCommandExecutor {
    fn execute(
        &self,
        cmd: &str,
        args: &[&str],
        envs: &[(&str, &str)],
    ) -> zed::Result<zed::process::Output> {
        let invocation = command_invocation(
            matches!(zed::current_platform().0, zed::Os::Mac),
            None,
            cmd,
            args,
        );

        zed::Command::new(invocation.command)
            .args(invocation.args.iter().map(String::as_str))
            .envs(envs.iter().copied())
            .output()
    }

    fn execute_in_dir(
        &self,
        cmd: &str,
        args: &[&str],
        envs: &[(&str, &str)],
        cwd: &str,
    ) -> zed::Result<zed::process::Output> {
        let invocation = command_invocation(
            matches!(zed::current_platform().0, zed::Os::Mac),
            Some(cwd),
            cmd,
            args,
        );

        zed::Command::new(invocation.command)
            .args(invocation.args.iter().map(String::as_str))
            .envs(envs.iter().copied())
            .output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_macos_commands_through_shell() {
        assert_eq!(
            command_invocation(true, None, "ruby", &["--version"]),
            CommandInvocation {
                command: "/bin/sh".to_string(),
                args: vec![
                    "-c".to_string(),
                    "exec \"$0\" \"$@\"".to_string(),
                    "ruby".to_string(),
                    "--version".to_string(),
                ],
            }
        );
    }

    #[test]
    fn preserves_macos_args_positionally() {
        assert_eq!(
            command_invocation(true, None, "bundle", &["info", "--version", "ruby-lsp"]),
            CommandInvocation {
                command: "/bin/sh".to_string(),
                args: vec![
                    "-c".to_string(),
                    "exec \"$0\" \"$@\"".to_string(),
                    "bundle".to_string(),
                    "info".to_string(),
                    "--version".to_string(),
                    "ruby-lsp".to_string(),
                ],
            }
        );
    }

    #[test]
    fn keeps_non_macos_commands_direct() {
        assert_eq!(
            command_invocation(false, None, "gem", &["outdated", "--norc"]),
            CommandInvocation {
                command: "gem".to_string(),
                args: vec!["outdated".to_string(), "--norc".to_string()],
            }
        );
    }

    #[test]
    fn wraps_macos_commands_through_shell_in_dir() {
        assert_eq!(
            command_invocation(true, Some("/project"), "ruby", &["--version"]),
            CommandInvocation {
                command: "/bin/sh".to_string(),
                args: vec![
                    "-c".to_string(),
                    "cd \"$0\" && command=\"$1\" && shift && exec \"$command\" \"$@\"".to_string(),
                    "/project".to_string(),
                    "ruby".to_string(),
                    "--version".to_string(),
                ],
            }
        );
    }
}
