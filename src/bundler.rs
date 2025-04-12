use std::path::Path;

use zed_extension_api::{Command, Result};

/// A simple wrapper around the `bundle` command.
pub struct Bundler {
    pub working_dir: String,
}

impl Bundler {
    pub fn new(working_dir: String) -> Self {
        Bundler { working_dir }
    }

    pub fn installed_gem_version(&self, name: &str) -> Result<String> {
        let args = vec!["info", "--version", name];

        self.execute_gem_command(args)
            .map_err(|e| format!("Failed to get version for gem '{}': {}", name, e))
    }

    fn execute_gem_command(&self, args: Vec<&str>) -> Result<String> {
        let bundle_gemfile_path = Path::new(&self.working_dir).join("Gemfile");
        let bundle_gemfile = bundle_gemfile_path
            .to_str()
            .ok_or("Invalid path to Gemfile")?;

        Command::new("bundle")
            .args(args)
            .env("BUNDLE_GEMFILE", bundle_gemfile)
            .output()
            .map_err(|e| format!("Failed to execute 'bundle' command: {}", e))
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
