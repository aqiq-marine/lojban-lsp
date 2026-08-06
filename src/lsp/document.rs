use crate::{
    ParserOptions,
    ast::{self, Node},
    cst::Parse,
    features::Diagnostic,
};
use std::collections::HashMap;
use tower_lsp::lsp_types::Url;

pub struct Document {
    source: String,
    parse: Parse,
}
impl Document {
    pub fn source(&self) -> &str { &self.source }
    pub fn parse(source: impl Into<String>, options: ParserOptions) -> Self {
        let source = source.into();
        let parse = crate::parse(&source, options);
        Self { source, parse }
    }
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.parse.diagnostics()
    }
    pub fn syntax(&self) -> Node {
        self.parse.syntax()
    }
    pub fn semantic_model(&self) -> crate::semantic::SemanticModel {
        self.parse.semantic_model()
    }
    pub fn completion_rules(&self, offset: u32) -> Vec<crate::features::ExpectedRule> {
        let ast = self.parse.ast();
        let semantic = self.parse.semantic_model();
        crate::features::completion_context(&ast, &semantic, offset).expected
    }
    pub fn hover(&self, offset: u32) -> Option<tower_lsp::lsp_types::Hover> {
        let token = ast::token_at(&self.syntax(), offset)?;
        let node = self.parse.semantic_model().node_at(offset)?;
        Some(tower_lsp::lsp_types::Hover {
            contents: tower_lsp::lsp_types::HoverContents::Scalar(
                tower_lsp::lsp_types::MarkedString::String(format!(
                    "Token: `{}`\\nSyntaxKind: `{:?}`\\nSemanticNodeKind: `{:?}`",
                    token.text(),
                    token.kind(),
                    node.kind
                )),
            ),
            range: Some(tower_lsp::lsp_types::Range {
                start: tower_lsp::lsp_types::Position {
                    line: 0,
                    character: token.text_range().start().into(),
                },
                end: tower_lsp::lsp_types::Position {
                    line: 0,
                    character: token.text_range().end().into(),
                },
            }),
        })
    }
    pub fn apply_change(&mut self, source: impl Into<String>, options: ParserOptions) {
        *self = Self::parse(source, options);
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
    pub fn update(&mut self, uri: &Url, source: impl Into<String>, options: ParserOptions) -> bool {
        self.documents
            .get_mut(uri)
            .map(|d| {
                d.apply_change(source, options);
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
