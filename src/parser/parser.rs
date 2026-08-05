use peg::parser;
use crate::ast::LojbanAST;
use crate::lexer::Token;
use crate::parser::morphology::*;

parser! {
    pub grammar lojban_parser<'a>() for [Token<'a>] {
        rule boundary() = [Token::PAUSE]
        rule CU() = [Token::CU]

        rule LA() -> LojbanAST = [Token::LA(s)] { cmavo_rule("LA", s) }
        rule KOhA() -> LojbanAST = [Token::KOhA(s)] { cmavo_rule("KOhA", s) }
        rule LE() -> LojbanAST = [Token::LE(s)] { cmavo_rule("LE", s) }
        rule DUhU() -> LojbanAST = [Token::DUhU] { cmavo_rule("DUhU", "du'u") }
        rule NU() -> LojbanAST = [Token::NU] { cmavo_rule("NU", "nu") }
        rule KA() -> LojbanAST = [Token::KA] { cmavo_rule("KA", "ka") }
        rule BRIVLA() -> LojbanAST = [Token::BRIVLA(s)] { LojbanAST::Brivla(vec![LojbanAST::Token(s.to_string())]) }
        rule CMENE() -> LojbanAST = [Token::CMENE(s)] { LojbanAST::Cmene(s.to_string()) }
        
        rule EOF() = [Token::EOF]
        
        pub rule text() -> LojbanAST = p:paragraph()+ EOF() { LojbanAST::Text(p) }

        rule paragraph() -> LojbanAST = s:statement() { LojbanAST::Paragraph(vec![s]) }

        rule statement() -> LojbanAST = s:sentence() / s:sumti() { 
            match s {
                LojbanAST::Sumti(children) => LojbanAST::Statement(children),
                _ => LojbanAST::Statement(vec![s])
            }
        }

        rule sentence() -> LojbanAST = b:bridi() { LojbanAST::Sentence(vec![b]) }
        
        rule bridi() -> LojbanAST = s1:sumti()* CU()? sel:selbri() s2:sumti()* { 
            let mut args = s1;
            args.extend(s2);
            LojbanAST::Bridi(vec![sel, LojbanAST::Sumti(args)]) 
        }

        rule selbri() -> LojbanAST = b:BRIVLA() { b }
        
        pub rule sumti() -> LojbanAST = s:sumti_start() { s }

        rule sumti_start() -> LojbanAST = 
            k:koha_sumti() { k } /
            l:la_sumti() { l } /
            le:le_sumti() { le } /
            la:le_abstraction_sumti() { la } /
            a:abstraction_sumti() { a }

        rule koha_sumti() -> LojbanAST = k:KOhA() { LojbanAST::Sumti(vec![k]) }
        rule la_sumti() -> LojbanAST = la:LA() boundary() c:CMENE() boundary()? { LojbanAST::Sumti(vec![la, c]) }
        rule le_sumti() -> LojbanAST = le:LE() b:BRIVLA() { LojbanAST::Sumti(vec![le, b]) }
        rule le_abstraction_sumti() -> LojbanAST = le:LE() a:abstraction_sumti() { LojbanAST::Sumti(vec![le, a]) }
        rule abstractor() -> LojbanAST = d:DUhU() { d } / n:NU() { n } / k:KA() { k }
        rule abstraction_sumti() -> LojbanAST = a:abstractor() b:bridi() { LojbanAST::Abstractor(vec![a, b]) }
    }
}
