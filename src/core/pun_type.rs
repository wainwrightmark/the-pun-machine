use strum::EnumString;

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
    serde::Serialize,
    serde::Deserialize,
)]
pub enum PunType {
    /// The exact same word - not really a pun
    SameWord,

    /// Bass / Base
    Identity,

    /// Multiple vowel sounds and all subsequent syllables match
    RichRhyme,

    /// Final vowel sound and all subsequent syllables match
    PerfectRhyme,

    /// Final vowel segments are different while the consonants are identical, or vice versa
    ImperfectRhyme, //Worse than prefix and infix

    /// One word is a prefix to the other
    Prefix,

    PrefixRhyme,

    /// One word is contained within the other
    Infix,

    /// Both words share at least four syllables of prefix
    SharedPrefix,
    SameConsonants,
}

impl std::fmt::Display for PunType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: &'static str = match self {
            PunType::SameWord => "Same Word",
            PunType::Identity => "Identity",
            PunType::RichRhyme => "Rich Rhyme",
            PunType::PerfectRhyme => "Perfect Rhyme",
            PunType::ImperfectRhyme => "Imperfect Rhyme",
            PunType::Prefix => "Prefix",
            PunType::PrefixRhyme => "Prefix Rhyme",
            PunType::Infix => "Infix",
            PunType::SharedPrefix => "Shared Prefix",
            PunType::SameConsonants => "Same Consonants",
        };

        write!(f, "{}", text)
    }
}
