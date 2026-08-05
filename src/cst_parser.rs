use crate::cst::{Event, Parse, build_green};
use crate::lexer::{LexToken, TokenKind, lex_lossless};
use crate::syntax::SyntaxKind;

/// First syntax slice migrated from the Camxes top-level shape:
/// text → paragraphs → paragraph → statement → sentence → bridi.
///
/// This parser is intentionally conservative. Rules not migrated from
/// camxes.peg are emitted as errors instead of being silently flattened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserOptions {
    pub recovery: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self { recovery: true }
    }
}

pub fn parse(source: &str, options: ParserOptions) -> Parse {
    let tokens = lex_lossless(source);
    let mut p = Parser {
        tokens,
        pos: 0,
        events: vec![Event::Start(SyntaxKind::Text)],
        recovery: options.recovery,
    };
    p.start(SyntaxKind::Paragraph);
    p.start(SyntaxKind::Statement);
    p.parse_sentence();
    while p.current_text().is_some_and(|s| s == "i" || s == "gi") {
        p.start(SyntaxKind::SentenceConnective);
        p.emit_current();
        p.finish();
        p.skip_trivia();
        p.parse_sentence();
    }
    p.finish(); // Statement
    p.finish(); // Paragraph
    while p.pos < p.tokens.len() {
        p.consume_error("unparsed input");
    }
    p.finish(); // Text
    let (green, errors) = build_green(p.events, &p.tokens);
    Parse { green, errors }
}

struct Parser<'a> {
    tokens: Vec<LexToken<'a>>,
    pos: usize,
    events: Vec<Event>,
    recovery: bool,
}

impl<'a> Parser<'a> {
    fn start(&mut self, kind: SyntaxKind) {
        self.events.push(Event::Start(kind));
    }
    fn finish(&mut self) {
        self.events.push(Event::Finish);
    }

    fn parse_bridi(&mut self) {
        self.skip_trivia();
        self.parse_indicators();
        self.parse_tags();
        self.start(SyntaxKind::Terms);
        while self.is_sumti_start() || self.is_place_tag() {
            if self.is_place_tag() {
                self.start(SyntaxKind::PlaceTag);
                self.emit_current();
                self.finish();
            } else {
                self.parse_sumti();
            }
            self.skip_trivia();
            if self.is_sumti_connective() {
                self.start(SyntaxKind::LogicalConnective);
                self.emit_current();
                self.finish();
                self.skip_trivia();
            }
        }
        self.finish();
        self.parse_tags();
        self.parse_indicators();
        // camxes.peg: sentence / bridi_tail uses CU as an optional
        // separator between terms and the predicate.
        if self.current_text() == Some("cu") {
            self.emit_current();
            self.skip_trivia();
        }
        if self.current_text() == Some("na") {
            self.start(SyntaxKind::Negation);
            self.emit_current();
            self.skip_trivia();
            self.finish();
        }
        if self.current_word().is_some() {
            self.parse_selbri();
        } else {
            self.error_at_current("expected selbri");
        }
        self.start(SyntaxKind::TailTerms);
        while self.pos < self.tokens.len() {
            if self
                .current_text()
                .is_some_and(|s| matches!(s, "i" | "gi" | "kei" | "ku"))
            {
                break;
            }
            if self.tokens[self.pos].kind == TokenKind::Eof {
                self.emit_current();
                break;
            }
            if self.tokens[self.pos].kind == TokenKind::Word {
                if self.is_indicator() {
                    self.start(SyntaxKind::Indicator);
                    self.emit_current();
                    self.finish();
                    self.skip_trivia();
                } else if self.is_place_tag() {
                    self.start(SyntaxKind::PlaceTag);
                    self.emit_current();
                    self.finish();
                    self.skip_trivia();
                } else if self.is_sumti_connective() {
                    self.start(SyntaxKind::LogicalConnective);
                    self.emit_current();
                    self.finish();
                    self.skip_trivia();
                } else {
                    self.parse_sumti();
                }
            } else {
                self.emit_current();
            }
        }
        self.finish();
    }

    fn parse_sentence(&mut self) {
        if self
            .current_text()
            .is_some_and(|s| matches!(s, "ge" | "ga" | "go" | "gu"))
        {
            self.parse_gek_sentence();
            return;
        }
        self.parse_vocative();
        self.start(SyntaxKind::Sentence);
        self.start(SyntaxKind::Bridi);
        self.parse_prenex();
        self.parse_bridi();
        self.finish();
        self.finish();
    }

