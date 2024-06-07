use error::MorpherError;
use sentence::{Sentence, SentencePart};
use word::Word;

use crate::lexer::token::Token;

pub mod error;
pub mod sentence;
pub mod word;
pub mod word_stem;

pub struct Morpher<'a> {
    tokens: &'a [Token],
    all_words: &'a [Word],
}

impl<'a> Morpher<'a> {
    pub fn new(tokens: &'a [Token], all_words: &'a [Word]) -> Self {
        Morpher { tokens, all_words }
    }

    pub fn morph(&self) -> Result<Vec<Sentence>, MorpherError> {
        let mut sentences = Vec::new();

        let mut current_sentence_parts = Vec::new();

        for token in self.tokens {
            match token {
                Token::Word(word) => {
                    let matching_words: Vec<Word> = self
                        .all_words
                        .iter()
                        .filter(|w| w.word() == word)
                        .map(|w| w.clone())
                        .collect();

                    if matching_words.is_empty() {
                        return Err(MorpherError::NoMatchingWord { word: word.clone() });
                    }

                    current_sentence_parts.push(SentencePart::Word {
                        possible_words: matching_words,
                    });
                }
                Token::Dot => {
                    sentences.push(Sentence::new(current_sentence_parts));
                    current_sentence_parts = Vec::new();
                }
                _ => {}
            }
        }

        Ok(sentences)
    }
}
