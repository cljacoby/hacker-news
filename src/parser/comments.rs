use std::error::Error;
use log;
use scraper::Html;
use scraper::ElementRef;
use scraper::Selector;
use lazy_static::lazy_static;
use crate::model::Comment;
use crate::error::HnError;
use crate::model::Id;
use crate::parser::HtmlParse;

lazy_static! {
    // Applied to root of HTML document
    static ref QS_COMMENT_TABLE: Selector = Selector::parse("table.comment-tree").unwrap();
    
    // Applied to comment tree root (i.e. node `table.comment-tree`)
    static ref QS_COMMENT: Selector = Selector::parse("tr.athing.comtr").unwrap();
    
    // Applied to comment node (i.e. node `tr.athing.comtr`)
    static ref QS_COMMENT_TEXT: Selector = Selector::parse("span.commtext").unwrap();
    static ref QS_COMMENT_MORE_TEXT: Selector = Selector::parse("span.commtext p").unwrap();
    static ref QS_COMMENT_USER: Selector = Selector::parse("a.hnuser").unwrap();
    static ref QS_COMMENT_INDENT: Selector = Selector::parse("td.ind img").unwrap();
}


pub struct CommentsParser;

impl HtmlParse for CommentsParser {
    type Item = Vec<Comment>;

    fn parse(html: &Html) -> Result<Self::Item, Box<dyn Error>> {
        let mut comments = Vec::new();

        let root = match Self::query_comment_root(html)? {
            Some(root) => root,
            None => {
                // TODO: Is it possible there are other erroneous reasones this branch could get
                // hit? It could be misleading if you get an empty Vec of comments if the HTML page
                // itself was bad.

                // If querying comment root gets no results, then this Id has no comments
                return Ok(comments);
            }
        };

        for node in root.select(&QS_COMMENT) {
            let id = Self::parse_id(&node)?;
            let text = Self::parse_text(&node, id)?;
            let user = Self::parse_user(&node, id)?;
            let indent = Self::parse_indent(&node, id)?;
            let children = Vec::new();
            comments.push(Comment {
                user,
                id,
                text,
                indent,
                children 
            });
        }

        Ok(comments)
    }
}

impl CommentsParser {

    fn query_comment_root(html: &Html) -> Result<Option<ElementRef>, Box<dyn Error>> {
        // Note: This uses the first comment table found. There shouldn't ever
        // be more than one comment table; however, as is there is not an explicit check
        let root = html.select(&QS_COMMENT_TABLE)
            .next();

        Ok(root)
    }

    fn parse_id(node: &ElementRef) -> Result<Id, Box<dyn Error>> {
        let id = node.value()
            .id()
            .ok_or_else(|| {
                log::error!("Failed to find id for comment; html = '{:?}'", node.html());
                HnError::HtmlParsingError
            })?
            .parse::<Id>()?;

        Ok(id)
    }

    fn parse_text(node: &ElementRef, id: Id) -> Result<String, Box<dyn Error>> {

        // Select inner text from root of comment text node
        let mut text = node.select(&QS_COMMENT_TEXT)
            .next()
            .ok_or_else(|| {
                log::error!("Failed to find comment text for id = {}", id);
                HnError::HtmlParsingError
            })?
            .text()
            .next()
            .ok_or_else(|| {
                log::error!("Failed to extract inner text for comment id = {}", id);
                let msg = format!("Failed to extract inner text for comment id = {}", id);
                msg.as_str().to_owned()
            })?
            .to_string();

        // Search for any additional nodes of text, which appear as distinct <p> nodes
        for child in node.select(&QS_COMMENT_MORE_TEXT) {
            let more_text = match child.text().next() {
                None => {
                    // This branch handles a <p> node with no inner text.
                    // With no inner text, there is nothing to append, and
                    // we simply continue
                    continue;
                }
                Some(more_text) => more_text,
            };

            // We add a newline since we're concatenating <p> node text together
            text.push('\n');
            text.push_str(&more_text);
        }

        Ok(text)
    }

    fn parse_user(node: &ElementRef, id: Id) -> Result<String, Box<dyn Error>> {
        let user = node.select(&QS_COMMENT_USER)
            .next()
            .ok_or_else(|| {
                log::error!("Failed to find the user node for comment id={}", id);
                HnError::HtmlParsingError
            })?
            .text()
            .next()
            .ok_or_else(|| {
                log::error!("Failed to extract user text for comment id = {}", id);
                HnError::HtmlParsingError
            })?
            .to_string();

        Ok(user)
    }

    fn parse_indent(node: &ElementRef, id: Id) -> Result<u32, Box<dyn Error>> {
        let indent = node.select(&QS_COMMENT_INDENT)
            .next()
            .ok_or_else(|| {
                log::error!("Failed to find indent node under comment id = {}", id);
                HnError::HtmlParsingError
            })?
            .value()
            .attr("width")
            .ok_or_else(|| {
                log::error!("Failed to extract indent width attribute from comment id = {}", id);
                HnError::HtmlParsingError
            })?
            .parse::<u32>()?;

        Ok(indent)
    }
}

