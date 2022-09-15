use include_flate::flate;
use strum::*;

impl core::fmt::Display for PunCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text : &'static str = match self {
            PunCategory::Artists => "Artists",
            PunCategory::Idiom => "Idiom",
            PunCategory::Books => "Books",
            PunCategory::Movies => "Movies",
            PunCategory::Musicals => "Musicals",
            PunCategory::Songs => "Songs",
            PunCategory::Bands => "Bands",
            PunCategory::Wedding => "Wedding",
            PunCategory::MovieQuotes => "Movie Quotes",
            PunCategory::Brands => "Artists",
            PunCategory::Celebs => "Celebs",
            PunCategory::Countries => "Countries",
            PunCategory::TVShows => "TV Shows",
            PunCategory::CountrySongs => "Country Songs",
            PunCategory::ChristmasSongs => "Christmas Songs",
        };

        write!(f, "{}",text)
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
pub enum PunCategory {
    Artists,
    Idiom,
    Books,

    Movies,
    Musicals,
    Songs,
    Bands,

    Wedding,
    MovieQuotes,

    Brands,
    Celebs,
    Countries,
    TVShows,

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

impl PunCategory {

    pub fn get_all_words() -> impl Iterator<Item = &'static str> {
        PunCategory::iter().flat_map(|x|x.get_words())
    }

    pub fn get_words(self) -> impl Iterator<Item = &'static str> {
        match self {
            PunCategory::Artists => ARTISTS.lines(),
            PunCategory::Idiom => IDIOMS.lines(),
            PunCategory::Books => BOOKS.lines(),
            PunCategory::Movies => MOVIES.lines(),
            PunCategory::Musicals => MUSICALS.lines(),
            PunCategory::Songs => SONGS.lines(),
            PunCategory::Bands => BANDS.lines(),
            PunCategory::Wedding => WEDDING.lines(),
            PunCategory::MovieQuotes => MOVIEQUOTES.lines(),
            PunCategory::Brands => BRANDS.lines(),
            PunCategory::Celebs => CELEBS.lines(),
            PunCategory::Countries => COUNTRIES.lines(),
            PunCategory::TVShows => TVSHOWS.lines(),
            PunCategory::CountrySongs => COUNTRYSONGS.lines(),
            PunCategory::ChristmasSongs => CHRISTMASSONGS.lines(),
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
