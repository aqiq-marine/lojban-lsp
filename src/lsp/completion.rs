use crate::{
    features::{CompletionContext, ExpectedRule},
    lsp::dictionary::Dictionary,
};

pub fn expected_rules<'a>(context: &'a CompletionContext<'a>) -> &'a [ExpectedRule] {
    &context.expected
}
pub fn candidates(
    context: &CompletionContext<'_>,
    dictionary: &impl Dictionary,
) -> Vec<crate::lsp::dictionary::Entry> {
    context
        .expected
        .iter()
        .filter_map(|_| dictionary.lookup(""))
        .collect()
}
