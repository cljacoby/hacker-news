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
    pub fn new(msg: &str, src: Option<Box<dyn Error + 'static>>) -> Self {
        Self {
            msg: msg.to_string(),
            src
        }
    }

    pub fn boxed(msg: &str) -> Box<Self> {
        Box::new(Self {
            msg: msg.to_string(),
            src: None,
        })
    }

    pub fn boxed_with_src(msg: &str, src: Box<dyn Error>) -> Box<Self> {
        Box::new(Self {
            msg: msg.to_string(),
            src: Some(src),
        })
    }

}
