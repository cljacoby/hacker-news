use std::error::Error;
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub struct HttpError {
    pub code: u16,
    pub url: String,
}

impl HttpError {
    pub fn new(code: u16, url: String) -> Self {
        Self { code, url }
    }
}

// TODO: For variants with context messages (i.e. ArugmentError) consider referencing Anyhow's
// method of generics with trait boundaries rather than just &'static str.
// See anyhow::Error::context()

#[derive(Debug)]
pub enum HnError {
    // Error used when parsing of an HTML document fails
    HtmlParsingError,
    // Error used when program attempts to invoke an action requiring authentication,
    // but is not authenticated
    UnauthenticatedError,
    // Error used when a client fails to authenticate
    AuthenticationError,
    // Error raised from a failure during an HTTP request/response 
    HttpError(HttpError),
    // Error raised from Network connectivity problems
    NetworkError(Option<Box<dyn Error>>),
    // Error from incorrect Argument configuration from the user
    ArgumentError(Option<&'static str>),
    // Error due to inability to Serialize or Deserialize data with respect to a type.
    SerializationError(Option<&'static str>)
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
                write!(f, "AuthenticationError: A client failed to authenticate.")
            }
            HnError::HttpError(http_err) => {
                write!(f, "HttpError: Unsuccesful HTTP response code, url '{}', code '{}'",
                    http_err.url,
                    http_err.code,
                )
            },
            HnError::NetworkError(source) => {
                match source {
                    Some(source) => write!(f, "Network Error: {}", source.to_string()),
                    None => write!(f, "Network Error."),
                }
            },
            HnError::ArgumentError(msg) => {
                match msg {
                    Some(msg) => write!(f, "Argument Error: {}.", msg),
                    None => write!(f, "Incorrect Argument Configuration."),
                }
            }
            HnError::SerializationError(msg) => {
                match msg {
                    Some(msg) => write!(f, "Serialization Error: {}.", msg),
                    None => write!(f, "Serialization Error."),
                }
            }
        }
    }
}


impl HnError {
    pub fn print(&self) {
        eprintln!("Hacker News Error:\n{}", self);
    }
}


impl Error for HnError {}
