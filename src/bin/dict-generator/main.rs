use itertools::Itertools;
use quick_xml::de::from_reader;

use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs::File;

use the_pun_machine::core::prelude::*;

pub fn main() {
    let reader = quick_xml::Reader::from_file("data\\english-wordnet-2021.xml")
        .expect("Could not read English Wordnet file. You may need to download this");
    let resource: LexicalResource = from_reader(reader.into_inner()).unwrap();

    // let synset_dic: HashMap<_, _> = resource
    //     .lexicon
    //     .synsets
    //     .iter()
    //     .map(|s| (s.id.clone(), s))
    //     .collect();

    // let words_path = "data/generated/words.tsv";
    // let mut words_output = File::create(words_path).expect("Could not open file for writing");

    // let meanings_path = "data/generated/meanings.tsv";
    // let mut meanings_output = File::create(meanings_path).expect("Could not open file for writing");

    let output_words = resource
        .lexicon
        .lexical_entries
        .into_iter()
        .filter(|x| x.lemma.is_dictionary_word())
        .flat_map(|entry| {
            let mut word = OutputWord::try_from(entry.lemma.written_form).ok()?;

            let meanings = entry
                .senses
                .iter()
                .map(|x| synset_to_id(&x.synset))
                .collect_vec();

            word.meanings = meanings;

            Some(word)
        })
        .collect_vec();

    let good_synsets = output_words
        .iter()
        .flat_map(|x| x.meanings.iter())
        .cloned()
        .collect::<HashSet<_>>();

    let all_meanings: BTreeMap<u32, OutputMeaning> = resource
        .lexicon
        .synsets
        .iter()
        .map(|synset| {
            let children = synset
                .synset_relations
                .iter()
                .filter(|rel| PARENTCHILDRELATIONS.contains(&rel.rel_type.as_str()))
                .map(|rel| synset_to_id(&rel.target))
                .collect_vec();

            let id = synset_to_id(&synset.id);

            (id, OutputMeaning { id, children })
        })
        .collect();

    fn get_child_ids(
        id: u32,
        good_synsets: &HashSet<u32>,
        all_meanings: &BTreeMap<u32, OutputMeaning>,
    ) -> HashSet<u32> {
        let mut cont = true;
        let mut set: HashSet<u32> = Default::default();
        set.insert(id);

        while cont {
            cont = false;
            for s in set.clone().iter() {
                if !good_synsets.contains(s) {
                    set.remove(s);
                    if let Some(meaning) = all_meanings.get(s) {
                        for new_child in meaning.children.iter() {
                            if good_synsets.contains(&new_child) {
                                if set.insert(*new_child) {
                                    cont = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        return set;
    }

    let meanings: BTreeMap<_, _> = all_meanings
        .values()
        .filter(|x| good_synsets.contains(&x.id))
        .map(|x| {
            let new_children = x.children.iter().flat_map(|child| {
                if good_synsets.contains(child) {
                    vec![*child]
                } else {
                    get_child_ids(*child, &good_synsets, &all_meanings)
                        .into_iter()
                        .collect_vec()
                }
            });
            (x.id, new_children.collect_vec())
        })
        .collect();

    let words: BTreeMap<_, _> = output_words
        .into_iter()
        .map(|x| (x.spelling, (x.pronunciation, x.meanings)))
        .collect();

    let word_dictionary = WordDictionary { words, meanings };

    let mut mp_file = &File::create("data.mp").unwrap();

    word_dictionary
        .serialize(&mut rmp_serde::Serializer::new(&mut mp_file))
        .unwrap();

    let mut yaml_file = &File::create("data.yaml").unwrap();

    word_dictionary
        .serialize(&mut serde_yaml::Serializer::new(&mut yaml_file))
        .unwrap();
}

fn synset_to_id(synset: &String) -> u32 {
    if let Ok(r) = u32::from_str_radix(&synset[5..13], 10) {
        return r;
    } else {
        panic!(
            "Could not read '{synset}' ({}) as synset id",
            &synset[5..13]
        )
    }
}

lazy_static::lazy_static! {
    static ref PARENTCHILDRELATIONS: HashSet<&'static str> = {
        let mut set = HashSet::<&'static str>::default();
        set.insert("hyponym");
        set.insert("instance_hyponym");
        set.insert("has_domain_topic");
        set.insert("has_domain_region");
        set.insert("has_domain_usage");
        set
    };
}

include_flate::flate!(pub static PRONOUNCIATIONS: str from "data/syllables/pronounciation.txt");

#[derive(Clone, Debug, Default, PartialEq)]
pub struct OutputWord {
    pub spelling: String,
    pub pronunciation: Vec<Syllable>,
    pub meanings: Vec<u32>,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OutputMeaning {
    pub id: u32,
    pub children: Vec<u32>,
}

impl TryFrom<String> for OutputWord {
    type Error = &'static str;

    fn try_from(spelling: String) -> Result<Self, Self::Error> {
        let pw = PhoeneticsWord::try_from(spelling.clone())?;

        Ok(OutputWord {
            spelling: spelling.clone(),
            pronunciation: pw.syllables,
            ..Default::default()
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct LexicalResource {
    #[serde(rename = "Lexicon", default)]
    pub lexicon: Lexicon,
}
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]

pub struct Lexicon {
    pub id: String,
    pub label: String,
    pub language: String,
    pub email: String,
    pub license: String,
    pub version: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub citation: Option<String>,
    //TODO more fields
    #[serde(rename = "LexicalEntry", default)]
    pub lexical_entries: Vec<LexicalEntry>,
    #[serde(rename = "Synset", default)]
    pub synsets: Vec<Synset>,
    #[serde(rename = "SyntacticBehaviour", default)]
    pub behaviours: Vec<SyntacticBehaviour>,
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct LexicalEntry {
    pub id: String,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Sense", default)]
    pub senses: Vec<Sense>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Lemma {
    #[serde(rename = "writtenForm")]
    pub written_form: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Pronunciation", default)]
    pub pronunciations: Vec<Pronunciation>,
}

impl Lemma {
    pub fn is_dictionary_word(&self) -> bool {
        if self.written_form.len() <= 2 {
            return false;
        }

        if self
            .written_form
            .chars()
            .all(|c| c.is_ascii_alphabetic() && c.is_ascii_lowercase())
        {
            return true;
        }
        return false;
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Pronunciation {
    #[serde(rename = "variety", default)]
    pub variety: Option<String>,
    #[serde(rename = "$value")]
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Sense {
    pub id: String,
    pub synset: String,
    #[serde(rename = "SenseRelation", default)]
    pub sense_relations: Vec<Relation>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Relation {
    #[serde(rename = "relType")]
    pub rel_type: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Synset {
    pub id: String,
    pub ili: String,
    pub members: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "dc:subject", default)]
    pub subject: Option<String>,

    #[serde(rename = "$unflatten=Definition", default)]
    pub definition: Option<String>,

    #[serde(rename = "SynsetRelation", default)]
    pub synset_relations: Vec<Relation>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct SyntacticBehaviour {}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum PartOfSpeech {
    #[serde(rename = "n")]
    Noun,
    #[serde(rename = "v")]
    Verb,
    #[serde(rename = "a")]
    Adjective,
    #[serde(rename = "r")]
    Adverb,

    #[serde(rename = "s")]
    AdjectiveSatellite,

    #[serde(rename = "f")]
    FirstName,
    #[serde(rename = "l")]
    LastName,
}
