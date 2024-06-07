use strum::IntoEnumIterator;

use super::word::{
    attributes::{noun::NounAttributes, verb::VerbAttributes},
    Word, WordAttributes, WordCase, WordGender, WordNumber, WordVerbConjugationPerson,
    WordVerbConjugationTempus,
};

#[derive(Debug)]
pub enum NounDeclination {
    A,
}

impl NounDeclination {
    pub fn get_ending(&self, case: WordCase, number: WordNumber, gender: WordGender) -> String {
        match self {
            NounDeclination::A => match (number, case) {
                (WordNumber::Singular, WordCase::Nominative) => "a".to_owned(),
                (WordNumber::Singular, WordCase::Genitive) => "ae".to_owned(),
                (WordNumber::Singular, WordCase::Dative) => "ae".to_owned(),
                (WordNumber::Singular, WordCase::Accusative) => "am".to_owned(),
                (WordNumber::Singular, WordCase::Ablative) => "a".to_owned(),
                (WordNumber::Plural, WordCase::Nominative) => "ae".to_owned(),
                (WordNumber::Plural, WordCase::Genitive) => "arum".to_owned(),
                (WordNumber::Plural, WordCase::Dative) => "is".to_owned(),
                (WordNumber::Plural, WordCase::Accusative) => "as".to_owned(),
                (WordNumber::Plural, WordCase::Ablative) => "is".to_owned(),
            },
        }
    }
}

#[derive(Debug)]
pub enum WordLemmaAttributes {
    Noun {
        gender: WordGender,
        declination: NounDeclination,
    },
    Verb,
}

#[derive(Debug)]
pub struct WordLemma {
    lemma: String,
    attributes: WordLemmaAttributes,
}

impl WordLemma {
    pub fn new(lemma: &str, attributes: WordLemmaAttributes) -> Self {
        Self {
            lemma: lemma.to_owned(),
            attributes,
        }
    }

    pub fn lemma(&self) -> &str {
        &self.lemma
    }

    pub fn get_matching_words(&self, word: &str) -> Vec<Word> {
        self.get_all_words()
            .into_iter()
            .filter(|w| w.word() == word)
            .collect()
    }

    pub fn get_all_words(&self) -> Vec<Word> {
        let mut words = Vec::new();

        match &self.attributes {
            WordLemmaAttributes::Noun {
                gender,
                declination,
            } => {
                for case in WordCase::iter() {
                    for number in WordNumber::iter() {
                        let word_str = format!(
                            "{}{}",
                            self.lemma,
                            declination.get_ending(case, number, *gender)
                        );
                        words.push(Word::new(
                            &word_str,
                            WordAttributes::Noun(NounAttributes::new(case, number, *gender)),
                        ));
                    }
                }
            }
            WordLemmaAttributes::Verb => {
                for person in WordVerbConjugationPerson::iter() {
                    for number in WordNumber::iter() {
                        for tempus in WordVerbConjugationTempus::iter() {
                            let word = match tempus {
                                WordVerbConjugationTempus::Present => match (number, person) {
                                    (WordNumber::Singular, WordVerbConjugationPerson::First) => {
                                        format!(
                                            "{}o",
                                            self.lemma
                                                .chars()
                                                .take(self.lemma.len() - 1)
                                                .collect::<String>()
                                        )
                                    }
                                    (WordNumber::Singular, WordVerbConjugationPerson::Second) => {
                                        format!("{}s", self.lemma)
                                    }
                                    (WordNumber::Singular, WordVerbConjugationPerson::Third) => {
                                        format!("{}t", self.lemma)
                                    }
                                    (WordNumber::Plural, WordVerbConjugationPerson::First) => {
                                        format!("{}mus", self.lemma)
                                    }
                                    (WordNumber::Plural, WordVerbConjugationPerson::Second) => {
                                        format!("{}tis", self.lemma)
                                    }
                                    (WordNumber::Plural, WordVerbConjugationPerson::Third) => {
                                        format!("{}nt", self.lemma)
                                    }
                                },
                            };

                            words.push(Word::new(
                                &word,
                                WordAttributes::Verb(VerbAttributes::new(person, number, tempus)),
                            ));
                        }
                    }
                }
            }
        }

        let mut actual_words: Vec<Word> = Vec::new();
        for word in words {
            let existing = actual_words.iter_mut().find(|w| w.word() == word.word());
            if let Some(existing) = existing {
                *existing = Word::com
            }
        }

        words
    }
}
