use std::error::Error;
use log;
use std::collections::HashMap;
use std::cell::RefCell;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest;
use reqwest::blocking::ClientBuilder;
use reqwest::header::HeaderValue;
use reqwest::header::HeaderMap;
use reqwest::cookie::Cookie;
use reqwest::redirect::Policy;
use scraper;
use scraper::Html;
use scraper::Selector;
use scraper::ElementRef;
use crate::error::HttpError;
use crate::error::HnError;
use crate::parser::HtmlParse;
use crate::parser::ListingsParser;
use crate::parser::CommentsParser;
use crate::parser::extract_fnid;
use crate::parser::comments::create_comment_tree;
use crate::model::Id;
use crate::model::Listing;
use crate::model::Date;
use crate::model::Thread;


const URL_LOGIN: &str = "https://news.ycombinator.com/login";
const URL_SUBMIT_FORM: &str = "https://news.ycombinator.com/submit";
const URL_SUBMIT: &str = "https://news.ycombinator.com/r";

lazy_static! {
    static ref FNID_REGEX: Regex =  Regex::new(r#"<input.*value="(.+?)".*>"#).unwrap();
}

pub struct Client {
    http_client: reqwest::blocking::Client,
    cookie: RefCell<Option<(String, String)>>,
}

impl Client {

    pub fn new() -> Self {
        Self {
            http_client: reqwest::blocking::Client::new(),
            cookie: RefCell::new(None),
        }
    }

    fn cookie(&self) -> Result<String, Box<dyn Error>> {
        // Note: Chaining these causes a compiler error about dropping to early
        let pair = self.cookie.borrow();
        let pair = pair.as_ref().ok_or(HnError::UnauthenticatedError)?;

        Ok(format!("{}={};", pair.0, pair.1))
    }

    pub fn submit(
        &self,
        title: String,
        url: Option<String>,
        text: Option<String>,
    ) -> Result<(), Box<dyn Error>> {

        let cookie_string = self.cookie()?;
        let cookie: HeaderValue = cookie_string.parse()
            .expect("Got a user cookie, but failed to parse it to a header");

        let mut formdata = HashMap::new();
        formdata.insert("fnid", self.get_fnid()?);
        formdata.insert("fnop", "submit-page".to_string());
        formdata.insert("url", url.unwrap_or_else(|| "".to_string()));
        formdata.insert("text", text.unwrap_or_else(|| "".to_string()));
        log::debug!("submit post body = {:?}", formdata);
        formdata.insert("title", title);
        
        let req = self.http_client.post(URL_SUBMIT)
            .header("Cookie", cookie)
            .form(&formdata);
        log::debug!("submit post request = {:?}", req);
        let resp = req.send()?;
        log::debug!("submit post response = {:?}", resp);
        
        Ok(())

    }
    
    fn get_fnid(&self) -> Result<String, Box<dyn Error>> {
        let cookie_string = self.cookie()?;
        let cookie: HeaderValue = cookie_string.parse()
            .expect("Got a user cookie, but failed to parse it to a header");
    
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
                return Err(Box::new(HnError::HtmlParsingError));
            },
            Ok(selector) => selector,
        };
    
        let result: Vec<ElementRef> = dom.select(&selector).collect();
        let el = match result.get(0) {
            Some(el) => el,
            None => {
                return Err(Box::new(HnError::HtmlParsingError));
            }
        };
        let fnid = extract_fnid(el)?;
    
        Ok(fnid)
    }

    pub fn login(&self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        let mut formdata = HashMap::new();
        formdata.insert("acct", username);
        formdata.insert("pw", password);
        let goto = "newest".to_string();
        formdata.insert("goto", &goto);

        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", "hacker-news client/0.0.1".parse().unwrap());

        // Login request requires no redirect on response, therefore we build a 
        // new one rather than referencing self.http_client.
        // TODO: Is there a better way to accomodate this?
        let client = ClientBuilder::new()
            .redirect(Policy::none())
            .build()?;

        // Send login request
        let req = client.post(URL_LOGIN)
            .headers(headers)
            .form(&formdata);
        log::debug!("login request = {:?}", req);
        let resp = req.send()?;
        if resp.status().as_u16() != 302 {
            log::error!("login response = {:?}", resp);
            return Err(Box::new(HnError::AuthenticationError));
        }
        log::debug!("login response = {:?}", resp);

        // Store user session cookie
        let cookies: Vec<Cookie> = resp.cookies().collect();
        let cookie = cookies.get(0)
            // .ok_or("Unable to retrieve user cookie")?;
            .ok_or_else(|| {
                log::error!("Unable to parse user cookie from succesful login response, \
                    response = {:?}, cookies = {:?}", resp, cookies);
                HnError::HtmlParsingError
            })?;
        let cookie = Some((cookie.name().to_string(), cookie.value().to_string()));

        // Store on client instance field
        *self.cookie.borrow_mut() = cookie;
        println!("cookie = {:?}", self.cookie);

        Ok(())
    }
    
    pub fn item(&self, id: Id) -> Result<Listing, Box<dyn Error>> {
        let url = format!("https://news.ycombinator.com/item?id={}", id);
        let req = self.http_client.get(&url);
        log::debug!("Send GET request to {:?}", url);
        let resp = req.send()?;
        let status = resp.status().as_u16();
        if status != 200 {
            let http_err = HttpError::new(status, resp.url().to_string());
            log::error!("Received not 200 response: {:?}, thread id: {:?}", http_err, id);
            return Err(Box::new(HnError::HttpError(http_err)));
        }
        log::debug!("Received 200 response from {:?}", url);

        let text = resp.text()?;
        let html = Html::parse_document(&text);

        // Note: There is an assumption here that given an item ID, we should
        // only receive one listing per page. Therefore, we can simply pop once
        // from the Vec of extracted listings.

        let item = ListingsParser::parse(&html)?
            .pop()
            .ok_or(format!("Did not find item {}", id))?;

        Ok(item)
    }

    pub fn thread(&self, id: Id) -> Result<Thread, Box<dyn Error>> {
        log::debug!("HTML client attempting comments for id = {:?}", id);
        let url = format!("https://news.ycombinator.com/item?id={}", id);
        let req = self.http_client.get(&url);
        let resp = req.send()
            .map_err(|src| HnError::NetworkError(Some(Box::new(src))))?;
        let text = resp.text()
            .map_err(|src| HnError::NetworkError(Some(Box::new(src))))?;
        let html = Html::parse_document(&text);
        let comments = CommentsParser::parse(&html)
            .map_err(|src| HnError::HtmlParsingError)?;
        let comments = create_comment_tree(comments);
        let listings = ListingsParser::parse(&html)
            .map_err(|src| HnError::HtmlParsingError)?;
        if listings.len() > 1 {
            log::warn!("Parsed multiple listings for a thread, where only 1 is expected");
        }
        let listing = listings.into_iter()
            .next()
            .ok_or_else(|| {
                log::error!("Succesfully parsed HTML, but found no listings");
                HnError::HtmlParsingError
            })?;
        let thread = Thread { listing, comments };
        
        Ok(thread)
    }

    pub fn news(&self) -> Result<Vec<Listing>, Box<HnError>> {
        self.listings("https://news.ycombinator.com/news")
    }

    pub fn past(&self, date: Date) -> Result<Vec<Listing>, Box<HnError>> {
        let url = format!("https://news.ycombinator.com/front?day={}-{}-{}",
            date.0, date.1, date.2);

        self.listings(&url)
    }

    /// Retrieve a page of HackerNews Listings, such as that delivered from:
    /// * `https://news.ycombinator.com/`
    /// * `https://news.ycombinator.com/newest`
    /// * `https://news.ycombinator.com/front`
    /// * `https://news.ycombinator.com/newcomments`
    /// * `https://news.ycombinator.com/ask`
    /// * `https://news.ycombinator.com/show`
    /// * `https://news.ycombinator.com/jobs`
    pub fn listings(&self, url: &str) -> Result<Vec<Listing>, Box<HnError>> {
        let req = self.http_client.get(url);
        let resp = req.send()
            .map_err(|src| Box::new(HnError::NetworkError(Some(Box::new(src)))))?;
        let text = resp.text()
            .map_err(|src| Box::new(HnError::NetworkError(Some(Box::new(src)))))?;
        let html = Html::parse_document(&text);
        // TODO: Should the originating of the HtmlParsingError happend within the parse logic,
        // and isntead be bubbled up to this level?
        let listings = ListingsParser::parse(&html)
            .map_err(|src| Box::new(HnError::HtmlParsingError))?;

        Ok(listings)
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    use crate::util::setup;

    #[test]
    fn test_news() -> Result<(), Box<dyn Error>> {
        setup();
        let client = Client::new();
        let listings = client.news()?;
        log::info!("Successfully called Client::news()");
        log::trace!("Listings output from Client::news() = {:?}", listings);

        Ok(())
    }

    #[test]
    fn test_item() -> Result<(), Box<dyn Error>> {
        setup();
        let client = Client::new();
        let item = client.item(25925926)?;
        log::debug!("test_item item = {:#?}", item);

        Ok(())
    }

    #[test]
    fn test_comments() -> Result<(), Box<dyn Error>> {
        setup();
        let client = Client::new();
        let comments = client.thread(100)?;
        log::debug!("comments = {:?}", comments);

        Ok(())
    }

    #[test]
    fn test_login() -> Result<(), Box<dyn Error>> {
        setup();
        let user: String = match std::env::var("HN_USER") {
            Ok(user) => user,
            Err(_) => {
                log::warn!("login test unable to retrieve Hacker News username from \
                environment variable $HN_USER. Omitting test.");
                return Ok(());
            }
        };

        let pwd: String = match std::env::var("HN_PASS") {
            Ok(pwd) => pwd,
            Err(_) => {
                log::warn!("login test unable to retrieve Hacker News password from \
                environment variable $HN_PASS. Omitting test.");
                return Ok(());
            }
        };
        
        let client = Client::new();
        client.login(&user, &pwd)?;

        Ok(())
    }

}
