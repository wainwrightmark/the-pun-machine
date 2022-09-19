use crate::core::prelude::*;
use std::{collections::BTreeMap, str::FromStr, sync::Arc};

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde:: Deserialize,
    serde::Serialize,
)]
pub struct DictionaryWord {
    pub text: String,
    pub syllables: Vec<Syllable>,
    pub meanings: Vec<u32>,
}

impl FromStr for DictionaryWord {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        WORDDICTIONARY
            .words
            .get(s)
            .map(|(syllables, meanings)| DictionaryWord {
                text: s.to_string().into(),
                syllables: syllables.clone(),
                meanings: meanings.clone(),
            })
            .ok_or("Could not find word")
    }
}

impl DictionaryWord {
    pub fn self_and_children(&self) -> Vec<DictionaryWord> {
        let mut result = vec![self.clone()];

        for meaning_id in self.meanings.iter() {
            let meaning = WORDSBYMEANING[&meaning_id].clone();

            for child in meaning.self_and_children() {
                result.push(child);
            }
        }

        result
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde:: Deserialize, serde::Serialize)]
pub struct WordDictionary {
    pub words: BTreeMap<String, (Vec<Syllable>, Vec<u32>)>,
    pub meanings: BTreeMap<u32, Vec<u32>>,
}

lazy_static::lazy_static! {
    static ref WORDSBYMEANING : BTreeMap<u32, Arc<DictionaryWord>> = WORDDICTIONARY.words.iter()

    .flat_map(|(text, (syllables, meanings))|
        {
            let dw = DictionaryWord{text:text.clone().into(), syllables:syllables.clone(), meanings: meanings.clone()};
            let arc_dw = Arc::from(dw);

            meanings.iter().map(move |x|(x.clone(), arc_dw.clone()))
        }

).collect();
}

lazy_static::lazy_static! {
    static ref WORDDICTIONARY: WordDictionary = rmp_serde::from_slice(&WORDDICTIONARYSTR).unwrap();
}

include_flate::flate!(pub static WORDDICTIONARYSTR: [u8] from "data.mp");
