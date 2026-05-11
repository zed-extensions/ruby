/// Returns a copy of `shell_env` with `PWD` normalized to `pwd`.
///
/// Zed's Extensions API does not guarantee where `Command` will be invoked. It could be
/// the extension root directory or the user's home directory. Normalizing `PWD` to the
/// worktree root helps Ruby version managers like asdf/mise find and activate the correct
/// Ruby version.
pub(crate) fn shell_env_with_pwd(
    shell_env: Vec<(String, String)>,
    pwd: impl Into<String>,
) -> Vec<(String, String)> {
    shell_env
        .into_iter()
        .filter(|(key, _)| key != "PWD")
        .chain(std::iter::once(("PWD".to_string(), pwd.into())))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::shell_env_with_pwd;

    #[test]
    fn test_shell_env_with_pwd_overrides_existing_pwd() {
        let env = shell_env_with_pwd(
            vec![
                ("PWD".to_string(), "/wrong/path".to_string()),
                ("PATH".to_string(), "/usr/bin".to_string()),
            ],
            "/path/to/project",
        );

        assert_eq!(
            env,
            vec![
                ("PATH".to_string(), "/usr/bin".to_string()),
                ("PWD".to_string(), "/path/to/project".to_string()),
            ]
        );
    }
}
