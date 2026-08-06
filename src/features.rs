//! Editor-facing diagnostics and completion context.

use crate::{
    ast::{self, Node, Text},
    cst::Parse,
    semantic::SemanticModel,
    syntax::SyntaxKind,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    ExpectedToken,
    MissingTerminator,
    UnexpectedToken,
    SyntaxError,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExpectedRule {
    Sumti,
    Selbri,
    Bridi,
    Sentence,
    KU,
    VAU,
    KEI,
    Quote,
    Tag,
    EditingMarker,
}
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub code: &'static str,
    pub message: String,
    pub range: rowan::TextRange,
}

pub fn diagnostics(parse: &Parse) -> Vec<Diagnostic> {
    parse
        .errors
        .iter()
        .map(|error| {
            let (code, expected) = match error.kind {
                crate::cst::ErrorKind::ExpectedSelbri => ("LOJ002", "expected selbri"),
                crate::cst::ErrorKind::ExpectedSumti => ("LOJ003", "expected sumti"),
                crate::cst::ErrorKind::ExpectedBridi => ("LOJ004", "expected bridi"),
                crate::cst::ErrorKind::ExpectedCmevla => ("LOJ005", "expected cmevla"),
                crate::cst::ErrorKind::UnexpectedToken => ("LOJ001", "unexpected token"),
                crate::cst::ErrorKind::SyntaxError => ("LOJ000", "syntax error"),
            };
            let found = error
                .found
                .map(|kind| format!(", found {kind:?}"))
                .unwrap_or_default();
            Diagnostic {
                kind: match error.kind {
                    crate::cst::ErrorKind::UnexpectedToken => DiagnosticKind::UnexpectedToken,
                    crate::cst::ErrorKind::ExpectedSelbri
                    | crate::cst::ErrorKind::ExpectedSumti
                    | crate::cst::ErrorKind::ExpectedBridi
                    | crate::cst::ErrorKind::ExpectedCmevla => DiagnosticKind::ExpectedToken,
                    crate::cst::ErrorKind::SyntaxError => DiagnosticKind::SyntaxError,
                },
                code,
                message: format!("{expected}{found}"),
                range: crate::cst::rowan_range(error.range),
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
pub struct CompletionContext<'a> {
    pub ast: &'a Text,
    pub semantic: &'a SemanticModel,
    pub current: Option<Node>,
    pub parent_node: Option<Node>,
    pub expected: Vec<ExpectedRule>,
}

pub fn completion_context<'a>(
    ast: &'a Text,
    model: &'a SemanticModel,
    offset: u32,
) -> CompletionContext<'a> {
    let current_node = ast::token_at(model.root(), offset).and_then(|t| t.parent());
    let parent_node = current_node.as_ref().and_then(|n| n.parent());
    let expected = current_node
        .as_ref()
        .map(|n| match n.kind() {
            SyntaxKind::Terms => vec![ExpectedRule::Sumti, ExpectedRule::Selbri],
            SyntaxKind::Sumti => vec![ExpectedRule::Sumti],
            SyntaxKind::Selbri => vec![ExpectedRule::Selbri],
            SyntaxKind::Quoting => vec![ExpectedRule::Quote],
            SyntaxKind::Tag => vec![ExpectedRule::Tag],
            SyntaxKind::EditingMarker => vec![ExpectedRule::EditingMarker],
            _ => vec![ExpectedRule::Sentence],
        })
        .unwrap_or_else(|| vec![ExpectedRule::Sentence]);
    CompletionContext {
        ast,
        semantic: model,
        current: current_node,
        parent_node,
        expected,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ParserOptions, parse};

    #[test]
    fn diagnostics_and_completion_are_tree_based() {
        let parse = parse("mi @ do", ParserOptions::default());
        assert!(!parse.diagnostics().is_empty());
        let model = parse.semantic_model();
        let ast = parse.ast();
        let context = completion_context(&ast, &model, 1);
        assert!(context.current.is_some());
        assert!(!context.expected.is_empty());
    }

    #[test]
    fn classifies_unexpected_tokens_and_editor_nodes() {
        let invalid = parse("mi", ParserOptions::default());
        assert!(
            invalid
                .diagnostics()
                .iter()
                .any(|d| d.kind == DiagnosticKind::ExpectedToken)
        );

        let quoted = parse("mi cusku lu mi klama li'u", ParserOptions::default());
        let model = quoted.semantic_model();
        let ast = quoted.ast();
        let context = completion_context(&ast, &model, 11);
        assert!(context.expected.contains(&ExpectedRule::Quote));
    }
}
