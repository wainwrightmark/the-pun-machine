use crate::core::prelude::*;
use std::{
    collections::{BTreeMap},
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde:: Deserialize, serde::Serialize)]
pub struct WordDictionary {
    pub words: Vec<DictionaryWord>,
    pub meanings: BTreeMap<u32, Vec<u32>>,
}

lazy_static::lazy_static! {
    pub static ref WORDDICTIONARY: WordDictionary = rmp_serde::from_slice(&WORDDICTIONARYSTR).unwrap();
}

include_flate::flate!(pub static WORDDICTIONARYSTR: [u8] from "data.mp");
