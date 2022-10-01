use std::str::FromStr;

use crate::core::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct PhraseWord {
    pub text: String,
    pub word: Option<DictionaryWord<'static>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Phrase {
    pub text: String,
    pub words: Vec<PhraseWord>,
    pub category: Category,
}

impl Phrase {
    pub fn new(value: String, category: Category) -> Self {
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
            text: value,
            words,
            category,
        }
    }
}
