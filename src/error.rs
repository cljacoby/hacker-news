use std::error::Error;
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub enum HnError {
    HtmlParsingErr,
    AuthErr,
}

impl Display for HnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HnError::HtmlParsingErr => {
                write!(f, "HtmlParsingErr: There was a problem parsing HTML data. This is an internal library error.")
            },
            HnError::AuthErr => {
                write!(f, "AuthErr: An unauthenticated client attempted an action requiring authorization.")
            }
        }
    }
}


impl Error for HnError {}
