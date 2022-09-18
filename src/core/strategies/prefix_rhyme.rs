use itertools::Itertools;
use std::{collections::HashMap, vec};

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct PrefixRhyme {}

impl PunStrategy for PrefixRhyme {
    fn get_relevant_syllables(&self, word: &PhoeneticsWord) -> Vec<Vec<Syllable>> {
        if word.syllables.len() > 0 {
            if let Some(syllable) = word.syllables.get(word.syllables.len() - 1) {
                if syllable.nucleus().is_stressed_vowel() {
                    let rhyme_syllable = syllable.get_rhymes_syllable();
                    return vec![vec![rhyme_syllable]];
                }
            }
        }

        vec![]
    }

    fn get_possible_replacements(
        &self,
        original_word: &PhoeneticsWord,
        dict: &HashMap<Vec<Syllable>, Vec<PhoeneticsWord>>,
    ) -> Vec<PunReplacement> {
        if let Some(first_syllable) = original_word.syllables.first() {
            if original_word.syllables.len() > 1 {
                // && first_syllable.nucleus().is_stressed_vowel() {
                let mut rhyme_syllable = first_syllable.get_rhymes_syllable();
                if rhyme_syllable.coda().next().is_none() {
                    rhyme_syllable = rhyme_syllable.add_next_offset(&original_word.syllables[1]);
                }
                let rhyme_word = vec![rhyme_syllable];

                if let Some(theme_words) = dict.get(&rhyme_word) {
                    return theme_words
                        .iter()
                        .filter(|x| {
                            x.syllables.len() == 1
                                || x.syllables.get(x.syllables.len() - 2) == Some(first_syllable)
                        })
                        .map(|theme_word| {
                            let suffix = original_word
                                .syllables
                                .iter()
                                .skip(1)
                                .map(|x| x.get_spelling())
                                .join("");

                            let replacement_string = theme_word.text.clone() + suffix.as_str();

                            PunReplacement {
                                pun_type: PunType::PrefixRhyme,
                                is_amalgam: true,
                                pun_word: theme_word.text.clone(),
                                replacement_string,
                            }
                        })
                        .collect_vec();
                }
            }
        }

        vec![]
    }
}
