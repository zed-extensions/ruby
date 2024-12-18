use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

#[derive(Clone, Debug)]
pub struct RubocopBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
}

pub struct Rubocop {}

impl Rubocop {
    pub const LANGUAGE_SERVER_ID: &str = "rubocop";
    pub const EXECUTABLE_NAME: &str = "rubocop";

    pub fn new() -> Self {
        Self {}
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary.path,
            args: binary.args.unwrap_or(vec!["--lsp".into()]),
            env: Default::default(),
        })
    }

    fn language_server_binary(
        &self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<RubocopBinary> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let binary_settings = lsp_settings.binary;
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(RubocopBinary {
                path,
                args: binary_args,
            });
        }

        let use_bundler = lsp_settings
            .settings
            .as_ref()
            .and_then(|settings| settings["use_bundler"].as_bool())
            .unwrap_or(true);

        if use_bundler {
            worktree
                .which("bundle")
                .map(|path| RubocopBinary {
                    path,
                    args: Some(vec![
                        "exec".into(),
                        Rubocop::EXECUTABLE_NAME.into(),
                        "--lsp".into(),
                    ]),
                })
                .ok_or_else(|| "Unable to find the 'bundle' command.".into())
        } else {
            worktree
                .which(Rubocop::EXECUTABLE_NAME)
                .map(|path| RubocopBinary {
                    path,
                    args: Some(vec!["--lsp".into()]),
                })
                .ok_or_else(|| {
                    format!("Unable to find the '{}' command.", Rubocop::EXECUTABLE_NAME)
                })
        }
    }
}
