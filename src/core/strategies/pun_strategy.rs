use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use strum::EnumIter;

use crate::core::prelude::*;
use crate::core::strategies::prelude::*;

#[enum_dispatch]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, EnumIter)]
pub enum PunStrategyEnum {
    Homophone,
     PerfectRhyme,
    Prefix,
    // PrefixRhyme,
    // SameConsonants,
    InfixRhyme,
    // SharedPrefix,
}

#[enum_dispatch(PunStrategyEnum)]
pub trait PunStrategy {
    fn get_relevant_syllables(&self, word: &PhoeneticsWord) -> Vec<Vec<Syllable>>;

    fn get_possible_replacements(
        &self,
        original_word: &PhoeneticsWord,
        dict: &HashMap<Vec<Syllable>, Vec<PhoeneticsWord>>,
    ) -> Vec<PunReplacement>;
}