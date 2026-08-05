pub mod morphology;
pub mod parser;
pub mod syntax;

pub mod builder;
pub mod ast;
pub mod oracle;
pub mod oracle_parser;
pub mod lexer;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::parser::lojban_parser;
    use crate::oracle::run_js_parser;
    use crate::oracle_parser::parse_js_output;
    use crate::lexer;

    #[test]
    fn test_oracle_batch() {
        let inputs = vec![
            "mi klama",
            "do tavla mi",
            "mi viska lo gerku",
            "le nanmu cu klama",
            "lo gerku cu citka lo plise",
            "mi pu klama le zarci",
            "mi ba tavla do",
            "mi ca pinxe lo djacu",
            "ta citka lo plise",
            "ti melbi",
            "mi nelci lo cukta",
            "le mlatu cu sipna",
            "lo prenu cu tavla lo prenu",
            "mi klama le zdani poi melbi",
            "do viska le gerku poi sutra ku",
            "le nanmu poi tavla ku cu klama",
            "lo plise poi xunre cu kukte",
            "mi tavla fi do",
            "mi dunda lo cukta do",
            "mi dunda lo cukta do ti",
            "mi na klama",
            "mi na pu klama",
            "mi pu na klama",
            "mi ba citka lo plise",
            "za ba klama",
            "ta'e tavla",
            "ru'a mi djuno",
            "pei do nelci lo nu klama",
            "coi do",
            "co'o do",
            "da zo'u da nelci mi",
            "mi ce'e do nelci lo cukta",
            "mi nelci le ka melbi",
            "la .alis. djuno le du'u la .bob. klama",
            "la .alis. noi melbi cu klama",
            "lo nanmu poi viska lo gerku poi cadzu cu tavla",
            "lo nanmu poi melbi zi'e poi sutra cu klama",
            "ci gerku ce'e re nanmu cu batci",
            "mi klama le zarci ce'e le briju pe'e je le zdani ce'e le ckule",
        ];

        for input in inputs {
            // Run JS parser
            let js_output = run_js_parser(input);
            let js_tree = parse_js_output(&js_output);
            
            // Run Rust parser
            let tokens = lexer::tokenize(input);
            let rust_ast = lojban_parser::text(&tokens);
            
            // For now, ensure both at least parse successfully
            assert!(rust_ast.is_ok(), "Rust parser failed for input: {}, error: {:?}", input, rust_ast.err());
            match js_tree {
                crate::oracle_parser::JSOutputNode::Node(_, children) => assert!(!children.is_empty()),
                _ => panic!("Expected JS node for input: {}", input),
            }
        }
    }
}
