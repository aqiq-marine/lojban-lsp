use crate::{
    ParserOptions,
    lsp::{completion::DictionaryCompletionProvider, dictionary::BasicDictionary, document::{DocumentChange, DocumentManager}},
    features::CompletionMode,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

pub struct Backend {
    client: Client,
    documents: Arc<RwLock<DocumentManager>>,
    completion_provider: DictionaryCompletionProvider<BasicDictionary>,
}
impl Backend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: Arc::new(RwLock::new(DocumentManager::default())),
            completion_provider: DictionaryCompletionProvider::new(BasicDictionary),
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
                semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
                    SemanticTokensOptions {
                        legend: SemanticTokensLegend {
                            token_types: get_semantic_token_types(),
                            token_modifiers: Vec::new(),
                        },
                        range: Some(true),
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                        work_done_progress_options: Default::default(),
                    }
                )),
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
        let uri = p.text_document.uri;
        {
            let mut guard = self.documents.write().await;
            // FULL sync: each content_change carries the complete new text.
            // Apply all changes in order; the final state is what matters.
            for change in p.content_changes {
                guard.update(&uri, DocumentChange::Full(change.text), ParserOptions { recovery: true });
            }
        }
        self.publish(uri).await;
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
        let mode = if matches!(
            p.context.as_ref().map(|c| c.trigger_kind),
            None | Some(CompletionTriggerKind::INVOKED)
        ) {
            CompletionMode::Invoked
        } else {
            CompletionMode::Automatic
        };
        let q = p.text_document_position;
        let guard = self.documents.read().await;
        let Some(d) = guard.get(&q.text_document.uri) else {
            return Ok(None);
        };
        let offset = d.offset(q.position);
        let source = d.source();
        let prefix = if offset as usize <= source.len() {
            source[..offset as usize]
                .split_whitespace()
                .last()
                .unwrap_or("")
        } else {
            ""
        };
        let out = d.completions(offset, prefix.to_owned(), mode, &self.completion_provider);
        Ok(Some(CompletionResponse::Array(out)))
    }

    async fn semantic_tokens_full(&self, p: SemanticTokensParams) -> tower_lsp::jsonrpc::Result<Option<SemanticTokensResult>> {
        let guard = self.documents.read().await;
        let Some(document) = guard.get(&p.text_document.uri) else { return Ok(None); };
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens { result_id: None, data: encode_semantic_tokens(document, document.lexical_tokens()) })))
    }

    async fn semantic_tokens_range(&self, p: SemanticTokensRangeParams) -> tower_lsp::jsonrpc::Result<Option<SemanticTokensRangeResult>> {
        let guard = self.documents.read().await;
        let Some(document) = guard.get(&p.text_document.uri) else { return Ok(None); };
        let range = p.range;
        let tokens = document.lexical_tokens().into_iter().filter(|token| {
            let start = document.position(token.range.start().into());
            let end = document.position(token.range.end().into());
            start < range.end && end > range.start
        });
        Ok(Some(SemanticTokensRangeResult::Tokens(SemanticTokens {
            result_id: None,
            data: encode_semantic_tokens(document, tokens.collect()),
        })))
    }
}

fn encode_semantic_tokens(
    document: &crate::lsp::document::Document,
    tokens: Vec<crate::semantic::SemanticTokenInfo>,
) -> Vec<SemanticToken> {
    let mut previous_line = 0u32;
    let mut previous_start = 0u32;
    tokens.into_iter().filter_map(|token| {
        let start: u32 = token.range.start().into();
        let end: u32 = token.range.end().into();
        let position = document.position(start);
        let end_position = document.position(end);
        
        if position.line != end_position.line {
            return None; 
        }

        let token_type = match (token.lexical_kind, token.selmaho) {
            (crate::semantic::LexicalKind::Brivla, _) => 0,
            (crate::semantic::LexicalKind::Cmene, _) => 1,
            (crate::semantic::LexicalKind::Invalid, _) => 2,
            (crate::semantic::LexicalKind::Cmavo, Some(s)) => {
                4 + s.index() as u32
            }
            (crate::semantic::LexicalKind::Cmavo, None) => 3,
        };
        let delta_line = position.line - previous_line;
        let delta_start = if delta_line == 0 { position.character - previous_start } else { position.character };
        previous_line = position.line;
        previous_start = position.character;
        Some(SemanticToken { 
            delta_line, 
            delta_start, 
            length: end_position.character - position.character, 
            token_type, 
            token_modifiers_bitset: 0 
        })
    }).collect()
}
impl Backend {
    async fn publish(&self, uri: Url) {
        let diagnostics = {
            let guard = self.documents.read().await;
            guard
                .get(&uri)
                .map(|d| {
                    d.diagnostics()
                        .into_iter()
                        .map(|x| Diagnostic {
                            range: Range {
                                start: d.position(x.range.start().into()),
                                end: d.position(x.range.end().into()),
                            },
                            severity: Some(
                                if matches!(x.category, crate::features::DiagnosticCategory::Pause | crate::features::DiagnosticCategory::UnknownWord) {
                                    DiagnosticSeverity::WARNING
                                } else {
                                    DiagnosticSeverity::ERROR
                                },
                            ),
                            message: x.message,
                            code: Some(NumberOrString::String(x.code.into())),
                            source: Some("lojban".into()),
                            ..Default::default()
                        })
                        .collect()
                })
                .unwrap_or_default()
        };

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

pub fn get_semantic_token_types() -> Vec<SemanticTokenType> {
    let mut types = vec![
        SemanticTokenType::new("brivla"),
        SemanticTokenType::new("cmene"),
        SemanticTokenType::new("invalid"),
        SemanticTokenType::new("unknown_cmavo"),
    ];
    types.extend(crate::lsp::dictionary::Selmaho::all().iter().map(|s| SemanticTokenType::new(s.to_str())));
    types
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_semantic_token_types_count() {
        let types = get_semantic_token_types();
        assert_eq!(types.len(), 4 + 122);
    }
}
