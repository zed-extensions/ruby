mod bundler;
mod command_executor;
mod gemset;
mod language_servers;

use std::collections::HashMap;
use std::path::PathBuf;

use bundler::Bundler;
use command_executor::RealCommandExecutor;
use gemset::{versioned_gem_home, Gemset};
use language_servers::{
    FuzzyRubyServer, Herb, Kanayago, LanguageServer, Rubocop, RubyLsp, Solargraph, Sorbet, Steep,
};
use serde::{Deserialize, Serialize};
use zed_extension_api::{
    self as zed, resolve_tcp_template, DebugAdapterBinary, DebugConfig, DebugRequest,
    DebugScenario, DebugTaskDefinition, StartDebuggingRequestArguments,
    StartDebuggingRequestArgumentsRequest, TcpArgumentsTemplate, Worktree,
};

#[derive(Default)]
struct RubyExtension {
    solargraph: Option<Solargraph>,
    ruby_lsp: Option<RubyLsp>,
    rubocop: Option<Rubocop>,
    sorbet: Option<Sorbet>,
    steep: Option<Steep>,
    herb: Option<Herb>,
    kanayago: Option<Kanayago>,
    fuzzy_ruby_server: Option<FuzzyRubyServer>,
}

#[derive(Serialize, Deserialize)]
struct RubyDebugConfig {
    script_or_command: Option<String>,
    script: Option<String>,
    command: Option<String>,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: HashMap<String, String>,
    cwd: Option<String>,
}

/// Zed's default grace period for a TCP debug adapter to start listening is
/// 2000ms (`DebuggerSettings::timeout`). Booting Ruby on Windows routinely
/// takes ~1.9s, and ~2.8s when it goes through `bundle exec`, so the default
/// loses that race and the user sees "unable to connect to the debug service".
/// Widening the timeout only affects the failing path; a successful start
/// still connects as soon as the socket is up.
const WINDOWS_DAP_CONNECT_TIMEOUT_MS: u64 = 30_000;

/// Renders the host of a TCP debug connection the way `rdbg` expects it.
///
/// `TcpArguments::host` carries an IPv4 address as a big-endian `u32`, so
/// formatting it directly yields `--host=2130706433`. `rdbg` passes that to
/// name resolution, which fails with "getaddrinfo: no such host" and exits
/// before the adapter ever listens - which Zed reports as a connect timeout.
fn format_debug_host(host: u32) -> String {
    std::net::Ipv4Addr::from(host).to_string()
}

/// Applies `overrides` on top of `envs`, replacing values in place instead of
/// duplicating keys, so later (more specific) sources win.
fn merge_envs(
    mut envs: Vec<(String, String)>,
    overrides: impl IntoIterator<Item = (String, String)>,
) -> Vec<(String, String)> {
    for (key, value) in overrides {
        match envs.iter_mut().find(|(existing, _)| *existing == key) {
            Some(entry) => entry.1 = value,
            None => envs.push((key, value)),
        }
    }
    envs
}

/// Returns the extensionless Ruby script behind a Windows batch shim.
///
/// RubyGems writes both files next to each other (`bin/rdbg` and
/// `bin/rdbg.bat`) and the shim locates the script through `%~dpn0`, i.e. its
/// own path without the extension. Returns `None` for anything that is not a
/// `.bat`/`.cmd` shim.
///
/// This splits the string by hand instead of using `std::path`: the extension
/// runs as a WASI module, where `Path` follows POSIX rules even when the host
/// is Windows, so `Path::extension` would treat `C:\...\rdbg.bat` as a single
/// file name and never see the `.bat` suffix.
fn gem_script_path(shim: &str) -> Option<String> {
    let last_separator = shim.rfind(|character| character == '/' || character == '\\');
    let (directory, file_name) = match last_separator {
        Some(index) => shim.split_at(index + 1),
        None => ("", shim),
    };
    let (stem, extension) = file_name.rsplit_once('.')?;
    if stem.is_empty() {
        return None;
    }
    if !extension.eq_ignore_ascii_case("bat") && !extension.eq_ignore_ascii_case("cmd") {
        return None;
    }
    Some(format!("{directory}{stem}"))
}