    fn parse_gek_sentence(&mut self) {
        self.start(SyntaxKind::GekSentence);
        self.start(SyntaxKind::SentenceConnective);
        self.emit_current();
        self.finish();
        self.skip_trivia();

        self.start(SyntaxKind::Sentence);
        self.start(SyntaxKind::Bridi);
        self.parse_prenex();
        self.parse_bridi();
        self.finish();
        self.finish();
        self.skip_trivia();

        if self.current_text() == Some("gi") {
            self.start(SyntaxKind::SentenceConnective);
            self.emit_current();
            self.finish();
            self.skip_trivia();
        } else {
            self.error_at_current("expected gi in gek sentence");
        }

        self.start(SyntaxKind::Sentence);
        self.start(SyntaxKind::Bridi);
        self.parse_prenex();
        self.parse_bridi();
        self.finish();
        self.finish();
        self.finish();
    }

    fn parse_selbri(&mut self) {
        self.start(SyntaxKind::Selbri);
        if self
            .current_text()
            .is_some_and(|s| matches!(s, "se" | "te" | "ve" | "xe"))
        {
            self.emit_current();
            self.skip_trivia();
        }
        self.start(SyntaxKind::Tanru);
        if self.current_word().is_some() {
            self.emit_current();
        }
        loop {
            self.skip_trivia();
            if self.current_text() == Some("be") {
                self.parse_linkargs();
                continue;
            }
            if self
                .current_text()
                .is_some_and(crate::parser::grammar::is_selbri_connective)
            {
                self.start(SyntaxKind::LogicalConnective);
                self.emit_current();
                self.finish();
                self.skip_trivia();
                if self.current_word().is_some() {
                    self.emit_current();
                }
                continue;
            }
            if self.current_word().is_some()
                && !self.is_sumti_start()
                && !self.is_place_tag()
                && !self.is_indicator()
                && !matches!(self.current_text(), Some("cu" | "i" | "gi"))
            {
                self.emit_current();
            } else {
                break;
            }
        }
        self.finish();
        self.finish();
    }

    fn parse_linkargs(&mut self) {
        self.start(SyntaxKind::LinkArgs);
        self.emit_current(); // BE
        self.skip_trivia();
        loop {
            if self
                .current_text()
                .is_some_and(|s| s == "bei" || s == "be'o")
            {
                let end = self.current_text() == Some("be'o");
                self.emit_current();
                if end {
                    break;
                }
                self.skip_trivia();
                continue;
            }
            if self.current_text().is_some_and(|s| s == "i" || s == "gi")
                || self
                    .tokens
                    .get(self.pos)
                    .is_some_and(|t| t.kind == TokenKind::Eof)
            {
                break;
            }
            if self.is_sumti_start() {
                self.parse_sumti();
            } else {
                self.emit_current();
            }
            self.skip_trivia();
        }
        self.finish();
    }

    fn parse_sumti(&mut self) {
        self.start(SyntaxKind::Sumti);
        self.skip_trivia();
        if self.current_text() == Some("li") {
            self.emit_current();
            self.parse_mex();
            if self.current_text() == Some("lo'o") {
                self.emit_current();
            }
        } else if self
            .current_text()
            .is_some_and(|s| matches!(s, "le" | "lo"))
        {
            self.emit_current();
            self.skip_trivia();
            if self
                .current_text()
                .is_some_and(|s| matches!(s, "nu" | "du'u" | "ka"))
            {
                self.parse_abstraction();
            } else if self.current_word().is_some() {
                self.emit_current();
            } else {
                self.error_at_current("expected brivla after LE/LO");
            }
        } else if self.current_text().is_some_and(|s| s == "la") {
            self.emit_current();
            self.skip_trivia();
            // cmevla is surrounded by pauses in the ordinary spelling:
            // `la .alis.`. The pauses belong to the lossless CST.
            if self
                .tokens
                .get(self.pos)
                .is_some_and(|t| t.kind == TokenKind::Pause)
            {
                self.emit_current();
            }
            self.skip_trivia();
            if self.current_word().is_some() {
                self.emit_current();
            } else {
                self.error_at_current("expected cmevla after LA");
            }
            if self
                .tokens
                .get(self.pos)
                .is_some_and(|t| t.kind == TokenKind::Pause)
            {
                self.emit_current();
            }
        } else if self.current_word().is_some() {
            self.emit_current();
        }
        self.skip_trivia();
        if self
            .current_text()
            .is_some_and(|s| matches!(s, "poi" | "noi" | "voi"))
        {
            self.parse_relative_clause();
        }
        self.finish();
    }

