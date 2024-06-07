use error::TokenizationError;
use token::Token;

pub mod error;
pub mod token;

pub struct Lexer<'a> {
    buffer: &'a str,
    curr_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(buffer: &'a str) -> Lexer<'a> {
        Lexer {
            buffer,
            curr_pos: 0,
        }
    }

    fn advance(&mut self) {
        self.curr_pos += 1;
    }

    fn peek(&self) -> char {
        self.buffer.chars().nth(self.curr_pos).unwrap()
    }

    fn consume(&mut self) -> char {
        let c = self.peek();
        self.advance();
        c
    }

    fn is_eof(&self) -> bool {
        self.curr_pos >= self.buffer.len()
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizationError> {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            if self.peek().is_alphabetic() {
                let mut word = String::new();

                while self.peek().is_alphabetic() {
                    word.push(self.consume());
                }

                tokens.push(Token::Word(word));
            } else if self.peek() == '.' {
                self.advance();
                tokens.push(Token::Dot);
            } else if self.peek() == ',' {
                self.advance();
                tokens.push(Token::Comma);
            } else if self.peek() == '!' {
                self.advance();
                tokens.push(Token::Exclamation);
            } else if self.peek() == '?' {
                self.advance();
                tokens.push(Token::Question);
            } else if self.peek().is_whitespace() {
                self.advance();
            } else {
                return Err(TokenizationError::UnexpectedCharacter(self.peek()));
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }
}