/// Runs the Ruby script behind `shim` with `ruby` as interpreter, keeping the
/// arguments the shim used to receive.
fn ruby_interpreter_command(
    shim: &str,
    ruby: String,
    arguments: Vec<String>,
) -> (String, Vec<String>) {
    match gem_script_path(shim) {
        Some(script) => {
            let mut script_arguments = Vec::with_capacity(arguments.len() + 1);
            script_arguments.push(script);
            script_arguments.extend(arguments);
            (ruby, script_arguments)
        }
        // Not a batch shim: nothing to unwrap, keep the direct launch.
        None => (shim.to_string(), arguments),
    }
}

/// Decides how the adapter process is spawned.
///
/// Only Windows rewrites the launch so the Ruby script runs under an explicit
/// interpreter; every other platform gets back the exact command it resolved
/// (shebang launchers, version-manager shims, ...) with unchanged arguments.
fn resolve_launcher(
    is_windows: bool,
    command: String,
    arguments: Vec<String>,
    ruby: Option<String>,
) -> (String, Vec<String>) {
    match (is_windows, ruby) {
        (true, Some(ruby)) => ruby_interpreter_command(&command, ruby, arguments),
        _ => (command, arguments),
    }
}

impl zed::Extension for RubyExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        match language_server_id.as_ref() {
            Solargraph::SERVER_ID => {
                let solargraph = self.solargraph.get_or_insert_with(Solargraph::new);
                solargraph.language_server_command(language_server_id, worktree)
            }
            RubyLsp::SERVER_ID => {
                let ruby_lsp = self.ruby_lsp.get_or_insert_with(RubyLsp::new);
                ruby_lsp.language_server_command(language_server_id, worktree)
            }
            Rubocop::SERVER_ID => {
                let rubocop = self.rubocop.get_or_insert_with(Rubocop::new);
                rubocop.language_server_command(language_server_id, worktree)
            }
            Sorbet::SERVER_ID => {
                let sorbet = self.sorbet.get_or_insert_with(Sorbet::new);
                sorbet.language_server_command(language_server_id, worktree)
            }
            Steep::SERVER_ID => {
                let steep = self.steep.get_or_insert_with(Steep::new);
                steep.language_server_command(language_server_id, worktree)
            }
            Herb::SERVER_ID => {
                let herb = self.herb.get_or_insert_with(Herb::new);
                herb.language_server_command(language_server_id, worktree)
            }
            Kanayago::SERVER_ID => {
                let kanayago = self.kanayago.get_or_insert_with(Kanayago::new);
                kanayago.language_server_command(language_server_id, worktree)
            }
            FuzzyRubyServer::SERVER_ID => {
                let server = self
                    .fuzzy_ruby_server
                    .get_or_insert_with(FuzzyRubyServer::new);
                server.language_server_command(language_server_id, worktree)
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        match language_server_id.as_ref() {
            RubyLsp::SERVER_ID => {
                let ruby = self.ruby_lsp.get_or_insert_with(RubyLsp::new);
                ruby.language_server_initialization_options(language_server_id, worktree)
            }
            _ => Ok(Some(
                zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                    .ok()
                    .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
                    .unwrap_or_default(),
            )),
        }
    }

    fn label_for_completion(
        &self,
        language_server_id: &zed::LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        match language_server_id.as_ref() {
            Solargraph::SERVER_ID => self.solargraph.as_ref()?.label_for_completion(completion),
            RubyLsp::SERVER_ID => self.ruby_lsp.as_ref()?.label_for_completion(completion),
            _ => None,
        }
    }

