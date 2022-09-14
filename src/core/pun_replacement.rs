use itertools::Itertools;

use crate::core::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, serde::Serialize, serde::Deserialize)]
pub struct PunReplacement{
    pub pun_type: PunType,

    pub pun_word: String,
    pub replacement_string: String,
    pub is_amalgam: bool
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, serde::Serialize, serde::Deserialize)]
pub struct PunPhrase{
    pub phrase : Phrase,
    pub replacement : PunReplacement,
    pub index: usize
}

impl PunPhrase{
    pub fn replacement_text(&self)-> String{
        self.phrase.words.iter().enumerate()
        .map(|(i,w)| if i == self.index{self.replacement.replacement_string.clone()} else {w.text.clone()})
        .join(" ")
    }
}