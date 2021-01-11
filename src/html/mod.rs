use std::collections::HashMap;
use std::error::Error;
use std::io::Write;
use reqwest::blocking::Client as rClient;
use reqwest::blocking::ClientBuilder;
use reqwest::cookie::Cookie;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::redirect::Policy;
use env_logger;
use env_logger::Builder;
use log;
use log::LevelFilter;
use scraper::Html;
use scraper::element_ref::ElementRef;
use scraper::selector::Selector;
use lazy_static::lazy_static;
use regex::Regex;
use crate::error::HNError;

pub mod unauth_client;

const LOGIN_URL: &'static str = "https://news.ycombinator.com/login";
const SUBMIT_FORM_URL: &'static str = "https://news.ycombinator.com/submit";
const SUBMIT_URL: &'static str = "https://news.ycombinator.com/r";

/*
 * TODO:
 *
 *  - Consider a library or a type to represent a cookie
 *  - Move the fnid request process to a separate method than the submit method
 *  - Create a re-usable method to encode the client's cookie string
 *  - I'm again thinking that having separate AuthClient and UnaauthClient structs
 *    might be a good idea. Basically avoiding checking in every authentication-required
 *    method whether or not the user has a user cookie.
 * */


lazy_static! {
    static ref FNID_REGEX: Regex =  Regex::new(r#"<input.*value="(.+?)".*>"#).unwrap();
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

#[derive(Debug)]
struct Client {
    username: String,
    password: String,
    client: rClient,
    cookie: Option<(String, String)>,
}

impl Client {
    pub fn new(username: &str, password: &str) -> Self {
        
        // We want no redirect for the login request, but what about the other requests?

        let rclient = ClientBuilder::new()
            .redirect(Policy::none())
            .build()
            .unwrap();

        Self {
            username: String::from(username),
            password: String::from(password),
            client: rclient,
            cookie: None,
        }
    }

    fn get_fnid(&self) -> Result<String, Box<dyn Error>> {
        if self.cookie.is_none() {
            return Err(self.fail_unauthenticated());
        }

        let cookie: HeaderValue = self.encode_user_cooke()?.parse().unwrap();
        let req = self.client.get(SUBMIT_FORM_URL)
            .header("Cookie", cookie);
        log::debug!("submit form request = {:?}", req);
        let resp = req.send()?;
        log::debug!("submit form response = {:?}", resp);
        let body = resp.text()?;
        let dom = Html::parse_document(&body);
        

        // TODO: Maybe this actually should use an unwrap. We're unwrapping a
        // static string slice the user does not configure, so if its fails
        // to parse its a library failure
        
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

        // Send login request
        #[rustfmt::skip]
        let req = self.client
            .post(LOGIN_URL)
            .headers(headers)
            .form(&formdata);
        log::debug!("login req = {:?}", req);
        let resp = req.send()?;
        log::debug!("login resp = {:?}", resp);

        // Store user session cookie
        let cookies: Vec<Cookie> = resp.cookies().collect();
        let cookie = match cookies.get(0) {
            None => {
                return Err(HNError::boxed("Unable to retrieve user cookie from login response"));
            },
            Some(cookie) => (cookie.name().to_string(), cookie.value().to_string()),
        };
        self.cookie = Some(cookie);

        Ok(())
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
        let fnid = self.get_fnid()?;
        let fnop = "submit-page".to_string();
        let url = url.unwrap_or("".to_string());
        let text = text.unwrap_or("".to_string());
        formdata.insert("fnid", &fnid);
        formdata.insert("fnop", &fnop);
        formdata.insert("title", &title);
        formdata.insert("url", &url);
        formdata.insert("text", &text);
        log::debug!("submit post body = {:?}", formdata);
        
        let cookie: HeaderValue = self.encode_user_cooke()?.parse().unwrap();
        let req = self.client.post(SUBMIT_URL)
            .header("Cookie", cookie)
            .form(&formdata);
        log::debug!("submit post request = {:?}", req);
        let resp = req.send()?;
        log::debug!("submit post response = {:?}", resp);
        
        Ok(())

    }

}

pub fn init_logger() {
    let mut logger = Builder::from_default_env();
    logger.filter(Some("hnews::html"), LevelFilter::Trace);
    logger.format(|buf, record| {
        
        let timestamp = buf.timestamp();
        let level = record.level();
        let mod_path = record.module_path().unwrap_or("Could not obtain module path");
        let file =record.file().unwrap_or("Could not obtain file");
        let line = record.line().unwrap_or(0);
        let args = record.args();

        writeln!(
            buf,
            "[{timestamp} {level} {mod_path} {file}:{line}] {args}",
            timestamp = timestamp,
            level = level,
            mod_path = mod_path,
            file = file,
            line = line,
            args = args
        )
    });

    logger.init();
}

#[cfg(test)]
mod tests {

    use super::*;

    const USERNAME: &'static str = "test_acct";
    const PASSWORD: &'static str = "test_pwd";
    const DIR: &'static str = "/Users/chris";

    #[test]
    fn test_client() -> Result<(), Box<dyn Error>> {
        env_logger::init();
        let client = Client::new(USERNAME, PASSWORD);
        println!("{:#?}", client);

        Ok(())
    }

    #[test]
    fn test_login() -> Result<(), Box<dyn Error>> {
        env_logger::init();
        let mut client = Client::new(USERNAME, PASSWORD);
        client.login()?;

        Ok(())
    }

    #[test]
    fn test_submit() -> Result<(), Box<dyn Error>> {
        env_logger::init();
        let mut client = Client::new(USERNAME, PASSWORD);
        client.login()?;
        client.submit("test title rust".to_string(), None, None)?;

        Ok(())
    }
}
