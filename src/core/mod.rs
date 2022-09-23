mod casing;
mod phrase;
mod pronounciation;
mod pun;
mod pun_category;
mod pun_replacement;
mod pun_strategy;
mod pun_type;
mod related_word;
mod strategies;
mod theme_suggestion;
mod word_dictionary;
mod helpers;

pub mod prelude {

    pub use crate::core::casing::*;
    pub use crate::core::phrase::*;
    pub use crate::core::pronounciation::prelude::*;
    pub use crate::core::pun::*;
    pub use crate::core::pun_category::*;
    pub use crate::core::pun_replacement::*;
    pub use crate::core::pun_strategy::*;
    pub use crate::core::pun_type::*;
    pub use crate::core::related_word::*;
    pub use crate::core::strategies::prelude::*;
    pub use crate::core::theme_suggestion::*;
    pub use crate::core::word_dictionary::*;
    pub use crate::core::helpers::*;
}
