mod homophone;
mod infix_rhyme;
mod perfect_rhyme;
mod prefix;
mod prefix_rhyme;
mod pun_strategy;
mod same_consonants;
mod shared_prefix;

pub mod prelude {
    pub use crate::core::strategies::{
        homophone::*, infix_rhyme::*, perfect_rhyme::*, prefix::*, prefix_rhyme::*,
        pun_strategy::*, same_consonants::*, shared_prefix::*,
    };
}
