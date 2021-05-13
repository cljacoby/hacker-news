use std::error::Error;
use std::collections::HashMap;
use std::sync::Once;
use std::cell::RefCell;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest;
use reqwest::blocking::Client as ReqwestClient;
use reqwest::blocking::ClientBuilder;
use reqwest::header::HeaderValue;
use reqwest::header::HeaderMap;
use reqwest::cookie::Cookie;
use reqwest::redirect::Policy;
use scraper::Html;
use scraper::Selector;
use scraper::element_ref::Select;
use scraper::ElementRef;
use env_logger;
use log;
use log::LevelFilter;
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

const URL_LOGIN: &'static str =         "https://news.ycombinator.com/login";
const URL_SUBMIT_FORM: &'static str =   "https://news.ycombinator.com/submit";
const URL_SUBMIT: &'static str =        "https://news.ycombinator.com/r";

lazy_static! {
    static ref FNID_REGEX: Regex =  Regex::new(r#"<input.*value="(.+?)".*>"#).unwrap();
}

pub type Score=u32;
pub type Id=u32;

pub struct Date(pub u16, pub u8, pub u8);

#[derive(Debug)]
pub struct Listing {
    pub title: String,
    pub id: Id,
    pub score: Option<Score>,
    pub user: Option<String>,
    // comments: u32,
    pub url: String,
}

pub struct Client {
    http_client: reqwest::blocking::Client,
    username: String,
    password: String,
    cookie: Option<(String, String)>,
}

impl Client {

    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
            http_client: ReqwestClient::new(),
            cookie: None,
        }
    }

    fn fail_unauthenticated(&self) -> Box<HNError> {
        HNError::boxed("Cannot perform action because client is unauthenticated")
    }

    fn encode_user_cooke(&self) -> Result<String, Box<dyn Error>> {
        match self.cookie {
            None => Err(self.fail_unauthenticated()),
            Some(ref cookie) => Ok(format!("{}={};", cookie.0, cookie.1))
        }
    }

    fn submit(
        &self,
        title: String,
        url: Option<String>,
        text: Option<String>,
    ) -> Result<(), Box<dyn Error>> {

        if self.cookie.is_none() {
            return Err(self.fail_unauthenticated());
        }

        let mut formdata = HashMap::new();
        formdata.insert("fnid", self.get_fnid()?);
        formdata.insert("fnop", "submit-page".to_string());
        formdata.insert("url", url.unwrap_or("".to_string()));
        formdata.insert("text", text.unwrap_or("".to_string()));
        log::debug!("submit post body = {:?}", formdata);
        formdata.insert("title", title);
        
        let cookie: HeaderValue = self.encode_user_cooke()?.parse().unwrap();
        let req = self.http_client.post(URL_SUBMIT)
            .header("Cookie", cookie)
            .form(&formdata);
        log::debug!("submit post request = {:?}", req);
        let resp = req.send()?;
        log::debug!("submit post response = {:?}", resp);
        
        Ok(())

    }
    
    fn get_fnid(&self) -> Result<String, Box<dyn Error>> {
        if self.cookie.is_none() {
            return Err(self.fail_unauthenticated());
        }
    
        let cookie: HeaderValue = self.encode_user_cooke()?.parse().unwrap();
        let req = self.http_client
            .get(URL_SUBMIT_FORM)
            .header("Cookie", cookie);
        log::debug!("submit form request = {:?}", req);
        let resp = req.send()?;
        log::debug!("submit form response = {:?}", resp);
        let body = resp.text()?;
        let dom = Html::parse_document(&body);
        
        // Underlying library doesn't implement std::error::Error on their
        // Error structs, so I can't include it as the src error in my struct
        let selector = match Selector::parse("input[name='fnid']") {
            Err(_src) => {
                return Err(HNError::boxed("Unable to parse css query selector."));
            },
            Ok(selector) => selector,
        };
    
        let result: Vec<ElementRef> = dom.select(&selector).collect();
        let el = match result.get(0) {
            Some(el) => el,
            None => {
                return Err(HNError::boxed("Could not locate fnid input from submission form."));
            }
        };
        let fnid = extract_fnid(el)?;
    
        Ok(fnid)
    }

    pub fn login(&mut self) -> Result<(), Box<dyn Error>> {
        
        // Create form-data body parameters
        let mut formdata = HashMap::new();
        formdata.insert("acct", &self.username);
        formdata.insert("pw", &self.password);
        let goto = "newest".to_string();
        formdata.insert("goto", &goto);

        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", "hnews client/0.0.1".parse().unwrap());

        // Login request requires no redirect on response, therefore we build a 
        // new one rather than referencing self.http_client.
        let client = ClientBuilder::new()
            .redirect(Policy::none())
            .build()?;

        // Send login request
        let req = client.post(URL_LOGIN)
            .headers(headers)
            .form(&formdata);
        log::debug!("login req = {:?}", req);
        let resp = req.send()?;
        log::debug!("login resp = {:?}", resp);

        // Store user session cookie
        let cookies: Vec<Cookie> = resp.cookies().collect();
        let cookie = cookies.get(0)
            .ok_or("Unable to retrieve user cookie")?;
        self.cookie = Some((cookie.name().to_string(), cookie.value().to_string()));
        println!("cookie = {:?}", self.cookie);

        Ok(())
    }
    
    pub fn item(&self, id: Id) -> Result<Listing, Box<dyn Error>> {
        // Retrieve front page HTML
        let url = format!("https://news.ycombinator.com/item?id={}", id);
        let req = self.http_client.get(&url);
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

    pub fn news(&self) -> Result<Vec<Listing>, Box<dyn Error>> {
        self.listings("https://news.ycombinator.com/news")
    }

    pub fn front(&self, date: Option<Date>) -> Result<Vec<Listing>, Box<dyn Error>> {
        let url = match date {
            None => "https://news.ycombinator.com/front".to_string(),
            Some(d) => format!("https://news.ycombinator.com/front?day={}-{}-{}", d.0, d.1, d.2),
        };

        self.listings(&url)
    }

    /// Retrieve a page of HackerNews listings, such as the front page, or the
    /// sub-paths:
    ///   - /news
    ///   - /front
    ///   - /ask
    ///   - /show
    ///   - /jobs
    pub fn listings(&self, url: &str) -> Result<Vec<Listing>, Box<dyn Error>> {
        let req = self.http_client.get(url);
        let resp = req.send()?;
        let text = resp.text()?;
        let html = Html::parse_document(&text);
        let listings = extract_listings(html)?;

        Ok(listings)
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
        let client = Client::new("filler_user", "filler_pwd");
        let listings = client.news()?;
        log::debug!("test_news listings = {:#?}", listings);

        Ok(())
    }

    #[test]
    fn test_item() -> Result<(), Box<dyn Error>> {
        setup();
        let client = Client::new("filler_user", "filler_pwd");
        let item = client.item(25925926)?;
        log::debug!("test_item item = {:#?}", item);

        Ok(())
    }

}

// =============================================================================
// =============================================================================
// =============================================================================
// HTML Parsing Helpers

fn extract_listings(html: Html) -> Result<Vec<Listing>, Box<dyn Error>> {

    /* Note:
     * The ':not' clause in selector_title is to obtain only the html representing
     * listings, and not also the html representing comments. The need for this
     * makes me wonder if there is a better way to extract expected HTML into a
     * struct.
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

        /* Note:
         * The subtext node is assumed to be the next adjacent sibling
         * node from a given title node. There are no other distinguishing HTML
         * patterns with which to capture this node.
         * */
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

fn extract_fnid(el: &ElementRef) -> Result<String, Box<dyn std::error::Error>> {
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
