use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

use crate::gemset::Gemset;

#[derive(Clone, Debug)]
pub struct LanguageServerBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<(String, String)>>,
}

pub trait LanguageServer {
    const SERVER_ID: &str;
    const EXECUTABLE_NAME: &str;
    const GEM_NAME: &str;

    fn default_use_bundler() -> bool {
        true // Default for most LSPs except Ruby LSP
    }

    fn get_executable_args() -> Vec<String> {
        Vec::new()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;


        Ok(zed::Command {
            command: binary.path,
            args: binary.args.unwrap_or(Self::get_executable_args()),
            env: binary.env.unwrap_or_default(),
        })
    }

    fn language_server_binary(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<LanguageServerBinary> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        if let Some(binary_settings) = lsp_settings.binary {
            if let Some(path) = binary_settings.path {
                return Ok(LanguageServerBinary {
                    path,
                    args: binary_settings.arguments,
                    env: Default::default(),
                });
            }
        }

        let use_bundler = lsp_settings
            .settings
            .as_ref()
            .and_then(|settings| settings["use_bundler"].as_bool())
            .unwrap_or(Self::default_use_bundler());

        if use_bundler {
            worktree
                .which("bundle")
                .map(|path| LanguageServerBinary {
                    path,
                    args: Some(
                        [
                            vec!["exec".to_string(), Self::EXECUTABLE_NAME.to_string()],
                            Self::get_executable_args(),
                        ]
                        .concat(),
                    ),
                    env: Default::default(),
                })
                .ok_or_else(|| "Unable to find the 'bundle' command.".into())
        } else {
            let gem_home = std::env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {}", e))?
                .to_string_lossy()
                .to_string();

            let gemset = Gemset::new(gem_home.clone());

            match gemset
                .installed_gem_version(Self::GEM_NAME.into())
                .map_err(|e| e.to_string())?
            {
                Some(_version) => {
                    if gemset
                        .is_outdated_gem(Self::GEM_NAME.into())
                        .map_err(|e| e.to_string())?
                    {
                        gemset
                            .update_gem(Self::GEM_NAME.into())
                            .map_err(|e| e.to_string())?;
                    }
                    Ok(LanguageServerBinary {
                        path: format!("{}/bin/{}", gem_home, Self::EXECUTABLE_NAME),
                        args: Some(Self::get_executable_args()),
                        env: Some(vec![
                            ("GEM_PATH".to_string(), format!("{}:$GEM_PATH", gem_home)),
                            ("GEM_HOME".to_string(), gem_home),
                        ]),
                    })
                }
                None => {
                    gemset
                        .install_gem(Self::GEM_NAME.into())
                        .map_err(|e| e.to_string())?;

                    Ok(LanguageServerBinary {
                        path: format!("{}/bin/{}", gem_home, Self::EXECUTABLE_NAME),
                        args: Some(Self::get_executable_args()),
                        env: Some(vec![
                            ("GEM_PATH".to_string(), format!("{}:$GEM_PATH", gem_home)),
                            ("GEM_HOME".to_string(), gem_home),
                        ]),
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestServer {}
    impl LanguageServer for TestServer {
        const SERVER_ID: &'static str = "test-server";
        const EXECUTABLE_NAME: &'static str = "test-exe";
        const GEM_NAME: &'static str = "test";

        fn get_executable_args() -> Vec<String> {
            vec!["--test-arg".into()]
        }
    }

    #[test]
    fn test_default_use_bundler() {
        assert!(TestServer::default_use_bundler());
    }

    #[test]
    fn test_default_executable_args() {
        assert!(TestServer::get_executable_args() == vec!["--test-arg"]);
    }
}
