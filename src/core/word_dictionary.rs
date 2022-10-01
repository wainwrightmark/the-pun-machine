use crate::core::prelude::*;
use std::{
    collections::{BTreeMap},
};

#[derive(Clone, Debug, Default, PartialEq, Eq, serde:: Deserialize, serde::Serialize)]
pub struct WordDictionary {
    pub words: Vec<DictionaryWord>,
    pub meanings: BTreeMap<u32, Vec<u32>>,
}