    fn label_for_symbol(
        &self,
        language_server_id: &zed::LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<zed::CodeLabel> {
        match language_server_id.as_ref() {
            Solargraph::SERVER_ID => self.solargraph.as_ref()?.label_for_symbol(symbol),
            RubyLsp::SERVER_ID => self.ruby_lsp.as_ref()?.label_for_symbol(symbol),
            _ => None,
        }
    }

    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        _: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        let is_windows = matches!(zed::current_platform().0, zed::Os::Windows);

        let shell_env = worktree.shell_env();
        let env_vars: Vec<(&str, &str)> = shell_env
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();

        // On Windows the adapter process only inherits the environment we hand
        // to Zed on top of Zed's own process environment, so start from the
        // user's shell environment - the same source the language servers use.
        // That keeps `ruby` and `GEM_PATH` resolvable inside the adapter, which
        // the `.bat`/`.cmd` shims implicitly rely on.
        let mut launcher_envs: Vec<(String, String)> = Vec::new();
        if is_windows {
            launcher_envs.extend(shell_env.iter().cloned());
        }

        let (command, arguments) = {
            let bundler = Bundler::new(PathBuf::from(worktree.root_path()), RealCommandExecutor);
            if bundler.installed_gem_version("debug", &env_vars).is_ok() {
                let bundle = worktree.which("bundle").ok_or_else(|| {
                    "debug gem present, but unable to find 'bundle' command".to_string()
                })?;
                (bundle, vec!["exec".to_string(), "rdbg".to_string()])
            } else if let Some(path) = worktree.which(&adapter_name) {
                (path, Vec::new())
            } else {
                let base_dir = std::env::current_dir()
                    .map_err(|e| format!("Failed to get extension directory: {e:#}"))?;
                let gem_home = versioned_gem_home(&base_dir, &env_vars, &RealCommandExecutor)
                    .map_err(|e| format!("{:#}", e))?;
                let gemset = Gemset::new(
                    gem_home,
                    zed::current_platform().0,
                    Some(&env_vars),
                    Box::new(RealCommandExecutor),
                );
                gemset
                    .install_gem("debug")
                    .map_err(|e| format!("Failed to install debug gem: {e:#}"))?;
                let rdbg = gemset
                    .gem_bin_path("rdbg")
                    .map_err(|e| format!("{:#}", e))?;
                if is_windows {
                    // The gemset is only reachable through `GEM_PATH` (and its
                    // `bin` through `PATH`); without them the spawned adapter
                    // cannot require `debug`.
                    launcher_envs = merge_envs(launcher_envs, gemset.env().iter().cloned());
                }
                (rdbg, Vec::new())
            }
        };

        // Windows resolves gem executables to `.bat`/`.cmd` shims. Their wrapper
        // re-resolves an interpreter on its own (from `PATH` for `--install-dir`
        // gemsets, from the sibling `ruby.exe` for system installs) and adds a
        // `cmd.exe` layer. Run the extensionless script with the interpreter we
        // resolved instead. Other platforms keep their shebang launchers.
        let ruby = if is_windows {
            worktree.which("ruby")
        } else {
            None
        };
        let (command, mut arguments) = resolve_launcher(is_windows, command, arguments, ruby);

        let tcp_connection = config.tcp_connection.unwrap_or(TcpArgumentsTemplate {
            port: None,
            host: None,
            timeout: None,
        });
        let configured_timeout = tcp_connection.timeout;
        let mut connection = resolve_tcp_template(tcp_connection)?;
        if is_windows && configured_timeout.is_none() {
            // Zed's default grace period for a TCP DAP server is 2000ms
            // (`DebuggerSettings::timeout`). Starting Ruby on Windows routinely
            // takes ~1.9s and ~2.8s through `bundle exec`, so the adapter loses
            // that race and surfaces as "Connection to TCP DAP timeout", i.e.
            // "unable to connect to the debug service". The longer timeout only
            // widens the failing path; a successful start is unaffected.
            connection.timeout = Some(WINDOWS_DAP_CONNECT_TIMEOUT_MS);
        }
        let mut configuration: serde_json::Value = serde_json::from_str(&config.config)
            .map_err(|e| format!("`config` is not a valid JSON: {e:#}"))?;
        if let Some(configuration) = configuration.as_object_mut() {
            configuration
                .entry("cwd")
                .or_insert_with(|| worktree.root_path().into());
        }

        let ruby_config: RubyDebugConfig = serde_json::from_value(configuration.clone())
            .map_err(|e| format!("`config` is not a valid rdbg config: {e:#}"))?;

        if let Some(host) = ruby_config.env.get("RUBY_DEBUG_HOST") {
            connection.host = host
                .parse::<std::net::Ipv4Addr>()
                .map(|ip_addr| ip_addr.into())
                .map_err(|_| format!("Invalid host '{host}' specified via RUBY_DEBUG_HOST"))?;
        } else {
            arguments.push(format!("--host={}", format_debug_host(connection.host)));
        }

        if let Some(port) = ruby_config.env.get("RUBY_DEBUG_PORT") {
            connection.port = port.parse::<u16>().map_err(|_| {
                format!("Invalid port number '{port}' specified via RUBY_DEBUG_PORT")
            })?;
        } else {
            arguments.push(format!("--port={}", connection.port));
        }

        let request_type = self.dap_request_kind(adapter_name.clone(), configuration.clone())?;
        match request_type {
            StartDebuggingRequestArgumentsRequest::Launch => {
                if !ruby_config.env.contains_key("RUBY_DEBUG_OPEN") {
                    arguments.push("--open".to_string());
                }
                arguments.push("--stop-at-load".to_string());

                let (is_command, program) = if let Some(script) = &ruby_config.script {
                    (false, script.clone())
                } else if let Some(command) = &ruby_config.command {
                    (true, command.clone())
                } else if let Some(command_or_script) = &ruby_config.script_or_command {
                    (
                        worktree.which(command_or_script).is_some(),
                        command_or_script.clone(),
                    )
                } else {
                    return Err(
                        "Ruby debug config must have 'script', 'command', or 'script_or_command' arg"
                            .into(),
                    );
                };

                if is_command {
                    arguments.push("--command".to_string());
                }
                arguments.push(program);
                arguments.push("--".to_string());
                arguments.extend(ruby_config.args);
            }
            StartDebuggingRequestArgumentsRequest::Attach => {
                arguments.push("--attach".to_string());
            }
        };

        Ok(DebugAdapterBinary {
            command: Some(command),
            arguments,
            connection: Some(connection),
            cwd: ruby_config.cwd.or(Some(worktree.root_path())),
            envs: merge_envs(launcher_envs, ruby_config.env),
            request_args: StartDebuggingRequestArguments {
                configuration: configuration.to_string(),
                request: request_type,
            },
        })
    }

