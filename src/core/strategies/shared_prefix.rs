use itertools::Itertools;
use std::{collections::HashMap, vec};

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct SharedPrefix {}

impl PunStrategy for SharedPrefix {
    fn get_relevant_syllables(&self, word: &DictionaryWord) -> Vec<Vec<Syllable>> {
        if word.syllables.len() > 2 {
            return vec![word.syllables.iter().take(2).cloned().collect_vec()];
        }
        vec![]
    }

    fn get_possible_replacements(
        &self,
        original_word: &DictionaryWord,
        dict: &HashMap<Vec<Syllable>, Vec<DictionaryWord>>,
    ) -> Vec<PunReplacement> {
        if original_word.syllables.len() > 2 {
            let first_two_syllables = original_word
                .syllables
                .iter()
                .take(2)
                .cloned()
                .collect_vec();

            if let Some(theme_words) = dict.get(&first_two_syllables) {
                return theme_words
                    .iter()
                    .filter(|theme_word| !theme_word.eq(&original_word))
                    .filter(|theme_word| !theme_word.syllables.is_empty())
                    .map(|theme_word| PunReplacement {
                        pun_type: PunType::SharedPrefix,
                        is_amalgam: false,
                        pun_word: theme_word.spellings[0].clone(),
                        replacement_string: theme_word.spellings[0].clone(),
                    })
                    .collect_vec();
            }
        }

        vec![]
    }
}
