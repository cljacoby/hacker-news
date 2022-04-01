use std::error::Error;
use std::fmt::Display;
use std::fmt;
use std::io::Write;
use termcolor::BufferWriter;
use termcolor::ColorChoice;
use termcolor::ColorSpec;
use termcolor::Color;
use termcolor::WriteColor;

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
                write!(f, "There was a problem parsing HTML data. This is an internal library error.")
            },
            HnError::UnauthenticatedError => {
                write!(f, "An unauthenticated client attempted an action requiring authentication.")
            }
            HnError::AuthenticationError => {
                write!(f, "A Hacker News client failed to authenticate.")
            }
            HnError::HttpError(http_err) => {
                write!(f, "Unsuccesful HTTP response code, url '{}', code '{}'",
                    http_err.url,
                    http_err.code,
                )
            },
            HnError::NetworkError(src) => {
                match src {
                    Some(src) => write!(f, "Failed to make network request. {}", src.to_string()),
                    None => write!(f, "Failed to make network request."),
                }
            },
            HnError::ArgumentError(msg) => {
                match msg {
                    None => write!(f, "Incorrect argument configuration."),
                    Some(msg) => write!(f, "Incorrect argument configuration. {}.", msg),
                }
            }
            HnError::SerializationError(msg) => {
                match msg {
                    Some(msg) => write!(f, "Failed to serialize/deseralize data structure. {}.", msg),
                    None => write!(f, "Failed to serialize/deseralize data structure."),
                }
            }
        }
    }
}

impl HnError {
    pub fn variant_str(&self) -> &'static str {
        match self {
            HnError::HtmlParsingError => "HtmlParsingError",
            HnError::UnauthenticatedError => "UnauthenticatedError",
            HnError::AuthenticationError => "AuthenticationError",
            HnError::HttpError(_http_err) => "HttpError",
            HnError::NetworkError(_source) => "NetworkErr",
            HnError::ArgumentError(_msg) => "ArgumentError",
            HnError::SerializationError(_msg) => "SerializationError",
        }
    }
}

struct Colorizer {
    // use_stderr: bool,
    // use_color: bool,
    // pieces: Vec<String, Style>
    pieces: Vec<(String, Style)>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Style {
    Good,
    Warning,
    Error,
    Hint,
    Default,
}

impl Default for Style {
    fn default() -> Self {
        Self::Default
    }
}

impl Colorizer {
    pub fn print(&self) -> Result<(), Box<dyn std::error::Error>> {
        let writer = BufferWriter::stdout(ColorChoice::Always);
        let mut buffer = writer.buffer();

        for piece in self.pieces.iter() {
            let mut color = ColorSpec::new();
            match piece.1 {
                Style::Good => {
                    color.set_fg(Some(Color::Green));
                },
                Style::Warning => {
                    color.set_fg(Some(Color::Yellow));
                },
                Style::Error => {
                    color.set_fg(Some(Color::Red));
                    color.set_bold(true);
                },
                Style::Hint => {
                    color.set_dimmed(true);
                    color.set_italic(true);
                },
                Style::Default => {}
            }

            buffer.set_color(&color)?;
            buffer.write_all(piece.0.as_bytes())?;
            buffer.reset()?;
        }

        writer.print(&buffer)?;

        Ok(())
    }
}

impl HnError {
    pub fn formatted_print(&self) {
        let mut colorizer = Colorizer {
            pieces: vec![],
        };

        colorizer.pieces.push(("error: ".to_string(), Style::Error));
        colorizer.pieces.push((format!("{}\n", self), Style::Default));
        colorizer.pieces.push((format!("{}\n", self.variant_str()), Style::Hint));

        if let Err(_err) = colorizer.print() {
            log::error!("Failed formatted color print of {}", self);
            eprintln!("error: {}", self);
        }
    }
}


impl Error for HnError {}
