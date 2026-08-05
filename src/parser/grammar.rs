//! Grammar vocabulary shared by parser rule modules.

pub(crate) fn is_selbri_connective(text: &str) -> bool {
    matches!(text, "ja" | "je" | "jo" | "ju" | "jo'u" | "joi")
}
