#[derive(Debug)]
pub struct ParseError {
    message: String
}

impl ParseError {
    pub fn new(msg: &String) -> Self {
        Self { message: msg.clone() }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.message)
    }
}

impl std::error::Error for ParseError {}
