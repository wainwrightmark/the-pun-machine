use crate::core::prelude::*;
use std::str::FromStr;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, serde::Serialize, serde::Deserialize,
)]
pub struct Phrase {
    pub text: String,
    pub words: Vec<PhraseWord>,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, serde::Serialize, serde::Deserialize,
)]
pub struct PhraseWord {
    pub text: String,
    pub word: Option<DictionaryWord>,
}

impl From<String> for Phrase {
    fn from(value: String) -> Self {
        let words: Vec<_> = value
            .split(' ')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| PhraseWord {
                text: x.to_string(),
                word: DictionaryWord::from_str(x).ok(),
            })
            .collect();

        Phrase {
            text: value.clone(),
            words,
        }
    }
}
