use regex::Regex;
use zed_extension_api::{Command, Result};

/// A simple wrapper around the `gem` command.
pub struct Gemset {
    pub gem_home: String,
}

impl Gemset {
    pub fn new(gem_home: String) -> Self {
        Self { gem_home }
    }

    /// Returns the full path to a gem binary executable.
    ///
    /// # Arguments
    /// * `bin_name` - The name of the binary executable
    ///
    /// # Returns
    /// The full path to the binary as a `String`, or an error if the path
    /// cannot be represented as a valid UTF-8 string
    pub fn gem_bin_path(&self, bin_name: impl Into<String>) -> Result<String> {
        let bin_name = bin_name.into();
        let path = std::path::Path::new(&self.gem_home)
            .join("bin")
            .join(&bin_name);

        path.to_str()
            .map(ToString::to_string)
            .ok_or_else(|| format!("Failed to convert path for '{}'", bin_name))
    }

    /// Returns the environment variables required for gem operations.
    ///
    /// This function returns the necessary environment variables for Ruby gems:
    /// - GEM_PATH: Path where gems are installed
    /// - GEM_HOME: Directory where gems will be installed
    ///
    /// # Returns
    /// A vector of environment variable key-value pairs.
    pub fn gem_env(&self) -> Vec<(String, String)> {
        vec![
            ("GEM_PATH".to_string(), self.gem_home.clone()),
            ("GEM_HOME".to_string(), self.gem_home.clone()),
        ]
    }

    pub fn install_gem(&self, name: String) -> Result<()> {
        let args = vec![
            "install",
            "--no-user-install", // Do not install gems in user's home directory
            "--no-format-executable", // Do not make installed executable names match Ruby
            "--no-document",     // Do not generate documentation
            // "--env-shebang",     // Use /usr/bin/env as a shebang
            &name,
        ];

        self.execute_gem_command(args)
            .map_err(|e| format!("Failed to install gem '{}': {}", name, e))?;

        Ok(())
    }

    pub fn update_gem(&self, name: String) -> Result<()> {
        let args = vec!["update", &name];

        self.execute_gem_command(args)
            .map_err(|e| format!("Failed to update gem '{}': {}", name, e))?;

        Ok(())
    }

    pub fn installed_gem_version(&self, name: String) -> Result<Option<String>> {
        // Example output from `gem list`:
        /*
            *** LOCAL GEMS ***
            abbrev (0.1.2)
            prism (default: 1.2.0)
            test-unit (3.6.7)
        */
        let re = Regex::new(r"^(\S+) \((\S+)\)$")
            .map_err(|e| format!("Failed to compile regex: {}", e))?;

        let args = vec!["list", "--exact", &name];
        let output = self
            .execute_gem_command(args)
            .map_err(|e| format!("Failed to get version for gem '{}': {}", name, e))?;

        for line in output.lines() {
            let captures = match re.captures(line) {
                Some(c) => c,
                None => continue,
            };

            let gem_package = captures.get(1).map(|m| m.as_str());
            let version = captures.get(2).map(|m| m.as_str());

            if gem_package == Some(&name) {
                return Ok(version.map(|v| v.to_owned()));
            }
        }

        Ok(None)
    }

    pub fn is_outdated_gem(&self, name: String) -> Result<bool> {
        let args = vec!["outdated"];
        let output = self
            .execute_gem_command(args)
            .map_err(|e| format!("Failed to check if gem is outdated: {}", e))?;

        Ok(output
            .lines()
            .any(|line| line.split_whitespace().next().is_some_and(|n| n == name)))
    }

    fn execute_gem_command(&self, args: Vec<&str>) -> Result<String> {
        Command::new("gem")
            .envs(self.gem_env())
            .args(args)
            .arg("--norc")
            .output()
            .map_err(|e| format!("Failed to execute gem command: {}", e))
            .and_then(|output| match output.status {
                Some(0) => Ok(String::from_utf8_lossy(&output.stdout).to_string()),
                Some(status) => {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(format!(
                        "Gem command failed (status: {})\nError: {}",
                        status, stderr
                    ))
                }
                None => {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    Err(format!("Failed to execute gem command: {}", stderr))
                }
            })
    }
}
