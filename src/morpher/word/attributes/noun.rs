use crate::morpher::word::{WordCase, WordGender, WordNumber, WordType};

use super::WordAttributesTrait;

#[derive(Debug, Clone)]
pub struct NounAttributes {
    case: Vec<WordCase>,
    number: Vec<WordNumber>,
    gender: Vec<WordGender>,
}

impl NounAttributes {
    pub fn new(case: Vec<WordCase>, number: Vec<WordNumber>, gender: Vec<WordGender>) -> Self {
        NounAttributes {
            case,
            number,
            gender,
        }
    }

    pub fn case(&self) -> &[WordCase] {
        &self.case
    }

    pub fn number(&self) -> &[WordNumber] {
        &self.number
    }

    pub fn gender(&self) -> &[WordGender] {
        &self.gender
    }
}

impl WordAttributesTrait<NounAttributes> for NounAttributes {
    fn combine(others: &[NounAttributes]) -> NounAttributes {
        let mut case = Vec::new();
        let mut number = Vec::new();
        let mut gender = Vec::new();

        for other in others {
            case.extend(other.case.iter().cloned());
            number.extend(other.number.iter().cloned());
            gender.extend(other.gender.iter().cloned());
        }

        case.sort();
        case.dedup();

        number.sort();
        number.dedup();

        gender.sort();
        gender.dedup();

        NounAttributes::new(case, number, gender)
    }

    fn try_merge_into(&mut self, other: &NounAttributes) -> Option<()> {
        let mut case = other.case.clone();
        case.retain(|c| self.case.contains(c));
        if case.is_empty() {
            return None;
        }

        let mut number = other.number.clone();
        number.retain(|n| self.number.contains(n));
        if number.is_empty() {
            return None;
        }

        let mut gender = other.gender.clone();
        gender.retain(|g| self.gender.contains(g));
        if gender.is_empty() {
            return None;
        }

        if case.is_empty() || number.is_empty() || gender.is_empty() {
            None
        } else {
            self.case = case;
            self.number = number;
            self.gender = gender;

            Some(())
        }
    }

    fn word_type() -> crate::morpher::word::WordType {
        WordType::Noun
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_noun_attributes_merge_success() {
        let a = NounAttributes::new(
            vec![WordCase::Nominative, WordCase::Accusative],
            vec![WordNumber::Singular],
            vec![WordGender::Masculine],
        );

        let b = NounAttributes::new(
            vec![WordCase::Nominative],
            vec![WordNumber::Singular],
            vec![WordGender::Masculine],
        );

        let c = a.try_merge_into(&b);

        assert!(c.is_some());

        let c = c.unwrap();
        assert_eq!(c.case()[0], WordCase::Nominative);
        assert_eq!(c.number()[0], WordNumber::Singular);
        assert_eq!(c.gender()[0], WordGender::Masculine);
    }
}
