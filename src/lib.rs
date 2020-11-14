use std::collections::VecDeque;
use std::error::Error;
use std::iter::Iterator;
// use std::iter::IntoIterator;

use serde;
use serde::Deserialize;
use serde::Serialize;
use log::debug;
use reqwest::Method;

pub mod error;
use error::HNError;

pub mod thread;
use thread::Thread;

pub mod models;
use models::Id;
use models::Comment;
use models::Item;

pub struct HNClient {
    // Using blocking for now becase its easier. Consider async client in the "future" (pun somewhat intended)
    client: reqwest::blocking::Client,
}

impl HNClient {
    pub const BASE_URL: &'static str = "https://hacker-news.firebaseio.com";

    pub fn new() -> Self {
        let client = reqwest::blocking::Client::new();

        Self { client }
    }

    pub fn get_by_id(&self, id: Id) -> Result<Item, Box<dyn Error>> {
        let url = format!(
            "{base}/v0/item/{id}.json?print=pretty",
            base = HNClient::BASE_URL,
            id = id
        );
        let text = self.client.request(Method::GET, &url).send()?.text()?;
        let story = serde_json::from_str(&text)?;
        debug!("{:?}", story);
        println!("{:?}", story);

        Ok(story)
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    fn get_comment() -> Comment {
        Comment {
            id: 25017340,
            deleted: None,
            by: Some("softwaredoug".to_string()),
            time: 1604774581,
            dead: None,
            kids: Some(
                vec![
                    25017485, 25017453, 25017607, 25017610, 25017531,
                    25017387, 25017782, 25019941, 25017943, 25017757,
                    25017520, 25019001, 25017807, 25018099, 25017797,
                    25023690, 25018128, 25018160, 25017766, 25019473,
                    25021402, 25017802,
                ]
            ),
            parent: Some(25015967),
            text: Some("On the data nerd side I continue to be shocked at how people misinterpret the certainty of polls&#x2F;forecasts. Forecasts give us probability distributions based on historical polling error data. Not infallible predictions the expected value will 100% happen.<p>It’s fairly revealing of society’s general innumeracy, just as it was 4 years ago when Trump won.".to_string()),
        }
    }
}
