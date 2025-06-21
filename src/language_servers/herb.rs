use std::{env, fs};

use super::{language_server::WorktreeLike, LanguageServer};
use zed_extension_api::{self as zed};

const SERVER_PATH: &str = "node_modules/@herb-tools/language-server/dist/herb-language-server";
const PACKAGE_NAME: &str = "@herb-tools/language-server";

pub struct Herb {
    did_find_server: bool,
}

impl LanguageServer for Herb {
    const SERVER_ID: &str = "herb";
    const EXECUTABLE_NAME: &str = "herb";
    const GEM_NAME: &str = "herb";

    fn get_executable_args<T: WorktreeLike>(&self, _worktree: &T) -> Vec<String> {
        vec!["--stdio".to_string()]
    }
}

impl Herb {
    pub fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let server_path = self.server_script_path(language_server_id, worktree)?;

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).is_ok_and(|stat| stat.is_file())
    }

    fn server_script_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                                    "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                                ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_extension_initial_state() {
        let ext = Herb::new();
        assert!(
            !ext.did_find_server,
            "A new extension instance should have did_find_server as false by default."
        );
    }
}
