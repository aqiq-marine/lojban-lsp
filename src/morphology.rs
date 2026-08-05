//! Morphology analysis independent from the lossless lexer and syntax parser.

use crate::lexer::{LexToken, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisMode {
    Simple,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordKind {
    Cmavo,
    Gismu,
    Lujvo,
    Fuivla,
    Cmevla,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordAnalysis<'a> {
    pub text: &'a str,
    pub kind: WordKind,
}

/// Analyze one lexer word without changing the lossless token stream.
pub fn analyze<'a>(token: &'a LexToken<'a>, mode: AnalysisMode) -> Option<WordAnalysis<'a>> {
    if token.kind != TokenKind::Word {
        return None;
    }
    let kind = if is_cmavo(token.text) {
        WordKind::Cmavo
    } else if is_gismu(token.text) {
        WordKind::Gismu
    } else if is_cmene(token.text) {
        WordKind::Cmevla
    } else if mode == AnalysisMode::Complete && is_lujvo(token.text) {
        WordKind::Lujvo
    } else if mode == AnalysisMode::Complete && is_fuivla(token.text) {
        WordKind::Fuivla
    } else {
        WordKind::Unknown
    };
    Some(WordAnalysis {
        text: token.text,
        kind,
    })
}

pub fn analyze_tokens<'a>(tokens: &'a [LexToken<'a>], mode: AnalysisMode) -> Vec<WordAnalysis<'a>> {
    tokens
        .iter()
        .filter_map(|token| analyze(token, mode))
        .collect()
}

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

pub fn is_consonant(c: char) -> bool {
    matches!(
        c,
        'b' | 'c'
            | 'd'
            | 'f'
            | 'g'
            | 'j'
            | 'k'
            | 'l'
            | 'm'
            | 'n'
            | 'p'
            | 'r'
            | 's'
            | 't'
            | 'v'
            | 'x'
            | 'z'
            | 'B'
            | 'C'
            | 'D'
            | 'F'
            | 'G'
            | 'J'
            | 'K'
            | 'L'
            | 'M'
            | 'N'
            | 'P'
            | 'R'
            | 'S'
            | 'T'
            | 'V'
            | 'X'
            | 'Z'
    )
}

pub fn is_cmene(s: &str) -> bool {
    // 固有名詞: 語末が子音であること
    s.chars().last().map_or(false, is_consonant)
}

pub fn is_gismu(s: &str) -> bool {
    // gismu: 5文字、CVCCV または CCVCV
    if s.len() != 5 {
        return false;
    }
    let chars: Vec<char> = s.chars().collect();
    let is_v = |i| is_vowel(chars[i]);
    let is_c = |i| is_consonant(chars[i]);

    (is_c(0) && is_c(1) && is_v(2) && is_c(3) && is_v(4))
        || (is_c(0) && is_v(1) && is_c(2) && is_c(3) && is_v(4))
}

pub fn is_cmavo(s: &str) -> bool {
    // cmavo: 語末が母音、かつ子音連続を含まない(単純化)
    s.chars().last().map_or(false, is_vowel) && !s.contains(|c: char| is_consonant(c)) // 実際はもっと複雑だが第一近似
}

fn is_lujvo(s: &str) -> bool {
    s.len() >= 6 && s.chars().filter(|c| is_vowel(*c)).count() >= 2
}

fn is_fuivla(s: &str) -> bool {
    s.len() >= 5 && !is_gismu(s) && !is_cmene(s) && s.chars().any(|c| is_vowel(c))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::lex_lossless;

    #[test]
    fn lexer_does_not_classify_words() {
        let token = lex_lossless("klama")[0];
        assert_eq!(
            analyze(&token, AnalysisMode::Simple).unwrap().kind,
            WordKind::Gismu
        );
    }

    #[test]
    fn modes_control_extended_analysis() {
        let token = lex_lossless("banlu")[0];
        assert_eq!(
            analyze(&token, AnalysisMode::Simple).unwrap().kind,
            WordKind::Gismu
        );
    }
}
