use std::path::Path;
use zed_extension_api::{Command, Result};

/// A simple wrapper around the `bundle` command.
pub struct Bundler {
    pub working_dir: String,
    envs: Vec<(String, String)>,
}

impl Bundler {
    pub fn new(working_dir: String, envs: Vec<(String, String)>) -> Self {
        Bundler { working_dir, envs }
    }

    pub fn installed_gem_version(&self, name: &str) -> Result<String> {
        let args = vec!["--version".into(), name.into()];

        self.execute_bundle_command("info".into(), args)
    }

    fn execute_bundle_command(&self, cmd: String, args: Vec<String>) -> Result<String> {
        let bundle_gemfile_path = Path::new(&self.working_dir).join("Gemfile");
        let bundle_gemfile = bundle_gemfile_path
            .to_str()
            .ok_or("Invalid path to Gemfile")?;

        Command::new("bundle")
            .arg(cmd)
            .args(args)
            .envs(self.envs.clone())
            .env("BUNDLE_GEMFILE", bundle_gemfile)
            .output()
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
