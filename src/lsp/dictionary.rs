#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub word: String,
    pub description: String,
}

pub trait Dictionary: Send + Sync {
    fn lookup(&self, word: &str) -> Option<Entry>;
}

#[derive(Default)]
pub struct EmptyDictionary;
impl Dictionary for EmptyDictionary {
    fn lookup(&self, _word: &str) -> Option<Entry> {
        None
    }
}
