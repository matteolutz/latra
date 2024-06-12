use error::ParserError;

use crate::morpher::sentence::{Sentence, SentencePart};

pub mod error;

pub struct Parser<'a> {
    sentence: &'a Sentence,
    curr_part: usize,
}

impl<'a> Parser<'a> {
    pub fn new(sentence: &'a Sentence) -> Self {
        Self {
            sentence,
            curr_part: 0,
        }
    }

    pub fn current_part(&self) -> &SentencePart {
        &self.sentence.parts()[self.curr_part]
    }

    pub fn advance(&mut self) {
        self.curr_part += 1;
    }

    pub fn parse_noun_phrase(&self) -> Result<(), ParserError> {
        let part = self.current_part();

        if let SentencePart::Word { possible_words } = part {}

        Ok(())
    }

    pub fn parse(&mut self) -> Result<(), ParserError> {
        self.parse_noun_phrase()?;

        Ok(())
    }
}