    fn parse_mex(&mut self) {
        self.start(SyntaxKind::Mex);
        let mut operand = true;
        while self.pos < self.tokens.len() {
            self.skip_trivia();
            let Some(text) = self.current_text() else {
                break;
            };
            if matches!(text, "lo'o" | "i" | "gi" | "ku" | "kei") {
                break;
            }
            self.start(if operand {
                SyntaxKind::Operand
            } else {
                SyntaxKind::MexOperator
            });
            self.emit_current();
            self.finish();
            operand = !operand;
        }
        self.finish();
    }

    fn parse_relative_clause(&mut self) {
        self.start(SyntaxKind::RelativeClause);
        self.emit_current();
        self.skip_trivia();
        while self.pos < self.tokens.len() {
            if self.current_text() == Some("ku") {
                self.emit_current();
                break;
            }
            if self.tokens[self.pos].kind == TokenKind::Eof {
                break;
            }
            self.emit_current();
        }
        self.finish();
    }

    fn parse_prenex(&mut self) {
        let Some(end) = self.tokens.iter().position(|token| token.text == "zo'u") else {
            return;
        };
        if end <= self.pos {
            return;
        }
        self.start(SyntaxKind::Prenex);
        while self.pos <= end {
            self.emit_current();
        }
        self.finish();
        self.skip_trivia();
    }

    fn parse_indicators(&mut self) {
        loop {
            self.skip_trivia();
            if !self.is_indicator() {
                break;
            }
            self.start(SyntaxKind::Indicator);
            self.emit_current();
            self.finish();
        }
    }

    fn parse_vocative(&mut self) {
        if !self
            .current_text()
            .is_some_and(|s| matches!(s, "coi" | "co'o" | "doi"))
        {
            return;
        }
        self.start(SyntaxKind::Vocative);
        self.emit_current();
        self.skip_trivia();
        if self.is_sumti_start() {
            self.parse_sumti();
        }
        self.finish();
    }

    fn parse_tags(&mut self) {
        loop {
            self.skip_trivia();
            if !self.is_tag() {
                break;
            }
            self.start(SyntaxKind::Tag);
            self.emit_current();
            self.finish();
        }
    }

    fn parse_abstraction(&mut self) {
        self.start(SyntaxKind::Abstractor);
        self.emit_current();
        self.skip_trivia();
        self.start(SyntaxKind::Bridi);
        self.parse_bridi();
        self.finish();
        if self
            .current_text()
            .is_some_and(|s| matches!(s, "kei" | "ku"))
        {
            self.emit_current();
        }
        self.finish();
    }

    fn is_sumti_start(&self) -> bool {
        self.current_text().is_some_and(|s| {
            matches!(
                s,
                "mi" | "do" | "da" | "ti" | "ta" | "tu" | "le" | "lo" | "la" | "li"
            )
        })
    }

    fn is_sumti_connective(&self) -> bool {
        self.current_text()
            .is_some_and(|s| matches!(s, "ce" | "ce'e" | "joi" | "jo'i"))
    }

    fn is_place_tag(&self) -> bool {
        self.current_text()
            .is_some_and(|s| matches!(s, "fa" | "fe" | "fi" | "fo" | "fu"))
    }

    fn is_tag(&self) -> bool {
        self.current_text().is_some_and(|s| {
            matches!(
                s,
                "pu" | "ba"
                    | "ca"
                    | "za"
                    | "zi"
                    | "vi"
                    | "va"
                    | "vu"
                    | "ze'a"
                    | "ze'i"
                    | "ze'u"
                    | "ve'a"
                    | "ve'i"
                    | "ve'u"
                    | "fa'a"
                    | "ca'u"
                    | "ri'u"
                    | "zu'a"
                    | "bu'a"
                    | "bu'u"
                    | "ne'u"
            )
        })
    }

    fn is_indicator(&self) -> bool {
        self.current_text().is_some_and(|s| {
            matches!(
                s,
                "ui" | "ue"
                    | "ua"
                    | "u'i"
                    | "oi"
                    | "a'o"
                    | "e'u"
                    | "ca'i"
                    | "cu'i"
                    | "pei"
                    | "ru'e"
                    | "sai"
                    | "nai"
                    | "cai"
            )
        })
    }

