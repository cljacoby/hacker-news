use std::error::Error;
use std::ops::Deref;
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

// Query for an ancestor node at a given height
fn ancestor<'a>(node: &'a ElementRef, height: u32) -> Option<ElementRef<'a>> {

    // Note: Declaring `parent` outside the loop resolves an error
    // regarding dropping to early; however, the compiler reports this as
    // an unused assignment

    let mut curr_node = Deref::deref(node);
    #[allow(unused_assignments)]
    let mut parent = curr_node.parent();
    let mut i = 0;

    while i < height {
        parent = curr_node.parent();
        curr_node = match parent {
            Some(ref node_ref) => node_ref,
            None => { return None; },
        };
        i += 1;
    }

    ElementRef::wrap(*curr_node)
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

