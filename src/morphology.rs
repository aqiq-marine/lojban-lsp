// src/morphology.rs

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U')
}

pub fn is_consonant(c: char) -> bool {
    matches!(c, 'b' | 'c' | 'd' | 'f' | 'g' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'r' | 's' | 't' | 'v' | 'x' | 'z' |
                'B' | 'C' | 'D' | 'F' | 'G' | 'J' | 'K' | 'L' | 'M' | 'N' | 'P' | 'R' | 'S' | 'T' | 'V' | 'X' | 'Z')
}

pub fn is_cmene(s: &str) -> bool {
    // 固有名詞: 語末が子音であること
    s.chars().last().map_or(false, is_consonant)
}

pub fn is_gismu(s: &str) -> bool {
    // gismu: 5文字、CVCCV または CCVCV
    if s.len() != 5 { return false; }
    let chars: Vec<char> = s.chars().collect();
    let is_v = |i| is_vowel(chars[i]);
    let is_c = |i| is_consonant(chars[i]);

    (is_c(0) && is_c(1) && is_v(2) && is_c(3) && is_v(4)) || 
    (is_c(0) && is_v(1) && is_c(2) && is_c(3) && is_v(4))
}

pub fn is_cmavo(s: &str) -> bool {
    // cmavo: 語末が母音、かつ子音連続を含まない(単純化)
    s.chars().last().map_or(false, is_vowel) && !s.contains(|c: char| is_consonant(c)) // 実際はもっと複雑だが第一近似
}
