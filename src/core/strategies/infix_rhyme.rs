use itertools::Itertools;
use std::collections::HashMap;

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct InfixRhyme {}

impl PunStrategy for InfixRhyme {
    fn get_relevant_syllables(&self, word: &DictionaryWord) -> Vec<Vec<Syllable>> {
        if word.syllables.len() == 1 {
            return vec![word
                .syllables
                .iter()
                .map(|x| x.get_rhymes_syllable())
                .collect_vec()];
        }

        vec![]
    }

    fn get_possible_replacements(
        &self,
        phrase_word: &PhraseWord,
        dict: &HashMap<Vec<Syllable>, Vec<DictionaryWord>>,
    ) -> Vec<PunReplacement> {
        if let Some(original_word) = &phrase_word.word {
            if original_word.syllables.len() <= 1 {}

            return original_word
                .syllables
                .iter()
                .take(original_word.syllables.len() - 1)
                .enumerate()
                .skip(1)
                .filter(|(_, syllable)| syllable.nucleus().is_stressed_vowel())
                .filter_map(|(index, syllable)| {
                    dict.get(&vec![syllable.get_rhymes_syllable()])
                        .map(|theme_words| (index, syllable, theme_words))
                })
                .flat_map(|(index, syllable, theme_words)| {
                    theme_words
                        .iter()
                        .map(move |theme_word| (index, syllable, theme_word))
                })
                .filter(|(_, syllable, theme_word)| {
                    theme_word.syllables.len() == 1 && &&theme_word.syllables[0] != syllable
                })
                .map(|(index, _, theme_word)| {
                    let replacement_string = Casing::unify_captialization(&original_word
                        .syllables
                        .iter()
                        .take(index)
                        .map(|x| x.get_spelling())
                        .join(""), &phrase_word.text)
                        + &theme_word.spelling
                        + &original_word
                            .syllables
                            .iter()
                            .skip(index + 1)
                            .map(|x| x.get_spelling())
                            .join("");
                    PunReplacement {
                        pun_type: PunType::Infix,
                        is_amalgam: true,
                        pun_word: theme_word.spelling.clone(),
                        replacement_string,
                    }
                })
                .collect_vec();
        }
        return vec![];
    }
}
