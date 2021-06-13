use std::error::Error;
use std::collections::VecDeque;
use lazy_static::lazy_static;
use regex::Regex;
use scraper;
use scraper::Html;
use scraper::ElementRef;
use crate::model::Comment;
use crate::error::HnError;

pub(crate) mod comments;
pub(crate) mod listings;

// Re-exports parser namespaces for conveniant library ergonmics
pub use crate::parser::comments::CommentsParser;
pub use crate::parser::listings::ListingsParser;

pub(crate) trait HtmlParse {
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

const COMMENT_INDENT_INCR: u32 = 40;


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

pub fn extract_comments(html: &Html) -> Result<Vec<Comment>, Box<dyn Error>> {
    let comments = CommentsParser::parse(html)?;

    Ok(comments)

}

pub fn create_comment_tree(comments: Vec<Comment>) -> Vec<Comment> {
    let mut q = VecDeque::from(comments);
    let mut forest = Vec::new();

    while let Some(root) = q.pop_front() {
        forest.push(root);
        let ptr = forest.last_mut().unwrap();
        _create_comment_tree(&mut q, ptr);
    }

    forest
}

#[allow(clippy::comparison_chain)]
fn _create_comment_tree(q: &mut VecDeque<Comment>, parent: &mut Comment) {
    let mut last: Option<&mut Comment> = None;
    while let Some(c) = q.front() {
        if c.indent == parent.indent + COMMENT_INDENT_INCR {
            let c = q.pop_front().unwrap();
            parent.children.push(c);
            last = Some(parent.children.last_mut().unwrap());
        }
        else if c.indent > parent.indent + COMMENT_INDENT_INCR {
            let next_parent = last.take()
                .expect("Jumped a nesting level in comment node hierarchy");
            _create_comment_tree(q, next_parent);
        }
        else {
            return;
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::get_test_text;

    #[test]
    fn test_extract_comments() -> Result<(), Box<dyn Error>> {
        let text = get_test_text()?;
        let html = Html::parse_fragment(&text);
        let comments = extract_comments(&html)?;
        println!("comments = {:#?}", comments);

        Ok(())
    }

    #[test]
    fn test_comment_tree() -> Result<(), Box<dyn Error>> {
        let text = get_test_text()?;
        let html = Html::parse_document(&text);
        let comments = extract_comments(&html)?;
        let forest = create_comment_tree(comments);
        println!("forest = {:#?}", forest);

        Ok(())
    }
}
