use itertools::Itertools;
use std::{collections::HashMap, vec};

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct SameConsonants {}

impl  SameConsonants {
    fn get_consonant_syllables(&self,word: &PhoeneticsWord)->Vec<Syllable>{
        word.syllables.iter().map(|x|x.clone().with_no_consonant()).collect_vec()
    }
}

impl PunStrategy for SameConsonants {

    

    fn get_relevant_syllables(&self,word: &PhoeneticsWord) -> Vec<Vec<Syllable>> {
        vec![self.get_consonant_syllables(word)]
    }

    fn get_possible_replacements(&self,original_word: &PhoeneticsWord,dict: &HashMap<Vec<Syllable>,Vec<PhoeneticsWord>>,) -> Vec<PunReplacement> {
        let sw = self.get_consonant_syllables(original_word);

        if let Some(theme_words) = dict.get(&sw) {
            return theme_words
                .iter()
                .filter(|theme_word| !theme_word.text.eq_ignore_ascii_case(&original_word.text))
                .map(|theme_word| PunReplacement {
                    pun_type: PunType::SameConsonants,
                    is_amalgam: false,
                    pun_word: theme_word.text.clone(),
                    replacement_string: theme_word.text.clone()
                })
                .collect_vec();
        }
        vec![]
    }
}