    fn current_word(&self) -> Option<&str> {
        self.tokens
            .get(self.pos)
            .filter(|t| t.kind == TokenKind::Word)
            .map(|t| t.text)
    }
    fn current_text(&self) -> Option<&str> {
        self.tokens
            .get(self.pos)
            .filter(|t| {
                !matches!(
                    t.kind,
                    TokenKind::Whitespace
                        | TokenKind::Newline
                        | TokenKind::Eof
                        | TokenKind::Invalid
                )
            })
            .map(|t| t.text)
    }

    fn skip_trivia(&mut self) {
        while self.pos < self.tokens.len()
            && matches!(
                self.tokens[self.pos].kind,
                TokenKind::Whitespace | TokenKind::Newline
            )
        {
            self.emit_current();
        }
    }

    fn emit_current(&mut self) {
        if let Some(token) = self.tokens.get(self.pos).copied() {
            let kind = crate::parser::tokens::syntax_kind(token.kind);
            if token.kind == TokenKind::Invalid {
                self.events.push(Event::Error {
                    token_index: Some(self.pos as u32),
                });
            } else {
                self.events.push(Event::Token {
                    kind,
                    token_index: self.pos as u32,
                });
            }
            self.pos += 1;
        }
    }

    fn consume_error(&mut self, _message: &str) {
        let _recovery_enabled = self.recovery;
        if self.tokens.get(self.pos).is_some() {
            crate::parser::recovery::error_at(&mut self.events, Some(self.pos));
            self.pos += 1;
        }
    }

