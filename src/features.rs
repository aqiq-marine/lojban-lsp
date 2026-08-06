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
}
#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub range: rowan::TextRange,
}

pub fn diagnostics(parse: &Parse) -> Vec<Diagnostic> {
    let root = parse.syntax();
    root.descendants()
        .filter(|n| n.kind() == SyntaxKind::Error)
        .map(|n| Diagnostic {
            kind: DiagnosticKind::SyntaxError,
            message: "syntax error".into(),
            range: n.text_range(),
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
}
