use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=dictionary.csv");
    let csv = fs::read_to_string("dictionary.csv").expect("read dictionary.csv");
    let mut generated = String::from("static WORDS: phf::Map<&'static str, (WordKind, &'static str)> = phf::phf_map! {\n");
    for (line_no, line) in csv.lines().enumerate().skip(1) {
        let fields: Vec<_> = line.splitn(6, ',').collect();
        if fields.len() != 4 && fields.len() != 5 && fields.len() != 6 { panic!("dictionary.csv:{}: expected 6 fields", line_no + 1); }
        let word = fields[0].trim();
        let kind = match fields[1].trim() {
            "cmavo" => format!("WordKind::Cmavo {{ selmaho: &[{}] }}", fields[2].split('|').filter(|s| !s.is_empty()).map(|s| format!("Selmaho::{s}")).collect::<Vec<_>>().join(", ")),
            "brivla" | "gismu" | "lujvo" | "fuhivla" => {
                let (brivla_kind_name, arity) = if fields.len() == 6 { (fields[3].trim(), fields[4]) } else { (fields[1].trim(), if fields.len() == 5 { fields[3] } else { "0" }) };
                let brivla_kind = match brivla_kind_name {
                    "gismu" => "BrivlaKind::Gismu",
                    "lujvo" => "BrivlaKind::Lujvo",
                    "fuhivla" => "BrivlaKind::Fuhivla",
                    _ => "BrivlaKind::Gismu",
                };
                format!("WordKind::Brivla {{ kind: {brivla_kind}, arity: {} }}", arity.parse::<u8>().expect("brivla arity must be an integer"))
            }
            other => panic!("dictionary.csv:{}: unknown kind {other}", line_no + 1),
        };
        let description = if fields.len() == 6 { fields[5] } else if fields.len() == 5 { fields[4] } else { fields[3] };
        generated.push_str(&format!("    {:?} => ({kind}, {:?}),\n", word, description));
    }
    generated.push_str("};\n");
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("dictionary_words.rs");
    fs::write(out, generated).expect("write generated dictionary");
}
