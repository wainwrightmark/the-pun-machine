use std::{collections::BTreeMap, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;
use arrayvec::ArrayVec;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, serde::Serialize, serde::Deserialize,
)]
pub struct Syllable {
    pub symbols: ArrayVec<Symbol,8>,
}

impl Syllable {
    ///Add this syllable to the offset of the next syllable
    pub fn add_next_offset(&self, other: &Self) -> Self {
        Self {
            symbols: self
                .symbols
                .iter()
                .chain(other.onset())
                .cloned()
                .collect(),
        }
    }

    pub fn new<T: IntoIterator<Item = Symbol>>(collection: T) -> Self {
        let symbols = collection.into_iter().collect();
        Self {  symbols }
    }

    pub fn onset<'l>(&'l self) -> impl Iterator<Item = &'l Symbol> {
        self.symbols.iter().take_while(|&x| !x.is_vowel())
    }

    pub fn nucleus(&self) -> Symbol {
        self.symbols.iter().filter(|&x| x.is_vowel()).next().unwrap().clone()
    }

    pub fn coda<'l>(&'l self) -> impl Iterator<Item = &'l Symbol> {
        self.symbols.iter().skip_while(|x|!x.is_vowel()).skip(1)
    }

    pub fn rhymes_with(&self, other: &Self) -> bool {
        self.nucleus() == other.nucleus() && self.coda().eq(other.coda())
    }

    ///Get the rhyming part of this syllable
    pub fn get_rhymes_syllable(&self) -> Syllable {
        let symbols = self
                .symbols
                .iter()
                .skip_while(|x|!x.is_vowel())
                .map(|x| x.normalize_vowel())
                .collect();
            Self {
                symbols,
            }
    }

    ///Get this syllable with the nucleus replaced by 'AA'
    pub fn with_no_consonant(mut self) -> Syllable {
        for s in self.symbols.iter_mut(){
            if s.is_vowel(){                
                *s = Symbol::AA;
                return self;
            }
        }
        self
    }
}

include_flate::flate!(pub static SPELLINGS: str from "data/syllables/spelling.txt");

impl Syllable {
    pub fn get_spelling(&self) -> String {
        //TODO improve perf of this

        lazy_static::lazy_static! {
            static ref SPELLINGS_MAP: BTreeMap<Syllable, String> = Syllable::create_spellings_map();
        }

        if let Some(string) = SPELLINGS_MAP.get(self) {
            string.clone()
        } else {
            self.symbols
                .iter()
                .map(|x| {
                    x.to_string()
                        .replace(['0', '1', '2'], "")
                        .to_ascii_lowercase()
                })
                .join("")
        }
    }

    fn create_spellings_map() -> BTreeMap<Syllable, String> {
        SPELLINGS
            .lines()
            .map(|l| {
                let symbols = l.split_ascii_whitespace().collect_vec();
                let spelling = symbols.last().unwrap().to_ascii_lowercase();

                let syllable = Syllable::new(
                    symbols
                        .iter()
                        .take(symbols.len() - 1)
                        .map(|z| Symbol::from_str(z).unwrap()),
                );

                (syllable, spelling)
            })
            .collect()
    }
}
