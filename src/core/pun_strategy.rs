use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use lazy_static::*;
use smallvec::SmallVec;
use strum::IntoEnumIterator;

use super::prelude::*;

#[derive(Debug)]
pub struct PunFactory {
    pub strategy: PunStrategyEnum,
    pub dict: HashMap<SmallVec<[Syllable; 4]>, Vec<DictionaryWord<'static>>>,
}

include_flate::flate!(pub static COMMONWORDSTEXT: str from "data/other/CommonWords.txt");

lazy_static! {
    pub static ref STOPWORDS: HashSet<&'static str> = {
        let set: HashSet<_> = COMMONWORDSTEXT.lines().collect();
        set
    };
}

impl PunFactory {
    pub fn build_all(words: &Vec<DictionaryWord<'static>>) -> Vec<PunFactory> {
        PunStrategyEnum::iter()
            .map(|strategy| PunFactory::create(strategy, words))
            .collect_vec()
    }

    pub fn solve(factories: &Vec<PunFactory>, phrase: &Phrase) -> Vec<PunPhrase> {
        phrase
            .words
            .iter()
            .enumerate()
            .filter(|(_, phrase_word)| {
                if let Some(word) = &phrase_word.word {
                    if word.syllables.is_empty() {
                        return false;
                    }

                    return !STOPWORDS.contains(phrase_word.text.to_ascii_lowercase().as_str());
                }
                false
            })
            .flat_map(|(index, phrase_word)| {
                factories
                    .iter()
                    .flat_map(move |f| f.get_possible_replacements(phrase_word))
                    .map(move |replacement| PunPhrase {
                        phrase: phrase.clone(),
                        replacement,
                        index,
                    })
            })
            .collect_vec()
    }

    fn get_possible_replacements(&self, original_word: &PhraseWord) -> Vec<PunReplacement> {
        self.strategy
            .get_possible_replacements(original_word, &self.dict)
    }

    pub fn create(strategy: PunStrategyEnum, words: &Vec<DictionaryWord<'static>>) -> Self {
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
