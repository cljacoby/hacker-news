use std::error::Error;
use log;
use lazy_static::lazy_static;
use scraper::Html;
use scraper::Selector;
use scraper::ElementRef;
use selectors::attr::CaseSensitivity;
use crate::parser::HtmlParse;
use crate::parser::ancestor;
use crate::error::HnError;
use crate::model::Listing;
use crate::model::Id;
use crate::model::Score;

/*
 * Examples of different table.fatitem row counts:
 *
 * AskHN, len=6, https://news.ycombinator.com/item?id=27650775
 * Jobs, len=2, https://news.ycombinator.com/item?id=27660734
 * Story, len=4, https://news.ycombinator.com/item?id=27649211
 *
 * First row is always the title node.
 * Second row is always subtext node; however, subtext only optionally has comment count,
 *   author, score, etc. Sometimes it only has the hide link (i.e. with jobs)
 * Third row (if present), is a spacer
 * Fourth row (if present) can either be the comment submission box, or the post's text
 *   depending on whether the OP had text assocaited with it
 * */

lazy_static! {
    // Applied to root of HTML document
    static ref QS_LISTING: Selector = Selector::parse("tr.athing:not(.comtr)").unwrap();
    
    // Applied to listing node (i.e. node `tr.athing:not(.comtr)"`)
    static ref QS_LISTING_TITLE: Selector = Selector::parse("td.title > a.storylink").unwrap();

    // Applied to listing subtext node
    static ref QS_LISTING_USER: Selector = Selector::parse("a.hnuser").unwrap();
    static ref QS_SELECTOR_SCORE: Selector = Selector::parse("span.score").unwrap();

    // Applied to the table root node of a listing; either table.fatitem, or table.itemlist
    static ref QS_TBODY: Selector = Selector::parse("tbody").unwrap();
}


// TODO: Add a description on what this is and why we need it.
#[derive(Debug)]
enum ListingType {
    ItemList,
    FatItem(usize),
}

pub struct ListingsParser;

impl HtmlParse for ListingsParser {
    type Item = Vec<Listing>;
    
    fn parse(html: &Html) -> Result<Self::Item, Box<dyn Error>> {
        let mut listings = Vec::new();
        for node in html.select(&QS_LISTING) {
            let id = Self::parse_id(&node)?;
            log::debug!("Attempting parse of listing for id = {:?}", id);
            let listing_type = Self::listing_type(&node, id)?;
            log::debug!("listing_type = {:?}", listing_type);
            let subtext_node = Self::query_subtext_node(&node, id)?; 
            let title_node = Self::query_title_node(&node, id)?;
            match listing_type {
                ListingType::ItemList => {
                }
                ListingType::FatItem(rows) => {
                    Self::parse_post_comment(&title_node, id)?;
                }
            }
            let user = Self::parse_user(&subtext_node, id)?;
            let score = Self::parse_score(&subtext_node, id)?;
            let title = Self::parse_title(&title_node, id)?;
            let url = Self::parse_url(&title_node, id)?;
            log::debug!("Succesfully parsed listing for id = {:?}", id);
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

}

impl ListingsParser {

    // This function identifies the subtext node for a listing, which contains
    // some of the key fields. The subtext node is the next adjacent sibling node
    // from the listing root node (i.e. the node located by the query selector QS_LISTING).
    // There are no other distinguishing features to otherwise query this node with a
    // query selector.
    fn query_subtext_node<'a>(node: &'a ElementRef, id: Id) -> Result<ElementRef<'a>, Box<dyn Error>> {
        let node_ref = node.next_sibling()
            .ok_or_else(|| {
                log::error!("Did not find subtext node as next sibling from listing node, id = {}", id);
                HnError::HtmlParsingError
            })?;

        let element_ref = ElementRef::wrap(node_ref)
            .ok_or_else(|| {
                log::error!("Could not wrap NodeRef in ElementRef for subtext node, id = {}", id);
                HnError::HtmlParsingError
            })?;
        
        Ok(element_ref)
    }

    fn listing_type(node: &ElementRef, id: Id) -> Result<ListingType, Box<dyn Error>> {
        let table = ancestor(&node, 2)
            .ok_or_else(|| {
                log::error!("Could not find listing table node for id = {:?}", id);
                HnError::HtmlParsingError
            })?;
        let el = table.value();

        if el.has_class("itemlist", CaseSensitivity::AsciiCaseInsensitive) {
            Ok(ListingType::ItemList)
        }
        else if el.has_class("fatitem", CaseSensitivity::AsciiCaseInsensitive) {
            let tbody = table.select(&QS_TBODY)
                .next()
                .ok_or_else(|| {
                    log::error!("Could not locate tbody for listing id = {:?}", id);
                    HnError::HtmlParsingError
                })?;
            let rows: Vec<_> = tbody.children().collect();
            Ok(ListingType::FatItem(rows.len()))
        }
        else {
            log::error!("Found table root for listing, but could not match any expected classes. id = {:?}", id);
            Err(Box::new(HnError::HtmlParsingError))
        }
    }
    
