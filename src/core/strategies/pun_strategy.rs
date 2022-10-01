use std::collections::HashMap;

use enum_dispatch::enum_dispatch;
use smallvec::SmallVec;
use strum::EnumIter;

use crate::core::{prelude::*, strategies::prelude::*};

#[enum_dispatch]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, EnumIter)]
pub enum PunStrategyEnum {
    Homophone,
    PerfectRhyme,
    Prefix,
    PrefixRhyme,
    SameConsonants,
    InfixRhyme,
    SharedPrefix,
}

#[enum_dispatch(PunStrategyEnum)]
pub trait PunStrategy {
    fn get_relevant_syllables(
        &self,
        word: &DictionaryWord<'static>,
    ) -> SmallVec<[SmallVec<[Syllable; 4]>; 2]>;

    fn get_possible_replacements(
        &self,
        original_word: &PhraseWord,
        dict: &HashMap<SmallVec<[Syllable; 4]>, Vec<DictionaryWord<'static>>>,
    ) -> Vec<PunReplacement>;
}
