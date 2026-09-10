//! Bitloom language-server binary (FR99 / FR113).
//!
//! Stdio LSP with initialize / capabilities, plus didSave full-design elaborate
//! diagnostics and document symbols. FR113 prefers Cargo metadata design roots when
//! the saved path sits in a discovered package. Public brand: **Bitloom**.

use std::path::PathBuf;
use std::sync::Mutex;

use bitloom_lsp::{AnalyzeResult, analyze_on_did_save, analyze_on_did_save_at};
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

fn mvp_root_uri() -> Uri {
    "file:///bitloom/fr99-mvp-root"
        .parse()
        .expect("mvp root uri")
}

fn path_from_uri(uri: &Uri) -> Option<PathBuf> {
    let s = uri.as_str();
    let path = s.strip_prefix("file://")?;
    let path = path.strip_prefix("//localhost").unwrap_or(path);
    Some(PathBuf::from(path))
}

#[derive(Debug)]
struct BitloomLsp {
    client: Client,
    last: Mutex<Option<AnalyzeResult>>,
}

impl BitloomLsp {
    async fn publish_from(&self, uri: Uri, result: &AnalyzeResult) {
        let diags: Vec<Diagnostic> = result
            .diagnostics
            .iter()
            .map(|d| Diagnostic {
                range: Range::default(),
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String(d.code.clone())),
                code_description: None,
                source: Some("bitloom-lsp".into()),
                message: d.message.clone(),
                related_information: None,
                tags: None,
                data: None,
            })
            .collect();
        self.client.publish_diagnostics(uri, diags, None).await;
    }
}

impl LanguageServer for BitloomLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Options(
                    TextDocumentSyncOptions {
                        open_close: Some(true),
                        change: Some(TextDocumentSyncKind::FULL),
                        will_save: None,
                        will_save_wait_until: None,
                        save: Some(TextDocumentSyncSaveOptions::Supported(true)),
                    },
                )),
                document_symbol_provider: Some(OneOf::Left(true)),
                definition_provider: Some(OneOf::Left(true)),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "bitloom-lsp".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
            offset_encoding: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(
                MessageType::INFO,
                "Bitloom LSP ready (FR99 + FR113: didSave → discover metadata roots or DesignFixture)",
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let hint = path_from_uri(&params.text_document.uri);
        let result = analyze_on_did_save_at(hint.as_deref());
        if let Ok(mut g) = self.last.lock() {
            *g = Some(result.clone());
        }
        self.publish_from(params.text_document.uri, &result).await;
    }

    async fn document_symbol(
        &self,
        _params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        let cached = self
            .last
            .lock()
            .ok()
            .and_then(|g| g.clone())
            .unwrap_or_else(analyze_on_did_save);

        let symbols: Vec<SymbolInformation> = cached
            .symbols
            .iter()
            .map(|s| {
                #[allow(deprecated)]
                SymbolInformation {
                    name: s.name.clone(),
                    kind: if s.kind == "module" {
                        SymbolKind::MODULE
                    } else {
                        SymbolKind::FIELD
                    },
                    tags: None,
                    deprecated: None,
                    location: Location {
                        uri: mvp_root_uri(),
                        range: Range::default(),
                    },
                    container_name: None,
                }
            })
            .collect();
        Ok(Some(DocumentSymbolResponse::Flat(symbols)))
    }

    async fn goto_definition(
        &self,
        _params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let cached = self
            .last
            .lock()
            .ok()
            .and_then(|g| g.clone())
            .unwrap_or_else(analyze_on_did_save);
        if cached.symbols.is_empty() {
            return Ok(None);
        }
        Ok(Some(GotoDefinitionResponse::Scalar(Location {
            uri: mvp_root_uri(),
            range: Range::default(),
        })))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| BitloomLsp {
        client,
        last: Mutex::new(None),
    });
    Server::new(stdin, stdout, socket).serve(service).await;
}
