use attributes::{noun::NounAttributes, verb::VerbAttributes};
use strum_macros::EnumIter;

pub mod attributes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordCase {
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Ablative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordNumber {
    Singular,
    Plural,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordGender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordVerbConjugationPerson {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordVerbConjugationTempus {
    Present,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordType {
    Noun,
    Verb,
}

#[derive(Debug, Clone)]
pub enum WordAttributes {
    Noun {
        case: WordCase,
        number: WordNumber,
        gender: WordGender,
    },
    Verb {
        person: WordVerbConjugationPerson,
        tempus: WordVerbConjugationTempus,
    },
}

#[derive(Debug, Clone)]
pub struct Word {
    word: String,
    attributes: Vec<WordAttributes>,
}

impl Word {
    pub fn new(word: &str, attributes: WordAttributes) -> Self {
        Self {
            word: word.to_owned(),
            attributes,
        }
    }

    pub fn word(&self) -> &str {
        &self.word
    }

    pub fn attributes(&self) -> &WordAttributes {
        &self.attributes
    }
}