    fn dap_request_kind(
        &mut self,
        _: String,
        value: serde_json::Value,
    ) -> zed_extension_api::Result<StartDebuggingRequestArgumentsRequest, String> {
        value
            .get("request")
            .and_then(|request| {
                request.as_str().and_then(|s| match s {
                    "launch" => Some(StartDebuggingRequestArgumentsRequest::Launch),
                    "attach" => Some(StartDebuggingRequestArgumentsRequest::Attach),
                    _ => None,
                })
            })
            .ok_or_else(|| {
                "Invalid request, expected `request` to be either `launch` or `attach`".into()
            })
    }

    fn dap_config_to_scenario(
        &mut self,
        zed_scenario: DebugConfig,
    ) -> Result<DebugScenario, String> {
        match zed_scenario.request {
            DebugRequest::Launch(launch) => {
                let config = RubyDebugConfig {
                    script_or_command: Some(launch.program),
                    script: None,
                    command: None,
                    args: launch.args,
                    env: launch.envs.into_iter().collect(),
                    cwd: launch.cwd.clone(),
                };

                let mut config = serde_json::to_value(config).map_err(|e| e.to_string())?;
                config
                    .as_object_mut()
                    .ok_or_else(|| "Failed to convert launch config into JSON object".to_string())?
                    .entry("request")
                    .or_insert("launch".into());
                let config = config.to_string();

                Ok(DebugScenario {
                    adapter: zed_scenario.adapter,
                    label: zed_scenario.label,
                    config,
                    tcp_connection: None,
                    build: None,
                })
            }
            DebugRequest::Attach(_) => {
                let config = RubyDebugConfig {
                    script_or_command: None,
                    script: None,
                    command: None,
                    args: vec![],
                    env: Default::default(),
                    cwd: None,
                };

                let mut config = serde_json::to_value(config).map_err(|e| e.to_string())?;
                config
                    .as_object_mut()
                    .ok_or_else(|| "Failed to convert attach config into JSON object".to_string())?
                    .entry("request")
                    .or_insert("attach".into());
                let config = config.to_string();

                Ok(DebugScenario {
                    adapter: zed_scenario.adapter,
                    label: zed_scenario.label,
                    config,
                    tcp_connection: None,
                    build: None,
                })
            }
        }
    }

