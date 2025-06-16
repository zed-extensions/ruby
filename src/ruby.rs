mod bundler;
mod command_executor;
mod gemset;
mod language_servers;

use std::{collections::HashMap, path::Path};

use language_servers::{LanguageServer, Rubocop, RubyLsp, Solargraph, Sorbet, Steep};
use zed_extension_api::{self as zed};

use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::path::Path;

use zed::lsp::{Completion, Symbol};
use zed::settings::LspSettings;
use zed::{serde_json, CodeLabel, LanguageServerId};
use zed::{DebugAdapterBinary, DebugRequest, DebugTaskDefinition};
use zed_extension_api::{
    self as zed, resolve_tcp_template, Command, DebugAdapterBinary, DebugConfig, DebugRequest,
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
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let initialization_options =
            zed::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                .ok()
                .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
                .unwrap_or_default();

        Ok(Some(zed::serde_json::json!(initialization_options)))
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
        let mut rdbg_path = Path::new(&adapter_name)
            .join("rdbg")
            .to_string_lossy()
            .into_owned();

        if worktree.which(&rdbg_path).is_none() {
            match worktree.which("rdbg".as_ref()) {
                Some(path) => rdbg_path = path.into(),
                None => {
                    let output = Command::new("gem")
                        .arg("install")
                        .arg("--no-document")
                        .arg("--bindir")
                        .arg(&adapter_name)
                        .arg("debug")
                        .output()?;
                    if !output.status.is_none_or(|status| status != 0) {
                        return Err(format!(
                            "Failed to install rdbg:\n{}",
                            String::from_utf8_lossy(&output.stderr).to_string()
                        ));
                    }
                }
            }
        }

        let tcp_connection =
            config
                .tcp_connection
                .clone()
                .unwrap_or_else(|| TcpArgumentsTemplate {
                    port: None,
                    host: None,
                    timeout: None,
                });
        let connection = resolve_tcp_template(tcp_connection)?;
        let DebugRequest::Launch(launch) = config.request.clone() else {
            return Err("rdbg does not yet support attaching".to_string());
        };

        let mut arguments = vec![
            "--open".to_string(),
            format!("--port={}", connection.port),
            format!("--host={}", connection.host),
        ];
        if let Some(script) = &ruby_config.script {
            arguments.push(script.clone());
        } else if let Some(command) = &ruby_config.command {
            arguments.push("--command".to_string());
            arguments.push(command.clone());
        } else if let Some(command_or_script) = &ruby_config.script_or_command {
            if worktree.which(&command_or_script).is_some() {
                arguments.push("--command".to_string());
            }
            arguments.push(command_or_script.clone());
        } else {
            return Err("Ruby debug config must have 'script' or 'command' args".into());
        }
        if let Some(configuration) = configuration.as_object_mut() {
            configuration
                .entry("cwd")
                .or_insert_with(|| worktree.root_path().into());
        }
        arguments.extend(ruby_config.args);

        Ok(DebugAdapterBinary {
            command: Some(rdbg_path.to_string()),
            arguments,
            connection: Some(connection),
            cwd: ruby_config.cwd,
            envs: ruby_config.env.into_iter().collect(),
            request_args: StartDebuggingRequestArguments {
                configuration: configuration.to_string(),
                request: StartDebuggingRequestArgumentsRequest::Launch,
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
            DebugRequest::Attach(_) => Err("Attach requests are unsupported".into()),
        }
    }
}

zed_extension_api::register_extension!(RubyExtension);
