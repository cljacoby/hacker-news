use std::error::Error;
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
use crate::error::HNError;

lazy_static! {
    static ref FNID_REGEX: Regex =  Regex::new(r#"<input.*value="(.+?)".*>"#).unwrap();
}

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

pub fn extract_comment_tree(html: &Html) -> Result<Vec<Comment>, Box<dyn Error>> {
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
        comments.push(Comment { user, id, text, indent });
    }


    Ok(comments)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_extract_comment_tree() -> Result<(), Box<dyn Error>> {
        let text = r#"
        <table border="0" class="comment-tree">
            <tbody>
                <tr class="athing comtr" id="27148500">
                <td>
                    <table border="0">
                    <tbody>
                        <tr>
                        <td class="ind"><img src="./hnews.27145911_files/s.gif" height="1" width="0"></td>
                        <td valign="top" class="votelinks">
                            <center>
                            <a id="up_27148500" onclick='return vote(event, this, "up")' href="https://news.ycombinator.com/vote?id=27148500&amp;how=up&amp;auth=fd562fec23feae003bfd2a9f341376bb745f9d3f&amp;goto=item%3Fid%3D27145911#27148500">
                                <div class="votearrow" title="upvote"></div>
                            </a>
                            </center>
                        </td>
                        <td class="default">
                            <div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
                            <a href="https://news.ycombinator.com/user?id=akersten" class="hnuser">akersten</a> <span class="age"><a href="https://news.ycombinator.com/item?id=27148500">32 minutes ago</a></span> <span id="unv_27148500"></span><span class="par"></span> <a class="togg" n="2" href="javascript:void(0)" onclick="return toggle(event, 27148500)">[&ndash;]</a>          <span class="storyon"></span>
                            </span>
                            </div>
                            <br>
                            <div class="comment">
                            <span class="commtext c00">
                                I'm going to close a website as soon as I get an unprompted popup that says "Firefox is trying to open Slack."
                                <p>It's clever but somewhat obvious (in both a to-the-user-that-its-happening and a "well of course it's possible" sense).</p>
                                <p>So it's cute, but not practical, and I won't lose sleep over it. I'll probably be more inconvenienced by the mitigations that will surely result that make it that much more painful to actually launch a URL scheme, sadly.</p>
                                <p>I've actually never checked the "Always open Slack for slack:// links" or similar checkboxes, precisely out of predicting shenanigans like this would happen eventually :)</p>
                                <p>I wouldn't be too offended if browsers changed the way they handle schemes: always open a "how would you like to handle  this link" dialog for any protocol (even if unhandled - like how Windows shows the "how would you like to open this file" dialog), to disguise whether the protocol is handled or not.  Not sure I have the answer for user convenience though if someone is used to things automatically opening. That's the "inconvenience" aspect of any potential mitigation, we probably have to get rid of that "remember this choice" checkbox (well, my point is that "have to" is debatable here).
                                </p>
                                <div class="reply">
                                <p><font size="1">
                                    <u><a href="https://news.ycombinator.com/reply?id=27148500&amp;goto=item%3Fid%3D27145911%2327148500">reply</a></u>
                                    </font>
                                </p>
                                </div>
                            </span>
                            </div>
                        </td>
                        </tr>
                    </tbody>
                    </table>
                </td>
                </tr>
                <tr class="athing comtr" id="27148679">
                    <td>
                      <table border="0">
                        <tbody>
                          <tr>
                            <td class="ind"><img src="./hnews.27145911_files/s.gif" height="1" width="40"></td>
                            <td valign="top" class="votelinks">
                              <center>
                                <a id="up_27148679" onclick='return vote(event, this, "up")' href="https://news.ycombinator.com/vote?id=27148679&amp;how=up&amp;auth=69ce9e200ff327bf18529fb30cbaea513b65f348&amp;goto=item%3Fid%3D27145911#27148679">
                                  <div class="votearrow" title="upvote"></div>
                                </a>
                              </center>
                            </td>
                            <td class="default">
                              <div style="margin-top:2px; margin-bottom:-10px;"><span class="comhead">
                                <a href="https://news.ycombinator.com/user?id=judge2020" class="hnuser">judge2020</a> <span class="age"><a href="https://news.ycombinator.com/item?id=27148679">10 minutes ago</a></span> <span id="unv_27148679"></span><span class="par"></span> <a class="togg" n="1" href="javascript:void(0)" onclick="return toggle(event, 27148679)">[&ndash;]</a>          <span class="storyon"></span>
                                </span>
                              </div>
                              <br>
                              <div class="comment">
                                <span class="commtext c00">
                                  On Chrome MacOS Big Sur, it doesn't require accepting the prompt, and the demo shows you can accomplish this in a small pop-under or pop-up, which a lot of inexperienced users might simply ignore.
                                  <p>Browser devs definitely still need to patch this vulnerability by making it an instant-return no-feedback prompt to open an application.
                                  </p>
                                  <div class="reply">
                                    <p><font size="1">
                                      <u><a href="https://news.ycombinator.com/reply?id=27148679&amp;goto=item%3Fid%3D27145911%2327148679">reply</a></u>
                                      </font>
                                    </p>
                                  </div>
                                </span>
                              </div>
                            </td>
                          </tr>
                        </tbody>
                      </table>
                    </td>
                  </tr>
            </tbody>
        </table>
        "#;

        let html = Html::parse_fragment(text);
        let comments = extract_comment_tree(&html)?;
        println!("comments = {:#?}", comments);

        Ok(())
    }

}