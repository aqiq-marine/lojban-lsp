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
    if context.expected.is_empty() {
        return Vec::new();
    }
    dictionary.iter()
}

pub fn candidates_with_prefix(
    prefix: &str,
    expected: &[ExpectedRule],
    dictionary: &impl Dictionary,
) -> Vec<crate::lsp::dictionary::Entry> {
    let entries = dictionary.search_prefix(prefix);
    entries
        .into_iter()
        .filter(|entry| {
            expected.iter().any(|rule| match rule {
                ExpectedRule::Sumti => {
                    matches!(entry.category, crate::lsp::dictionary::WordCategory::Cmavo)
                }
                ExpectedRule::Selbri | ExpectedRule::Bridi => {
                    matches!(entry.category, crate::lsp::dictionary::WordCategory::Gismu)
                }
                ExpectedRule::Quote | ExpectedRule::Tag | ExpectedRule::EditingMarker => {
                    matches!(entry.category, crate::lsp::dictionary::WordCategory::Cmavo)
                }
                ExpectedRule::Sentence => true,
                _ => matches!(entry.category, crate::lsp::dictionary::WordCategory::Cmavo),
            })
        })
        .collect()
}
