use itertools::Itertools;
use std::{collections::HashMap, vec};

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct Prefix {}

impl PunStrategy for Prefix {
    fn get_relevant_syllables(&self, word: &DictionaryWord) -> Vec<Vec<Syllable>> {
        if word.syllables.len() <= 1 {
            return vec![];
        }

        (1..word.syllables.len() - 1)
            .map(|l| word.syllables.iter().take(l).cloned().collect_vec())
            .chain((1..word.syllables.len() - 1).map(|l| {
                word.syllables
                    .iter()
                    .take(l - 1)
                    .cloned()
                    .chain(std::iter::once(
                        word.syllables[l - 1].add_next_offset(&word.syllables[l]),
                    ))
                    .collect_vec()
            }))
            .collect_vec()
    }

    fn get_possible_replacements(
        &self,
        phrase_word: &PhraseWord,
        dict: &HashMap<Vec<Syllable>, Vec<DictionaryWord>>,
    ) -> Vec<PunReplacement> {
        if let Some(original_word) = &phrase_word.word {
            if let Some(theme_words) = dict.get(&original_word.syllables) {
                return theme_words
                    .iter()
                    .map(|theme_word| {
                        if theme_word.spellings[0].starts_with(original_word.spellings[0].as_str())
                        {
                            PunReplacement {
                                pun_type: PunType::Prefix,
                                pun_word: theme_word.spellings[0].clone(),
                                replacement_string: Casing::unify_captialization(
                                    &theme_word.spellings[0],
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
                                original_word.spellings[0].clone() + suffix.as_str();

                            PunReplacement {
                                pun_type: PunType::Prefix,
                                pun_word: theme_word.spellings[0].clone(),
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