    fn dap_locator_create_scenario(
        &mut self,
        locator_name: String,
        build_task: zed_extension_api::TaskTemplate,
        resolved_label: String,
        debug_adapter_name: String,
    ) -> Option<DebugScenario> {
        if debug_adapter_name != "rdbg" || locator_name != "ruby" {
            return None;
        }

        let config = RubyDebugConfig {
            script_or_command: None,
            script: None,
            command: Some(build_task.command),
            args: build_task.args,
            env: build_task.env.into_iter().collect(),
            cwd: build_task.cwd,
        };

        let config = match serde_json::to_value(config) {
            Ok(mut value) => {
                let obj = value.as_object_mut()?;
                obj.entry("request").or_insert("launch".into());
                value.to_string()
            }
            Err(_) => return None,
        };

        Some(DebugScenario {
            adapter: debug_adapter_name,
            label: resolved_label,
            config,
            tcp_connection: None,
            build: None,
        })
    }
}

zed_extension_api::register_extension!(RubyExtension);

#[cfg(test)]
mod tests {
    use super::*;

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(ToString::to_string).collect()
    }

    #[test]
    fn test_format_debug_host_renders_dotted_quad() {
        // `TcpArguments.host` for 127.0.0.1 as a big-endian u32.
        assert_eq!(format_debug_host(2_130_706_433), "127.0.0.1");
        assert_eq!(format_debug_host(0), "0.0.0.0");
        assert_eq!(format_debug_host(u32::MAX), "255.255.255.255");
        assert_eq!(
            format_debug_host(u32::from(std::net::Ipv4Addr::new(192, 168, 1, 10))),
            "192.168.1.10"
        );
    }

    #[test]
    fn test_gem_script_path_strips_windows_shim_extensions() {
        assert_eq!(
            gem_script_path(r"C:\Ruby34-x64\bin\rdbg.bat").unwrap(),
            r"C:\Ruby34-x64\bin\rdbg"
        );
        assert_eq!(
            gem_script_path(r"C:\Ruby34-x64\bin\bundle.cmd").unwrap(),
            r"C:\Ruby34-x64\bin\bundle"
        );
        assert_eq!(
            gem_script_path("C:/Ruby User/zed/gems/abc/bin/rdbg.BAT").unwrap(),
            "C:/Ruby User/zed/gems/abc/bin/rdbg"
        );
        // Mixed separators must not confuse the split.
        assert_eq!(
            gem_script_path("C:/Ruby User/bin\\rdbg.bat").unwrap(),
            "C:/Ruby User/bin\\rdbg"
        );
        assert_eq!(gem_script_path("rdbg.bat").unwrap(), "rdbg");
    }

    #[test]
    fn test_gem_script_path_ignores_non_batch_paths() {
        // POSIX launchers and real executables are spawned as-is.
        assert_eq!(gem_script_path("/usr/local/bin/rdbg"), None);
        assert_eq!(gem_script_path(r"C:\Ruby34-x64\bin\ruby.exe"), None);
        assert_eq!(gem_script_path("rdbg"), None);
    }

    #[test]
    fn test_resolve_launcher_is_a_noop_off_windows() {
        let (command, arguments) = resolve_launcher(
            false,
            "/home/user/.rbenv/shims/ruby".to_string(),
            args(&["--host=127.0.0.1", "--port=1234"]),
            Some("/home/user/.rbenv/shims/ruby".to_string()),
        );
        assert_eq!(command, "/home/user/.rbenv/shims/ruby");
        assert_eq!(arguments, args(&["--host=127.0.0.1", "--port=1234"]));
    }

    #[test]
    fn test_resolve_launcher_is_a_noop_on_windows_without_ruby() {
        let (command, arguments) = resolve_launcher(
            true,
            r"C:\Ruby34-x64\bin\rdbg.bat".to_string(),
            args(&["--open"]),
            None,
        );
        assert_eq!(command, r"C:\Ruby34-x64\bin\rdbg.bat");
        assert_eq!(arguments, args(&["--open"]));
    }

    #[test]
    fn test_resolve_launcher_runs_script_with_explicit_ruby() {
        let (command, arguments) = resolve_launcher(
            true,
            r"C:\Ruby34-x64\bin\rdbg.bat".to_string(),
            args(&["--host=127.0.0.1", "--open"]),
            Some(r"C:\Ruby34-x64\bin\ruby.exe".to_string()),
        );
        assert_eq!(command, r"C:\Ruby34-x64\bin\ruby.exe");
        assert_eq!(
            arguments,
            args(&[r"C:\Ruby34-x64\bin\rdbg", "--host=127.0.0.1", "--open"])
        );
    }

    #[test]
    fn test_resolve_launcher_keeps_bundle_exec_arguments() {
        let (command, arguments) = resolve_launcher(
            true,
            r"C:\Ruby34-x64\bin\bundle.bat".to_string(),
            args(&["exec", "rdbg", "--open"]),
            Some(r"C:\Ruby34-x64\bin\ruby.exe".to_string()),
        );
        assert_eq!(command, r"C:\Ruby34-x64\bin\ruby.exe");
        assert_eq!(
            arguments,
            args(&[r"C:\Ruby34-x64\bin\bundle", "exec", "rdbg", "--open"])
        );
    }

    #[test]
    fn test_merge_envs_keeps_base_order_and_appends_overrides() {
        let base = vec![
            ("PATH".to_string(), "C:/Ruby/bin".to_string()),
            ("HOME".to_string(), "C:/Users/x".to_string()),
        ];
        let merged = merge_envs(
            base,
            vec![
                ("GEM_HOME".to_string(), "C:/gems".to_string()),
                ("PATH".to_string(), "C:/gems/bin".to_string()),
            ],
        );

        assert_eq!(
            merged,
            vec![
                ("PATH".to_string(), "C:/gems/bin".to_string()),
                ("HOME".to_string(), "C:/Users/x".to_string()),
                ("GEM_HOME".to_string(), "C:/gems".to_string()),
            ]
        );
    }

    #[test]
    fn test_merge_envs_with_empty_base_is_the_user_env() {
        let mut user_env = HashMap::new();
        user_env.insert("RUBY_DEBUG_PORT".to_string(), "1234".to_string());

        assert_eq!(
            merge_envs(Vec::new(), user_env),
            vec![("RUBY_DEBUG_PORT".to_string(), "1234".to_string())]
        );
    }
}
