#[derive(Debug)]
pub enum MorpherError {
    NoMatchingWord { word: String },
}

impl std::fmt::Display for MorpherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MorpherError::NoMatchingWord { word } => write!(f, "No matching word for '{}'", word),
        }
    }
}

impl std::error::Error for MorpherError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
