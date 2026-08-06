use crate::{
    ParserOptions,
    ast::{self, Node},
    cst::Parse,
    features::Diagnostic,
    lsp::{dictionary::BasicDictionary, hover as dictionary_hover},
};
use std::collections::HashMap;
use tower_lsp::lsp_types::Url;

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
        let entry = dictionary_hover::hover(self, offset, &BasicDictionary)?;
        Some(tower_lsp::lsp_types::Hover {
            contents: tower_lsp::lsp_types::HoverContents::Scalar(
                tower_lsp::lsp_types::MarkedString::String(format!(
                    "**{}**\\n\\n{}",
                    entry.word, entry.description
                )),
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
