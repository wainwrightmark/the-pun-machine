use itertools::Itertools;
use lazy_static::__Deref;

use crate::core::prelude::*;
use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    str::FromStr
};

#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde:: Deserialize,
    serde::Serialize,
)]
pub struct DictionaryWord {
    pub spellings: Vec<String>,
    pub syllables: Vec<Syllable>,
    pub meanings: Vec<u32>,
}

impl FromStr for DictionaryWord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        WORDSBYSPELLING
            .get(s.to_ascii_lowercase().as_str())
            .map(|x| x.first().unwrap().deref() .clone())
            .ok_or(anyhow::anyhow!("Could not find word '{}'", s))
    }
}

impl DictionaryWord {
    pub fn self_and_children(&self) -> HashSet<DictionaryWord> {
        


        let mut result = HashSet::<DictionaryWord>::default();
        result.insert(self.clone());
        let mut stack = VecDeque::<u32>::default();
        let mut used_meanings = HashSet::<u32>::default();
        for self_meaning in self.meanings.iter(){
            if used_meanings.insert(self_meaning.clone()){
                stack.push_back(self_meaning.clone());
            }            
        }

        while let Some(meaning) = stack.pop_front() {
            //return all words that have this meaning (do not check their meanings or you will get synonyms)
            for word in WORDSBYMEANING[&meaning].iter(){
                println!("{}", word.spellings[0]);
                result.insert(word.clone());
            }
            //add all child meanings we haven't seen yet to the stack
            for child in WORDDICTIONARY.meanings[&meaning].iter(){
                if used_meanings.insert(child.clone()){
                    stack.push_back(child.clone());
                }
            }
        }

        result
    }
}

#[derive(Clone, Debug, Default, PartialEq, serde:: Deserialize, serde::Serialize)]
pub struct WordDictionary {
    pub words: Vec<DictionaryWord>,
    pub meanings: BTreeMap<u32, Vec<u32>>,
}

lazy_static::lazy_static! {
    static ref WORDSBYMEANING : BTreeMap<u32, Vec<DictionaryWord>> = WORDDICTIONARY.words.iter()
    .flat_map(|entry|
        {
            entry.meanings.iter().map(move |x|(x.clone(), entry.clone()))
        }

).sorted_by_key(|x|x.0) .group_by(|x|x.0).into_iter().map(|x|(x.0,x.1.map(|y|y.1) .collect_vec())) .collect();
}

lazy_static::lazy_static! {
    static ref WORDSBYSPELLING : BTreeMap<String, Vec<DictionaryWord>> = WORDDICTIONARY.words.iter()
    .flat_map(|entry|
        {
            entry.spellings.iter().map(|spelling|spelling.to_ascii_lowercase().clone()) .map(move |x|(x, entry.clone()))
        }

).sorted_by_key(|x|x.0.clone()) .group_by(|x|x.0.clone()).into_iter().map(|x|(x.0,x.1.map(|y|y.1) .collect_vec())) .collect();
}

lazy_static::lazy_static! {
    static ref WORDDICTIONARY: WordDictionary = rmp_serde::from_slice(&WORDDICTIONARYSTR).unwrap();
}

include_flate::flate!(pub static WORDDICTIONARYSTR: [u8] from "data.mp");
