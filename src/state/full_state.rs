use std::{collections::HashSet, rc::Rc, str::FromStr};

use crate::core::prelude::*;
use itertools::Itertools;

use serde::*;

use yewdux::{
    prelude::*,
    storage::{self, StorageListener},
};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct FullState {
    pub text: String,
    pub category: Option<Category>,
    #[serde(skip)]
    pub data: Rc<Vec<PunPhrase>>,
    pub warning: Option<String>,

    #[serde(skip)]
    pub visible_groups: HashSet<String>,
}

impl Default for FullState {
    fn default() -> Self {
        let mut state = Self {
            text: "potato".to_string(),
            category: None,
            data: Default::default(),
            warning: Default::default(),
            visible_groups: Default::default(),
        };
        state.update();
        state
    }
}

impl Store for FullState {
    fn new() -> Self {
        init_listener(StorageListener::<Self>::new(storage::Area::Local));

        let mut result: FullState = storage::load(storage::Area::Local)
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
    pub fn toggle_group_visibility(&mut self, key: &String) {
        if !self.visible_groups.remove(key) {
            self.visible_groups.insert(key.clone());
        }
    }

    fn update(&mut self) {
        match DictionaryWord::from_str(self.text.as_str()) {
            Ok(p_word) => {
                let solutions = p_word.find_all_puns(&self.category);

                self.warning = None;
                self.data = solutions.into();
            }
            Err(err) => {
                self.data = Default::default();
                self.warning = Some(format!("{}: '{}'", err, self.text));
            }
        }
        self.visible_groups = Default::default();
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
