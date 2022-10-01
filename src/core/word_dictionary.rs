use std::collections::BTreeMap;

use crate::core::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde:: Deserialize, serde::Serialize)]
#[serde(bound(deserialize = "'de: 'a"))]

pub struct WordDictionary<'a> {
    pub words: Vec<DictionaryWord<'a>>,
    pub meanings: BTreeMap<u32, Vec<u32>>,
}

lazy_static::lazy_static! {
    pub static ref WORDDICTIONARY: WordDictionary<'static> = rmp_serde::from_slice(&WORDDICTIONARYSTR).unwrap();
}

include_flate::flate!(pub static WORDDICTIONARYSTR: [u8] from "data.mp");
