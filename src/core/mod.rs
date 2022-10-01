mod casing;
mod category;
mod dictionary_word;
mod phrase;
mod pronounciation;
mod pun_replacement;
mod pun_strategy;
mod pun_type;
mod related_word;
mod strategies;
mod theme_suggestion;
mod word_dictionary;

pub mod prelude {

    pub use crate::core::{
        casing::*, category::*, dictionary_word::*, phrase::*, pronounciation::prelude::*,
        pun_replacement::*, pun_strategy::*, pun_type::*, related_word::*, strategies::prelude::*,
        theme_suggestion::*, word_dictionary::*,
    };
}
