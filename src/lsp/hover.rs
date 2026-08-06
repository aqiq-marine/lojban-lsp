use crate::{
    ast,
    lsp::dictionary::{Dictionary, Entry},
    lsp::document::Document,
};
pub fn hover(document: &Document, offset: u32, dictionary: &impl Dictionary) -> Option<Entry> {
    let token = ast::token_at(document.semantic_model().root(), offset)?;
    dictionary.lookup(token.text())
}
