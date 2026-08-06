#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WordCategory {
    Cmavo,
    Gismu,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Selmaho {
    KOhA,
    LE,
    LA,
    PU,
    BA,
    CU,
    NU,
    KA,
    SE,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub word: String,
    pub category: WordCategory,
    pub selmaho: Option<Selmaho>,
    pub description: String,
}

pub trait Dictionary: Send + Sync {
    fn lookup(&self, word: &str) -> Option<Entry>;
    fn iter(&self) -> Vec<Entry>;
    fn search_prefix(&self, prefix: &str) -> Vec<Entry> {
        let prefix = prefix.to_ascii_lowercase();
        self.iter()
            .into_iter()
            .filter(|entry| entry.word.starts_with(&prefix))
            .collect()
    }
}

/// A small built-in dictionary for the most common words.
#[derive(Default)]
pub struct BasicDictionary;

impl Dictionary for BasicDictionary {
    fn lookup(&self, word: &str) -> Option<Entry> {
        let (kind, description) = match word {
            "mi" => ("cmavo", "I / me"),
            "do" => ("cmavo", "you"),
            "ti" => ("cmavo", "this (thing)"),
            "ta" => ("cmavo", "that (thing)"),
            "zo'e" => ("cmavo", "unspecified value"),
            "le" => ("cmavo", "the one(s) described as ..."),
            "lo" => ("cmavo", "the thing(s) which really are ..."),
            "la" => ("cmavo", "the named ..."),
            "cu" => ("cmavo", "separator before the selbri"),
            "nu" => ("cmavo", "abstractor: event/state of ..."),
            "ka" => ("cmavo", "abstractor: property of ..."),
            "se" => ("cmavo", "swap the first two places"),
            "pu" => ("cmavo", "past tense marker"),
            "ba" => ("cmavo", "future tense marker"),
            "ca" => ("cmavo", "present tense marker"),
            "vi" => ("cmavo", "short-distance spatial marker"),
            "va" => ("cmavo", "medium-distance spatial marker"),
            "vu" => ("cmavo", "long-distance spatial marker"),
            "ki" => ("cmavo", "set tense/modal state"),
            "roi" => ("cmavo", "number of times"),
            "bai" => ("cmavo", "modal tag: compelled by"),
            "bau" => ("cmavo", "modal tag: in language"),
            "fi'o" => ("cmavo", "modal tag introducing a selbri"),
            "fe'e" => ("cmavo", "spatial modal tag"),
            "jai" => ("cmavo", "conversion/modal prefix"),
            "soi" => ("cmavo", "discursive free modifier"),
            "doi" => ("cmavo", "vocative marker"),
            "sa" => ("cmavo", "replace the preceding grammatical construct"),
            "si" => ("cmavo", "erase the preceding word"),
            "su" => ("cmavo", "erase the preceding statement"),
            "faho" => ("cmavo", "paragraph boundary marker"),
            "zo" => ("cmavo", "quote the following word"),
            "lu" => ("cmavo", "quote a grammatical text"),
            "lo'u" => ("cmavo", "quote an ungrammatical text"),
            "li'u" => ("cmavo", "close grammatical quotation"),
            "le'u" => ("cmavo", "close ungrammatical quotation"),
            "klama" => (
                "gismu",
                "go to / come to (x1) from (x2) via (x3) using (x4)",
            ),
            "dunda" => ("gismu", "give (x1) a gift (x2)"),
            "tavla" => ("gismu", "talk / speak to (x1) about (x2) in language (x3)"),
            "prami" => ("gismu", "love (x1)"),
            "gerku" => ("gismu", "dog"),
            "mlatu" => ("gismu", "cat"),
            "cukta" => ("gismu", "book"),
            "vecnu" => ("gismu", "sell (x1) to (x2) for price (x3)"),
            _ => return None,
        };
        Some(Entry {
            word: word.to_owned(),
            category: if kind == "cmavo" {
                WordCategory::Cmavo
            } else {
                WordCategory::Gismu
            },
            selmaho: match word {
                "mi" | "do" | "ti" | "ta" | "zo'e" => Some(Selmaho::KOhA),
                "le" | "lo" => Some(Selmaho::LE),
                "la" => Some(Selmaho::LA),
                "pu" | "ca" => Some(Selmaho::PU),
                "ba" => Some(Selmaho::BA),
                "cu" => Some(Selmaho::CU),
                "nu" => Some(Selmaho::NU),
                "ka" => Some(Selmaho::KA),
                "se" => Some(Selmaho::SE),
                _ => None,
            },
            description: format!("{kind}: {description}"),
        })
    }

    fn iter(&self) -> Vec<Entry> {
        [
            "mi", "do", "ti", "ta", "zo'e", "le", "lo", "la", "cu", "nu", "ka", "se", "pu", "ca",
            "ba", "vi", "va", "vu", "ki", "roi", "bai", "bau", "fi'o", "fe'e", "jai", "soi", "doi",
            "sa", "si", "su", "faho", "zo", "lu", "lo'u", "li'u", "le'u", "klama", "dunda",
            "tavla", "prami", "gerku", "mlatu", "cukta", "vecnu",
        ]
        .into_iter()
        .filter_map(|word| self.lookup(word))
        .collect()
    }
}

#[derive(Default)]
pub struct EmptyDictionary;
impl Dictionary for EmptyDictionary {
    fn lookup(&self, _word: &str) -> Option<Entry> {
        None
    }
    fn iter(&self) -> Vec<Entry> {
        Vec::new()
    }
}
