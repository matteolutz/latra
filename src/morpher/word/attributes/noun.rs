use crate::morpher::word::{WordCase, WordGender, WordNumber, WordType};

use super::WordAttributes;

#[derive(Debug, Clone)]
pub struct NounAttributes {
    case: WordCase,
    number: WordNumber,
    gender: WordGender,
}

impl WordAttributes for NounAttributes {
    fn word_type(&self) -> crate::morpher::word::WordType {
        WordType::Noun
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn eq_box(&self, other: &Box<dyn WordAttributes>) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<NounAttributes>() {
            self.case == other.case && self.number == other.number && self.gender == other.gender
        } else {
            false
        }
    }
}

impl NounAttributes {
    pub fn new(case: WordCase, number: WordNumber, gender: WordGender) -> Self {
        Self {
            case,
            number,
            gender,
        }
    }

    pub fn case(&self) -> WordCase {
        self.case
    }

    pub fn number(&self) -> WordNumber {
        self.number
    }

    pub fn gender(&self) -> WordGender {
        self.gender
    }
}

impl std::fmt::Display for NounAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.case, self.number, self.gender)
    }
}
