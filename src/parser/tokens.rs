//! Conversion between lossless lexer tokens and CST token kinds.

use crate::lexer::TokenKind;
use crate::syntax::SyntaxKind;

pub(crate) fn syntax_kind(kind: TokenKind) -> SyntaxKind {
    match kind {
        TokenKind::Word => SyntaxKind::Word,
        TokenKind::Number => SyntaxKind::Number,
        TokenKind::Operator => SyntaxKind::Operator,
        TokenKind::Pause => SyntaxKind::Pause,
        TokenKind::Whitespace => SyntaxKind::Whitespace,
        TokenKind::Newline => SyntaxKind::Newline,
        TokenKind::Invalid => SyntaxKind::Invalid,
        TokenKind::Eof => SyntaxKind::Eof,
    }
}
