use super::prelude::*;
use itertools::Itertools;
use lazy_static::*;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct PunFactory {
    pub strategy: PunStrategyEnum,
    pub dict: HashMap<Vec<Syllable>, Vec<DictionaryWord>>,
}

include_flate::flate!(pub static COMMONWORDSTEXT: str from "data/other/CommonWords.txt");

lazy_static! {
    pub static ref STOPWORDS: HashSet<&'static str> = {
        let set: HashSet<_> = COMMONWORDSTEXT.lines().collect();
        set
    };
}

impl PunFactory {
    pub fn build_all(words: &Vec<DictionaryWord>) -> Vec<PunFactory> {
        PunStrategyEnum::iter()
            .map(|strategy| PunFactory::create(strategy, words))
            .collect_vec()
    }

    pub fn solve(factories: &Vec<PunFactory>, phrase: &Phrase) -> Vec<PunPhrase> {
        phrase
            .words
            .iter()
            .enumerate()
            .filter(|x| {
                !STOPWORDS.contains(&x.1.spellings[0].to_ascii_lowercase().as_str())
                    && !x.1.syllables.is_empty()
            })
            .flat_map(|(index, word)| {
                factories
                    .iter()
                    .flat_map(|f| f.get_possible_replacements(word))
                    .map(move |replacement| PunPhrase {
                        phrase: phrase.clone(),
                        replacement,
                        index,
                    })
            })
            .collect_vec()
    }

    fn get_possible_replacements(&self, original_word: &DictionaryWord) -> Vec<PunReplacement> {
        self.strategy
            .get_possible_replacements(original_word, &self.dict)
    }

    pub fn create(strategy: PunStrategyEnum, words: &Vec<DictionaryWord>) -> Self {
        let dict: HashMap<_, _> = words
            .iter()
            .cloned()
            .flat_map(|word| {
                strategy
                    .get_relevant_syllables(&word)
                    .into_iter()
                    .map(move |u| (u, word.clone()))
            })
            .into_group_map();

        Self { strategy, dict }
    }
}
