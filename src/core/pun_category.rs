use include_flate::flate;
use strum::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, EnumString, EnumIter,IntoStaticStr, serde::Serialize, serde::Deserialize)]
pub enum PunCategory {
    Artists,
    Idiom,
    //TODO flate and include
    // Books,

    // Movies,
    // Musicals,
    // Songs,
    // Bands,

    // Wedding,
    // MovieQuotes,

    // Brands,
    // Celebs,
    // Countries,
    // TVShows,

    // CountrySongs,
    // ChristmasSongs,

    //TODO
    //Historical figures
    //Historical events
    //Cocktails
    //Songs
    //Christmas Songs
    //Geography
    //Shakespeare
    //Movie Quotes
    //Video Games
    //Major cities
}

impl PunCategory {
    pub fn get_words(self) -> impl Iterator<Item = &'static str> {
        match self {
            PunCategory::Artists => ARTISTS.lines(),
            PunCategory::Idiom => IDIOMS.lines(),
        }
    }
}

flate!(pub static ARTISTS: str from "data/categories/Artists.txt");
flate!(pub static IDIOMS: str from "data/categories/Idioms.txt");
