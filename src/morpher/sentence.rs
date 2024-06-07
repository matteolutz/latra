use std::borrow::Borrow;

use super::word::Word;

#[derive(Debug)]
pub enum SentencePart {
    Word { possible_words: Vec<Word> },
    SubSentence { parts: Vec<SentencePart> },
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
