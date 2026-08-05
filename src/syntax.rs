#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    // Terminals
    Whitespace = 0,
    
    // Non-terminals
    Text,
    Paragraph,
    Statement,
    Sentence,
    Syllable,
    ConsonantCluster,
    Sumti,
    Selbri,
    Brivla,
    Gismu,
    Lujvo,
    Fuhivla,
    Negation,
    Tag,
    Quoting,
    Abstractor,
    Cmavo,
    FreeModifier,
    Prenex,
    Cmene,
    Bridi,
    
    // Errors
    Error,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

// Define the language for rowan
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LojbanLanguage {}
impl rowan::Language for LojbanLanguage {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        assert!(raw.0 <= SyntaxKind::Error as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        kind.into()
    }
}
