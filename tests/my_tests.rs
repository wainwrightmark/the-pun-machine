use std::str::FromStr;

use itertools::Itertools;
use the_pun_machine::core::prelude::*;

use ntest::test_case;

#[test_case("Lichtenstein", "L IH1 K T AH0 N S T AY2 N")]
#[test_case("Picasso", "P IH0 K AA1 S OW0")]
#[test_case("Pick", "P IH1 K")]
#[test_case("FISH", "F IH1 SH")]
#[test_case("fish", "F IH1 SH")]
#[test_case("HAPPY", "HH AE1 P IY0")]

fn test_syllables(input: &str, expected: &str) -> Result<(), anyhow::Error> {
    let word = DictionaryWord::from_str(input)?;

    assert_eq!(
        word.spelling.to_ascii_lowercase(),
        input.to_ascii_lowercase()
    );
    //assert_eq!(word.variant, 1);
    //assert_eq!(word.is_compound, false);

    let s_text = word
        .syllables
        .iter()
        .flat_map(|x| x.symbols.iter())
        .join(" ");

    assert_eq!(s_text, expected);

    Ok(())
}

//#[test_case("pisces", "pieces", "SameConsonants", "pisces")]
//#[test_case("pieces", "pisces", "SameConsonants", "pieces")]
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
fn test_pun_classification(
    theme_word_str: &str,
    original_word_str: &str,
    pun_type_str: &str,
    rep: &str,
) -> Result<(), anyhow::Error> {
    let theme_word: DictionaryWord = DictionaryWord::from_str(theme_word_str)?;
    let phrase = Phrase::new(original_word_str.to_string(), Category::Idiom);

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

#[test_case("Idiom", "A bed of roses")]
#[test_case("TVShows", "Doctor Who")]
fn test_category_words(category_text: &str, expected_text: &str) -> Result<(), String> {
    let category = Category::from_str(category_text).map_err(|e| e.to_string())?;

    let category_phrases: Vec<Phrase> = category.get_phrases().collect_vec();

    let expected_phrase = Phrase::new(expected_text.to_string(), category);

    assert!(category_phrases.contains(&expected_phrase));

    Ok(())
}

#[test_case("idiom", "idium")]
#[test_case("amazed", "amazed")]
#[test_case("fantastic", "fantastic")]
#[test_case("deplorable", "deplaurable")]
fn test_spelling(word: &str, expected: &str) -> Result<(), anyhow::Error> {
    let word = DictionaryWord::from_str(word)?;

    let spelling = word
        .syllables
        .into_iter()
        .map(|x| x.get_spelling())
        .join("");

    assert_eq!(spelling, expected);

    Ok(())
}

#[test_case("Idiom", "cake")]
#[test_case("TVShows", "meat")]
fn test_puns(category_text: &str, text: &str) -> Result<(), anyhow::Error> {
    let category = Category::from_str(category_text)?;

    let phrases: Vec<Phrase> = category.get_phrases().collect_vec();

    let p_word = DictionaryWord::from_str(text)?;

    let pun_words = vec![p_word];

    let factories = PunFactory::build_all(&pun_words);

    let solutions = phrases
        .into_iter()
        .flat_map(|x| PunFactory::solve(&factories, &x))
        .collect_vec();

    assert!(!solutions.is_empty());
    Ok(())
}

#[test_case("furniture", "chair")]
fn test_children(parent: &str, expected_child: &str) -> Result<(), anyhow::Error> {
    let word = DictionaryWord::from_str(parent)?;

    let all = word
        .self_and_children()
        .iter()
        .map(|x| x.spelling.clone())
        .collect_vec();

    for s in all.iter() {
        println!("{:?}", s);
    }

    assert!(all.contains(&expected_child));

    Ok(())
}

#[test]
fn test_maxes(){
    let max_meanings = WORDDICTIONARY.words.iter().max_by_key(|x|x.meanings.len()).unwrap();
    println!("{} has {} meanings", max_meanings.spelling, max_meanings.meanings.len());
    
    
    let max_syllables = WORDDICTIONARY.words.iter().max_by_key(|x|x.syllables.len()).unwrap();
    println!("{} has {} syllables", max_syllables.spelling, max_syllables.syllables.len());
    
    
    let max_spelling = WORDDICTIONARY.words.iter().max_by_key(|x|x.spelling.len()).unwrap();
    println!("{} has {} letters", max_spelling.spelling, max_spelling.spelling.len());
    
    let max_symbols = WORDDICTIONARY.words.iter().max_by_key(|x|x.syllables.iter().max_by_key(|x|x.symbols.len()).unwrap()).unwrap();
    println!("{} has {:?} symbols", max_symbols.spelling, max_symbols.syllables.iter().max_by_key(|x|x.symbols.len()).unwrap());
}