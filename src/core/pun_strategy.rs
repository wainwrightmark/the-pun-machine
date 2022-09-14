use super::prelude::*;
use itertools::Itertools;
use lazy_static::*;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct PunFactory {
    pub strategy: PunStrategyEnum,
    pub dict: HashMap<Vec<Syllable>, Vec<PhoeneticsWord>>,
}

lazy_static! {
    static ref STOPWORDS: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("the");
        m.insert("and");
        m.insert("in");
        m.insert("on");
        m.insert("by");
        m.insert("a");
        m.insert("an");
        m.insert("it");
        m.insert("to");
        m
    };
}

impl PunFactory {
    pub fn build_all(words: &Vec<PhoeneticsWord>) -> Vec<PunFactory> {
        PunStrategyEnum::iter()
            .map(|strategy| PunFactory::create(strategy, words))
            .collect_vec()
    }

    pub fn solve(factories: &Vec<PunFactory>, phrase: &Phrase) -> Vec<PunPhrase> {
        phrase
            .words
            .iter()
            .enumerate()
            .filter(|x| !STOPWORDS.contains(&x.1.text.to_ascii_lowercase().as_str()))
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

    fn get_possible_replacements(&self, original_word: &PhoeneticsWord) -> Vec<PunReplacement> {
        self.strategy
            .get_possible_replacements(original_word, &self.dict)
    }

    pub fn create(strategy: PunStrategyEnum, words: &Vec<PhoeneticsWord>) -> Self {
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
