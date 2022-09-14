use itertools::Itertools;
use std::{collections::HashMap, vec};

use crate::core::prelude::*;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Default)]
pub struct Prefix {}

impl PunStrategy for Prefix {
    fn get_relevant_syllables(&self,word: &PhoeneticsWord) -> Vec<Vec<Syllable>> {
        if word.syllables.len() <= 1{
            return vec![];
        }

        return 
        (1..word.syllables.len() - 1)
        .map(|l| word.syllables.iter().take(l).cloned().collect_vec())

        .chain(
            (1..word.syllables.len() - 1)
        .map(|l| word.syllables.iter().take(l - 1).cloned()
        .chain(std::iter::once(word.syllables[l-1].add_next_offset(&word.syllables[l])))
        
        .collect_vec())

        )

        .collect_vec();
    }

    fn get_possible_replacements(&self,original_word: &PhoeneticsWord,dict: &HashMap<Vec<Syllable>,Vec<PhoeneticsWord>>,) -> Vec<PunReplacement> {
        if let Some(words) = dict.get(&original_word.syllables){

            words.into_iter().map(|word|{

                if word.text.starts_with(original_word.text.as_str()){
                    PunReplacement{
                        pun_type: PunType::Prefix,
                        pun_word: word.text.clone(),
                        replacement_string: word.text.clone(),
                        is_amalgam: false,
                    }
                }
                else{
                    let suffix = word.syllables.iter().skip(1).map(Syllable::get_spelling).join("");
                    let replacement_string = original_word.text.clone() + suffix.as_str();
                    
                    PunReplacement{
                        pun_type: PunType::Prefix,
                        pun_word: word.text.clone(),
                        replacement_string,
                        is_amalgam: true,
                    }
                }

                
            }).collect_vec()
        }
        else{
            return vec![];
        }
    }
}
