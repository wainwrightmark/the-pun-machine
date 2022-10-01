use std::{collections::HashMap, vec};

use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct SharedPrefix {}

impl PunStrategy for SharedPrefix {
    fn get_relevant_syllables(
        &self,
        word: &DictionaryWord,
    ) -> SmallVec<[SmallVec<[Syllable; 4]>; 2]> {
        if word.syllables.len() > 2 {
            return smallvec![word.syllables.iter().take(2).cloned().collect()];
        }
        smallvec![]
    }

    fn get_possible_replacements(
        &self,
        phrase_word: &PhraseWord,
        dict: &HashMap<SmallVec<[Syllable; 4]>, Vec<DictionaryWord<'static>>>,
    ) -> Vec<PunReplacement> {
        if let Some(original_word) = &phrase_word.word {
            if original_word.syllables.len() > 2 {
                let first_two_syllables: SmallVec<_> =
                    original_word.syllables.iter().take(2).cloned().collect();

                if let Some(theme_words) = dict.get(&first_two_syllables) {
                    return theme_words
                        .iter()
                        .filter(|theme_word| !theme_word.eq(&original_word))
                        .filter(|theme_word| !theme_word.syllables.is_empty())
                        .map(|theme_word| PunReplacement {
                            pun_type: PunType::SharedPrefix,
                            is_amalgam: false,
                            pun_word: theme_word.spelling,
                            replacement_string: Casing::unify_captialization(
                                theme_word.spelling,
                                &phrase_word.text,
                            ),
                        })
                        .collect_vec();
                }
            }
        }

        vec![]
    }
}
