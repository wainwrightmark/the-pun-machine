use std::collections::HashMap;

use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct PerfectRhyme {}

impl PerfectRhyme {
    fn get_rhyme_syllables(
        &self,
        word: &DictionaryWord<'static>,
    ) -> Option<SmallVec<[Syllable; 4]>> {
        if let Some(last_index_position) = word
            .syllables
            .iter()
            .rev()
            .find_position(|x| x.nucleus().is_stressed_vowel())
            .map(|x| (word.syllables.len() - 1) - x.0)
        {
            let vec: SmallVec<_> = word
                .syllables
                .iter()
                .skip(last_index_position)
                .enumerate()
                .map(|(i, x)| {
                    if i == 0 {
                        x.get_rhymes_syllable()
                    } else {
                        *x
                    }
                })
                .collect();
            if vec.is_empty() {
                return None;
            }
            Some(vec)
        } else {
            None
        }
    }
}

impl PunStrategy for PerfectRhyme {
    fn get_relevant_syllables(
        &self,
        word: &DictionaryWord<'static>,
    ) -> SmallVec<[SmallVec<[Syllable; 4]>; 2]> {
        if let Some(s) = PerfectRhyme::get_rhyme_syllables(self, word) {
            smallvec![s]
        } else {
            smallvec![]
        }
    }

    fn get_possible_replacements(
        &self,
        phrase_word: &PhraseWord,
        dict: &HashMap<SmallVec<[Syllable; 4]>, Vec<DictionaryWord<'static>>>,
    ) -> Vec<PunReplacement> {
        if let Some(original_word) = &phrase_word.word {
            if let Some(key) = PerfectRhyme::get_rhyme_syllables(self, original_word) {
                if let Some(theme_words) = dict.get(&key) {
                    return theme_words
                        .iter()
                        .filter(|theme_word| {
                            theme_word.syllables.len() <= original_word.syllables.len()
                                && !theme_word.eq(&original_word)
                                && !theme_word.syllables.eq(&original_word.syllables)
                        })
                        .map(|theme_word| {
                            let replacement_string = if theme_word.syllables.len()
                                == original_word.syllables.len()
                            {
                                Casing::unify_captialization(theme_word.spelling, &phrase_word.text)
                            } else {
                                Casing::unify_captialization(
                                    &original_word
                                        .syllables
                                        .iter()
                                        .take(original_word.syllables.len() - key.len())
                                        .map(|x| x.get_spelling())
                                        .join(""),
                                    &phrase_word.text,
                                ) + theme_word.spelling
                            };

                            PunReplacement {
                                pun_type: PunType::PerfectRhyme,
                                replacement_string,
                                is_amalgam: false,
                                pun_word: theme_word.spelling,
                            }
                        })
                        .collect_vec();
                }
            };
        }

        Vec::<PunReplacement>::default()
    }
}
