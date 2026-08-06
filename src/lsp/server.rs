use crate::{
    ParserOptions,
    lsp::{completion, dictionary::BasicDictionary, document::DocumentManager},
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

pub struct Backend {
    client: Client,
    documents: Arc<RwLock<DocumentManager>>,
}
impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(DocumentManager::default())),
        }
    }
}
#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions::default()),
                ..Default::default()
            },
            server_info: None,
        })
    }
    async fn initialized(&self, _: InitializedParams) {}
    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }
    async fn did_open(&self, p: DidOpenTextDocumentParams) {
        let d = p.text_document;
        self.documents
            .write()
            .await
            .open(d.uri.clone(), d.text, ParserOptions { recovery: true });
        self.publish(d.uri).await;
    }
    async fn did_change(&self, p: DidChangeTextDocumentParams) {
        if let Some(c) = p.content_changes.into_iter().last() {
            let uri = p.text_document.uri;
            self.documents
                .write()
                .await
                .update(&uri, c.text, ParserOptions { recovery: true });
            self.publish(uri).await;
        }
    }
    async fn did_close(&self, p: DidCloseTextDocumentParams) {
        self.documents.write().await.close(&p.text_document.uri);
        self.client
            .publish_diagnostics(p.text_document.uri, Vec::new(), None)
            .await;
    }
    async fn hover(&self, p: HoverParams) -> tower_lsp::jsonrpc::Result<Option<Hover>> {
        let q = p.text_document_position_params;
        let guard = self.documents.read().await;
        Ok(guard
            .get(&q.text_document.uri)
            .and_then(|d| d.hover(d.offset(q.position))))
    }
    async fn completion(
        &self,
        p: CompletionParams,
    ) -> tower_lsp::jsonrpc::Result<Option<CompletionResponse>> {
        let q = p.text_document_position;
        let guard = self.documents.read().await;
        let Some(d) = guard.get(&q.text_document.uri) else {
            return Ok(None);
        };
        let offset = d.offset(q.position);
        let prefix = d.source()[..offset as usize]
            .split_whitespace()
            .last()
            .unwrap_or("");
        let dictionary = BasicDictionary;
        let expected = d.completion_rules(offset);
        let out = completion::candidates_with_prefix(prefix, &expected, &dictionary)
            .into_iter()
            .map(|entry| CompletionItem {
                label: entry.word,
                detail: Some(entry.description),
                ..Default::default()
            })
            .collect();
        Ok(Some(CompletionResponse::Array(out)))
    }
}
impl Backend {
    async fn publish(&self, uri: Url) {
        let guard = self.documents.read().await;
        let diagnostics = guard
            .get(&uri)
            .map(|d| {
                d.diagnostics()
                    .into_iter()
                    .map(|x| Diagnostic {
                        range: Range {
                            start: d.position(x.range.start().into()),
                            end: d.position(x.range.end().into()),
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: x.message,
                        code: Some(NumberOrString::String(x.code.into())),
                        source: Some("lojban".into()),
                        ..Default::default()
                    })
                    .collect()
            })
            .unwrap_or_default();
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }
}
pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (service, socket) = LspService::new(Backend::new);
    Server::new(tokio::io::stdin(), tokio::io::stdout(), socket)
        .serve(service)
        .await;
    Ok(())
}
