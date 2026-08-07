
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WordKind {
    Cmavo { selmaho: &'static [Selmaho] },
    Brivla { kind: BrivlaKind, arity: u8 },
    Cmevla,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrivlaKind {
    Gismu,
    Lujvo,
    Fuhivla,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Selmaho {
    A,
    BAI,
    BAhE,
    BE,
    BEI,
    BEhO,
    BIhE,
    BIhI,
    BO,
    BOI,
    BU,
    BY,
    CAI,
    CAhA,
    CEI,
    CEhE,
    CO,
    COI,
    CU,
    CUhE,
    DAhO,
    DOI,
    DOhU,
    FA,
    FAhA,
    FAhO,
    FEhE,
    FEhU,
    FIhO,
    FOI,
    FUhA,
    FUhE,
    FUhO,
    GA,
    GAhO,
    GEhU,
    GI,
    GIhA,
    GOI,
    GOhA,
    GUhA,
    I,
    JA,
    JAI,
    JOI,
    JOhI,
    KE,
    KEI,
    KEhE,
    KI,
    KOhA,
    KU,
    KUhE,
    KUhO,
    LA,
    LAU,
    LAhE,
    LE,
    LEhU,
    LI,
    LIhU,
    LOhO,
    LOhU,
    LU,
    LUhU,
    MAI,
    MAhO,
    ME,
    MEhU,
    MOI,
    MOhE,
    MOhI,
    NA,
    NAI,
    NAhE,
    NAhU,
    NIhE,
    NIhO,
    NOI,
    NU,
    NUhA,
    NUhI,
    NUhU,
    PA,
    PEhE,
    PEhO,
    PU,
    RAhO,
    ROI,
    SA,
    SE,
    SEI,
    SEhU,
    SI,
    SOI,
    SU,
    TAhE,
    TEI,
    TEhU,
    TO,
    TOI,
    TUhE,
    TUhU,
    UI,
    VA,
    VAU,
    VEI,
    VEhA,
    VEhO,
    VIhA,
    VUhO,
    VUhU,
    XI,
    Y,
    ZAhO,
    ZEI,
    ZEhA,
    ZI,
    ZIhE,
    ZO,
    ZOI,
    ZOhU,
}

impl Selmaho {
    pub fn all() -> &'static [Selmaho] {
        &[
            Selmaho::A, Selmaho::BAI, Selmaho::BAhE, Selmaho::BE, Selmaho::BEI, Selmaho::BEhO,
            Selmaho::BIhE, Selmaho::BIhI, Selmaho::BO, Selmaho::BOI, Selmaho::BU, Selmaho::BY,
            Selmaho::CAI, Selmaho::CAhA, Selmaho::CEI, Selmaho::CEhE, Selmaho::CO, Selmaho::COI,
            Selmaho::CU, Selmaho::CUhE, Selmaho::DAhO, Selmaho::DOI, Selmaho::DOhU, Selmaho::FA,
            Selmaho::FAhA, Selmaho::FAhO, Selmaho::FEhE, Selmaho::FEhU, Selmaho::FIhO,
            Selmaho::FOI, Selmaho::FUhA, Selmaho::FUhE, Selmaho::FUhO, Selmaho::GA,
            Selmaho::GAhO, Selmaho::GEhU, Selmaho::GI, Selmaho::GIhA, Selmaho::GOI,
            Selmaho::GOhA, Selmaho::GUhA, Selmaho::I, Selmaho::JA, Selmaho::JAI, Selmaho::JOI,
            Selmaho::JOhI, Selmaho::KE, Selmaho::KEI, Selmaho::KEhE, Selmaho::KI,
            Selmaho::KOhA, Selmaho::KU, Selmaho::KUhE, Selmaho::KUhO, Selmaho::LA,
            Selmaho::LAU, Selmaho::LAhE, Selmaho::LE, Selmaho::LEhU, Selmaho::LI,
            Selmaho::LIhU, Selmaho::LOhO, Selmaho::LOhU, Selmaho::LU, Selmaho::LUhU,
            Selmaho::MAI, Selmaho::MAhO, Selmaho::ME, Selmaho::MEhU, Selmaho::MOI,
            Selmaho::MOhE, Selmaho::MOhI, Selmaho::NA, Selmaho::NAI, Selmaho::NAhE,
            Selmaho::NAhU, Selmaho::NIhE, Selmaho::NIhO, Selmaho::NOI, Selmaho::NU,
            Selmaho::NUhA, Selmaho::NUhI, Selmaho::NUhU, Selmaho::PA, Selmaho::PEhE,
            Selmaho::PEhO, Selmaho::PU, Selmaho::RAhO, Selmaho::ROI, Selmaho::SA,
            Selmaho::SE, Selmaho::SEI, Selmaho::SEhU, Selmaho::SI, Selmaho::SOI,
            Selmaho::SU, Selmaho::TAhE, Selmaho::TEI, Selmaho::TEhU, Selmaho::TO,
            Selmaho::TOI, Selmaho::TUhE, Selmaho::TUhU, Selmaho::UI, Selmaho::VA,
            Selmaho::VAU, Selmaho::VEI, Selmaho::VEhA, Selmaho::VEhO, Selmaho::VIhA,
            Selmaho::VUhO, Selmaho::VUhU, Selmaho::XI, Selmaho::Y, Selmaho::ZAhO,
            Selmaho::ZEI, Selmaho::ZEhA, Selmaho::ZI, Selmaho::ZIhE, Selmaho::ZO,
            Selmaho::ZOI, Selmaho::ZOhU,
        ]
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Selmaho::A => "a", Selmaho::BAI => "bai", Selmaho::BAhE => "bahe",
            Selmaho::BE => "be", Selmaho::BEI => "bei", Selmaho::BEhO => "beho",
            Selmaho::BIhE => "bihe", Selmaho::BIhI => "bihi", Selmaho::BO => "bo",
            Selmaho::BOI => "boi", Selmaho::BU => "bu", Selmaho::BY => "by",
            Selmaho::CAI => "cai", Selmaho::CAhA => "caha", Selmaho::CEI => "cei",
            Selmaho::CEhE => "cehe", Selmaho::CO => "co", Selmaho::COI => "coi",
            Selmaho::CU => "cu", Selmaho::CUhE => "cuhe", Selmaho::DAhO => "daho",
            Selmaho::DOI => "doi", Selmaho::DOhU => "dohu", Selmaho::FA => "fa",
            Selmaho::FAhA => "faha", Selmaho::FAhO => "faho", Selmaho::FEhE => "fehe",
            Selmaho::FEhU => "fehu", Selmaho::FIhO => "fiho", Selmaho::FOI => "foi",
            Selmaho::FUhA => "fuha", Selmaho::FUhE => "fuhe", Selmaho::FUhO => "fuho",
            Selmaho::GA => "ga", Selmaho::GAhO => "gaho", Selmaho::GEhU => "gehu",
            Selmaho::GI => "gi", Selmaho::GIhA => "giha", Selmaho::GOI => "goi",
            Selmaho::GOhA => "goha", Selmaho::GUhA => "guha", Selmaho::I => "i",
            Selmaho::JA => "ja", Selmaho::JAI => "jai", Selmaho::JOI => "joi",
            Selmaho::JOhI => "johi", Selmaho::KE => "ke", Selmaho::KEI => "kei",
            Selmaho::KEhE => "kehe", Selmaho::KI => "ki", Selmaho::KOhA => "koha",
            Selmaho::KU => "ku", Selmaho::KUhE => "kuhe", Selmaho::KUhO => "kuho",
            Selmaho::LA => "la", Selmaho::LAU => "lau", Selmaho::LAhE => "lahe",
            Selmaho::LE => "le", Selmaho::LEhU => "lehu", Selmaho::LI => "li",
            Selmaho::LIhU => "lihu", Selmaho::LOhO => "loho", Selmaho::LOhU => "lohu",
            Selmaho::LU => "lu", Selmaho::LUhU => "luhu", Selmaho::MAI => "mai",
            Selmaho::MAhO => "maho", Selmaho::ME => "me", Selmaho::MEhU => "mehu",
            Selmaho::MOI => "moi", Selmaho::MOhE => "mohe", Selmaho::MOhI => "mohi",
            Selmaho::NA => "na", Selmaho::NAI => "nai", Selmaho::NAhE => "nahe",
            Selmaho::NAhU => "nahu", Selmaho::NIhE => "nihe", Selmaho::NIhO => "niho",
            Selmaho::NOI => "noi", Selmaho::NU => "nu", Selmaho::NUhA => "nuha",
            Selmaho::NUhI => "nuhi", Selmaho::NUhU => "nuhu", Selmaho::PA => "pa",
            Selmaho::PEhE => "pehe", Selmaho::PEhO => "peho", Selmaho::PU => "pu",
            Selmaho::RAhO => "raho", Selmaho::ROI => "roi", Selmaho::SA => "sa",
            Selmaho::SE => "se", Selmaho::SEI => "sei", Selmaho::SEhU => "sehu",
            Selmaho::SI => "si", Selmaho::SOI => "soi", Selmaho::SU => "su",
            Selmaho::TAhE => "tahe", Selmaho::TEI => "tei", Selmaho::TEhU => "tehu",
            Selmaho::TO => "to", Selmaho::TOI => "toi", Selmaho::TUhE => "tuhe",
            Selmaho::TUhU => "tuhu", Selmaho::UI => "ui", Selmaho::VA => "va",
            Selmaho::VAU => "vau", Selmaho::VEI => "vei", Selmaho::VEhA => "veha",
            Selmaho::VEhO => "veho", Selmaho::VIhA => "viha", Selmaho::VUhO => "vuho",
            Selmaho::VUhU => "vuhu", Selmaho::XI => "xi", Selmaho::Y => "y",
            Selmaho::ZAhO => "zaho", Selmaho::ZEI => "zei", Selmaho::ZEhA => "zeha",
            Selmaho::ZI => "zi", Selmaho::ZIhE => "zihe", Selmaho::ZO => "zo",
            Selmaho::ZOI => "zoi", Selmaho::ZOhU => "zohu",
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Selmaho::A => 0, Selmaho::BAI => 1, Selmaho::BAhE => 2,
            Selmaho::BE => 3, Selmaho::BEI => 4, Selmaho::BEhO => 5,
            Selmaho::BIhE => 6, Selmaho::BIhI => 7, Selmaho::BO => 8,
            Selmaho::BOI => 9, Selmaho::BU => 10, Selmaho::BY => 11,
            Selmaho::CAI => 12, Selmaho::CAhA => 13, Selmaho::CEI => 14,
            Selmaho::CEhE => 15, Selmaho::CO => 16, Selmaho::COI => 17,
            Selmaho::CU => 18, Selmaho::CUhE => 19, Selmaho::DAhO => 20,
            Selmaho::DOI => 21, Selmaho::DOhU => 22, Selmaho::FA => 23,
            Selmaho::FAhA => 24, Selmaho::FAhO => 25, Selmaho::FEhE => 26,
            Selmaho::FEhU => 27, Selmaho::FIhO => 28, Selmaho::FOI => 29,
            Selmaho::FUhA => 30, Selmaho::FUhE => 31, Selmaho::FUhO => 32,
            Selmaho::GA => 33, Selmaho::GAhO => 34, Selmaho::GEhU => 35,
            Selmaho::GI => 36, Selmaho::GIhA => 37, Selmaho::GOI => 38,
            Selmaho::GOhA => 39, Selmaho::GUhA => 40, Selmaho::I => 41,
            Selmaho::JA => 42, Selmaho::JAI => 43, Selmaho::JOI => 44,
            Selmaho::JOhI => 45, Selmaho::KE => 46, Selmaho::KEI => 47,
            Selmaho::KEhE => 48, Selmaho::KI => 49, Selmaho::KOhA => 50,
            Selmaho::KU => 51, Selmaho::KUhE => 52, Selmaho::KUhO => 53,
            Selmaho::LA => 54, Selmaho::LAU => 55, Selmaho::LAhE => 56,
            Selmaho::LE => 57, Selmaho::LEhU => 58, Selmaho::LI => 59,
            Selmaho::LIhU => 60, Selmaho::LOhO => 61, Selmaho::LOhU => 62,
            Selmaho::LU => 63, Selmaho::LUhU => 64, Selmaho::MAI => 65,
            Selmaho::MAhO => 66, Selmaho::ME => 67, Selmaho::MEhU => 68,
            Selmaho::MOI => 69, Selmaho::MOhE => 70, Selmaho::MOhI => 71,
            Selmaho::NA => 72, Selmaho::NAI => 73, Selmaho::NAhE => 74,
            Selmaho::NAhU => 75, Selmaho::NIhE => 76, Selmaho::NIhO => 77,
            Selmaho::NOI => 78, Selmaho::NU => 79, Selmaho::NUhA => 80,
            Selmaho::NUhI => 81, Selmaho::NUhU => 82, Selmaho::PA => 83,
            Selmaho::PEhE => 84, Selmaho::PEhO => 85, Selmaho::PU => 86,
            Selmaho::RAhO => 87, Selmaho::ROI => 88, Selmaho::SA => 89,
            Selmaho::SE => 90, Selmaho::SEI => 91, Selmaho::SEhU => 92,
            Selmaho::SI => 93, Selmaho::SOI => 94, Selmaho::SU => 95,
            Selmaho::TAhE => 96, Selmaho::TEI => 97, Selmaho::TEhU => 98,
            Selmaho::TO => 99, Selmaho::TOI => 100, Selmaho::TUhE => 101,
            Selmaho::TUhU => 102, Selmaho::UI => 103, Selmaho::VA => 104,
            Selmaho::VAU => 105, Selmaho::VEI => 106, Selmaho::VEhA => 107,
            Selmaho::VEhO => 108, Selmaho::VIhA => 109, Selmaho::VUhO => 110,
            Selmaho::VUhU => 111, Selmaho::XI => 112, Selmaho::Y => 113,
            Selmaho::ZAhO => 114, Selmaho::ZEI => 115, Selmaho::ZEhA => 116,
            Selmaho::ZI => 117, Selmaho::ZIhE => 118, Selmaho::ZO => 119,
            Selmaho::ZOI => 120, Selmaho::ZOhU => 121,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub word: String,
    pub kind: WordKind,
    pub description: String,
}

pub trait Dictionary: Send + Sync {
    fn lookup(&self, word: &str) -> Option<Entry>;
    fn lookup_by_rafsi(&self, rafsi: &str) -> Option<Entry>;
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

#[allow(dead_code)]
static LEGACY_WORDS: phf::Map<&'static str, (WordKind, &'static str)> = phf::phf_map! {
    "mi" => (WordKind::Cmavo { selmaho: &[Selmaho::KOhA] }, "I / me"),
    "do" => (WordKind::Cmavo { selmaho: &[Selmaho::KOhA] }, "you"),
    "ti" => (WordKind::Cmavo { selmaho: &[Selmaho::KOhA] }, "this (thing)"),
    "ta" => (WordKind::Cmavo { selmaho: &[Selmaho::KOhA] }, "that (thing)"),
    "zo'e" => (WordKind::Cmavo { selmaho: &[Selmaho::KOhA] }, "unspecified value"),

    "le" => (WordKind::Cmavo { selmaho: &[Selmaho::LE] }, "the one(s) described as ..."),
    "lo" => (WordKind::Cmavo { selmaho: &[Selmaho::LE] }, "the thing(s) which really are ..."),
    "la" => (WordKind::Cmavo { selmaho: &[Selmaho::LA] }, "the named ..."),

    "cu" => (WordKind::Cmavo { selmaho: &[Selmaho::CU] }, "separator before the selbri"),

    "nu" => (WordKind::Cmavo { selmaho: &[Selmaho::NU] }, "abstractor: event/state of ..."),
    "ka" => (WordKind::Cmavo { selmaho: &[] }, "abstractor: property of ..."),

    "se" => (WordKind::Cmavo { selmaho: &[Selmaho::SE] }, "swap the first two places"),

    "pu" => (WordKind::Cmavo { selmaho: &[Selmaho::PU] }, "past tense marker"),
    "ba" => (WordKind::Cmavo { selmaho: &[Selmaho::PU] }, "future tense marker"),
    "ca" => (WordKind::Cmavo { selmaho: &[Selmaho::PU] }, "present tense marker"),

    "vi" => (WordKind::Cmavo { selmaho: &[Selmaho::VA] }, "short-distance spatial marker"),
    "va" => (WordKind::Cmavo { selmaho: &[Selmaho::VA] }, "medium-distance spatial marker"),
    "vu" => (WordKind::Cmavo { selmaho: &[Selmaho::VA] }, "long-distance spatial marker"),

    "ki" => (WordKind::Cmavo { selmaho: &[Selmaho::KI] }, "set tense/modal state"),
    "roi" => (WordKind::Cmavo { selmaho: &[Selmaho::ROI] }, "number of times"),

    "bai" => (WordKind::Cmavo { selmaho: &[Selmaho::BAI] }, "modal tag: compelled by"),
    "bau" => (WordKind::Cmavo { selmaho: &[Selmaho::BAI] }, "modal tag: in language"),

    "fi'o" => (WordKind::Cmavo { selmaho: &[Selmaho::FIhO] }, "modal tag introducing a selbri"),
    "fe'e" => (WordKind::Cmavo { selmaho: &[Selmaho::FEhE] }, "spatial modal tag"),

    "jai" => (WordKind::Cmavo { selmaho: &[Selmaho::JAI] }, "conversion/modal prefix"),
    "soi" => (WordKind::Cmavo { selmaho: &[Selmaho::SOI] }, "reciprocal marker"),

    "doi" => (WordKind::Cmavo { selmaho: &[Selmaho::DOI] }, "vocative marker"),
    "sa" => (WordKind::Cmavo { selmaho: &[Selmaho::SA] }, "replace the preceding grammatical construct"),
    "si" => (WordKind::Cmavo { selmaho: &[Selmaho::SI] }, "erase the preceding word"),
    "su" => (WordKind::Cmavo { selmaho: &[Selmaho::SU] }, "erase the preceding statement"),

    "fa'o" => (WordKind::Cmavo { selmaho: &[Selmaho::FAhO] }, "end of text"),

    "zo" => (WordKind::Cmavo { selmaho: &[Selmaho::ZO] }, "quote the following word"),
    "lu" => (WordKind::Cmavo { selmaho: &[Selmaho::LU] }, "quote a grammatical text"),
    "lo'u" => (WordKind::Cmavo { selmaho: &[Selmaho::LOhU] }, "quote an ungrammatical text"),
    "li'u" => (WordKind::Cmavo { selmaho: &[Selmaho::LIhU] }, "close grammatical quotation"),
    "le'u" => (WordKind::Cmavo { selmaho: &[Selmaho::LEhU] }, "close ungrammatical quotation"),

    "klama" => (WordKind::Brivla { kind: BrivlaKind::Gismu, arity: 4 }, "go to / come to (x1) from (x2) via (x3) using (x4)"),
    "dunda" => (WordKind::Brivla { kind: BrivlaKind::Gismu, arity: 2 }, "give (x1) a gift (x2)"),
    "tavla" => (WordKind::Brivla { kind: BrivlaKind::Gismu, arity: 3 }, "talk / speak to (x1) about (x2) in language (x3)"),
    "prami" => (WordKind::Brivla { kind: BrivlaKind::Gismu, arity: 1 }, "love (x1)"),
    "gerku" => (WordKind::Brivla { kind: BrivlaKind::Gismu, arity: 1 }, "dog"),
    "mlatu" => (WordKind::Brivla { kind: BrivlaKind::Gismu, arity: 1 }, "cat"),
    "cukta" => (WordKind::Brivla { kind: BrivlaKind::Gismu, arity: 1 }, "book"),
    "vecnu" => (WordKind::Brivla { kind: BrivlaKind::Gismu, arity: 3 }, "sell (x1) to (x2) for price (x3)"),
};

include!(concat!(env!("OUT_DIR"), "/dictionary_words.rs"));

impl Dictionary for BasicDictionary {
    fn lookup(&self, word: &str) -> Option<Entry> {
        let (kind, description) = WORDS.get(word)?;

        Some(Entry {
            word: word.to_owned(),
            kind: kind.clone(),
            description: (*description).to_owned(),
        })
    }

    fn lookup_by_rafsi(&self, rafsi: &str) -> Option<Entry> {
        self.iter().into_iter().find(|entry| {
            if let WordKind::Brivla { kind: BrivlaKind::Gismu, .. } = &entry.kind {
                // This is a simplified check.
                // In a real dictionary, you'd have a mapping of rafsi to gismu.
                // For now, assume the first 3 letters are a valid rafsi.
                entry.word.starts_with(rafsi) || entry.word.get(..3) == Some(rafsi)
            } else {
                false
            }
        })
    }

    fn iter(&self) -> Vec<Entry> {
        WORDS
            .entries()
            .map(|(word, (kind, description))| Entry {
                word: (*word).to_owned(),
                kind: kind.clone(),
                description: (*description).to_owned(),
            })
            .collect()
    }
}

#[derive(Default)]
pub struct EmptyDictionary;
impl Dictionary for EmptyDictionary {
    fn lookup(&self, _word: &str) -> Option<Entry> {
        None
    }
    fn lookup_by_rafsi(&self, _rafsi: &str) -> Option<Entry> {
        None
    }
    fn iter(&self) -> Vec<Entry> {
        Vec::new()
    }
}
