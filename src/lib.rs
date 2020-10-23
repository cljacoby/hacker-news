use std::error::Error;

use reqwest::blocking::Client;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::rust::double_option;

pub mod loose;
pub mod strict;

pub struct HNClient {
    // Using blocking for now becase its quick and easy. Consider adding async client.
    client: reqwest::blocking::Client,
}

impl HNClient {
    pub const BASE_URL: &'static str = "https://hacker-news.firebaseio.com";

    pub fn new() -> Self {
        let client = reqwest::blocking::Client::new();

        Self { client }
    }

    #[rustfmt::skip]
    pub fn get_story_by_id(&self, id: &str) -> Result<loose::Story, Box<Error>> {
        let url = format!("{base}/v0/item/{id}.json?print=pretty", base = HNClient::BASE_URL, id = id);
        let story = self.client
            .request(Method::GET, &url)
            .send()?
            .json()?;
        
        Ok(story)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_request() -> Result<(), Box<std::error::Error>> {
        // TODO: Get a story and assert_eq! against expected value
        Ok(())
    }

}
