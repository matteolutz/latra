use crate::morpher::word::{
    WordNumber, WordType, WordVerbConjugationPerson, WordVerbConjugationTempus,
};

use super::WordAttributes;

#[derive(Debug, Clone)]
pub struct VerbAttributes {
    person: WordVerbConjugationPerson,
    number: WordNumber,
    tempus: WordVerbConjugationTempus,
}

impl WordAttributes for VerbAttributes {
    fn word_type(&self) -> crate::morpher::word::WordType {
        WordType::Verb
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn eq_box(&self, other: &Box<dyn WordAttributes>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<VerbAttributes>() {
            self.person == other.person
                && self.number == other.number
                && self.tempus == other.tempus
        } else {
            false
        }
    }
}

impl VerbAttributes {
    pub fn new(
        person: WordVerbConjugationPerson,
        number: WordNumber,
        tempus: WordVerbConjugationTempus,
    ) -> Self {
        Self {
            person,
            number,
            tempus,
        }
    }

    pub fn person(&self) -> WordVerbConjugationPerson {
        self.person
    }

    pub fn number(&self) -> WordNumber {
        self.number
    }

    pub fn tempus(&self) -> WordVerbConjugationTempus {
        self.tempus
    }
}

impl std::fmt::Display for VerbAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.person, self.number, self.tempus)
    }
}
