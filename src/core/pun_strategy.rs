use std::{collections::HashMap, hash::Hash};

use itertools::Itertools;
use strum::EnumIter;
use strum::IntoEnumIterator;

use super::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, EnumIter)]
pub enum PunStrategy {
    Homophone,
    PerfectRhyme,
    Prefix,
    PrefixRhyme,
    SameConsonants,
    InfixRhyme,
    SharedPrefix,
}

impl PunStrategy {
    fn get_relevant_syllables(&self, word: &PhoeneticsWord) -> Option<Vec<Syllable>> {
        match self {
            PunStrategy::Homophone => Some(word.syllables.clone()),
            PunStrategy::PerfectRhyme => {
                if let Some(last_index_position) = word
                    .syllables
                    .iter()
                    .rev()
                    .find_position(|x| x.nucleus().is_stressed_vowel())
                    .map(|x| (word.syllables.len() - 1) - x.0)
                {
                    let vec = word
                        .syllables
                        .iter()
                        .skip(last_index_position)
                        .enumerate()
                        .map(|(i, x)| {
                            if i == 0 {
                                x.get_rhymes_syllable()
                            } else {
                                x.clone()
                            }
                        })
                        .collect_vec();
                    if vec.is_empty() {
                        return None;
                    }
                    return Some(vec);
                } else {
                    None
                }
            }
            PunStrategy::InfixRhyme => {
                if word.syllables.len() == 1 {
                    return Some(
                        word.syllables
                            .iter()
                            .map(|x| x.get_rhymes_syllable())
                            .collect_vec(),
                    );
                }

                return None;
            }
            _ => None,
        }
    }

    fn get_possible_replacements(
        &self,
        original_word: &PhoeneticsWord,
        dict: &HashMap<Vec<Syllable>, Vec<PhoeneticsWord>>,
    ) -> Vec<PunReplacement> {
        match self {
            PunStrategy::Homophone => {
                if let Some(syllables) = self.get_relevant_syllables(original_word) {
                    if let Some(theme_words) = dict.get(&syllables) {
                        return theme_words
                            .into_iter()
                            .map(|theme_word| {
                                let pun_type =
                                    if original_word.text.eq_ignore_ascii_case(&theme_word.text) {
                                        PunType::Identity
                                    } else {
                                        PunType::SameWord
                                    };

                                PunReplacement {
                                    pun_type,
                                    pun_word: theme_word.text.clone(),
                                    replacement_string: theme_word.text.clone(),
                                    is_amalgam: false,
                                }
                            })
                            .collect_vec();
                    }
                }
            }
            PunStrategy::PerfectRhyme => {
                if let Some(key) = self.get_relevant_syllables(original_word) {
                    if let Some(theme_words) = dict.get(&key) {
                        return theme_words
                            .iter()
                            .filter(|theme_word| {
                                theme_word.syllables.len() <= original_word.syllables.len()
                                    && !theme_word.text.eq_ignore_ascii_case(&original_word.text)
                            })
                            .map(|theme_word| {
                                let replacement_string = if theme_word.syllables.len()
                                    == original_word.syllables.len()
                                {
                                    theme_word.text.clone()
                                } else {
                                    original_word
                                        .syllables
                                        .iter()
                                        .take(original_word.syllables.len() - key.len())
                                        .map(|x| x.get_spelling())
                                        .join("")
                                        + &theme_word.text
                                };

                                PunReplacement {
                                    pun_type: PunType::PerfectRhyme,
                                    replacement_string,
                                    is_amalgam: false,
                                    pun_word: theme_word.text.clone(),
                                }
                            })
                            .collect_vec();
                    }
                };
            }

            PunStrategy::InfixRhyme => {
                return original_word
                    .syllables
                    .iter()
                    .take(original_word.syllables.len() - 1)
                    .enumerate()
                    .filter(|(_, syllable)| syllable.nucleus().is_stressed_vowel())
                    .filter_map(|(index, syllable)| {
                        dict.get(&vec![syllable.clone()])
                            .map(|theme_words| (index, syllable, theme_words))
                    })
                    .flat_map(|(index, syllable, theme_words)| {
                        theme_words
                            .into_iter()
                            .map(move |theme_word| (index, syllable, theme_word))
                    })
                    .filter(|(_, syllable, theme_word)| {
                        theme_word.syllables.len() == 1 && &&theme_word.syllables[0] != syllable
                    })
                    .map(|(index, _, theme_word)| {
                        let replacement_string = original_word
                            .syllables
                            .iter()
                            .take(index)
                            .map(|x| x.get_spelling())
                            .join("")
                            + &theme_word.text
                            + &original_word
                                .syllables
                                .iter()
                                .skip(index + 1)
                                .map(|x| x.get_spelling())
                                .join("");
                        PunReplacement {
                            pun_type: PunType::Infix,
                            is_amalgam: true,
                            pun_word: theme_word.text.clone(),
                            replacement_string,
                        }
                    })
                    .collect_vec();
            }
            _ => {}
        };

        return Vec::<PunReplacement>::default();
    }
}

#[derive(Debug)]
pub struct PunFactory {
    pub strategy: PunStrategy,
    pub dict: HashMap<Vec<Syllable>, Vec<PhoeneticsWord>>,
}

impl PunFactory {
    pub fn build_all(words: &Vec<PhoeneticsWord>) -> Vec<PunFactory> {
        PunStrategy::iter()
            .map(|strategy| PunFactory::create(strategy, words))
            .collect_vec()
    }

    pub fn solve(factories: &Vec<PunFactory>, phrase: &Phrase) -> Vec<PunPhrase> {
        phrase.words.iter().enumerate().flat_map(|(index, word)| {
            factories
                .iter()
                .flat_map(|f| f.get_possible_replacements(word))                
                .map(move |replacement| PunPhrase{phrase: phrase.clone(), replacement,index:index.clone()})
        }).collect_vec()
    }

    fn get_possible_replacements(&self, original_word: &PhoeneticsWord) -> Vec<PunReplacement> {
        self.strategy
            .get_possible_replacements(original_word, &self.dict)
    }

    pub fn create(strategy: PunStrategy, words: &Vec<PhoeneticsWord>) -> Self {
        let dict: HashMap<_, _> = words
            .iter()
            .cloned()
            .filter_map(|word| strategy.get_relevant_syllables(&word).map(|u| (u, word)))
            .into_group_map();

        Self { strategy, dict }
    }
}
