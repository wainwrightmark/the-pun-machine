use std::{convert::TryFrom, rc::Rc, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;

use serde::*;

use yewdux::prelude::*;

#[derive(PartialEq, Eq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct FullState {
    pub text: String,
    pub category: Option<PunCategory>,
    #[serde(skip)]
    pub data: Rc<Vec<PunPhrase>>,
    pub warning: Option<String>,
}

impl Default for FullState {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            category: None,
            data: Default::default(),
            warning: Default::default(),
        }
    }
}

impl FullState {
    pub fn load_more(&mut self) {
        self.update();
    }

    fn update(&mut self) {
        let phrases: Vec<Phrase> = if let Some(category) = self.category {
            category
                .get_words()
                .filter_map(|text| Phrase::try_from(text.to_string()).ok())
                .collect_vec()
        } else {
            PunCategory::get_all_words()
                .filter_map(|text| Phrase::try_from(text.to_string()).ok())
                .collect_vec()
        };

        match DictionaryWord::from_str(self.text.as_str()) {
            Ok(p_word) => {
                let pun_words = p_word.self_and_children().into_iter().collect_vec();

                let factories = PunFactory::build_all(&pun_words);

                let solutions = phrases
                    .into_iter()
                    .flat_map(|x| PunFactory::solve(&factories, &x))
                    .collect_vec();

                self.warning = None;
                self.data = solutions.into();
            }
            Err(err) => {
                self.data = Default::default();
                self.warning = Some(format!("{}: '{}'", err, self.text));
            }
        }
    }

    pub fn change_text(&mut self, s: String) {
        if self.text.trim() == s.trim() {
        } else {
            self.text = s;

            self.update();
        }
    }

    pub fn change_category(&mut self, pc: Option<PunCategory>) {
        if self.category != pc {
            self.category = pc;

            self.update();
        }
    }
}