    // fn parse_listing_text(node: &ElementRef, id: Id) -> Result<String, Box<dyn Error>> {
    //     unimplemented!()
    // }

    fn query_title_node<'a>(node: &'a ElementRef, id: Id) -> Result<ElementRef<'a>, Box<dyn Error>> {
        let title_node = node.select(&QS_LISTING_TITLE)
            .next()
            .ok_or_else(|| {
                log::error!("Did not find title node under for listing id = {}", id);
                HnError::HtmlParsingError
            })?;

        Ok(title_node)
    }

    fn parse_id(node: &ElementRef) -> Result<Id, Box<dyn Error>> {
        let id = node
            .value()
            .id()
            .ok_or_else(|| {
                log::error!("Could not find id for listing node, HTML = '{:?}'", node.html());
                HnError::HtmlParsingError
            })?
            .parse::<Id>()?;

        Ok(id)
    }

    // Note: This function queries against the subtext node
    fn parse_user(node: &ElementRef, id: Id) -> Result<Option<String>, Box<dyn Error>> {
        let user_node = match node.select(&QS_LISTING_USER).next() {
            None => {
                log::info!("Failed to locate user node for listing. \
                This is probably a Jobs, Launch, or Poll listing. id = {}", id);
                return Ok(None);
            },
            Some(user_node) => user_node,
        };

        let user = user_node.text()
            .next()
            .ok_or_else(|| {
                log::error!("Failed to obtain user text from listing user node, id = {}", id);
                HnError::HtmlParsingError
            })?
            .to_string();

        Ok(Some(user))
    }
   
    // Note: This function queries against the subtext node
    fn parse_score(node: &ElementRef, id: Id) -> Result<Option<Score>, Box<dyn Error>> {
        let score_node = match node.select(&QS_SELECTOR_SCORE).next() {
            None => {
                log::info!("Failed to locate score node for listing. \
                This is probably a Jobs, Launch, or Poll listing. id = {}", id);
                return Ok(None);
            },
            Some(score_node) => score_node,
        };

        let text = score_node.text()
            .next()
            .ok_or_else(|| {
                log::error!("Failed to obtain score text from listing score node, id = {}", id);
                HnError::HtmlParsingError
            })?;

        let score = text.strip_suffix(" points")
            .ok_or_else(|| {
                log::error!("Failed to strip ' points' from listing score text. id = {}, text =  {},",
                    id, text);
                HnError::HtmlParsingError
            })?
            .parse::<Score>()
            .map_err(|_src_err| {
                log::error!("Failed to parse score numeric value from score text, id = {}, text = {}",
                    id, text);
                HnError::HtmlParsingError
            })?;

        Ok(Some(score))
    }
    
    // Note: This function queries against the title node
    fn parse_title(node: &ElementRef, id: Id) -> Result<String, Box<dyn Error>> {
        let title = node.text()
            .next()
            .ok_or_else(|| {
                log::error!("Could not get score text for listing = {}", id);
                HnError::HtmlParsingError
            })?
            .to_string();

        Ok(title)
    }
    
    fn parse_url(node: &ElementRef, id: Id) -> Result<String, Box<dyn Error>> {
        let url = node.value()
            .attr("href")
            .ok_or_else(|| {
                log::error!("Listing title link had missing 'href' attribute, listing id = {}", id);
                HnError::HtmlParsingError
            })?
            .to_string();

        Ok(url)
    }

    // Parse the text associated with original post. For example, the question associated with an
    // AskHN post. The comment node is located as the 3rd adjacent sibling from the title node.
    // fn parse_post_comment(title_node: &ElementRef, id: Id) -> Result<String, Box<dyn Error>> {
    fn parse_post_comment(title_node: &ElementRef, id: Id) -> Result<(), Box<dyn Error>> {
        let mut i = 0;
        let n = 3;
        let mut comment_node: Option<ElementRef> = None;

        while i < n {
            let node_ref = title_node.next_sibling().ok_or_else(|| {
                log::error!("Did not locate comment node for '.fatitem' listing id = {:?}", id);
                HnError::HtmlParsingError
            })?;
            let el = ElementRef::wrap(node_ref).ok_or_else(|| {
                log::error!("Could not wrap node_ref for listing id = {:?}", id);
                HnError::HtmlParsingError
            })?;
            comment_node = Some(el);
            i += 1;
        }

        let comment_node = comment_node.ok_or_else(|| {
            log::error!("Did not locate post commend node for listing id = {:?}", id);
            HnError::HtmlParsingError
        })?;
        log::debug!("comment_node.html() = {:?}", comment_node.html());

        Ok(())
    }


    

}
