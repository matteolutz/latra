#[derive(Debug)]
pub enum Token {
    Word(String),
    Dot,
    Comma,
    Exclamation,
    Question,
    Eof,
}
