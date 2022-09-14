use itertools::Itertools;

use crate::core::prelude::*;
use std::convert::TryFrom;

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, serde::Serialize, serde::Deserialize,
)]
pub struct Phrase {
    pub words: Vec<PhoeneticsWord>,
}

impl TryFrom<String> for Phrase {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let words_result: Result<Vec<_>, _> = value
            .split(' ')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| PhoeneticsWord::try_from(x.to_string()))
            .collect();

        let words = words_result?;

        Ok(Phrase { words })
    }
}

impl Phrase {
    pub fn full_text(&self) -> String {
        self.words.iter().map(|z| z.text.clone()).join(" ")
    }
}
