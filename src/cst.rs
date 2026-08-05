use rowan::{GreenNode, GreenNodeBuilder, SyntaxNode, TextRange, TextSize};

use crate::lexer::{LexToken, TextRange as SourceRange, TokenKind, lex_lossless};
use crate::syntax::{LojbanLanguage, SyntaxKind};

/// Parser output event. Grammar rules can emit these events without knowing
/// anything about rowan or about the eventual typed AST view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Start(SyntaxKind),
    Token { kind: SyntaxKind, token_index: u32 },
    Finish,
    Error { token_index: Option<u32> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub range: crate::lexer::TextRange,
}

#[derive(Debug, Clone)]
pub struct Parse {
    pub green: GreenNode,
    pub errors: Vec<ParseError>,
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode<LojbanLanguage> {
        SyntaxNode::new_root(self.green.clone())
    }
}

/// Convert parser events into a rowan tree. Error events are represented as
/// zero-width Error nodes, so diagnostics do not require a second parse.
pub fn build_green(
    events: impl IntoIterator<Item = Event>,
    tokens: &[LexToken<'_>],
) -> (GreenNode, Vec<ParseError>) {
    let mut builder = GreenNodeBuilder::new();
    let mut errors = Vec::new();
    for event in events {
        match event {
            Event::Start(kind) => builder.start_node(kind.into()),
            Event::Token { kind, token_index } => {
                if let Some(token) = tokens.get(token_index as usize) {
                    builder.token(kind.into(), token.text);
                }
            }
            Event::Finish => builder.finish_node(),
            Event::Error { token_index } => {
                let (message, range, text) = token_index
                    .and_then(|index| {
                        tokens
                            .get(index as usize)
                            .map(|token| ("syntax error".to_string(), token.range, token.text))
                    })
                    .unwrap_or_else(|| ("syntax error".to_string(), SourceRange::new(0, 0), ""));
                errors.push(ParseError { message, range });
                builder.start_node(SyntaxKind::Error.into());
                if !text.is_empty() {
                    builder.token(SyntaxKind::Invalid.into(), text);
                }
                builder.finish_node();
            }
        }
    }
    (builder.finish(), errors)
}

/// Lossless CST entry point. This is intentionally lexical at first; syntax
/// rules will replace the flat Text body one Camxes rule family at a time.
/// It already provides the invariant required by the LSP: every input byte is
/// represented by a token or an Error node.
pub fn parse_lossless(source: &str) -> Parse {
    let tokens = lex_lossless(source);
    let mut events = vec![Event::Start(SyntaxKind::Text)];
    for (index, token) in tokens.iter().enumerate() {
        events.push(event_for_token(index as u32, *token));
    }
    events.push(Event::Finish);
    let (green, errors) = build_green(events, &tokens);
    Parse { green, errors }
}

fn event_for_token(index: u32, token: LexToken<'_>) -> Event {
    let kind = match token.kind {
        TokenKind::Word => SyntaxKind::Word,
        TokenKind::Number => SyntaxKind::Number,
        TokenKind::Operator => SyntaxKind::Operator,
        TokenKind::Pause => SyntaxKind::Pause,
        TokenKind::Whitespace => SyntaxKind::Whitespace,
        TokenKind::Newline => SyntaxKind::Newline,
        TokenKind::Invalid => SyntaxKind::Invalid,
        TokenKind::Eof => SyntaxKind::Eof,
    };
    if token.kind == TokenKind::Invalid {
        Event::Error {
            token_index: Some(index),
        }
    } else {
        Event::Token {
            kind,
            token_index: index,
        }
    }
}

/// Convert a byte range into rowan's range type. Kept here so parser modules
/// do not each implement offset conversion.
pub fn rowan_range(range: crate::lexer::TextRange) -> TextRange {
    TextRange::new(
        TextSize::from(range.start as u32),
        TextSize::from(range.end as u32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cst_preserves_source_and_reports_errors() {
        let source = "mi @ do";
        let parse = parse_lossless(source);
        assert_eq!(parse.errors.len(), 1);
        assert_eq!(parse.syntax().text().to_string(), source);
        assert!(
            parse
                .syntax()
                .descendants()
                .any(|node| node.kind() == SyntaxKind::Error)
        );
    }

    #[test]
    fn event_builder_creates_nested_nodes() {
        let tokens = lex_lossless("mi");
        let (green, errors) = build_green(
            [
                Event::Start(SyntaxKind::Sentence),
                Event::Token {
                    kind: SyntaxKind::Word,
                    token_index: 0,
                },
                Event::Finish,
            ],
            &tokens,
        );
        assert!(errors.is_empty());
        let root = SyntaxNode::<LojbanLanguage>::new_root(green);
        assert_eq!(root.kind(), SyntaxKind::Sentence);
        assert_eq!(root.text().to_string(), "mi");
    }
}
