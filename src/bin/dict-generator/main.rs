mod phoenetics_word;

use std::{
    collections::{BTreeMap, HashSet},
    convert::TryFrom,
    fs::File,
};

use itertools::Itertools;
use quick_xml::de::from_reader;
use serde::{Deserialize, Serialize};
use the_pun_machine::core::prelude::*;

pub fn main() {
    let reader = quick_xml::Reader::from_file("data\\english-wordnet-2021.xml")
        .expect("Could not read English Wordnet file. You may need to download this");
    let resource: LexicalResource = from_reader(reader.into_inner()).unwrap();

    let output_words = resource
        .lexicon
        .lexical_entries
        .into_iter()
        .filter(|x| x.lemma.is_dictionary_word())
        .flat_map(|entry| {
            let meanings: smallvec::SmallVec<_> = entry
                .senses
                .iter()
                .map(|x| synset_to_id(&x.synset))
                .collect();

            

            entry
                .get_written_forms()
                .into_iter()
                .map(string_to_static_str)
                .flat_map(|spelling| {
                    phoenetics_word::PhoeneticsWord::try_from(spelling.to_string())
                        .ok()
                        .map(|x| DictionaryWord {
                            syllables: x.syllables,
                            meanings: meanings.clone(),
                            spelling,
                        })
                })
                .collect_vec()
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
                            if good_synsets.contains(new_child) && set.insert(*new_child) {
                                cont = true;
                            }
                        }
                    }
                }
            }
        }
        set
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

    let words: Vec<DictionaryWord> = output_words
        .into_iter()
        //.map(|x : DictionaryWord|x.into())
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
        r
    } else {
        panic!(
            "Could not read '{synset}' ({}) as synset id",
            &synset[5..13]
        )
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OutputMeaning {
    pub id: u32,
    pub children: Vec<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct LexicalResource {
    #[serde(rename = "Lexicon", default)]
    pub lexicon: Lexicon,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct LexicalEntry {
    pub id: String,
    #[serde(rename = "Lemma")]
    pub lemma: Lemma,
    #[serde(rename = "Sense", default)]
    pub senses: Vec<Sense>,
    #[serde(rename = "Form", default)]
    pub forms: Vec<Form>,
}

impl LexicalEntry {
    pub fn get_written_forms(&self) -> Vec<String> {
        let mut r = vec![self.lemma.written_form.clone()];
        for f in self.forms.iter() {
            r.push(f.written_form.clone());
        }

        if self.forms.is_empty() && self.lemma.part_of_speech == PartOfSpeech::Noun {
            r.push(self.lemma.written_form.clone() + "s");
        }
        r
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Lemma {
    #[serde(rename = "writtenForm")]
    pub written_form: String,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: PartOfSpeech,
    #[serde(rename = "Pronunciation", default)]
    pub pronunciations: Vec<Pronunciation>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct Form {
    #[serde(rename = "writtenForm")]
    pub written_form: String,
}

impl Lemma {
    pub fn is_dictionary_word(&self) -> bool {
        true
        //     if self.written_form.len() <= 2 {
        //         return false;
        //     }

        //     if self.written_form.chars().all(
        //         |c| c.is_ascii_alphabetic(), //&& c.is_ascii_lowercase()
        //     ) {
        //         return true;
        //     }
        //     return false;
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct Pronunciation {
    #[serde(rename = "variety", default)]
    pub variety: Option<String>,
    #[serde(rename = "$value")]
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct Sense {
    pub id: String,
    pub synset: String,
    #[serde(rename = "SenseRelation", default)]
    pub sense_relations: Vec<Relation>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct Relation {
    #[serde(rename = "relType")]
    pub rel_type: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize)]
pub struct SyntacticBehaviour {}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
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
