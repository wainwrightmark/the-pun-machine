#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum SymbolType {
    UnstressedVowel,
    StressedVowel,
    Stop,
    Affricate,
    Aspirate,
    Fricative,
    Liquid,
    Nasal,
    Semivowel,
}

impl SymbolType {
    pub fn is_vowel(self) -> bool {
        matches!(
            self,
            SymbolType::UnstressedVowel | SymbolType::StressedVowel | SymbolType::Semivowel
        )
    }
}
