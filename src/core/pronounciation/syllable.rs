use std::{collections::BTreeMap, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, serde::Serialize, serde::Deserialize,
)]
pub struct Syllable {
    onset_len: usize,
    pub symbols: Vec<Symbol>,
}

impl Syllable {
    ///Add this syllable to the offset of the next syllable
    pub fn add_next_offset(&self, other: &Self) -> Self {
        Self {
            onset_len: self.onset_len,
            symbols: self
                .symbols
                .iter()
                .chain(other.onset())
                .cloned()
                .collect_vec(),
        }
    }

    pub fn new<T: IntoIterator<Item = Symbol>>(collection: T) -> Self {
        let symbols = collection.into_iter().collect_vec();
        let onset_len = symbols.iter().take_while(|&x| !x.is_vowel()).count();

        Self { onset_len, symbols }
    }

    pub fn onset<'l>(&'l self) -> impl Iterator<Item = &'l Symbol> {
        self.symbols.iter().take(self.onset_len)
    }

    pub fn nucleus(&self) -> Symbol {
        self.symbols[self.onset_len]
    }

    pub fn coda<'l>(&'l self) -> impl Iterator<Item = &'l Symbol> {
        self.symbols.iter().skip(self.onset_len + 1)
    }

    pub fn rhymes_with(&self, other: &Self) -> bool {
        self.nucleus() == other.nucleus() && self.coda().eq(other.coda())
    }

    ///Get the rhyming part of this syllable
    pub fn get_rhymes_syllable(&self) -> Syllable {
        if self.onset_len == 0 {
            self.clone()
        } else {
            let symbols = self
                .symbols
                .iter()
                .skip(self.onset_len)
                .cloned()
                .collect_vec();
            Self {
                onset_len: 0,
                symbols,
            }
        }
    }

    ///Get this syllable with the nucleus replaced by 'AA'
    pub fn with_no_consonant(mut self) -> Syllable {
        self.symbols[self.onset_len] = Symbol::AA;
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
                        .replace('0', "")
                        .replace('1', "")
                        .replace('2', "")
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
