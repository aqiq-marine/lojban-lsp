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
    } else if mode == AnalysisMode::Complete && is_fuhivla(token.text) {
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
    s.chars().last().map_or(false, is_consonant)
}

pub fn is_valid_diphthong(v1: char, v2: char) -> bool {
    matches!(
        (v1, v2),
        ('a', 'i')
            | ('a', 'u')
            | ('e', 'i')
            | ('o', 'i')
    )
}

pub fn is_initial_consonant_pair(c1: char, c2: char) -> bool {
    matches!(
        (c1, c2),
        // voiced + liquid
        ('b', 'l') | ('b', 'r')
        | ('d', 'j') | ('d', 'r') | ('d', 'z')
        | ('g', 'l') | ('g', 'r')
        | ('v', 'l') | ('v', 'r')

        // voiceless + liquid
        | ('c', 'f') | ('c', 'k') | ('c', 'l') | ('c', 'm')
        | ('c', 'n') | ('c', 'p') | ('c', 'r') | ('c', 't')
        | ('f', 'l') | ('f', 'r')
        | ('j', 'b') | ('j', 'd') | ('j', 'g') | ('j', 'm') | ('j', 'v')
        | ('k', 'l') | ('k', 'r')
        | ('m', 'l') | ('m', 'r')
        | ('p', 'l') | ('p', 'r')
        | ('s', 'f') | ('s', 'k') | ('s', 'l') | ('s', 'm')
        | ('s', 'n') | ('s', 'p') | ('s', 'r') | ('s', 't')
        | ('t', 'c') | ('t', 'r') | ('t', 's')
        | ('x', 'l') | ('x', 'r')
        | ('z', 'b') | ('z', 'd') | ('z', 'g') | ('z', 'm') | ('z', 'v')
    )
}

fn is_liquid(c: char) -> bool {
    matches!(c, 'l' | 'm' | 'n' | 'r')
}

fn is_voiced_obstruent(c: char) -> bool {
    matches!(c, 'b' | 'd' | 'g' | 'v' | 'z' | 'j')
}

fn is_voiceless_obstruent(c: char) -> bool {
    matches!(c, 'p' | 't' | 'k' | 'f' | 's' | 'c' | 'x')
}

fn is_sibilant(c: char) -> bool {
    matches!(c, 'c' | 'j' | 's' | 'z')
}

pub fn is_medial_consonant_pair(c1: char, c2: char) -> bool {
    // 同じ子音は禁止
    if c1 == c2 {
        return false;
    }

    // c/j/s/z 同士は禁止
    if is_sibilant(c1) && is_sibilant(c2) {
        return false;
    }

    // 個別禁止
    if matches!(
        (c1, c2),
        ('c', 'x')
            | ('k', 'x')
            | ('x', 'c')
            | ('x', 'k')
            | ('m', 'z')
    ) {
        return false;
    }

    // 有声阻害音 + 無声阻害音は禁止（流音・鼻音は除く）
    if !is_liquid(c1)
        && !is_liquid(c2)
        && ((is_voiced_obstruent(c1) && is_voiceless_obstruent(c2))
            || (is_voiceless_obstruent(c1) && is_voiced_obstruent(c2)))
    {
        return false;
    }

    true
}


pub fn is_rafsi_shape(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    match chars.as_slice() {
        // CVC
        [c1, v1, c2]
            if is_consonant(*c1)
                && is_vowel(*v1)
                && is_consonant(*c2) =>
        {
            true
        }

        // CCV
        [c1, c2, v1]
            if is_consonant(*c1)
                && is_consonant(*c2)
                && is_vowel(*v1)
                && is_initial_consonant_pair(*c1, *c2) =>
        {
            true
        }

        // CVV
        [c1, v1, v2]
            if is_consonant(*c1)
                && is_vowel(*v1)
                && is_vowel(*v2)
                && is_valid_diphthong(*v1, *v2) =>
        {
            true
        }

        // CVCC
        [c1, v1, c2, c3]
            if is_consonant(*c1)
                && is_vowel(*v1)
                && is_consonant(*c2)
                && is_consonant(*c3) =>
        {
            true
        }

        // CCVC
        [c1, c2, v1, c3]
            if is_consonant(*c1)
                && is_consonant(*c2)
                && is_vowel(*v1)
                && is_consonant(*c3)
                && is_initial_consonant_pair(*c1, *c2) =>
        {
            true
        }

        // gismu
        [c1, c2, v1, c3, v2]
            if is_consonant(*c1)
                && is_consonant(*c2)
                && is_vowel(*v1)
                && is_consonant(*c3)
                && is_vowel(*v2)
                && is_initial_consonant_pair(*c1, *c2) =>
        {
            true
        }

        [c1, v1, c2, c3, v2]
            if is_consonant(*c1)
                && is_vowel(*v1)
                && is_consonant(*c2)
                && is_consonant(*c3)
                && is_vowel(*v2) =>
        {
            true
        }

        _ => false,
    }
}

