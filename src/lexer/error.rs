#[derive(Debug)]
pub enum TokenizationError {
    UnexpectedCharacter(char),
}

impl std::fmt::Display for TokenizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizationError::UnexpectedCharacter(c) => {
                write!(f, "Unexpected character: '{}'", c)
            }
        }
    }
}

impl std::error::Error for TokenizationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
