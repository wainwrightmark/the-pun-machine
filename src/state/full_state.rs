use std::{rc::Rc, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;

use serde::*;

use yewdux::{prelude::*, storage::{self, StorageListener}};

#[derive(PartialEq, Eq,  Clone, Serialize, Deserialize)]
pub struct FullState {
    pub text: String,
    pub category: Option<Category>,
    #[serde(skip)]
    pub data: Rc<Vec<PunPhrase>>,
    pub warning: Option<String>,
}

impl Default for FullState {
    fn default() -> Self {
        let mut state = Self {
            text: "potato".to_string(),
            category: None,
            data: Default::default(),
            warning: Default::default(),
        };
        state.update();
        state
    }
}

impl Store for FullState {
    fn new() -> Self {
        init_listener(StorageListener::<Self>::new(storage::Area::Local));

        let mut result: FullState =
        storage::load(storage::Area::Local)
            .expect("Unable to load state")
            .unwrap_or_default();
        result.update();

        result
    }

    fn should_notify(&self, other: &Self) -> bool {
        self != other
    }
}

impl FullState {
    

    fn update(&mut self) {
        let phrases: Vec<Phrase> = if let Some(category) = self.category {
            category.get_phrases().collect_vec()
        } else {
            Category::get_all_phrases().collect_vec()
        };

        match DictionaryWord::from_str(self.text.as_str()) {
            Ok(p_word) => {
                let pun_words = p_word
                .self_and_children()
                .into_iter()
                .unique_by(|x|x.syllables.clone()) //removes duplicates like mold / mould
                .collect_vec();

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

    pub fn change_category(&mut self, pc: Option<Category>) {
        if self.category != pc {
            self.category = pc;

            self.update();
        }
    }
}
