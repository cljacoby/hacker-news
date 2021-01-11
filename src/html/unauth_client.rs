use std::error::Error;
use reqwest::blocking::Client;
use scraper::Html;
use scraper::Selector;
use scraper::ElementRef;
use env_logger;
use log;

use crate::html::init_logger;
use crate::error::HNError;

/* Note:
 *
 * Job posts do not have subtext fields like the author or score, and
 * therefore both the listing struct needs these as optional, and the
 * parsing logic needs to know how to recognize the absence.
 *
 * Consider how fields like `score` and `id` should be shared between
 * firebase and html client
 *
 * */

#[derive(Debug)]
struct HNListig {
    title: String,
    id: u64,
    score: Option<u32>,
    user: Option<String>,
    comments: u32,
    url: String,
}

const FRONT_PAGE: &'static str = "https://news.ycombinator.com";

// const QUERYSEL_TITLE: &'static str = "td.title > a";
// const QUERYSEL_SUBTEXT: &'static str = "td.title > a";


fn extract_hn_listing(
    title_node: ElementRef,
    subtext_node: ElementRef
) -> HNListig {


    unimplemented!();
}

trait UnauthOps {

    fn http_client(&self) -> &Client;

    fn front_page(&self) -> Result<(), Box<dyn Error>> {

        // Retrieve front page HTML
        let client = self.http_client();
        let req = client.get(FRONT_PAGE);
        let resp = req.send()?;
        let html = resp.text()?;
        let dom = Html::parse_document(&html);

        let selector_top = Selector::parse("table.itemlist tr.athing")
            .expect("Failed to parse scraper Selector (listing)");

        let select_title = Selector::parse("td.title a.storylink")
            .expect("Failed to parse scraper Selector (title)");
        
        // Use the next sibling of the <tr.athing> as root node
        let select_subtext_score = Selector::parse("span.score")
            .expect("Failed to parse scraper Selector (score)");
        let select_subtext_user = Selector::parse("a.hnuser")
            .expect("Failed to parse scraper Selector (user)");

        // Parse HTML into Vec of HNListigs
        let nodes: Vec<ElementRef> = dom.select(&selector_top).collect();
        let hn_listings: Vec<HNListig> = Vec::new();
        
        for node in nodes {
            let element = node.value();


            let subtext_node = match node.next_sibling() {
                None => {
                    return Err(HNError::boxed("Could not find subtext as next sibling of title node."));
                },
                Some(subtext_node) => subtext_node,
            };
            let subtext_el = ElementRef::wrap(subtext_node).unwrap();
            // TODO: Replace with actual error handling
            let user_el: ElementRef = subtext_el.select(&select_subtext_user).next().unwrap();
            let user = user_el.text().next().unwrap();
            let score_el: ElementRef = subtext_el.select(&select_subtext_score).next().unwrap();
            let score = score_el.text().next().unwrap();


            let title_node = match node.select(&select_title).next() {
                None => return Err(HNError::boxed("Did not find HTML title for listing.")),
                Some(title_node) => title_node,
            };
            

            let id = element.id();
            let title_el = title_node.value();
            let title = title_node.text().next();
            let url = title_el.attr("href");

            println!("id = {:?}", id);
            println!("title = {:?}", title);
            println!("url = {:?}", url);
            println!("user = {:?}", user);
            println!("score = {:?}", score);
            println!("*******************")
        }


        
        Ok(())
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

    #[test]
    fn test_front_page() -> Result<(), Box<dyn Error>> {
        // env_logger::init();
        init_logger();
        let client = UnauthClient::new();
        client.front_page()?;

        Ok(())
    }

}
