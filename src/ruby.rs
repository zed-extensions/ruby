mod bundler;
mod command_executor;
mod gemset;
mod language_servers;

use std::{collections::HashMap, path::PathBuf};

use bundler::Bundler;
use command_executor::RealCommandExecutor;
use gemset::Gemset;
use language_servers::{Herb, LanguageServer, Rubocop, RubyLsp, Solargraph, Sorbet, Steep};
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
        let shell_env = worktree.shell_env();
        let env_vars: Vec<(&str, &str)> = shell_env
            .iter()
            .map(|(key, value)| (key.as_str(), value.as_str()))
            .collect();

        let mut rdbg_path = PathBuf::from(&adapter_name).to_string_lossy().into_owned();
        let mut use_bundler = false;

        if worktree.which(&rdbg_path).is_none() {
            let bundler = Bundler::new(
                PathBuf::from(worktree.root_path()),
                Box::new(RealCommandExecutor),
            );
            match bundler.installed_gem_version("debug", &env_vars) {
                Ok(_version) => {
                    rdbg_path = worktree
                        .which("bundle")
                        .ok_or_else(|| "Unable to find 'bundle' command".to_string())?;
                    use_bundler = true;
                }
                Err(_e) => {
                    let gem_home = std::env::current_dir()
                        .map_err(|e| format!("Failed to get extension directory: {e}"))?;
                    let gemset =
                        Gemset::new(gem_home, Some(&env_vars), Box::new(RealCommandExecutor));

                    match gemset.install_gem("debug") {
                        Ok(_) => rdbg_path = gemset.gem_bin_path("rdbg")?,
                        Err(e) => return Err(format!("Failed to install debug gem: {e}")),
                    }
                }
            }
        }

        let tcp_connection = config.tcp_connection.unwrap_or(TcpArgumentsTemplate {
            port: None,
            host: None,
            timeout: None,
        });
        let mut connection = resolve_tcp_template(tcp_connection)?;
        let mut configuration: serde_json::Value = serde_json::from_str(&config.config)
            .map_err(|e| format!("`config` is not a valid JSON: {e}"))?;
        if let Some(configuration) = configuration.as_object_mut() {
            configuration
                .entry("cwd")
                .or_insert_with(|| worktree.root_path().into());
        }

        let ruby_config: RubyDebugConfig = serde_json::from_value(configuration.clone())
            .map_err(|e| format!("`config` is not a valid rdbg config: {e}"))?;
        let mut arguments = vec![];

        if let Some(host) = ruby_config.env.get("RUBY_DEBUG_HOST") {
            connection.host = host
                .parse::<std::net::Ipv4Addr>()
                .map(|ip_addr| ip_addr.into())
                .map_err(|_| format!("Invalid host '{host}' specified via RUBY_DEBUG_HOST"))?;
        } else {
            arguments.push(format!("--host={}", connection.host));
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

        if use_bundler {
            arguments.splice(0..0, vec!["exec".to_string(), "rdbg".to_string()]);
        }

        Ok(DebugAdapterBinary {
            command: Some(rdbg_path.to_string()),
            arguments,
            connection: Some(connection),
            cwd: ruby_config.cwd.or_else(|| Some(worktree.root_path())),
            envs: ruby_config.env.into_iter().collect(),
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

                let config = serde_json::to_value(config)
                    .map_err(|e| e.to_string())?
                    .to_string();

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

                let config = serde_json::to_value(config)
                    .map_err(|e| e.to_string())?
                    .to_string();

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
                if let Some(obj) = value.as_object_mut() {
                    obj.entry("request").or_insert("launch".into());
                    value.to_string()
                } else {
                    return None;
                }
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
