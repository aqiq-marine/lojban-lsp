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
            if self.is_one_of(&["lo'o", "loho"]) {
                self.emit_current();
            }
        } else if self.current_text() == Some("vei") {
            self.parse_grouped_mex("ve'o");
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
        self.parse_mex_expression(0);
        self.finish();
    }

    fn parse_mex_expression(&mut self, precedence: u8) {
        self.skip_trivia();
        let Some(text) = self.current_text().map(str::to_owned) else {
            return;
        };

        // Prefix parselet
        if text == "vei" {
            self.parse_grouped_mex("ve'o");
        } else if text == "fu'a" {
            self.parse_reverse_polish_expression();
        } else if text == "pe'ho" {
            self.parse_forethought_expression();
        } else if matches!(text.as_str(), "maho" | "nihe" | "mohe" | "nahu") {
            self.start(SyntaxKind::PrefixMex);
            self.emit_current();
            self.skip_trivia();
            match text.as_str() {
                "nihe" | "nahu" => self.parse_selbri(),
                "mohe" => self.parse_sumti(),
                _ => self.parse_mex_expression(0),
            }
            self.consume_optional("tehu", "expected TEhU after MEX operator");
            self.finish();
        } else if matches!(text.as_str(), "se" | "te" | "ve" | "xe" | "na'e" | "ke") {
            self.start(SyntaxKind::PrefixMex);
            self.emit_current(); // Operator
            self.skip_trivia();
            self.parse_mex_expression(100); // High precedence for prefix
            if text == "ke" {
                self.consume_optional("ke'e", "expected KEhE after grouped operator");
            }
            self.finish();
        } else {
            self.parse_operand();
        }

        loop {
            self.skip_trivia();
            let Some(next_text) = self.current_text() else {
                break;
            };

            // Handle SA correction: skip preceding expressions
            if next_text == "sa" {
                self.emit_current(); // SA marks replacement of the preceding mex
                self.skip_trivia();
                self.parse_operand();
                continue;
            }

            if self.is_mex_terminator(next_text) {
                break;
            }

            if matches!(next_text, "ja" | "je" | "jo" | "ju" | "joi" | "jo'i") {
                self.start(SyntaxKind::LogicalConnective);
                self.emit_current();
                self.finish();
                self.skip_trivia();
                self.parse_mex_expression(precedence);
                continue;
            }

            let p = self.mex_precedence(next_text);
            if p <= precedence {
                break;
            }

            if next_text == "bi'e" {
                self.start(SyntaxKind::ModifiedOperator);
                self.emit_current(); // BIhE
                self.skip_trivia();

                let op_text = self.current_text().map(|s| s.to_string());
                if let Some(op_text) = op_text {
                    self.emit_current(); // The operator being modified

                    let p_op = self.mex_precedence(&op_text);
                    self.parse_mex_expression(p_op);
                } else {
                    self.error_at_current("expected operator after BIhE");
                }

                self.finish();
            } else {
                self.start(SyntaxKind::BinaryExpression);
                self.emit_current();
                self.parse_mex_expression(p);
                self.finish();
            }
        }
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
                "mi" | "do" | "da" | "ti" | "ta" | "tu" | "le" | "lo" | "la" | "li" | "vei"
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

    fn parse_grouped_mex(&mut self, end_token: &str) {
        self.start(SyntaxKind::GroupedMex);
        self.emit_current(); // VEI
        self.parse_mex_expression(0);
        if self.current_text() == Some(end_token) {
            self.emit_current(); // VE'O
        }
        self.finish();
    }

    fn parse_reverse_polish_expression(&mut self) {
        self.start(SyntaxKind::ReversePolishExpression);
        self.emit_current(); // FUhA

        let mut operands = 0usize;
        loop {
            self.skip_trivia();
            let Some(text) = self.current_text() else {
                break;
            };
            if self.is_mex_terminator(text) {
                break;
            }
            if self.mex_precedence(text) > 0 {
                self.start(SyntaxKind::MexOperator);
                self.emit_current();
                self.finish();
            } else {
                self.parse_operand();
                operands += 1;
            }
        }
        if operands == 0 {
            self.error_at_current("expected operand in reverse-polish expression");
        }
        self.finish();
    }

    fn parse_forethought_expression(&mut self) {
        self.start(SyntaxKind::ForethoughtExpression);
        self.emit_current(); // PEhO

        self.skip_trivia();
        self.emit_current(); // The operator

        loop {
            self.skip_trivia();
            let Some(text) = self.current_text() else {
                break;
            };
            if text == "ku'e" {
                self.emit_current();
                break;
            }
            if self.is_mex_terminator(text) {
                break;
            }
            self.parse_operand();
        }
        self.finish();
    }

    fn parse_operand(&mut self) {
        self.start(SyntaxKind::Operand);
        self.skip_trivia();
        let Some(text) = self.current_text() else {
            self.finish();
            return;
        };

        if text == "nihe" {
            self.emit_current();
            self.skip_trivia();
            self.parse_selbri();
            self.consume_optional("tehu", "expected TEhU after NIhE expression");
        } else if text == "mohe" {
            self.emit_current();
            self.skip_trivia();
            self.parse_sumti();
            self.consume_optional("tehu", "expected TEhU after MOhE expression");
        } else if text == "johi" {
            self.emit_current();
            self.skip_trivia();
            while !self.is_one_of(&["tehu", "te'u", "tehU"])
                && !self.is_mex_terminator_current()
                && self.pos < self.tokens.len()
            {
                self.parse_mex_expression(0);
                self.skip_trivia();
            }
            self.consume_optional("tehu", "expected TEhU after JOhI expression");
            self.consume_optional("te'u", "expected TEhU after JOhI expression");
        } else if text == "ke" {
            self.emit_current();
            self.skip_trivia();
            self.parse_operand();
            self.consume_optional("kehe", "expected KEhE after operand group");
            self.consume_optional("ke'e", "expected KEhE after operand group");
        } else {
            self.parse_operand_3();
        }
        self.skip_trivia();
        while self.is_operand_connective() {
            self.start(SyntaxKind::LogicalConnective);
            self.emit_current();
            self.finish();
            self.skip_trivia();
            self.parse_operand_3();
            self.skip_trivia();
        }
        self.finish();
    }

    fn parse_operand_3(&mut self) {
        self.skip_trivia();
        let Some(text) = self.current_text().map(str::to_owned) else {
            return;
        };

        if text == "vei" {
            self.parse_grouped_mex("ve'o");
        } else if self.tokens[self.pos].kind == TokenKind::Number
            || self.is_number_word(text.as_str())
        {
            self.start(SyntaxKind::Quantifier);
            while self
                .tokens
                .get(self.pos)
                .is_some_and(|t| t.kind == TokenKind::Number || self.is_number_word(t.text))
            {
                self.emit_current();
                self.skip_trivia();
            }
            self.consume_optional("boi", "optional BOI after number");
            self.finish();
        } else if text == "ge" || text == "ga" || text == "go" || text == "gu" {
            self.parse_gek_operand();
        } else if matches!(text.as_str(), "la'e" | "lahe" | "na'e" | "nahe") {
            self.emit_current();
            self.skip_trivia();
            if matches!(text.as_str(), "na'e" | "nahe") && self.current_text() == Some("bo") {
                self.emit_current();
            }
            self.parse_operand();
            self.consume_optional("lu'u", "expected LUhU after operand modifier");
            self.consume_optional("luhu", "expected LUhU after operand modifier");
        } else if text == "tei" {
            self.start(SyntaxKind::Operand);
            self.emit_current();
            self.skip_trivia();
            while self
                .current_text()
                .is_some_and(|value| value != "foi" && self.is_lerfu_word(value))
            {
                self.emit_current();
                self.skip_trivia();
            }
            self.consume_optional("foi", "expected FOI after TEI lerfu string");
            self.finish();
        } else if self.is_lerfu_word(text.as_str()) {
            self.start(SyntaxKind::Operand);
            self.emit_current();
            self.skip_trivia();
            while self
                .current_text()
                .is_some_and(|value| self.is_lerfu_word(value))
            {
                self.emit_current();
                self.skip_trivia();
            }
            self.finish();
        } else {
            self.emit_current();
        }
    }

    fn parse_gek_operand(&mut self) {
        self.start(SyntaxKind::GekSentence);
        self.emit_current();
        self.skip_trivia();
        self.parse_operand();
        self.skip_trivia();
        if self.is_one_of(&["gi", "gik"]) {
            self.emit_current();
        }
        self.skip_trivia();
        self.parse_operand_3();
        self.finish();
    }

    fn is_mex_terminator(&self, text: &str) -> bool {
        matches!(
            text,
            "lo'o"
                | "loho"
                | "ve'o"
                | "veho"
                | "tehu"
                | "te'u"
                | "kuhe"
                | "ku'e"
                | "luhu"
                | "lu'u"
                | "boi"
                | "moi"
                | "kehe"
                | "ke'e"
        )
    }

    fn is_mex_terminator_current(&self) -> bool {
        self.current_text()
            .is_some_and(|text| self.is_mex_terminator(text))
    }

    fn is_number_word(&self, text: &str) -> bool {
        matches!(
            text,
            "no" | "pa"
                | "re"
                | "ci"
                | "vo"
                | "mu"
                | "xa"
                | "ze"
                | "bi"
                | "so"
                | "dau"
                | "fei"
                | "ga'"
                | "pi'e"
        )
    }

    fn is_operand_connective(&self) -> bool {
        self.current_text()
            .is_some_and(|text| matches!(text, "ja" | "je" | "jo" | "ju" | "joi" | "jo'i" | "ce'e"))
    }

    fn is_lerfu_word(&self, text: &str) -> bool {
        matches!(
            text,
            "by" | "cy"
                | "dy"
                | "fy"
                | "gy"
                | "my"
                | "ny"
                | "py"
                | "ry"
                | "sy"
                | "ty"
                | "vy"
                | "xy"
                | "zy"
                | "bu"
        )
    }

    fn is_one_of(&self, values: &[&str]) -> bool {
        self.current_text()
            .is_some_and(|text| values.contains(&text))
    }

    fn consume_optional(&mut self, token: &str, _message: &str) {
        if self.is_one_of(&[token]) {
            self.emit_current();
        }
    }

    fn mex_precedence(&self, operator: &str) -> u8 {
        match operator {
            "=" => 40,
            "+" | "su'i" | "vu'u" | "fe'i" | "ju'u" | "gei" => 50,
            "*" | "/" | "pi'i" | "fa'i" | "te'a" | "cu'a" | "va'a" | "ne'o" | "de'o" | "fe'a"
            | "sa'o" | "re'a" | "ri'o" | "sa'i" | "pi'a" | "si'i" => 60,
            "bi'e" => 70,
            _ => 0,
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
        assert!(
            root.descendants()
                .any(|n| n.kind() == SyntaxKind::BinaryExpression)
        );
        assert_eq!(root.text().to_string(), "li 1 + 2 lo'o du");
    }

    #[test]
    fn parses_vei_mex_as_sumti() {
        let parsed = parse("vei 1 + 2 ve'o du");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let root = parsed.syntax();
        assert!(
            root.descendants()
                .any(|n| n.kind() == SyntaxKind::GroupedMex)
        );
        assert!(
            root.descendants()
                .any(|n| n.kind() == SyntaxKind::BinaryExpression)
        );
        assert_eq!(root.text().to_string(), "vei 1 + 2 ve'o du");
    }

    #[test]
    fn parses_reverse_polish_mex_operator_separately() {
        let parsed = parse("li fu'a 1 2 + lo'o du");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        let root = parsed.syntax();
        assert!(
            root.descendants()
                .any(|n| n.kind() == SyntaxKind::ReversePolishExpression)
        );
        assert!(
            root.descendants()
                .any(|n| n.kind() == SyntaxKind::MexOperator)
        );
    }

    #[test]
    fn parses_forethought_mex() {
        let parsed = parse("li pe'ho su'i 1 2 ku'e lo'o du");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        assert!(
            parsed
                .syntax()
                .descendants()
                .any(|n| n.kind() == SyntaxKind::ForethoughtExpression)
        );
    }

    #[test]
    fn parses_connected_mex_operators() {
        let parsed = parse("li 1 ja 2 lo'o du");
        assert!(parsed.errors.is_empty(), "{:?}", parsed.errors);
        assert!(
            parsed
                .syntax()
                .descendants()
                .any(|n| n.kind() == SyntaxKind::LogicalConnective)
        );
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
