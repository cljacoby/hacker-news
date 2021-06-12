use std::error::Error;
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub enum HnError {
    HtmlParsingError,
    AuthError,
}

impl Display for HnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HnError::HtmlParsingError => {
                write!(f, "HtmlParsingErr: There was a problem parsing HTML data. This is an internal library error.")
            },
            HnError::AuthError => {
                write!(f, "AuthError: An unauthenticated client attempted an action requiring authorization.")
            }
        }
    }
}


impl Error for HnError {}
