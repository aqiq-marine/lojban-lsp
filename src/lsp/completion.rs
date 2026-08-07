use crate::{
    features::{CompletionContext, CompletionMode, ExpectedRule},
    lsp::dictionary::{Dictionary, WordKind},
};

pub type CompletionItem = tower_lsp::lsp_types::CompletionItem;

pub trait CompletionProvider: Send + Sync {
    fn complete(&self, context: &CompletionContext<'_>) -> Vec<CompletionItem>;
}

pub struct DictionaryCompletionProvider<D> { pub dictionary: D }

impl<D> DictionaryCompletionProvider<D> {
    pub fn new(dictionary: D) -> Self { Self { dictionary } }
}

impl<D: Dictionary> CompletionProvider for DictionaryCompletionProvider<D> {
    fn complete(&self, context: &CompletionContext<'_>) -> Vec<CompletionItem> {
        let entries = if matches!(context.mode, CompletionMode::Automatic) {
            self.dictionary.search_prefix(&context.prefix)
        } else if context.prefix.is_empty() {
            self.dictionary.iter()
        } else {
            self.dictionary.search_prefix(&context.prefix)
        };
        entries.into_iter().filter(|e| allowed(e, &context.expected)).map(|e| {
            CompletionItem {
                label: e.word.clone(),
                detail: Some(e.description.clone()),
                documentation: Some(tower_lsp::lsp_types::Documentation::String(e.description)),
                kind: Some(if matches!(e.kind, WordKind::Brivla { .. }) { tower_lsp::lsp_types::CompletionItemKind::FUNCTION } else { tower_lsp::lsp_types::CompletionItemKind::KEYWORD }),
                filter_text: Some(e.word.clone()),
                sort_text: Some(format!("{}-{}", if matches!(context.mode, CompletionMode::Automatic) { "0" } else { "1" }, e.word)),
                ..Default::default()
            }
        }).collect()
    }
}

fn allowed(entry: &crate::lsp::dictionary::Entry, expected: &[ExpectedRule]) -> bool {
    expected.iter().any(|rule| match rule {
        ExpectedRule::Sumti => matches!(entry.kind, WordKind::Cmavo { .. }),
        ExpectedRule::Selbri | ExpectedRule::Bridi => matches!(entry.kind, WordKind::Brivla { .. }),
        ExpectedRule::Sentence => true,
        _ => matches!(entry.kind, WordKind::Cmavo { .. }),
    })
}

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
                    matches!(entry.kind, WordKind::Cmavo { .. })
                }
                ExpectedRule::Selbri | ExpectedRule::Bridi => {
                    matches!(entry.kind, WordKind::Brivla { .. })
                }
                ExpectedRule::Quote | ExpectedRule::Tag | ExpectedRule::EditingMarker => {
                    matches!(entry.kind, WordKind::Cmavo { .. })
                }
                ExpectedRule::Sentence => true,
                _ => matches!(entry.kind, WordKind::Cmavo { .. }),
            })
        })
        .collect()
}
