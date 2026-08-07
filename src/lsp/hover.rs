use crate::{
    lsp::dictionary::{BrivlaKind, Dictionary, WordKind},
    lsp::document::Document,
    semantic::LexicalKind,
    morphology,
};

pub fn hover(document: &Document, offset: u32, dictionary: &impl Dictionary) -> Option<String> {
    let semantic = document.semantic_model();
    let source = document.source();
    let tokens = semantic.lexical_tokens(source, dictionary);
    
    let token_info = tokens.iter().find(|t| {
        let start: u32 = t.range.start().into();
        let end: u32 = t.range.end().into();
        start <= offset && offset < end
    })?;

    let word = &source[t_range(token_info.range)];
    let entry = dictionary.lookup(word);
    
    let mut lines = Vec::new();


    match token_info.lexical_kind {
        LexicalKind::Brivla => {
            lines.push("**Brivla**".to_string());
    
            if morphology::is_gismu(word) {
                lines.push("Kind: Gismu".to_string());
            } else if morphology::is_lujvo(word) {
                lines.push("Kind: Lujvo".to_string());
    
                let decomp = morphology::split_lujvo(word)
                    .map(|rafsi| rafsi.join(" + "))
                    .unwrap_or_else(|| "(unknown)".to_string());
    
                lines.push(format!("Decomposition: {}", decomp));
            } else if morphology::is_fuhivla(word) {
                lines.push("Kind: Fuhivla".to_string());
            }
        }
    
        LexicalKind::Cmavo => {
            lines.push("**Cmavo**".to_string());
        }
    
        LexicalKind::Cmene => {
            lines.push("**Cmene**".to_string());
        }
    
        LexicalKind::Invalid => {
            lines.push("**Invalid**".to_string());
        }
    }
    
    if let Some(entry) = entry {
        match &entry.kind {
            WordKind::Cmavo { selmaho } => {
                if let Some(selmaho) = selmaho.first() {
                    lines.push(format!("Selmaho: {}", selmaho.to_str()));
                }
            }
            WordKind::Brivla { arity, .. } => {
                lines.push(format!("Arity: {}", arity));
            }
            WordKind::Cmevla => {}
        }
    
        lines.push(format!("Description: {}", entry.description));
    } else {
        lines.push("Description: Unknown".to_string());
    }


    Some(lines.join("\n\n"))
}

fn t_range(range: rowan::TextRange) -> std::ops::Range<usize> {
    range.start().into()..range.end().into()
}
