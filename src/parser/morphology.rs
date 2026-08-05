use crate::ast::LojbanAST;

// Phonemes
pub(crate) fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

pub(crate) fn is_consonant(c: char) -> bool {
    matches!(c, 'b' | 'c' | 'd' | 'f' | 'g' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'r' | 's' | 't' | 'v' | 'x' | 'z' |
                'B' | 'C' | 'D' | 'F' | 'G' | 'J' | 'K' | 'L' | 'M' | 'N' | 'P' | 'R' | 'S' | 'T' | 'V' | 'X' | 'Z')
}

// AST helpers
pub(crate) fn gismu_rule(s: &str) -> LojbanAST {
    LojbanAST::Gismu(vec![LojbanAST::Token(s.to_string())])
}

pub(crate) fn lujvo_rule(s: &str) -> LojbanAST {
    LojbanAST::Lujvo(vec![LojbanAST::Token(s.to_string())])
}

pub(crate) fn fuhivla_rule(s: &str) -> LojbanAST {
    LojbanAST::Fuhivla(vec![LojbanAST::Token(s.to_string())])
}

pub(crate) fn cmavo_rule(kind: &str, s: &str) -> LojbanAST {
    LojbanAST::Cmavo(kind.to_string(), vec![LojbanAST::Token(s.to_string())])
}
