use std::{collections::HashMap, vec};

use itertools::Itertools;
use smallvec::SmallVec;

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct SameConsonants {}

impl SameConsonants {
    fn get_consonant_syllables(&self, word: &DictionaryWord<'static>) -> SmallVec<[Syllable; 4]> {
        word.syllables
            .iter()
            .map(|x| x.clone().with_no_consonant())
            .collect()
    }
}

impl PunStrategy for SameConsonants {
    fn get_relevant_syllables(
        &self,
        word: &DictionaryWord<'static>,
    ) -> Vec<SmallVec<[Syllable; 4]>> {
        vec![self.get_consonant_syllables(word)]
    }

    fn get_possible_replacements(
        &self,
        phrase_word: &PhraseWord,
        dict: &HashMap<SmallVec<[Syllable; 4]>, Vec<DictionaryWord<'static>>>,
    ) -> Vec<PunReplacement> {
        if let Some(original_word) = &phrase_word.word {
            let sw = self.get_consonant_syllables(original_word);

            if let Some(theme_words) = dict.get(&sw) {
                return theme_words
                    .iter()
                    .filter(|theme_word| {
                        !theme_word.eq(&original_word)
                            && !theme_word.syllables.eq(&original_word.syllables)
                    })
                    .map(|theme_word| PunReplacement {
                        pun_type: PunType::SameConsonants,
                        is_amalgam: false,
                        pun_word: theme_word.spelling.clone(),
                        replacement_string: Casing::unify_captialization(
                            theme_word.spelling,
                            &phrase_word.text,
                        ),
                    })
                    .collect_vec();
            }
        }
        vec![]
    }
}
