use std::collections::HashMap;

use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct Homophone {}

impl PunStrategy for Homophone {
    fn get_relevant_syllables(
        &self,
        word: &DictionaryWord<'static>,
    ) -> SmallVec<[SmallVec<[Syllable; 4]>; 2]> {
        smallvec![word.syllables.clone()]
    }

    fn get_possible_replacements(
        &self,
        phrase_word: &PhraseWord,
        dict: &HashMap<SmallVec<[Syllable; 4]>, Vec<DictionaryWord<'static>>>,
    ) -> Vec<PunReplacement> {
        if let Some(original_word) = &phrase_word.word {
            if let Some(theme_words) = dict.get(&original_word.syllables) {
                return theme_words
                    .iter()
                    .map(|theme_word| {
                        let pun_type = if original_word.eq(theme_word) {
                            PunType::Identity
                        } else {
                            PunType::SameWord
                        };

                        PunReplacement {
                            pun_type,
                            pun_word: theme_word.spelling,
                            replacement_string: Casing::unify_captialization(
                                theme_word.spelling,
                                &phrase_word.text,
                            ),
                            is_amalgam: false,
                        }
                    })
                    .collect_vec();
            }
        }

        Vec::<PunReplacement>::default()
    }
}
