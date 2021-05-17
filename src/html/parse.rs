use std::error::Error;
use std::collections::VecDeque;
use lazy_static::lazy_static;
use regex::Regex;
use scraper::Html;
use scraper::Selector;
use scraper::element_ref::Select;
use scraper::ElementRef;
use log;
use crate::html::models::Score;
use crate::html::models::Id;
use crate::html::models::Listing;
use crate::html::models::Date;
use crate::html::models::Comment;
use crate::html::get_test_text;
use crate::error::HNError;

lazy_static! {
    static ref FNID_REGEX: Regex =  Regex::new(r#"<input.*value="(.+?)".*>"#).unwrap();
}

const COMMENT_INDENT_INCR: i32 = 40;

/// The following example uses this Hacker News post:
/// * https://news.ycombinator.com/item?id=27145911
/// 
/// The HTML for the title of this post appears as follows:
/// ```xml
/// <tr class="athing" id="27145911">
///     <td align="right" valign="top" class="title"><span class="rank"></span></td>
///     <td valign="top" class="votelinks">
///         <center>
///             <a id="up_27145911" onclick='return vote(event, this, "up")'
///                 href="https://news.ycombinator.com/vote?id=27145911&amp;how=up&amp;auth=b07f7bc2e1dd7deabe0369a86cb670b69f833b83&amp;goto=item%3Fid%3D27145911">
///                 <div class="votearrow" title="upvote"></div>
///             </a>
///         </center>
///     </td>
///     <td class="title">
///         <a href="https://fingerprintjs.com/blog/external-protocol-flooding/"
///         class="storylink">Vulnerability allows cross-browser tracking in Chrome, Firefox, Safari, and Tor</a>
///     <span class="sitebit comhead"> (<a href="https://news.ycombinator.com/from?site=fingerprintjs.com"><span class="sitestr">fingerprintjs.com</span></a>)</span>
///     </td>
/// </tr>
/// ```
pub fn extract_listings(html: &Html) -> Result<Vec<Listing>, Box<dyn Error>> {

    // Selectors applied to root html node to locate title nodes 
    let selector_title = Selector::parse("tr.athing:not(.comtr)").unwrap();

    // Selectors applied to title node
    let selector_titlelink = Selector::parse("td.title > a.storylink").unwrap();
    let selector_sitebit = Selector::parse("td.title > span.sitebit.comhead > a").unwrap();

    // Selectors applied to the subtext node
    let selector_score = Selector::parse("span.score").unwrap();
    let selector_user = Selector::parse("a.hnuser").unwrap();

    // Parse each HTML listing into a Listing instance
    let nodes = html.select(&selector_title);
    let mut listings: Vec<Listing> = Vec::new(); 
    for title_node in nodes {

        // Note:
        // The subtext node is assumed to be the next adjacent sibling
        // node from a given title node. There are no other distinguishing HTML
        // patterns with which to capture this node.
        let subtext_node = title_node.next_sibling()
            .ok_or("Could not find subtext node as next sibling of title node.")?;
        let subtext_node = ElementRef::wrap(subtext_node)
            .ok_or("Could not wrap subtext node in ElementRef")?;

        // Obtain the user, if it exists
        let user = match subtext_node.select(&selector_user).next() {
            None => None,
            Some(user_node) => {
                Some(user_node.text()
                    .next()
                    .ok_or("User node found, but failed to obtain inner text")?
                    .to_string()
                )}
        };

        // Obtain the score, if it exists
        let score = match subtext_node.select(&selector_score).next() {
            None => None,
            Some(score_node) => {
                Some(score_node.text()
                    .next()
                    .ok_or("Score node found, but failed to obtain inner text")?
                    .strip_suffix(" points")
                    .ok_or("failed to strip points suffix ' points'")?
                    .parse::<Score>()?
                )}
        };

        // Obtain the title, URL, and HackerNews item ID. These should always exist.
        let title_el = title_node.select(&selector_titlelink)
            .next()
            .ok_or("title query selector got no matches")?;
        let title = title_el
            .text()
            .next()
            .ok_or("Could not get inner text for score HTML element")?
            .to_string();
        let url = title_el
            .value()
            .attr("href")
            .ok_or("Title link had missing 'href' attribute")?
            .to_string();
        let id = title_node
            .value()
            .id()
            .ok_or("Title node did not have HTML Id attribute")?
            .parse::<Id>()?;

        listings.push(Listing {
            title,
            id,
            score,
            user,
            url
        });
    }

    Ok(listings)
}

pub fn extract_fnid(el: &ElementRef) -> Result<String, Box<dyn Error>> {
    let text = el.html();
    let captures = match FNID_REGEX.captures(&text) {
        Some(captures) => captures,
        None => {
            return Err(HNError::boxed("Fnid regex failed to process input HMTL text"));
        }
    };
    let fnid = match captures.get(1) {
        Some(fnid) => {
            fnid.as_str().to_string()
        },
        None => {
            return Err(HNError::boxed("Fnid capture group prouced no matches"));
        }
    };

    Ok(fnid)
}

pub fn extract_comments(html: &Html) -> Result<Vec<Comment>, Box<dyn Error>> {
    // Applied to root of HTML document
    let selector_comment_tree = Selector::parse("table.comment-tree").unwrap();
    // Applied to comment tree root (i.e. node `table.comment-tree`)
    let selector_comment = Selector::parse("tr.athing.comtr").unwrap();
    // Applied to comment node (i.e. node `tr.athing.comtr`)
    let selector_comment_text = Selector::parse("span.commtext").unwrap();
    let selector_comment_user = Selector::parse("a.hnuser").unwrap();
    let selector_indent = Selector::parse("td.ind img").unwrap();

    // Query the HTML for the root of the comment tree
    let nodes: Vec<ElementRef> = html.select(&selector_comment_tree)
        .collect();
    if nodes.len() != 1 {
        log::warn!("Found multiple comment tree roots; using first");
    }
    let root = nodes.get(0)
        .ok_or("Did not find comment-tree root.")?;

    // TODO: When using next(), should these check whether the length > 1? 
    // TODO: Write some kind of helper function to avoid the current use of
    // .next().ok_or("blah")?.text().next().ok_or("more blah").to_string()

    // This ID fails on comment parsing: 27165954

    // Query the HTML for each comment node. Parse to a Comment structs,
    // and collect the Comments in a Vec.
    let mut comments: Vec<Comment> = Vec::new();
    for node in root.select(&selector_comment) {
        let id = node.value()
            .id()
            .ok_or("Title node did not have HTML Id attribute")?
            .parse::<Id>()?;
        let text = node.select(&selector_comment_text)
            .next()
            .ok_or("Failed to find comment text under a comment node")?
            .text()
            .next()
            .ok_or("Failed to extract inner text from comment text node")?
            .to_string();
        let user = node.select(&selector_comment_user)
            .next()
            .ok_or("Failed to find comment user under a comment node")?
            .text()
            .next()
            .ok_or("Failed to extract inner text from comment user node")?
            .to_string();
        let indent = node.select(&selector_indent)
            .next()
            .ok_or("Failed to find indent node under a comment node")?
            .value()
            .attr("width")
            .ok_or("Failed to extract width attr from comment indent node")?
            .parse::<i32>()?;
        let children = vec![];
        comments.push(Comment { user, id, text, indent, children });
    }

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

    #[test]
    fn test_extract_comments() -> Result<(), Box<dyn Error>> {
        let text = get_test_text();
        let html = Html::parse_fragment(&text);
        let comments = extract_comments(&html)?;
        println!("comments = {:#?}", comments);

        Ok(())
    }

    #[test]
    fn test_comment_tree() -> Result<(), Box<dyn Error>> {
        let text = get_test_text();
        let html = Html::parse_document(&text);
        let comments = extract_comments(&html)?;
        let forest = create_comment_tree(comments);
        println!("forest = {:#?}", forest);

        Ok(())
    }
}