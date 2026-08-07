use crate::{
    ParserOptions,
    ast::{self, Node},
    cst::Parse,
    features::{Diagnostic, CompletionMode},
    lsp::{dictionary::BasicDictionary, hover as dictionary_hover},
};
use std::collections::HashMap;
use tower_lsp::lsp_types::Url;

/// Represents an edit sent by the editor.
///
/// Currently only `Full` (full-text replacement) is used, since the server
/// advertises `TextDocumentSyncKind::FULL`.  The `Incremental` variant is
/// reserved for a future migration to incremental re-parsing without changing
/// the call-sites.
pub enum DocumentChange {
    /// The editor sends the complete new text of the document.
    Full(String),
    /// The editor sends a partial replacement (not yet implemented).
    #[allow(dead_code)]
    Incremental {
        range: crate::lexer::TextRange,
        text: String,
    },
}

#[derive(Clone, Debug)]
pub struct LineIndex {
    line_starts: Vec<u32>,
}
impl LineIndex {
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0];
        for (offset, byte) in source.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push((offset + 1) as u32);
            }
        }
        Self { line_starts }
    }
    pub fn position(&self, offset: u32) -> tower_lsp::lsp_types::Position {
        let line = self
            .line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1);
        tower_lsp::lsp_types::Position {
            line: line as u32,
            character: offset - self.line_starts[line],
        }
    }
    pub fn offset(&self, position: tower_lsp::lsp_types::Position) -> u32 {
        self.line_starts
            .get(position.line as usize)
            .copied()
            .unwrap_or(0)
            + position.character
    }
}

pub struct Document {
    source: String,
    parse: Parse,
    line_index: LineIndex,
}
impl Document {
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn parse(source: impl Into<String>, options: ParserOptions) -> Self {
        let source = source.into();
        let parse = crate::parse(&source, options);
        let line_index = LineIndex::new(&source);
        Self {
            source,
            parse,
            line_index,
        }
    }
    pub fn position(&self, offset: u32) -> tower_lsp::lsp_types::Position {
        self.line_index.position(offset)
    }
    pub fn offset(&self, position: tower_lsp::lsp_types::Position) -> u32 {
        self.line_index.offset(position)
    }
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        let mut diagnostics = self.parse.diagnostics();
        diagnostics.extend(self.lexical_tokens().into_iter().filter_map(|token| {
            token.diagnostic_kind.map(|diagnostic_kind| Diagnostic {
                kind: match diagnostic_kind {
                    crate::semantic::DiagnosticKind::InvalidWord => crate::features::DiagnosticKind::SyntaxError,
                    crate::semantic::DiagnosticKind::DictionaryEntryNotFound => crate::features::DiagnosticKind::UnknownWord,
                },
                code: "LOJ100",
                message: match diagnostic_kind {
                    crate::semantic::DiagnosticKind::InvalidWord => "Invalid word.".to_owned(),
                    crate::semantic::DiagnosticKind::DictionaryEntryNotFound => "Dictionary entry not found.".to_owned(),
                },
                range: token.range,
                category: crate::features::DiagnosticCategory::UnknownWord,
                replacement: None,
            })
        }));
        diagnostics
    }
    pub fn syntax(&self) -> Node {
        self.parse.syntax()
    }
    pub fn semantic_model(&self) -> crate::semantic::SemanticModel {
        self.parse.semantic_model()
    }
    pub fn lexical_tokens(&self) -> Vec<crate::semantic::SemanticTokenInfo> {
        self.parse.semantic_model().lexical_tokens(&self.source, &BasicDictionary)
    }
    pub fn completion_rules(&self, offset: u32) -> Vec<crate::features::ExpectedRule> {
        let ast = self.parse.ast();
        let semantic = self.parse.semantic_model();
        crate::features::completion_context(&ast, &semantic, offset).expected
    }
    pub fn completions<P: crate::lsp::completion::CompletionProvider>(&self, offset: u32, prefix: String, mode: CompletionMode, provider: &P) -> Vec<tower_lsp::lsp_types::CompletionItem> {
        let ast = self.parse.ast();
        let semantic = self.parse.semantic_model();
        let context = crate::features::completion_context_with(&ast, &semantic, offset, prefix, mode);
        provider.complete(&context)
    }
    pub fn hover(&self, offset: u32) -> Option<tower_lsp::lsp_types::Hover> {
        let token = ast::token_at(&self.syntax(), offset)?;
        let hover_text = dictionary_hover::hover(self, offset, &BasicDictionary)?;
        Some(tower_lsp::lsp_types::Hover {
            contents: tower_lsp::lsp_types::HoverContents::Scalar(
                tower_lsp::lsp_types::MarkedString::String(hover_text),
            ),
            range: Some(tower_lsp::lsp_types::Range {
                start: tower_lsp::lsp_types::Position {
                    ..self.position(token.text_range().start().into())
                },
                end: tower_lsp::lsp_types::Position {
                    ..self.position(token.text_range().end().into())
                },
            }),
        })
    }
    /// Apply a change from the editor.  Currently only `Full` is handled;
    /// `Incremental` falls back to a full re-parse until incremental parsing
    /// is implemented.
    pub fn apply_change(&mut self, change: DocumentChange, options: ParserOptions) {
        let new_source = match change {
            DocumentChange::Full(text) => text,
            DocumentChange::Incremental { range, text } => {
                // Fallback: apply the partial edit to the existing source and
                // re-parse the whole document.  Replace this with a proper
                // incremental re-parse once the infrastructure is ready.
                let mut source = self.source.clone();
                source.replace_range(range.start..range.end, &text);
                source
            }
        };
        *self = Self::parse(new_source, options);
    }
}
#[derive(Default)]
pub struct DocumentManager {
    documents: HashMap<Url, Document>,
}
impl DocumentManager {
    pub fn open(&mut self, uri: Url, source: impl Into<String>, options: ParserOptions) {
        self.documents.insert(uri, Document::parse(source, options));
    }
    pub fn update(&mut self, uri: &Url, change: DocumentChange, options: ParserOptions) -> bool {
        self.documents
            .get_mut(uri)
            .map(|d| {
                d.apply_change(change, options);
                true
            })
            .unwrap_or(false)
    }
    pub fn close(&mut self, uri: &Url) -> Option<Document> {
        self.documents.remove(uri)
    }
    pub fn get(&self, uri: &Url) -> Option<&Document> {
        self.documents.get(uri)
    }
}
