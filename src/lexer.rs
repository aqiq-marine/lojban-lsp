/// A byte range in the original document.
///
/// Keeping byte offsets (rather than character indices) makes the range
/// directly usable with Rust string slices and with rowan's text offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    pub start: usize,
    pub end: usize,
}

impl TextRange {
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Word,
    Number,
    Operator,
    Pause,
    Whitespace,
    Newline,
    Invalid,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LexToken<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub range: TextRange,
}

/// Lossless lexical scan used by the future CST parser and by the LSP.
/// Trivia is deliberately retained; the old `tokenize` API below is kept as
/// a compatibility adapter for the first parser migration step.
pub fn lex_lossless(input: &str) -> Vec<LexToken<'_>> {
    let mut result = Vec::new();
    let mut offset = 0;
    while offset < input.len() {
        let start = offset;
        let ch = input[offset..]
            .chars()
            .next()
            .expect("valid UTF-8 boundary");
        if ch == '.' {
            offset += ch.len_utf8();
            result.push(LexToken {
                kind: TokenKind::Pause,
                text: &input[start..offset],
                range: TextRange::new(start, offset),
            });
        } else if ch == ',' {
            // Commas are pronunciation aids, not grammar tokens. Preserve
            // them losslessly as trivia so the parser never has to handle
            // them as syntax.
            offset += ch.len_utf8();
            result.push(LexToken {
                kind: TokenKind::Whitespace,
                text: &input[start..offset],
                range: TextRange::new(start, offset),
            });
        } else if ch == '\n' {
            offset += 1;
            result.push(LexToken {
                kind: TokenKind::Newline,
                text: &input[start..offset],
                range: TextRange::new(start, offset),
            });
        } else if ch == '\r' {
            offset += 1;
            if offset < input.len() && input[offset..].chars().next() == Some('\n') {
                offset += 1;
            }
            result.push(LexToken {
                kind: TokenKind::Newline,
                text: &input[start..offset],
                range: TextRange::new(start, offset),
            });
        } else if ch.is_whitespace() {
            offset += ch.len_utf8();
            while offset < input.len() {
                let next = input[offset..].chars().next().unwrap();
                if next == '\n' || next == '\r' || !next.is_whitespace() {
                    break;
                }
                offset += next.len_utf8();
            }
            result.push(LexToken {
                kind: TokenKind::Whitespace,
                text: &input[start..offset],
                range: TextRange::new(start, offset),
            });
        } else if ch.is_alphabetic() || ch == '\'' {
            offset += ch.len_utf8();
            while offset < input.len() {
                let next = input[offset..].chars().next().unwrap();
                if !next.is_alphabetic() && next != '\'' {
                    break;
                }
                offset += next.len_utf8();
            }
            let text = &input[start..offset];
            result.push(LexToken {
                kind: TokenKind::Word,
                text,
                range: TextRange::new(start, offset),
            });
        } else if ch.is_ascii_digit() {
            offset += ch.len_utf8();
            while offset < input.len()
                && input[offset..]
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_digit() || c == '.')
            {
                offset += input[offset..].chars().next().unwrap().len_utf8();
            }
            result.push(LexToken {
                kind: TokenKind::Number,
                text: &input[start..offset],
                range: TextRange::new(start, offset),
            });
        } else if matches!(ch, '+' | '-' | '*' | '/' | '^' | '(' | ')' | '=') {
            offset += ch.len_utf8();
            result.push(LexToken {
                kind: TokenKind::Operator,
                text: &input[start..offset],
                range: TextRange::new(start, offset),
            });
        } else {
            offset += ch.len_utf8();
            result.push(LexToken {
                kind: TokenKind::Invalid,
                text: &input[start..offset],
                range: TextRange::new(start, offset),
            });
        }
    }
    result.push(LexToken {
        kind: TokenKind::Eof,
        text: "",
        range: TextRange::new(input.len(), input.len()),
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lossless_lexer_handles_crlf_sentence() {
        let tokens = lex_lossless(".i mi klama\r\n.i mi pu klama le zarci");
        let newline = tokens
            .iter()
            .find(|t| t.kind == TokenKind::Newline)
            .unwrap();
        assert_eq!(newline.text, "\r\n");
    }

    #[test]
    fn lossless_lexer_handles_crlf() {
        let tokens = lex_lossless("mi\r\nklama");
        assert_eq!(tokens[1].kind, TokenKind::Newline);
        assert_eq!(tokens[1].text, "\r\n");
        assert_eq!(tokens[1].range, TextRange::new(2, 4));
        assert_eq!(tokens[2].text, "klama");
    }

    #[test]
    fn lossless_lexer_handles_cr() {
        let tokens = lex_lossless("mi\rklama");
        assert_eq!(tokens[1].kind, TokenKind::Newline);
        assert_eq!(tokens[1].text, "\r");
        assert_eq!(tokens[1].range, TextRange::new(2, 3));
        assert_eq!(tokens[2].text, "klama");
    }

    #[test]
    fn lossless_lexer_keeps_trivia_and_ranges() {
        let tokens = lex_lossless("mi\n klama.");
        assert_eq!(tokens[0].kind, TokenKind::Word);
        assert_eq!(tokens[0].text, "mi");
        assert_eq!(tokens[0].range, TextRange::new(0, 2));
        assert_eq!(tokens[1].kind, TokenKind::Newline);
        assert_eq!(tokens[2].kind, TokenKind::Whitespace);
        assert_eq!(tokens[3].text, "klama");
        assert_eq!(tokens[4].kind, TokenKind::Pause);
        assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn lossless_lexer_reports_invalid_input_without_dropping_it() {
        let tokens = lex_lossless("mi @ do");
        let invalid = tokens
            .iter()
            .find(|token| token.kind == TokenKind::Invalid)
            .unwrap();
        assert_eq!(invalid.text, "@");
        assert_eq!(invalid.range, TextRange::new(3, 4));
    }

    #[test]
    fn commas_are_preserved_as_trivia() {
        let tokens = lex_lossless("k,l,a,m,a");
        assert!(
            !tokens
                .iter()
                .any(|token| token.text == "," && token.kind == TokenKind::Invalid)
        );
        assert!(
            tokens
                .iter()
                .filter(|token| token.text == ",")
                .all(|token| token.kind == TokenKind::Whitespace)
        );
    }
}