    fn error_at_current(&mut self, _message: &str) {
        if self.tokens.get(self.pos).is_some() {
            crate::parser::recovery::error_at(&mut self.events, Some(self.pos));
            self.pos += 1;
        } else {
            crate::parser::recovery::error_at(&mut self.events, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> Parse {
        super::parse(source, ParserOptions::default())
    }

    #[test]
    fn parses_basic_bridi_as_cst() {
        let parsed = parse("mi klama lo gerku");
        assert!(parsed.errors.is_empty());
        let root = parsed.syntax();
        assert_eq!(root.text().to_string(), "mi klama lo gerku");
        assert!(root.descendants().any(|n| n.kind() == SyntaxKind::Bridi));
        assert!(root.descendants().any(|n| n.kind() == SyntaxKind::Sumti));
        assert!(root.descendants().any(|n| n.kind() == SyntaxKind::Selbri));
        assert!(root.descendants().any(|n| n.kind() == SyntaxKind::Terms));
        assert!(
            root.descendants()
                .any(|n| n.kind() == SyntaxKind::TailTerms)
        );
    }

    #[test]
    fn parses_cu_and_la_pause_form() {
        for source in ["le nanmu cu klama", "la .alis. djuno"] {
            let parsed = parse(source);
            assert!(parsed.errors.is_empty(), "{source}: {:?}", parsed.errors);
            assert_eq!(parsed.syntax().text().to_string(), source);
        }
    }

    #[test]
    fn parses_mex_sumti() {
        let parsed = parse("li 1 + 2 lo'o du");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let root = parsed.syntax();
        assert!(root.descendants().any(|n| n.kind() == SyntaxKind::Mex));
        assert!(root.descendants().any(|n| n.kind() == SyntaxKind::MexOperator));
        assert_eq!(root.text().to_string(), "li 1 + 2 lo'o du");
    }

    #[test]
    fn keeps_abstraction_as_a_typed_node() {
        let parsed = parse("mi nelci lo nu klama");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        assert!(
            parsed
                .syntax()
                .descendants()
                .any(|n| n.kind() == SyntaxKind::Abstractor)
        );
        assert!(
            parsed
                .syntax()
                .descendants()
                .filter(|n| n.kind() == SyntaxKind::Bridi)
                .count()
                >= 2
        );
    }

    #[test]
    fn abstraction_returns_at_kei() {
        let source = "mi nelci lo nu klama kei";
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        assert_eq!(parsed.syntax().text().to_string(), source);
        assert!(
            parsed
                .syntax()
                .descendants()
                .any(|n| n.kind() == SyntaxKind::Abstractor)
        );
    }

    #[test]
    fn parses_relative_clause_and_negation() {
        let parsed = parse("lo nanmu poi tavla ku cu na klama");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let root = parsed.syntax();
        assert!(
            root.descendants()
                .any(|n| n.kind() == SyntaxKind::RelativeClause)
        );
        assert!(root.descendants().any(|n| n.kind() == SyntaxKind::Negation));
    }

    #[test]
    fn parses_prenex_boundary() {
        let parsed = parse("da zo'u da nelci mi");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        assert!(
            parsed
                .syntax()
                .descendants()
                .any(|n| n.kind() == SyntaxKind::Prenex)
        );
    }

    #[test]
    fn keeps_selbri_conversion_as_one_boundary() {
        let parsed = parse("mi se klama");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        assert!(
            parsed
                .syntax()
                .descendants()
                .any(|n| n.kind() == SyntaxKind::Selbri)
        );
    }

    #[test]
    fn groups_tanru_and_selbri_connectives() {
        for source in ["mi sutra klama", "mi klama je tavla do"] {
            let parsed = parse(source);
            assert!(parsed.errors.is_empty(), "{source}: {:?}", parsed.errors);
            let root = parsed.syntax();
            assert!(root.descendants().any(|n| n.kind() == SyntaxKind::Tanru));
            if source.contains(" je ") {
                assert!(
                    root.descendants()
                        .any(|n| n.kind() == SyntaxKind::LogicalConnective)
                );
            }
        }
    }

    #[test]
    fn splits_i_connected_sentences() {
        let parsed = parse("mi klama i do tavla");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let root = parsed.syntax();
        assert_eq!(
            root.descendants()
                .filter(|n| n.kind() == SyntaxKind::Sentence)
                .count(),
            2
        );
        assert_eq!(
            root.descendants()
                .filter(|n| n.kind() == SyntaxKind::SentenceConnective)
                .count(),
            1
        );
        assert_eq!(root.text().to_string(), "mi klama i do tavla");
    }

    #[test]
    fn parses_sumti_connectives_inside_terms() {
        for source in ["mi ce do klama", "mi joi do klama", "mi ce'e do klama"] {
            let parsed = parse(source);
            assert!(parsed.errors.is_empty(), "{source}: {:?}", parsed.errors);
            let root = parsed.syntax();
            assert!(
                root.descendants()
                    .any(|n| n.kind() == SyntaxKind::LogicalConnective)
            );
            assert_eq!(root.text().to_string(), source);
        }
    }

    #[test]
    fn parses_selbri_linkargs() {
        let source = "mi klama be lo zarci bei lo zdani be'o";
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        assert!(
            parsed
                .syntax()
                .descendants()
                .any(|n| n.kind() == SyntaxKind::LinkArgs)
        );
        assert_eq!(parsed.syntax().text().to_string(), source);
    }

    #[test]
    fn parses_explicit_place_tags() {
        let source = "fa mi klama fe do";
        let parsed = parse(source);
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        assert_eq!(
            parsed
                .syntax()
                .descendants()
                .filter(|n| n.kind() == SyntaxKind::PlaceTag)
                .count(),
            2
        );
        assert_eq!(parsed.syntax().text().to_string(), source);
    }

    #[test]
    fn parses_tense_and_space_tags() {
        for source in ["pu mi klama", "mi pu klama", "ba vi klama"] {
            let parsed = parse(source);
            assert!(parsed.errors.is_empty(), "{source}: {:?}", parsed.errors);
            assert!(
                parsed
                    .syntax()
                    .descendants()
                    .any(|n| n.kind() == SyntaxKind::Tag)
            );
        }
    }

    #[test]
    fn parses_indicators_and_vocatives() {
        let first = parse("coi do ui klama");
        assert!(first.errors.is_empty(), "{:?}", first.errors);
        let root = first.syntax();
        assert!(root.descendants().any(|n| n.kind() == SyntaxKind::Vocative));
        assert!(
            root.descendants()
                .any(|n| n.kind() == SyntaxKind::Indicator)
        );

        let second = parse("mi klama cai");
        assert!(second.errors.is_empty(), "{:?}", second.errors);
        assert!(
            second
                .syntax()
                .descendants()
                .any(|n| n.kind() == SyntaxKind::Indicator)
        );
    }

    #[test]
    fn parses_gek_sentence_with_gi_separator() {
        let parsed = parse("ge mi klama gi do tavla");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let root = parsed.syntax();
        assert!(
            root.descendants()
                .any(|n| n.kind() == SyntaxKind::GekSentence)
        );
        assert_eq!(
            root.descendants()
                .filter(|n| n.kind() == SyntaxKind::Sentence)
                .count(),
            2
        );
        assert_eq!(root.text().to_string(), "ge mi klama gi do tavla");
    }
}
