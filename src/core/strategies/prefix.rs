use std::{collections::HashMap, vec};

use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct Prefix {}

impl PunStrategy for Prefix {
    fn get_relevant_syllables(
        &self,
        word: &DictionaryWord<'static>,
    ) -> SmallVec<[SmallVec<[Syllable; 4]>; 2]> {
        if word.syllables.len() <= 1 {
            return smallvec![];
        }

        (1..word.syllables.len() - 1)
            .map(|l| word.syllables.iter().take(l).cloned().collect())
            .chain((1..word.syllables.len() - 1).map(|l| {
                word.syllables
                    .iter()
                    .take(l - 1)
                    .cloned()
                    .chain(std::iter::once(
                        word.syllables[l - 1].add_next_offset(&word.syllables[l]),
                    ))
                    .collect()
            }))
            .collect()
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
                        if theme_word.spelling.starts_with(original_word.spelling) {
                            PunReplacement {
                                pun_type: PunType::Prefix,
                                pun_word: theme_word.spelling,
                                replacement_string: Casing::unify_captialization(
                                    theme_word.spelling,
                                    &phrase_word.text,
                                ),
                                is_amalgam: false,
                            }
                        } else {
                            let suffix = theme_word
                                .syllables
                                .iter()
                                .skip(1)
                                .map(Syllable::get_spelling)
                                .join("");
                            let replacement_string =
                                original_word.spelling.to_string() + suffix.as_str();

                            PunReplacement {
                                pun_type: PunType::Prefix,
                                pun_word: theme_word.spelling,
                                replacement_string,
                                is_amalgam: true,
                            }
                        }
                    })
                    .collect_vec();
            }
        }
        vec![]
    }
}
