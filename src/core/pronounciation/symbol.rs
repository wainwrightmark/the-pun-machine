use strum::{Display, EnumString};

use super::prelude::SymbolType;

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    EnumString,
    Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Symbol {
    AA,
    AA0,
    AA1,
    AA2,
    AE,
    AE0,
    AE1,
    AE2,
    AH,
    AH0,
    AH1,
    AH2,
    AO,
    AO0,
    AO1,
    AO2,
    AW,
    AW0,
    AW1,
    AW2,
    AY,
    AY0,
    AY1,
    AY2,
    B,
    CH,
    D,
    DH,
    EH,
    EH0,
    EH1,
    EH2,
    ER,
    ER0,
    ER1,
    ER2,
    EY,
    EY0,
    EY1,
    EY2,
    F,
    G,
    HH,
    IH,
    IH0,
    IH1,
    IH2,
    IY,
    IY0,
    IY1,
    IY2,
    JH,
    K,
    L,
    M,
    N,
    NG,
    OW,
    OW0,
    OW1,
    OW2,
    OY,
    OY0,
    OY1,
    OY2,
    P,
    R,
    S,
    SH,
    T,
    TH,
    UH,
    UH0,
    UH1,
    UH2,
    UW,
    UW0,
    UW1,
    UW2,
    V,
    W,
    Y,
    Z,
    ZH,
}

impl Symbol {
    pub fn is_vowel(self) -> bool {
        self.symbol_type().is_vowel()
    }

    pub fn is_stressed_vowel(self) -> bool {
        self.symbol_type() == SymbolType::StressedVowel
    }

    pub fn normalize_vowel(self) -> Self {
        match self {
            Symbol::AA0 => Symbol::AA,
            Symbol::AA1 => Symbol::AA,
            Symbol::AA2 => Symbol::AA,

            Symbol::AE0 => Symbol::AE,
            Symbol::AE1 => Symbol::AE,
            Symbol::AE2 => Symbol::AE,

            Symbol::AH0 => Symbol::AH,
            Symbol::AH1 => Symbol::AH,
            Symbol::AH2 => Symbol::AH,

            Symbol::AO0 => Symbol::AO,
            Symbol::AO1 => Symbol::AO,
            Symbol::AO2 => Symbol::AO,

            Symbol::AW0 => Symbol::AW,
            Symbol::AW1 => Symbol::AW,
            Symbol::AW2 => Symbol::AW,

            Symbol::AY0 => Symbol::AY,
            Symbol::AY1 => Symbol::AY,
            Symbol::AY2 => Symbol::AY,

            Symbol::EH0 => Symbol::EH,
            Symbol::EH1 => Symbol::EH,
            Symbol::EH2 => Symbol::EH,

            Symbol::ER0 => Symbol::ER,
            Symbol::ER1 => Symbol::ER,
            Symbol::ER2 => Symbol::ER,

            Symbol::EY0 => Symbol::EY,
            Symbol::EY1 => Symbol::EY,
            Symbol::EY2 => Symbol::EY,

            Symbol::IH0 => Symbol::IH,
            Symbol::IH1 => Symbol::IH,
            Symbol::IH2 => Symbol::IH,

            Symbol::IY0 => Symbol::IY,
            Symbol::IY1 => Symbol::IY,
            Symbol::IY2 => Symbol::IY,

            Symbol::OW0 => Symbol::OW,
            Symbol::OW1 => Symbol::OW,
            Symbol::OW2 => Symbol::OW,

            Symbol::OY0 => Symbol::OY,
            Symbol::OY1 => Symbol::OY,
            Symbol::OY2 => Symbol::OY,

            Symbol::UH0 => Symbol::UH,
            Symbol::UH1 => Symbol::UH,
            Symbol::UH2 => Symbol::UH,

            Symbol::UW0 => Symbol::UW,
            Symbol::UW1 => Symbol::UW,
            Symbol::UW2 => Symbol::UW,
            _ => self,
        }
    }

