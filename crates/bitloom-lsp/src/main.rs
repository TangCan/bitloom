//! Bitloom language-server binary (FR99 / Story 44.2 MVP).
//!
//! Stdio LSP with minimal `initialize` / `initialized` / `shutdown`.
//! Keystroke full-elaborate diagnostics belong to Story 44.3 — not here.
//! Public brand: **Bitloom**. Unrelated to `samitbasu/rhdl`.

use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::ls_types::*;
use tower_lsp_server::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct BitloomLsp {
    client: Client,
}

impl LanguageServer for BitloomLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                // MVP: declare textDocument sync so clients see a real capability set.
                // Full-elaborate publishDiagnostics / symbols → Story 44.3.
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
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
                "Bitloom LSP MVP initialized (FR99 Story 44.2; elaborate diagnostics deferred to 44.3)",
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| BitloomLsp { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
