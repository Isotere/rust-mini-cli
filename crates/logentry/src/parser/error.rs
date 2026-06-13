#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    Empty,
    BadFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "parsed string is empty!"),
            ParseError::BadFormat(s) => write!(f, "bad log format: {s}"),
        }
    }
}

impl std::error::Error for ParseError {}
