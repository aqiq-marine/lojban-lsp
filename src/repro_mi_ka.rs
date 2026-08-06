use crate::cst_parser::{ParserOptions, parse};

#[test]
fn test_mi_ka_error() {
    let source = "mi ka";
    let parsed = parse(source, ParserOptions { recovery: true });

    println!("Errors for '{}': {:?}", source, parsed.errors);
    println!("CST for '{}': {:?}", source, parsed.syntax());
    assert!(
        !parsed.errors.is_empty(),
        "Should have reported errors for 'mi ka'"
    );
}

#[test]
fn test_mi_ka_klama_error() {
    let source = "mi ka klama";
    let parsed = parse(source, ParserOptions { recovery: true });

    println!("Errors for '{}': {:?}", source, parsed.errors);
    println!("CST for '{}': {:?}", source, parsed.syntax());
}
