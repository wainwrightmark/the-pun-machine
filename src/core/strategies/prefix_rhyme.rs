use std::{collections::HashMap, vec};

use itertools::Itertools;
use smallvec::SmallVec;

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct PrefixRhyme {}

impl PunStrategy for PrefixRhyme {
    fn get_relevant_syllables(
        &self,
        word: &DictionaryWord<'static>,
    ) -> Vec<SmallVec<[Syllable; 4]>> {
        if !word.syllables.is_empty() {
            if let Some(syllable) = word.syllables.last() {
                if syllable.nucleus().is_stressed_vowel() {
                    let rhyme_syllable = syllable.get_rhymes_syllable();
                    return vec![smallvec::smallvec![rhyme_syllable]];
                }
            }
        }

        vec![]
    }

    fn get_possible_replacements(
        &self,
        phrase_word: &PhraseWord,
        dict: &HashMap<SmallVec<[Syllable; 4]>, Vec<DictionaryWord<'static>>>,
    ) -> Vec<PunReplacement> {
        if let Some(original_word) = &phrase_word.word {
            if let Some(first_syllable) = original_word.syllables.first() {
                if original_word.syllables.len() > 1 {
                    // && first_syllable.nucleus().is_stressed_vowel() {
                    let mut rhyme_syllable = first_syllable.get_rhymes_syllable();
                    if rhyme_syllable.coda().next().is_none() {
                        rhyme_syllable =
                            rhyme_syllable.add_next_offset(&original_word.syllables[1]);
                    }
                    let rhyme_word = smallvec::smallvec![rhyme_syllable];

                    if let Some(theme_words) = dict.get(&rhyme_word) {
                        return theme_words
                            .iter()
                            .filter(|x| {
                                x.syllables.len() == 1
                                    || x.syllables.get(x.syllables.len() - 2)
                                        == Some(first_syllable)
                            })
                            .map(|theme_word| {
                                let suffix = original_word
                                    .syllables
                                    .iter()
                                    .skip(1)
                                    .map(|x| x.get_spelling())
                                    .join("");

                                let replacement_string = Casing::unify_captialization(
                                    theme_word.spelling,
                                    &phrase_word.text,
                                ) + suffix.as_str();

                                PunReplacement {
                                    pun_type: PunType::PrefixRhyme,
                                    is_amalgam: true,
                                    pun_word: theme_word.spelling,
                                    replacement_string,
                                }
                            })
                            .collect_vec();
                    }
                }
            }
        }
        vec![]
    }
}
