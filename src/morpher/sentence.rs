use itertools::Itertools;

use super::word::Word;

#[derive(Debug)]
pub enum SentencePart {
    Word { possible_words: Vec<Word> },
    SubSentence { parts: Vec<SentencePart> },
}

impl std::fmt::Display for SentencePart {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SentencePart::Word { possible_words } => {
                write!(
                    f,
                    "{} [{}]",
                    possible_words[0].word(),
                    possible_words.iter().map(|w| w.attributes_str()).join(", ")
                )
            }

            SentencePart::SubSentence { parts } => {
                write!(
                    f,
                    ", {},",
                    parts
                        .iter()
                        .map(|part| part.to_string())
                        .collect::<String>()
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct Sentence {
    parts: Vec<SentencePart>,
}

impl Sentence {
    pub fn new(parts: Vec<SentencePart>) -> Self {
        Sentence { parts }
    }

    pub fn parts(&self) -> &[SentencePart] {
        &self.parts
    }
}

impl std::fmt::Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.parts.iter().map(|part| part.to_string()).join(" ")
        )
    }
}
