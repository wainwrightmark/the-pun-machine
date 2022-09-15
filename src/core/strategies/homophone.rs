use itertools::Itertools;
use std::collections::HashMap;

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct Homophone {}

impl PunStrategy for Homophone {
    fn get_relevant_syllables(&self, word: &PhoeneticsWord) -> Vec<Vec<Syllable>> {
        vec![word.syllables.clone()]
    }

    fn get_possible_replacements(
        &self,
        original_word: &PhoeneticsWord,
        dict: &HashMap<Vec<Syllable>, Vec<PhoeneticsWord>>,
    ) -> Vec<PunReplacement> {
        if let Some(theme_words) = dict.get(&original_word.syllables) {
            return theme_words
                .iter()
                .map(|theme_word| {
                    let pun_type = if original_word.text.eq_ignore_ascii_case(&theme_word.text) {
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

        Vec::<PunReplacement>::default()
    }
}