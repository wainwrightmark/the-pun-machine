use std::{collections::BTreeMap, str::FromStr};

use itertools::Itertools;

use crate::core::prelude::*;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, serde::Serialize, serde::Deserialize,
)]
pub struct Syllable {
    pub symbols: SymbolArr,
}

impl Syllable {
    ///Add this syllable to the offset of the next syllable
    pub fn add_next_offset(&self, other: &Self) -> Self {
        Self {
            symbols: self.symbols.into_iter().chain(other.onset()).collect(),
        }
    }

    pub fn new<T: IntoIterator<Item = Symbol>>(collection: T) -> Self {
        let symbols = collection.into_iter().collect();
        Self { symbols }
    }

    pub fn onset(&self) -> impl Iterator<Item = Symbol> {
        self.symbols.into_iter().take_while(|&x| !x.is_vowel())
    }

    pub fn nucleus(&self) -> Symbol {
        self.symbols.into_iter().find(|&x| x.is_vowel()).unwrap()
    }

    pub fn coda(&self) -> impl Iterator<Item = Symbol> {
        self.symbols
            .into_iter()
            .skip_while(|x| !x.is_vowel())
            .skip(1)
    }

    pub fn rhymes_with(&self, other: &Self) -> bool {
        self.nucleus() == other.nucleus() && self.coda().eq(other.coda())
    }

    ///Get the rhyming part of this syllable
    pub fn get_rhymes_syllable(&self) -> Syllable {
        let symbols = self
            .symbols
            .into_iter()
            .skip_while(|x| !x.is_vowel())
            .map(|x| x.normalize_vowel())
            .collect();
        Self { symbols }
    }

    ///Get this syllable with the nucleus replaced by 'AA'
    pub fn with_no_consonant(self) -> Syllable {
        Syllable {
            symbols: self
                .symbols
                .into_iter()
                .map(|x| if x.is_vowel() { Symbol::AA } else { x })
                .collect(),
        }
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
                .into_iter()
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
