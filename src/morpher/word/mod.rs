use attributes::{noun::NounAttributes, verb::VerbAttributes, WordAttributes};
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

impl std::fmt::Display for WordCase {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WordCase::Nominative => write!(f, "Nominative"),
            WordCase::Genitive => write!(f, "Genitive"),
            WordCase::Dative => write!(f, "Dative"),
            WordCase::Accusative => write!(f, "Accusative"),
            WordCase::Ablative => write!(f, "Ablative"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordNumber {
    Singular,
    Plural,
}

impl std::fmt::Display for WordNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WordNumber::Singular => write!(f, "Singular"),
            WordNumber::Plural => write!(f, "Plural"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordGender {
    Masculine,
    Feminine,
    Neuter,
}

impl std::fmt::Display for WordGender {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WordGender::Masculine => write!(f, "Masculine"),
            WordGender::Feminine => write!(f, "Feminine"),
            WordGender::Neuter => write!(f, "Neuter"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordVerbConjugationPerson {
    First,
    Second,
    Third,
}

impl std::fmt::Display for WordVerbConjugationPerson {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WordVerbConjugationPerson::First => write!(f, "1st person"),
            WordVerbConjugationPerson::Second => write!(f, "2nd person"),
            WordVerbConjugationPerson::Third => write!(f, "3rd person"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordVerbConjugationTempus {
    Present,
}

impl std::fmt::Display for WordVerbConjugationTempus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            WordVerbConjugationTempus::Present => write!(f, "Present"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum WordType {
    Noun,
    Verb,
}

#[derive(Debug, Clone)]
pub enum Word {
    Noun {
        word: String,
        attributes: Vec<NounAttributes>,
    },
    Verb {
        word: String,
        attributes: Vec<VerbAttributes>,
    },
}

impl Word {
    pub fn word(&self) -> &str {
        match self {
            Word::Noun { word, .. } => word,
            Word::Verb { word, .. } => word,
        }
    }

    pub fn attributes_str(&self) -> String {
        match self {
            Word::Noun { attributes, .. } => format!(
                "(Noun => {})",
                attributes
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Word::Verb { attributes, .. } => format!(
                "(Verb => {})",
                attributes
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }

    pub fn try_into(&self, attributes: &Box<dyn WordAttributes>) -> Option<Self> {
        match self {
            Word::Noun {
                word: _,
                attributes: word_attributes,
            } => {
                for word_attribute in word_attributes {
                    if attributes.eq_box(word_attribute.as_any().downcast_ref().unwrap()) {
                        return Some(self.clone());
                    }
                }

                None
            }
            _ => unimplemented!(),
        }
    }

    pub fn combine(&self, other: &Word) -> Option<Word> {
        match (self, other) {
            (
                Word::Noun {
                    word: w1,
                    attributes: a1,
                },
                Word::Noun {
                    word: w2,
                    attributes: a2,
                },
            ) => {
                if w1 == w2 {
                    let mut attributes = a1.clone();
                    attributes.extend(a2.clone());
                    Some(Word::Noun {
                        word: w1.clone(),
                        attributes,
                    })
                } else {
                    None
                }
            }
            (
                Word::Verb {
                    word: w1,
                    attributes: a1,
                },
                Word::Verb {
                    word: w2,
                    attributes: a2,
                },
            ) => {
                if w1 == w2 {
                    let mut attributes = a1.clone();
                    attributes.extend(a2.clone());
                    Some(Word::Verb {
                        word: w1.clone(),
                        attributes,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
