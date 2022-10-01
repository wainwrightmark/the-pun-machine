use std::{
    collections::{BTreeMap, HashSet, VecDeque},
    str::FromStr,
};

use itertools::Itertools;
use lazy_static::__Deref;
use smallvec::SmallVec;

use crate::core::prelude::*;

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
pub struct DictionaryWord<'a> {
    pub spelling: &'a str,
    pub syllables: SmallVec<[Syllable; 4]>,
    pub meanings: SmallVec<[u32; 4]>,
}

impl FromStr for DictionaryWord<'static> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        WORDSBYSPELLING
            .get(s.to_ascii_lowercase().as_str())
            .map(|x| x.first().unwrap().deref().clone())
            .ok_or_else(|| anyhow::anyhow!("Could not find word '{}'", s))
    }
}

impl DictionaryWord<'static> {
    pub fn find_all_puns(&self, category_option: &Option<Category>) -> Vec<PunPhrase> {
        let phrases: Vec<Phrase> = if let Some(category) = category_option {
            category.get_phrases().collect_vec()
        } else {
            Category::get_all_phrases().collect_vec()
        };
        let pun_words = self
            .self_and_children()
            .into_iter()
            .filter(|z|z.spelling.len() > 1)
            .unique_by(|x| x.syllables.clone()) //removes duplicates like mold / mould
            .collect_vec();

        let factories = PunFactory::build_all(&pun_words);

        phrases
            .into_iter()
            .flat_map(|x| PunFactory::solve(&factories, &x))
            .collect_vec()
    }
}

impl DictionaryWord<'static> {
    pub fn self_and_children(&self) -> HashSet<Self> {
        let mut result = HashSet::<Self>::default();
        result.insert(self.clone());
        let mut stack = VecDeque::<u32>::default();
        let mut used_meanings = HashSet::<u32>::default();
        for self_meaning in self.meanings.iter() {
            if used_meanings.insert(*self_meaning) {
                stack.push_back(*self_meaning);
            }
        }

        while let Some(meaning) = stack.pop_front() {
            //return all words that have this meaning (do not check their meanings or you will get synonyms)
            for word in WORDSBYMEANING[&meaning].iter() {
                //println!("{}", word.spellings[0]);
                result.insert(word.clone());
            }
            //add all child meanings we haven't seen yet to the stack
            for child in WORDDICTIONARY.meanings[&meaning].iter() {
                if used_meanings.insert(*child) {
                    stack.push_back(*child);
                }
            }
        }

        result
    }
}

lazy_static::lazy_static! {
    static ref WORDSBYSPELLING : BTreeMap<String, Vec<DictionaryWord<'static>>> = WORDDICTIONARY.words.iter()
    .map(|entry|

            (entry.spelling.to_ascii_lowercase(), entry.clone())



).sorted_by_key(|x|x.0.clone()) .group_by(|x|x.0.clone()).into_iter().map(|x|(x.0,x.1.map(|y|y.1) .collect_vec())) .collect();
}

lazy_static::lazy_static! {
    static ref WORDSBYMEANING : BTreeMap<u32, Vec<DictionaryWord<'static>>> = WORDDICTIONARY.words.iter()
    .flat_map(|entry|
        {
            entry.meanings.iter().map(move |x|(*x, entry.clone()))
        }

).sorted_by_key(|x|x.0) .group_by(|x|x.0).into_iter().map(|x|(x.0,x.1.map(|y|y.1) .collect_vec())) .collect();
}
