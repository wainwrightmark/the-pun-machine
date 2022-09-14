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
