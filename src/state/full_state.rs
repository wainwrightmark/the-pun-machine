use std::{convert::TryFrom, rc::Rc};

use crate::core::prelude::*;
use itertools::Itertools;
use serde::*;

use yewdux::prelude::*;

#[derive(PartialEq, Store, Clone, Serialize, Deserialize)]
#[store(storage = "local")] // can also be "session"
pub struct FullState {
    pub text: String,
    pub category: PunCategory,
    pub data: Rc<Vec<PunPhrase>>,
    pub warning: Option<String>,
}

impl Default for FullState {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            category: PunCategory::Idiom,
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
        let phrases: Vec<Phrase> = self
            .category
            .get_words()
            .filter_map(|text| Phrase::try_from(text.to_string()).ok())
            .collect_vec();

        match PhoeneticsWord::try_from(self.text.clone()) {
            Ok(p_word) => {
                let pun_words = vec![p_word];

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
                self.warning = Some(format!("{} {}", err.to_string(), self.text));
                return;
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

    pub fn change_category(&mut self, pc: PunCategory) {
        if self.category != pc {
            self.category = pc;

            self.update();
        }
    }
}
