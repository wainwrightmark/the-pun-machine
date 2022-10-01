use std::{collections::BTreeMap, convert::TryFrom, str::FromStr};

use anyhow::anyhow;
use itertools::Itertools;
use regex::Regex;
use smallvec::SmallVec;
use the_pun_machine::core::prelude::*;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, serde::Serialize, serde::Deserialize,
)]
pub struct PhoeneticsWord {
    pub text: String,
    pub variant: u8,
    pub is_compound: bool,
    pub syllables: SmallVec<[Syllable; 4]>,
}

impl TryFrom<String> for PhoeneticsWord {
    type Error = anyhow::Error;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        let splits = text
            .split('-')
            // .map(|x|{
            //     let mut s = x.to_string();
            //     s.retain(|c|c.is_ascii_alphabetic());
            //     s
            // })
            .filter(|x| !x.is_empty())
            .collect_vec();

        if splits.len() == 1 {
            Self::try_get_single(splits[0].to_string())
        } else {
            let words: Vec<_> = splits
                .into_iter()
                .map(|x| Self::try_get_single(x.to_string()))
                .try_collect()?;
            let syllables = words.into_iter().flat_map(|z| z.syllables).collect();

            Ok(PhoeneticsWord {
                syllables,
                is_compound: true,
                text,
                variant: 1,
            })
        }
    }
}

include_flate::flate!(pub static PRONOUNCIATIONS: str from "data/syllables/pronounciation.txt");

impl PhoeneticsWord {
    fn try_get_single(text: String) -> Result<Self, anyhow::Error> {
        lazy_static::lazy_static! {
            static ref PRONOUNCIATIONS_MAP: BTreeMap<String, PhoeneticsWord> = PhoeneticsWord::create_words_map();
        }

        let mut key = text.to_ascii_lowercase();
        key.retain(|c| c.is_ascii_alphabetic());

        if let Some(w) = PRONOUNCIATIONS_MAP.get(&key) {
            Ok(PhoeneticsWord {
                text,
                variant: w.variant,
                is_compound: w.is_compound,
                syllables: w.syllables.clone(),
            })
        } else {
            Err(anyhow!("Word not found {}", text))
        }
    }

    fn create_words_map() -> BTreeMap<String, PhoeneticsWord> {
        PRONOUNCIATIONS
            .lines()
            .map(|l| {
                let terms = l
                    .split_ascii_whitespace()
                    .map(|x| x.trim())
                    .filter(|x| !x.is_empty())
                    .collect_vec();

                assert!(terms.len() > 1);
                let t1 = terms[0];

                lazy_static::lazy_static! {
                    static ref RE: Regex = Regex::new(r"^(.+?)\((\d+)\)$").unwrap();
                }

                let (text, variant) = if let Some(captures) = RE.captures(t1) {
                    (
                        captures.get(1).unwrap().as_str(),
                        captures.get(2).unwrap().as_str().parse::<u8>().unwrap(),
                    )
                } else {
                    (t1, 1)
                };

                let mut syllables = SmallVec::<[Syllable; 4]>::new();
                let mut symbols = Vec::<Symbol>::new();

                for symbol_string in terms.into_iter().skip(1) {
                    if symbol_string == "-" {
                        if !symbols.is_empty() {
                            syllables.push(Syllable::new(symbols));
                            symbols = Vec::new();
                        }
                    } else if let Ok(symbol) = Symbol::from_str(symbol_string) {
                        symbols.push(symbol);
                    } else {
                        panic!("'{}' was not a valid symbol", symbol_string);
                    }
                }
                if !symbols.is_empty() {
                    syllables.push(Syllable::new(symbols))
                }
                let pw = PhoeneticsWord {
                    text: text.to_string(),
                    variant,
                    is_compound: false,
                    syllables,
                };

                (text.to_ascii_lowercase(), pw)
            })
            .collect()
    }
}
