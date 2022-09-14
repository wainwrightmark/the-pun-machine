mod homophone;
mod infix_rhyme;
mod perfect_rhyme;
mod prefix;
mod prefix_rhyme;
mod pun_strategy;
mod same_consonants;
mod shared_prefix;

pub mod prelude {
    pub use crate::core::strategies::pun_strategy::*;

    pub use crate::core::strategies::homophone::*;
    pub use crate::core::strategies::infix_rhyme::*;
    pub use crate::core::strategies::perfect_rhyme::*;
    pub use crate::core::strategies::prefix::*;
    pub use crate::core::strategies::prefix_rhyme::*;
    pub use crate::core::strategies::same_consonants::*;
    pub use crate::core::strategies::shared_prefix::*;
}
