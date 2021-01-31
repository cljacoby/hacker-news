use std::error::Error;
use reqwest::blocking::Client;
use scraper::Html;
use scraper::Selector;
use scraper::element_ref::Select;
use scraper::ElementRef;
use env_logger;
use log;

use std::sync::Once;

use crate::html::init_logger;
use crate::error::HNError;

/* Note:
 *
 * Consider how fields like `score` and `id` should be shared between
 * firebase and html clients
 *
 * Selector::parse() returns an Error type which doesn't implement
 * std::error::Error, and therefore doesn't work using Result<T, Box<dyn Error>>
 * as a return type. Maybe open a pull request on the lib to implement
 * std::error::Error.
 *
 * Because all the methods are implemented in a trait, there is no concept
 * of public vs. private. Maybe consider not using the trait? Or maybe
 * consider not having seperate structs for auth and unauth
 * */


/* Todo:
 *
 * Add a `comment` attribute to the Listing struct, and have it be the comment
 * count displayed on a Listing.
 *
 * If Item and Struct are basically all the same information, consider representing
 * them as a single struct.
 *
 * Consider placing all HTML parsing logic into the `extract` function
 * */

type Score=u32;
type Id=u32;

#[derive(Debug)]
struct Listing {
    title: String,
    id: Id,
    score: Option<Score>,
    user: Option<String>,
    // comments: u32,
    url: String,
}

const FRONT_PAGE: &'static str = "https://news.ycombinator.com";



fn extract_listings(html: Html) -> Result<Vec<Listing>, Box<dyn Error>> {

    /* Note:
     * The :not clause in selector_title is to obtain only the html representing
     * listings, and not also the html representing comments. The need for this
     * makes me wonder if there is a better way to extract expected HTML into an
     * expected struct.
     * */

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

        // Note: The subtext node is assumed to be the next adjacent sibling
        // node from a given title node. There are no other distintive HTML
        // patterns which capture this node.
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


trait UnauthOps {

    /* public methods */

    fn news(&self) -> Result<Vec<Listing>, Box<dyn Error>> {
        self.get_listings(FRONT_PAGE)
    }
    
    fn item(&self, id: Id) -> Result<Listing, Box<dyn Error>> {
        // Retrieve front page HTML
        let url = format!("https://news.ycombinator.com/item?id={}", id);
        let client = self.http_client();
        let req = client.get(&url);
        let resp = req.send()?;
        let text = resp.text()?;
        let html = Html::parse_document(&text);

        // Note: There is an assumption here that given an item ID, we should
        // only extract one listing from a page. Therefore, we can simply pop once
        // from the Vec obtained by extract listings.

        let item = extract_listings(html)?
            .pop()
            .ok_or(format!("Did not find item {}", id))?;

        Ok(item)
    }

    fn front(&self, date: Option<String>) -> Result<Listing, Box<dyn Error>> {



        unimplemented!()
    }


    
    /* private methods */

    fn http_client(&self) -> &Client;

    fn get_listings(&self, url: &str) -> Result<Vec<Listing>, Box<dyn Error>> {
        // Retrieve front page HTML
        let client = self.http_client();
        let req = client.get(url);
        let resp = req.send()?;
        let text = resp.text()?;
        let html = Html::parse_document(&text);
        let listings = extract_listings(html)?;

        Ok(listings)
    }


}

struct UnauthClient {
    http_client: Client,
}

impl UnauthOps for UnauthClient {
    fn http_client(&self) -> &Client {
        &self.http_client
    }
}

impl UnauthClient {
    fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    static LOG_INIT: Once = Once::new(); 
    
    fn setup() {
        LOG_INIT.call_once(|| {
            init_logger()
        });
    }

    #[test]
    fn test_news() -> Result<(), Box<dyn Error>> {
        setup();
        let client = UnauthClient::new();
        let listings = client.news()?;
        log::debug!("test_news listings = {:#?}", listings);

        Ok(())
    }

    #[test]
    fn test_item() -> Result<(), Box<dyn Error>> {
        setup();
        let client = UnauthClient::new();
        let item = client.item(25925926)?;
        log::debug!("test_item item = {:#?}", item);

        Ok(())
    }

}
