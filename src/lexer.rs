use crate::morphology;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    LA(&'a str),
    CU,
    CMENE(&'a str),
    BRIVLA(&'a str),
    CMAVO(&'a str),
    KOhA(&'a str),
    LE(&'a str),
    DUhU,
    NU,
    KA,
    PAUSE,
    EOF,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((idx, c)) = chars.peek().cloned() {
        match c {
            '.' => {
                tokens.push(Token::PAUSE);
                chars.next();
            }
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            _ => {
                let start = idx;
                let mut end = idx;
                while let Some((i, c)) = chars.peek().cloned() {
                    if c.is_alphabetic() || c == '\'' {
                        end = i + c.len_utf8();
                        chars.next();
                    } else {
                        break;
                    }
                }
                if start < end {
                    let s = &input[start..end];
                    match s {
                        "la" => tokens.push(Token::LA(s)),
                        "cu" => tokens.push(Token::CU),
                        "mi" | "do" | "da" | "ti" | "ta" | "tu" => tokens.push(Token::KOhA(s)),
                        "le" | "lo" => tokens.push(Token::LE(s)),
                        "du'u" => tokens.push(Token::DUhU),
                        "nu" => tokens.push(Token::NU),
                        "ka" => tokens.push(Token::KA),
                        _ if morphology::is_cmene(s) => tokens.push(Token::CMENE(s)),
                        _ if morphology::is_gismu(s) => tokens.push(Token::BRIVLA(s)),
                        _ => tokens.push(Token::BRIVLA(s)),
                    }
                }
            }
        }
    }
    tokens.push(Token::EOF);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_la_alis_djuno() {
        let input = "la .alis. djuno";
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec![
                Token::LA("la"),
                Token::PAUSE,
                Token::CMENE("alis"),
                Token::PAUSE,
                Token::BRIVLA("djuno"),
                Token::EOF
            ]
        );
    }
}
