use itertools::Itertools;
use std::{collections::HashMap, vec};

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct SharedPrefix {}

impl PunStrategy for SharedPrefix {
    fn get_relevant_syllables(&self, word: &PhoeneticsWord) -> Vec<Vec<Syllable>> {
        if word.syllables.len() > 2 {
            return vec![word.syllables.iter().take(2).cloned().collect_vec()];
        }
        vec![]
    }

    fn get_possible_replacements(
        &self,
        original_word: &PhoeneticsWord,
        dict: &HashMap<Vec<Syllable>, Vec<PhoeneticsWord>>,
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
                    .filter(|theme_word| !theme_word.text.eq_ignore_ascii_case(&original_word.text))
                    .filter(|theme_word| !theme_word.syllables.is_empty())
                    .map(|theme_word| PunReplacement {
                        pun_type: PunType::SharedPrefix,
                        is_amalgam: false,
                        pun_word: theme_word.text.clone(),
                        replacement_string: theme_word.text.clone(),
                    })
                    .collect_vec();
            }
        }

        vec![]
    }
}
