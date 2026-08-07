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
    UnknownWord,
    PauseWarning,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticCategory { Syntax, Pause, UnknownWord, Style }

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
    pub category: DiagnosticCategory,
    pub code: &'static str,
    pub message: String,
    pub range: rowan::TextRange,
    /// Reserved for future pause/style code actions.
    pub replacement: Option<String>,
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
                category: DiagnosticCategory::Syntax,
                code,
                message: format!("{expected}{found}"),
                range: crate::cst::rowan_range(error.range),
                replacement: None,
            }
        })
        .collect()
}

/// Pause rules deliberately live outside the parser.  The parser only needs
/// the lossless token stream; this pass can therefore evolve independently
/// and provide fixes without making completion/hover depend on punctuation.
pub fn pause_diagnostics(parse: &Parse) -> Vec<Diagnostic> {
    use crate::syntax::SyntaxKind;
    let tokens: Vec<_> = parse.syntax().descendants_with_tokens().filter_map(|element| {
        element.into_token().filter(|token| token.kind() != SyntaxKind::Whitespace && token.kind() != SyntaxKind::Newline)
    }).collect();
    let mut result = Vec::new();
    for (index, token) in tokens.iter().enumerate() {
        if token.text() == "i" {
            let has_pause = index > 0 && tokens[index - 1].kind() == SyntaxKind::Pause;
            if !has_pause {
                result.push(Diagnostic { kind: DiagnosticKind::PauseWarning, category: DiagnosticCategory::Pause,
                    code: "LOJ200", message: "Missing pause before sentence connective.".into(),
                    range: token.text_range(), replacement: Some(".i".into()) });
            }
        }
        if token.text() == "la" {
            if let Some(next) = tokens.get(index + 1) {
                if next.kind() == SyntaxKind::Word && (index + 1 == 0 || tokens[index + 1 - 1].kind() != SyntaxKind::Pause) {
                    result.push(Diagnostic { kind: DiagnosticKind::PauseWarning, category: DiagnosticCategory::Pause,
                        code: "LOJ201", message: "Missing pause before cmevla.".into(),
                        range: next.text_range(), replacement: Some(format!(".{}", next.text())) });
                }
            }
        }
    }
    result
}

#[derive(Clone, Debug)]
pub struct CompletionContext<'a> {
    pub ast: &'a Text,
    pub semantic: &'a SemanticModel,
    pub current: Option<Node>,
    pub parent_node: Option<Node>,
    pub expected: Vec<ExpectedRule>,
    pub prefix: String,
    pub mode: CompletionMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompletionMode { Automatic, Invoked }

pub fn completion_context<'a>(
    ast: &'a Text,
    model: &'a SemanticModel,
    offset: u32,
) -> CompletionContext<'a> {
    completion_context_with(ast, model, offset, "", CompletionMode::Automatic)
}

pub fn completion_context_with<'a>(
    ast: &'a Text,
    model: &'a SemanticModel,
    offset: u32,
    prefix: impl Into<String>,
    mode: CompletionMode,
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
        prefix: prefix.into(),
        mode,
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

    #[test]
    fn pause_is_an_editor_warning_not_a_parse_error() {
        let missing_connective_pause = parse("i mi klama", ParserOptions::default());
        assert!(missing_connective_pause.errors.is_empty());
        assert!(missing_connective_pause.diagnostics().iter().any(|d| {
            d.kind == DiagnosticKind::PauseWarning && d.message.contains("sentence connective")
        }));

        let missing_name_pause = parse("la alis.", ParserOptions::default());
        assert!(missing_name_pause.semantic_model().nodes().any(|n| n.node.text().to_string().contains("alis")));
        assert!(missing_name_pause.diagnostics().iter().any(|d| {
            d.kind == DiagnosticKind::PauseWarning && d.message.contains("cmevla")
        }));
    }
}
