use std::error::Error;
use lazy_static::lazy_static;
use regex::Regex;
use scraper;
use scraper::Html;
use scraper::ElementRef;
use crate::error::HnError;

pub mod comments;
pub mod listings;

// Re-exports parser namespaces for conveniant library ergonmics
pub use crate::parser::comments::CommentsParser;
pub use crate::parser::listings::ListingsParser;

pub trait HtmlParse {
    type Item;

    fn parse(html: &Html) -> Result<Self::Item, Box<dyn Error>>;

}


// --------------------------------------------------------
// The code below here is from before I refactored
// the HTML parsing logic. It should be appropriately
// integrated into the new HTML parsing model
// --------------------------------------------------------

lazy_static! {
    static ref FNID_REGEX: Regex =  Regex::new(r#"<input.*value="(.+?)".*>"#).unwrap();
}

pub fn extract_fnid(el: &ElementRef) -> Result<String, Box<dyn Error>> {
    let text = el.html();
    let captures = match FNID_REGEX.captures(&text) {
        Some(captures) => captures,
        None => {
            return Err(Box::new(HnError::HtmlParsingError));
        }
    };
    let fnid = match captures.get(1) {
        Some(fnid) => {
            fnid.as_str().to_string()
        },
        None => {
            return Err(Box::new(HnError::HtmlParsingError));
        }
    };

    Ok(fnid)
}

