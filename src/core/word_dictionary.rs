use itertools::Itertools;
use regex::Regex;

use crate::core::prelude::*;
use std::{collections::BTreeMap, convert::TryFrom, str::FromStr};

#[derive(Clone, Debug, Default, PartialEq,serde:: Deserialize, serde::Serialize)]
pub struct WordDictionary {
    pub words: BTreeMap<String, (Vec<Syllable>, Vec<u32>)>,
    pub meanings: BTreeMap<u32, Vec<u32>>,
}

lazy_static::lazy_static! {
    static ref WORDDICTIONARY: WordDictionary = rmp_serde::from_slice(&WORDDICTIONARYSTR).unwrap();
}

include_flate::flate!(pub static WORDDICTIONARYSTR: [u8] from "data.mp");