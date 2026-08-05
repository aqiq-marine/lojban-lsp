//! Parser-facing module boundary.
//!
//! The implementation is kept behind this boundary so grammar-rule modules can
//! be moved here incrementally without changing the crate's public API.

pub(crate) mod grammar;
pub(crate) mod recovery;
pub(crate) mod tokens;

pub use crate::cst_parser::{ParserOptions, parse};

#[cfg(test)]
mod tests {
    use super::{ParserOptions, parse};

    #[test]
    fn public_parser_boundary_builds_a_tree() {
        let parsed = parse("mi klama", ParserOptions::default());
        assert!(!parsed.syntax().to_string().is_empty());
    }
}
