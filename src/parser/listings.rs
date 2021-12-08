use std::error::Error;
use log;
use lazy_static::lazy_static;
use scraper::Html;
use scraper::Selector;
use scraper::ElementRef;
use selectors::attr::CaseSensitivity;
use crate::parser;
use crate::parser::HtmlParse;
use crate::parser::ancestor;
use crate::error::HnError;
use crate::model::Listing;
use crate::model::Id;
use crate::model::Score;

lazy_static! {
    // Applied to root of HTML document
    static ref QS_LISTING: Selector = Selector::parse("tr.athing:not(.comtr)").unwrap();
    
    // Applied to listing node (i.e. node `tr.athing:not(.comtr)"`)
    static ref QS_LISTING_TITLE: Selector = Selector::parse("td.title > a.titlelink").unwrap();

    // Applied to listing subtext node
    static ref QS_LISTING_USER: Selector = Selector::parse("a.hnuser").unwrap();
    static ref QS_SELECTOR_SCORE: Selector = Selector::parse("span.score").unwrap();

    // Applied to the table root node of a listing; either table.fatitem, or table.itemlist
    static ref QS_TBODY: Selector = Selector::parse("tbody").unwrap();

    // Applied to the tbody node of a listing's table
    static ref QS_TR: Selector = Selector::parse("tr").unwrap();

    // Applied to the 'tr' node containing a listing's associated text
    static ref QS_MORE_TEXT: Selector = Selector::parse("p").unwrap();
}

pub struct ListingsParser;

impl HtmlParse for ListingsParser {
    type Item = Vec<Listing>;
    
    fn parse(html: &Html) -> Result<Self::Item, Box<dyn Error>> {
        let mut listings = Vec::new();
        for node in html.select(&QS_LISTING) {
            let id = Self::parse_id(&node)?;
            log::debug!("Attempting parse of listing for id = {:?}", id);
            let text = Self::parse_text(&node, id)?;
            log::debug!("text = {:?}", text);
            let subtext_node = Self::query_subtext_node(&node, id)?; 
            let title_node = Self::query_title_node(&node, id)?;
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
                url,
                text
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


    fn parse_text(node: &ElementRef, id: Id) -> Result<Option<String>, Box<dyn Error>> {
        let table = ancestor(node, 2).ok_or_else(|| {
            log::error!("Did not find listing table node for id = {:?}", id);
            HnError::HtmlParsingError
        })?;
        let el = table.value();

        if el.has_class("itemlist", CaseSensitivity::AsciiCaseInsensitive) {
            log::debug!("Classed listing as '.itemlist' for id = {:?}", id);
            return Ok(None);
        }

        if el.has_class("fatitem", CaseSensitivity::AsciiCaseInsensitive) {
            log::debug!("Classed listing as '.fatitem' for id = {:?}", id);
            let tbody = table.select(&QS_TBODY)
                .next()
                .ok_or_else(|| {
                    log::error!("Did not find tbody for listing id = {:?}", id);
                    HnError::HtmlParsingError
            })?;
            let rows: Vec<_> = tbody.select(&QS_TR).collect();
            log::debug!("Listing .fatitem table row count = {:?}", rows.len());
            if rows.len() == 6 {
                let text_node = rows.get(3)
                    .ok_or_else(|| {
                    log::error!("Did not find listing text node for id = {:?}", id);
                    HnError::HtmlParsingError
                })?;
                let text = Self::parse_text_helper(text_node, id)?;
                return Ok(Some(text));
            }

            log::warn!("Classed listing as .fatitem, but row count was not 6 for id = {:?}", id);
            return Ok(None);
        }

        log::error!("Found listing table, but did not match any expected classes. id = {:?}", id);
        Err(Box::new(HnError::HtmlParsingError))
    }

    fn parse_text_helper(node: &ElementRef, id: Id) -> Result<String, Box<dyn Error>> {
        let mut text = node.text()
            .next()
            .ok_or_else(|| {
                log::error!("Did not find listing text from expected text node, id = {}", id);
                HnError::HtmlParsingError
            })?
            .to_string();
        parser::append_more_text_nodes(node, &QS_MORE_TEXT, &mut text);

        Ok(text)
    }

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

}
