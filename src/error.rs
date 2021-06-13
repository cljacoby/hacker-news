use std::error::Error;
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub enum HnError {
    // Error used when parsing of an HTML document fails
    HtmlParsingError,
    // Error used when program attempts to invoke an action requiring authentication,
    // but is not authenticated
    UnauthenticatedError,
    // Error used when a client fails to authenticate
    AuthenticationError,
}

impl Display for HnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HnError::HtmlParsingError => {
                write!(f, "HtmlParsingErr: There was a problem parsing HTML data. This is an internal library error.")
            },
            HnError::UnauthenticatedError => {
                write!(f, "UnauthenticatedError: An unauthenticated client attempted an action requiring authorization.")
            }
            HnError::AuthenticationError => {
                write!(f, "A client failed to authenticate. Please check credential information, and authentication frequency")
            }
        }
    }
}


impl Error for HnError {}
