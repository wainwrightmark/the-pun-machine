use include_flate::flate;
use strum::*;

use super::prelude::Phrase;

impl core::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: &'static str = match self {
            Category::Artists => "Artists",
            Category::Idiom => "Idiom",
            Category::Books => "Books",
            Category::Movies => "Movies",
            Category::Musicals => "Musicals",
            Category::Songs => "Songs",
            Category::Bands => "Bands",
            Category::Wedding => "Wedding",
            Category::MovieQuotes => "Movie Quotes",
            Category::Brands => "Artists",
            Category::Celebs => "Celebs",
            Category::Countries => "Countries",
            Category::TVShows => "TV Shows",
            Category::CountrySongs => "Country Songs",
            Category::ChristmasSongs => "Christmas Songs",
        };

        write!(f, "{}", text)
    }
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    EnumString,
    EnumIter,
    IntoStaticStr,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum Category {
    //Note the ordering of this really matters
    Idiom,
    Movies,
    Bands,
    Books,
    Songs,
    Artists,
    Countries,
    TVShows,

    Musicals,

    Wedding,
    MovieQuotes,

    Brands,
    Celebs,

    CountrySongs,
    ChristmasSongs,
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

impl Category {
    pub fn get_phrases(self) -> impl Iterator<Item = Phrase> {
        self.get_words()
            .map(move |text| Phrase::new(text.to_string(), self))
    }

    pub fn get_all_phrases() -> impl Iterator<Item = Phrase> {
        Category::iter().flat_map(|x| x.get_phrases())
    }

    pub fn get_words(self) -> impl Iterator<Item = &'static str> {
        match self {
            Category::Artists => ARTISTS.lines(),
            Category::Idiom => IDIOMS.lines(),
            Category::Books => BOOKS.lines(),
            Category::Movies => MOVIES.lines(),
            Category::Musicals => MUSICALS.lines(),
            Category::Songs => SONGS.lines(),
            Category::Bands => BANDS.lines(),
            Category::Wedding => WEDDING.lines(),
            Category::MovieQuotes => MOVIEQUOTES.lines(),
            Category::Brands => BRANDS.lines(),
            Category::Celebs => CELEBS.lines(),
            Category::Countries => COUNTRIES.lines(),
            Category::TVShows => TVSHOWS.lines(),
            Category::CountrySongs => COUNTRYSONGS.lines(),
            Category::ChristmasSongs => CHRISTMASSONGS.lines(),
        }
    }
}

flate!(pub static ARTISTS: str from "data/categories/Artists.txt");
flate!(pub static IDIOMS: str from "data/categories/Idioms.txt");
flate!(pub static BOOKS: str from "data/categories/Books.txt");
flate!(pub static BRANDS: str from "data/categories/Brands.txt");
flate!(pub static BANDS: str from "data/categories/Bands.txt");
flate!(pub static CELEBS: str from "data/categories/Celebs.txt");
flate!(pub static COUNTRYSONGS: str from "data/categories/CountrySongs.txt");
flate!(pub static CHRISTMASSONGS: str from "data/categories/ChristmasSongs.txt");
flate!(pub static COUNTRIES: str from "data/categories/Countries.txt");
flate!(pub static MOVIEQUOTES: str from "data/categories/MovieQuotes.txt");
flate!(pub static MOVIES: str from "data/categories/Movies.txt");
flate!(pub static MUSICALS: str from "data/categories/Musicals.txt");
flate!(pub static SONGS: str from "data/categories/Songs.txt");
//flate!(pub static THEMESUGGESTIONS: str from "data/categories/ThemeSuggestions.txt");
flate!(pub static TVSHOWS: str from "data/categories/TVShows.txt");
flate!(pub static WEDDING: str from "data/categories/Wedding.txt");
