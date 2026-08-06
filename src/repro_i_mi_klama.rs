use crate::cst_parser::{ParserOptions, parse};

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
