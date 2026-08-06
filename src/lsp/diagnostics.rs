use crate::{cst::Parse, features::Diagnostic};
pub fn diagnostics(parse: &Parse) -> Vec<Diagnostic> {
    parse.diagnostics()
}
