use crate::cst_parser::{ParserOptions, parse};
use crate::semantic::{SemanticModel, SemanticTokenInfo};
use crate::lsp::dictionary::BasicDictionary;
use crate::semantic::DiagnosticKind;

#[test]
fn test_i_mi_klama_semantic_tokens() {
    let source = ".i mi klama";
    let parsed = parse(source, ParserOptions { recovery: false });
    let semantic = SemanticModel::new(parsed.syntax());
    let tokens = semantic.lexical_tokens(source, &BasicDictionary);

    println!("Tokens: {:?}", tokens);
    
    // Check if any token is marked as InvalidWord
    let invalid_tokens: Vec<_> = tokens.iter()
        .filter(|t| t.diagnostic_kind == Some(DiagnosticKind::InvalidWord))
        .collect();
        
    assert!(invalid_tokens.is_empty(), "Should not have InvalidWord tokens, found: {:?}", invalid_tokens);
}

#[test]
fn test_i_mi_klama_success() {
    let source = ".i mi klama";
    let parsed = parse(source, ParserOptions { recovery: false });

    println!("Errors for '{}': {:?}", source, parsed.errors);
    assert!(
        parsed.errors.is_empty(),
        "Should not have reported errors for '.i mi klama'"
    );
}

#[test]
fn test_crlf_sentence_parses_successfully() {
    let source = ".i mi klama\r\n.i mi pu klama le zarci";
    let parsed = parse(source, ParserOptions { recovery: false });

    println!("Errors for '{}': {:?}", source, parsed.errors);
    assert!(
        parsed.errors.is_empty(),
        "Should not have reported errors for '.i mi klama\\r\\n.i mi pu klama le zarci'"
    );
}

