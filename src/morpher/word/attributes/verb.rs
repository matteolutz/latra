use crate::morpher::word::{
    WordNumber, WordType, WordVerbConjugationPerson, WordVerbConjugationTempus,
};

#[derive(Debug, Clone)]
pub struct VerbAttributes {
    person: Vec<WordVerbConjugationPerson>,
    number: Vec<WordNumber>,
    tempus: Vec<WordVerbConjugationTempus>,
}

impl VerbAttributes {
    pub fn new(
        person: Vec<WordVerbConjugationPerson>,
        number: Vec<WordNumber>,
        tempus: Vec<WordVerbConjugationTempus>,
    ) -> Self {
        VerbAttributes {
            person,
            number,
            tempus,
        }
    }

    pub fn person(&self) -> &[WordVerbConjugationPerson] {
        &self.person
    }

    pub fn number(&self) -> &[WordNumber] {
        &self.number
    }

    pub fn tempus(&self) -> &[WordVerbConjugationTempus] {
        &self.tempus
    }
}

impl WordAttributesTrait<VerbAttributes> for VerbAttributes {
    fn combine(others: &[VerbAttributes]) -> VerbAttributes {
        let mut person = Vec::new();
        let mut number = Vec::new();
        let mut tempus = Vec::new();

        for other in others {
            person.extend(other.person.iter().cloned());
            number.extend(other.number.iter().cloned());
            tempus.extend(other.tempus.iter().cloned());
        }

        person.sort();
        person.dedup();

        number.sort();
        number.dedup();

        tempus.sort();
        tempus.dedup();

        VerbAttributes::new(person, number, tempus)
    }

    fn try_merge_into(&mut self, other: &VerbAttributes) -> Option<()> {
        let mut person = other.person.clone();
        person.retain(|c| self.person.contains(c));

        let mut number = other.number.clone();
        number.retain(|c| self.number.contains(c));

        let mut tempus = other.tempus.clone();
        tempus.retain(|c| self.tempus.contains(c));

        if person.is_empty() || number.is_empty() || tempus.is_empty() {
            None
        } else {
            self.person = person;
            self.number = number;
            self.tempus = tempus;
            Some(())
        }
    }

    fn word_type() -> WordType {
        WordType::Verb
    }
}
