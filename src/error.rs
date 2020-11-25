use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct HNError {
    msg: String,
    src: Option<Box<dyn Error + 'static>>,
}

impl fmt::Display for HNError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

impl Error for HNError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.src.as_ref().map(|e| e.as_ref())
    }
}

impl HNError {
    pub fn new(msg: String, src: Option<Box<dyn Error + 'static>>) -> Self {
        Self { msg, src }
    }
}
