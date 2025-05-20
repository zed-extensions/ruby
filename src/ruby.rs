mod bundler;
mod command_executor;
mod gemset;
mod language_servers;

use language_servers::{LanguageServer, Rubocop, RubyLsp, Solargraph, Sorbet, Steep};
use zed_extension_api::{self as zed};

use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};
use std::path::Path;

use zed::lsp::{Completion, Symbol};
use zed::settings::LspSettings;
use zed::{serde_json, CodeLabel, LanguageServerId};
use zed::{DebugAdapterBinary, DebugRequest, DebugTaskDefinition};
use zed_extension_api::{
    self as zed, resolve_tcp_template, Command, Result, StartDebuggingRequestArguments,
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
        if worktree.which(launch.program.as_ref()).is_some() {
            arguments.push("--command".to_string())
        }
        arguments.push(launch.program);
        arguments.extend(launch.args);
        let request = match config.request {
            DebugRequest::Launch(_) => StartDebuggingRequestArgumentsRequest::Launch,
            DebugRequest::Attach(_) => StartDebuggingRequestArgumentsRequest::Attach,
        };
        Ok(DebugAdapterBinary {
            command: rdbg_path.to_string(),
            arguments,
            connection: Some(connection),
            cwd: launch.cwd,
            envs: launch.envs.into_iter().collect(),
            request_args: StartDebuggingRequestArguments {
                configuration: serde_json::Value::Object(Default::default()).to_string(),
                request,
            },
        })
    }
}

zed_extension_api::register_extension!(RubyExtension);