pub fn split_lujvo(word: &str) -> Option<Vec<String>> {
    fn is_cvv(rafsi: &str) -> bool {
        let chars: Vec<char> = rafsi.chars().collect();
        matches!(
            chars.as_slice(),
            [c, v1, v2]
                if is_consonant(*c)
                    && is_vowel(*v1)
                    && is_vowel(*v2)
                    && is_valid_diphthong(*v1, *v2)
        )
    }

    fn dfs(rest: &str, prev_was_cvv: bool, out: &mut Vec<String>) -> bool {
        if rest.is_empty() {
            return true;
        }

        // 残り全体が gismu なら最後の要素として採用
        if is_gismu(rest) {
            out.push(rest.to_string());
            return true;
        }

        // y は常にハイフン
        if let Some(next) = rest.strip_prefix('y') {
            if dfs(next, false, out) {
                return true;
            }
        }

        // r / n は直前が CVV rafsi の場合のみ
        if prev_was_cvv {
            if let Some(next) = rest.strip_prefix('r') {
                if dfs(next, false, out) {
                    return true;
                }
            }

            if let Some(next) = rest.strip_prefix('n') {
                let mut ok = false;

                for len in 3..=5 {
                    if next.len() < len {
                        continue;
                    }

                    let candidate = &next[..len];
                    if is_rafsi_shape(candidate) && candidate.starts_with('r') {
                        ok = true;
                        break;
                    }
                }

                if ok && dfs(next, false, out) {
                    return true;
                }
            }
        }

        // rafsi を探索
        for len in (3..=5).rev() {
            if rest.len() < len {
                continue;
            }

            let candidate = &rest[..len];

            if !is_rafsi_shape(candidate) {
                continue;
            }

            out.push(candidate.to_string());

            if dfs(&rest[len..], is_cvv(candidate), out) {
                return true;
            }

            out.pop();
        }

        false
    }

    let mut parts = Vec::new();

    if dfs(word, false, &mut parts) && parts.len() >= 2 {
        Some(parts)
    } else {
        None
    }
}


pub fn is_gismu(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    match chars.as_slice() {
        // CCVCV
        [c1, c2, v1, c3, v2]
            if is_consonant(*c1)
                && is_consonant(*c2)
                && is_vowel(*v1)
                && is_consonant(*c3)
                && is_vowel(*v2)
                && is_initial_consonant_pair(*c1, *c2) =>
        {
            true
        }

        // CVCCV
        [c1, v1, c2, c3, v2]
            if is_consonant(*c1)
                && is_vowel(*v1)
                && is_consonant(*c2)
                && is_consonant(*c3)
                && is_vowel(*v2)
                && is_medial_consonant_pair(*c2, *c3) =>
        {
            true
        }

        _ => false,
    }
}

pub fn is_cmavo(s: &str) -> bool {
    s.len() <= 4 && s.chars().last().map_or(false, is_vowel) && !is_gismu(s)
}

pub fn is_lujvo(s: &str) -> bool {
    split_lujvo(s).is_some()
}

pub fn is_fuhivla(s: &str) -> bool {
    s.len() >= 5 && !is_gismu(s) && !is_lujvo(s) && !s.contains('y') && s.chars().any(is_vowel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_lujvo() {
        assert_eq!(
            split_lujvo("blazda"),
            Some(vec![
                "bla".to_string(),
                "zda".to_string(),
            ])
        );
        
        assert_eq!(
            split_lujvo("gerkuzdani"),
            Some(vec![
                "gerku".to_string(),
                "zdani".to_string(),
            ])
        );
        
        assert_eq!(
            split_lujvo("jbobau"),
            Some(vec![
                "jbo".to_string(),
                "bau".to_string(),
            ])
        );
        
        assert_eq!(
            split_lujvo("selci"),
            None
        );
    }
}
