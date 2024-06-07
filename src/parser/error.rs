use crate::morpher::word::WordAttributes;

#[derive(Debug)]
pub enum ParserError {
    ExpectedWord { attributes: WordAttributes },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::ExpectedWord { attributes } => {
                write!(f, "Expected word with attributes: {:?}", attributes)
            }
        }
    }
}

impl std::error::Error for ParserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
