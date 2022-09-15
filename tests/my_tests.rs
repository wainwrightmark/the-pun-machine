use std::{convert::TryFrom, str::FromStr};

use itertools::Itertools;
use the_pun_machine::core::prelude::*;

use ntest::test_case;

#[test_case("Lichtenstein", "L IH1 K T AH0 N S T AY2 N")]
#[test_case("Picasso", "P IH0 K AA1 S OW0")]
#[test_case("Pick", "P IH1 K")]
#[test_case("FISH", "F IH1 SH")]
#[test_case("fish", "F IH1 SH")]
#[test_case("HAPPY", "HH AE1 P IY0")]

fn test_syllables(input: &str, expected: &str) -> Result<(), &'static str> {
    let word = PhoeneticsWord::try_from(input.to_string())?;

    assert_eq!(word.text.to_ascii_lowercase(), input.to_ascii_lowercase());
    //assert_eq!(word.variant, 1);
    assert_eq!(word.is_compound, false);

    let s_text = word
        .syllables
        .iter()
        .flat_map(|x| x.symbols.iter())
        .join(" ");

    assert_eq!(s_text, expected);

    Ok(())
}

#[test_case("pick", "Lichtenstein", "PrefixRhyme", "Picktonstein")]
#[test_case("pick", "Picasso", "PrefixRhyme", "Pickcoso")]
#[test_case("far", "carnage", "PrefixRhyme", "farnage")]
#[test_case("colt", "bolt", "PerfectRhyme", "colt")]
#[test_case("here", "appear", "PerfectRhyme", "ahere")]
#[test_case("knight", "night", "SameWord", "knight")]
#[test_case("night", "night", "Identity", "night")]
#[test_case("ray", "amazed", "InfixRhyme", "arayzed")]
#[test_case("artichoke", "art", "Prefix", "artichoke")]
#[test_case("cinema", "sin", "Prefix", "sinnama")]
#[test_case("butterscotch", "butterfield", "SharedPrefix", "butterscotch")]
#[test_case("butterfield", "butterscotch", "SharedPrefix", "butterfield")]
#[test_case("pisces", "pieces", "SameConsonants", "pisces")]
#[test_case("pieces", "pisces", "SameConsonants", "pieces")]

fn test_pun_classification(
    theme_word_str: &str,
    original_word_str: &str,
    pun_type_str: &str,
    rep: &str,
) -> Result<(), &'static str> {
    let theme_word: PhoeneticsWord = PhoeneticsWord::try_from(theme_word_str.to_string())?;
    let phrase = Phrase::try_from(original_word_str.to_string())?;

    let theme_words = vec![theme_word];

    let factories = PunFactory::build_all(&theme_words);

    let solution = PunFactory::solve(&factories, &phrase);

    if let Ok(expected) = PunType::from_str(pun_type_str) {
        assert!(!solution.is_empty());
        assert_eq!(solution[0].replacement.pun_type, expected);

        assert_eq!(
            solution[0].replacement_text().to_ascii_lowercase(),
            rep.to_ascii_lowercase()
        );
    } else {
        assert!(solution.is_empty());
    }
    Ok(())
}

#[test_case("Idiom", "a bed of roses")]
#[test_case("TVShows", "Doctor Who")]
fn test_cateogry_words(category_text: &str, expected_text: &str) -> Result<(), String> {
    let category = PunCategory::from_str(category_text).map_err(|e| e.to_string())?;

    let category_phrases: Vec<Phrase> = category
        .get_words()
        .filter_map(|t| Phrase::try_from(t.to_string()).ok())
        .collect_vec();

    let expected_phrase = Phrase::try_from(expected_text.to_string()).unwrap();

    assert!(category_phrases.contains(&expected_phrase));

    Ok(())
}

#[test_case("idiom", "idium")]
#[test_case("amazed", "amazed")]
#[test_case("fantastic", "fantastic")]
#[test_case("deplorable", "deplaurable")]
fn test_spelling(word: &str, expected: &str) -> Result<(), String>{
    let word = PhoeneticsWord::try_from(word.to_string())?;

    let spelling = word.syllables.into_iter().map(|x|x.get_spelling()).join("");

    assert_eq!(spelling, expected);

    Ok(())
}


#[test_case("Idiom", "cake")]
#[test_case("TVShows", "meat")]
fn test_puns(category_text: &str, text: &str) -> Result<(), String> {
    let category = PunCategory::from_str(category_text).map_err(|e| e.to_string())?;

    let phrases: Vec<Phrase> = category
        .get_words()
        .filter_map(|t| Phrase::try_from(t.to_string()).ok())
        .collect_vec();

    let p_word = PhoeneticsWord::try_from(text.to_string()).map_err(|e| e.to_string())?;

    let pun_words = vec![p_word];

    let factories = PunFactory::build_all(&pun_words);

    let solutions = phrases
        .into_iter()
        .flat_map(|x| PunFactory::solve(&factories, &x))
        .collect_vec();

    // println!("Solution Count: {:?}", solutions.len());

    assert!(!solutions.is_empty());

    // for s in solutions {
    //     println!("{:?}", s.replacement_text());
    // }
    Ok(())
}