    pub fn symbol_type(self) -> SymbolType {
        match self {
            Symbol::AA => SymbolType::UnstressedVowel,
            Symbol::AE => SymbolType::UnstressedVowel,
            Symbol::AH => SymbolType::UnstressedVowel,
            Symbol::AO => SymbolType::UnstressedVowel,
            Symbol::AW => SymbolType::UnstressedVowel,
            Symbol::AY => SymbolType::UnstressedVowel,
            Symbol::EH => SymbolType::UnstressedVowel,
            Symbol::ER => SymbolType::UnstressedVowel,
            Symbol::EY => SymbolType::UnstressedVowel,
            Symbol::IH => SymbolType::UnstressedVowel,
            Symbol::IY => SymbolType::UnstressedVowel,
            Symbol::OW => SymbolType::UnstressedVowel,
            Symbol::OY => SymbolType::UnstressedVowel,
            Symbol::UH => SymbolType::UnstressedVowel,
            Symbol::UW => SymbolType::UnstressedVowel,

            Symbol::AA0 => SymbolType::UnstressedVowel,
            Symbol::AE0 => SymbolType::UnstressedVowel,
            Symbol::AH0 => SymbolType::UnstressedVowel,
            Symbol::AO0 => SymbolType::UnstressedVowel,
            Symbol::AW0 => SymbolType::UnstressedVowel,
            Symbol::AY0 => SymbolType::UnstressedVowel,
            Symbol::EH0 => SymbolType::UnstressedVowel,
            Symbol::ER0 => SymbolType::UnstressedVowel,
            Symbol::EY0 => SymbolType::UnstressedVowel,
            Symbol::IH0 => SymbolType::UnstressedVowel,
            Symbol::IY0 => SymbolType::UnstressedVowel,
            Symbol::OW0 => SymbolType::UnstressedVowel,
            Symbol::OY0 => SymbolType::UnstressedVowel,
            Symbol::UH0 => SymbolType::UnstressedVowel,
            Symbol::UW0 => SymbolType::UnstressedVowel,

            Symbol::AA1 => SymbolType::StressedVowel,
            Symbol::AE1 => SymbolType::StressedVowel,
            Symbol::AH1 => SymbolType::StressedVowel,
            Symbol::AO1 => SymbolType::StressedVowel,
            Symbol::AW1 => SymbolType::StressedVowel,
            Symbol::AY1 => SymbolType::StressedVowel,
            Symbol::EH1 => SymbolType::StressedVowel,
            Symbol::ER1 => SymbolType::StressedVowel,
            Symbol::EY1 => SymbolType::StressedVowel,
            Symbol::IH1 => SymbolType::StressedVowel,
            Symbol::IY1 => SymbolType::StressedVowel,
            Symbol::OW1 => SymbolType::StressedVowel,
            Symbol::OY1 => SymbolType::StressedVowel,
            Symbol::UH1 => SymbolType::StressedVowel,
            Symbol::UW1 => SymbolType::StressedVowel,

            Symbol::AA2 => SymbolType::StressedVowel,
            Symbol::AE2 => SymbolType::StressedVowel,
            Symbol::AH2 => SymbolType::StressedVowel,
            Symbol::AO2 => SymbolType::StressedVowel,
            Symbol::AW2 => SymbolType::StressedVowel,
            Symbol::AY2 => SymbolType::StressedVowel,
            Symbol::EH2 => SymbolType::StressedVowel,
            Symbol::ER2 => SymbolType::StressedVowel,
            Symbol::EY2 => SymbolType::StressedVowel,
            Symbol::IH2 => SymbolType::StressedVowel,
            Symbol::IY2 => SymbolType::StressedVowel,
            Symbol::OW2 => SymbolType::StressedVowel,
            Symbol::OY2 => SymbolType::StressedVowel,
            Symbol::UH2 => SymbolType::StressedVowel,
            Symbol::UW2 => SymbolType::StressedVowel,

            Symbol::B => SymbolType::Stop,
            Symbol::CH => SymbolType::Affricate,
            Symbol::D => SymbolType::Stop,
            Symbol::DH => SymbolType::Fricative,
            Symbol::F => SymbolType::Fricative,
            Symbol::G => SymbolType::Stop,
            Symbol::HH => SymbolType::Aspirate,
            Symbol::JH => SymbolType::Affricate,
            Symbol::K => SymbolType::Stop,
            Symbol::L => SymbolType::Liquid,
            Symbol::M => SymbolType::Nasal,
            Symbol::N => SymbolType::Nasal,
            Symbol::NG => SymbolType::Nasal,
            Symbol::P => SymbolType::Stop,
            Symbol::R => SymbolType::Liquid,
            Symbol::S => SymbolType::Fricative,
            Symbol::SH => SymbolType::Fricative,
            Symbol::T => SymbolType::Stop,
            Symbol::TH => SymbolType::Fricative,
            Symbol::V => SymbolType::Fricative,
            Symbol::W => SymbolType::Semivowel,
            Symbol::Y => SymbolType::Semivowel,
            Symbol::Z => SymbolType::Fricative,
            Symbol::ZH => SymbolType::Fricative,
        }
    }
}