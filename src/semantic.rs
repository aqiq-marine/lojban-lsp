//! Lightweight semantic projection used by editor features.

use crate::{
    ast::{self, Node},
    syntax::SyntaxKind,
    lexer::{LexToken, TokenKind},
    morphology::{self, AnalysisMode},
    lsp::dictionary::{Dictionary, Selmaho},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LexicalKind { Brivla, Cmene, Cmavo, Invalid }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticKind { InvalidWord, DictionaryEntryNotFound }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticTokenInfo { 
    pub range: rowan::TextRange, 
    pub lexical_kind: LexicalKind, 
    pub selmaho: Option<Selmaho>, 
    pub diagnostic_kind: Option<DiagnosticKind> 
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeKind {
    Sentence,
    Sumti,
    Selbri,
    Brivla,
    Cmavo,
    PlaceTag,
    Indicator,
    Vocative,
    Error,
    Other(SyntaxKind),
}

#[derive(Clone, Debug)]
pub struct SemanticNode {
    pub kind: NodeKind,
    pub node: Node,
}

#[derive(Clone, Debug)]
pub struct SemanticModel {
    root: Node,
}

impl SemanticModel {
    pub fn new(root: Node) -> Self {
        Self { root }
    }
    pub fn root(&self) -> &Node {
        &self.root
    }
    pub fn nodes(&self) -> impl Iterator<Item = SemanticNode> + '_ {
        self.root.descendants().map(|node| SemanticNode {
            kind: classify(node.kind()),
            node,
        })
    }
    pub fn node_at(&self, offset: u32) -> Option<SemanticNode> {
        let token = ast::token_at(&self.root, offset)?;
        token.parent().map(|node| SemanticNode {
            kind: classify(node.kind()),
            node,
        })
    }

    pub fn lexical_tokens(&self, source: &str, dictionary: &dyn Dictionary) -> Vec<SemanticTokenInfo> {
        crate::lexer::lex_lossless(source)
            .iter()
            .filter(|token| {
                token.kind != crate::lexer::TokenKind::Whitespace
                    && token.kind != crate::lexer::TokenKind::Newline
                    && token.kind != crate::lexer::TokenKind::Eof
                    && token.kind != crate::lexer::TokenKind::Pause
            })
            .map(|token| analyze_token(token, dictionary))
            .collect()
    }
}

pub fn analyze_token(
    token: &LexToken,
    dictionary: &dyn Dictionary,
) -> SemanticTokenInfo {
    let range = rowan::TextRange::new(rowan::TextSize::from(token.range.start as u32), rowan::TextSize::from(token.range.end as u32));
    if token.kind == TokenKind::Invalid { 
        return SemanticTokenInfo { range, lexical_kind: LexicalKind::Invalid, selmaho: None, diagnostic_kind: Some(DiagnosticKind::InvalidWord) }; 
    }
    
    let Some(analysis) = morphology::analyze(token, AnalysisMode::Complete) else {
        return SemanticTokenInfo { range, lexical_kind: LexicalKind::Invalid, selmaho: None, diagnostic_kind: Some(DiagnosticKind::InvalidWord) };
    };
    
    let (lexical_kind, selmaho, diagnostic_kind) = match analysis.kind {
        morphology::WordKind::Gismu | morphology::WordKind::Lujvo | morphology::WordKind::Fuivla => {
            let found = dictionary.lookup(analysis.text).is_some();
            (LexicalKind::Brivla, None, (!found).then_some(DiagnosticKind::DictionaryEntryNotFound))
        },
        morphology::WordKind::Cmevla => (LexicalKind::Cmene, None, None),
        morphology::WordKind::Cmavo => {
            let entry = dictionary.lookup(analysis.text);
            let selmaho = entry.as_ref().and_then(|e| match &e.kind { crate::lsp::dictionary::WordKind::Cmavo { selmaho } => selmaho.first().copied(), _ => None });
            (LexicalKind::Cmavo, selmaho, (!entry.is_some()).then_some(DiagnosticKind::DictionaryEntryNotFound))
        },
        morphology::WordKind::Unknown => (LexicalKind::Invalid, None, Some(DiagnosticKind::InvalidWord)),
    };
    SemanticTokenInfo { range, lexical_kind, selmaho, diagnostic_kind }
}
// ... (previous code) ...
fn classify(kind: SyntaxKind) -> NodeKind {
    match kind {
        SyntaxKind::Sentence => NodeKind::Sentence,
        SyntaxKind::Sumti => NodeKind::Sumti,
        SyntaxKind::Selbri => NodeKind::Selbri,
        SyntaxKind::Brivla => NodeKind::Brivla,
        SyntaxKind::Cmavo => NodeKind::Cmavo,
        SyntaxKind::PlaceTag => NodeKind::PlaceTag,
        SyntaxKind::Indicator => NodeKind::Indicator,
        SyntaxKind::Vocative => NodeKind::Vocative,
        SyntaxKind::Error => NodeKind::Error,
        other => NodeKind::Other(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lsp::dictionary::{Entry, WordKind as DictWordKind};
    use crate::lexer::{LexToken, TokenKind, TextRange};
    use std::collections::HashMap;

    struct MockDictionary {
        entries: HashMap<String, Entry>,
    }

    impl Dictionary for MockDictionary {
        fn lookup(&self, word: &str) -> Option<Entry> {
            self.entries.get(word).cloned()
        }
        fn lookup_by_rafsi(&self, _rafsi: &str) -> Option<Entry> {
            None
        }
        fn iter(&self) -> Vec<Entry> {
            self.entries.values().cloned().collect()
        }
    }

    #[test]
    fn test_analyze_token() {
        let mut entries = HashMap::new();
        entries.insert("klama".to_string(), Entry {
            word: "klama".to_string(),
            kind: DictWordKind::Brivla { kind: crate::lsp::dictionary::BrivlaKind::Gismu, arity: 4 },
            description: "go".to_string(),
        });
        let dict = MockDictionary { entries };

        // Registered Brivla
        let token = LexToken { text: "klama", range: TextRange::new(0, 5), kind: TokenKind::Word };
        let info = analyze_token(&token, &dict);
        assert_eq!(info.lexical_kind, LexicalKind::Brivla);
        assert_eq!(info.diagnostic_kind, None);

        // Unregistered Brivla (gismu form)
        let token = LexToken { text: "spage", range: TextRange::new(0, 5), kind: TokenKind::Word };
        let info = analyze_token(&token, &dict);
        assert_eq!(info.lexical_kind, LexicalKind::Brivla);
        assert_eq!(info.diagnostic_kind, Some(DiagnosticKind::DictionaryEntryNotFound));

        // Invalid word
        // let token = LexToken { text: "djpta", range: TextRange::new(0, 5), kind: TokenKind::Word };
        // let info = analyze_token(&token, &dict);
        // assert_eq!(info.lexical_kind, LexicalKind::Invalid);
        // assert_eq!(info.diagnostic_kind, Some(DiagnosticKind::InvalidWord));
    }
}

