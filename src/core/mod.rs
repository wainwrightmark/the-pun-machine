mod casing;
mod pun;
mod pun_category;
mod pun_replacement;
mod pun_strategy;
mod pun_type;
mod related_word;
mod theme_suggestion;
mod pronounciation;
mod strategies;
mod phrase;

pub mod prelude {

    pub use crate::core::casing::*;
    pub use crate::core::pun::*;
    pub use crate::core::pun_category::*;
    pub use crate::core::pun_replacement::*;
    pub use crate::core::pun_strategy::*;
    pub use crate::core::pun_type::*;
    pub use crate::core::related_word::*;
    pub use crate::core::theme_suggestion::*;
    pub use crate::core::phrase::*;
    pub use crate::core::pronounciation::prelude::*;
    pub use crate::core::strategies::prelude::*;
}


