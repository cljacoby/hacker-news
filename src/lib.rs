use std::error::Error;

use reqwest::blocking::Client;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_with::rust::double_option;

// TODO: Using `serde_with::double_option` means an expected type
// will parse from json, even if some keys are missing, and even if
// some fields are null. Is this desired? If a field is missing, is
// do we actually want to succesfuly parse that payload?

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    /// The item's unique id.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub id: Option<Option<i64>>,

    /// true if the item is deleted.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub deleted: Option<Option<bool>>,

    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    #[serde(
        default,
        rename = "type",
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub hn_type: Option<Option<String>>,

    /// The username of the item's author.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub by: Option<Option<String>>,

    /// Creation date of the item, in Unix Time.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub time: Option<Option<i64>>,

    /// The comment, story or poll text. HTML.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub text: Option<Option<String>>,

    /// true if the item is dead.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub dead: Option<Option<bool>>,

    /// The comment's parent: either another comment or the relevant story.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub parent: Option<Option<i64>>,

    /// The pollopt's associated poll.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub poll: Option<Option<String>>,

    /// The ids of the item's comments, in ranked display order.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub kids: Option<Option<Vec<i64>>>,

    /// The URL of the story.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub url: Option<Option<String>>,

    /// The story's score, or the votes for a pollopt.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub score: Option<Option<i64>>,

    /// The title of the story, poll or job. HTML.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub title: Option<Option<String>>,

    /// A list of related pollopts, in display order.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub parts: Option<Option<String>>,

    /// In the case of stories or polls, the total comment count/
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "double_option"
    )]
    pub descendants: Option<Option<i64>>,
}

impl Story {
    pub fn fmt_doub_opt<T>(opt: &Option<Option<T>>) -> String
    where
        T: std::fmt::Debug,
    {
        match opt {
            None => String::from("None"),
            Some(opt) => match opt {
                None => String::from("null"),
                Some(t) => format!("{:?}", t),
            },
        }
    }

    pub fn print(&self) {
        println!("id = {:?}", Story::fmt_doub_opt(&self.id));
        println!("deleted = {:?}", Story::fmt_doub_opt(&self.deleted));
        println!("hn_type = {:?}", Story::fmt_doub_opt(&self.hn_type));
        println!("by = {:?}", Story::fmt_doub_opt(&self.by));
        println!("time = {:?}", Story::fmt_doub_opt(&self.time));
        println!("text = {:?}", Story::fmt_doub_opt(&self.text));
        println!("dead = {:?}", Story::fmt_doub_opt(&self.dead));
        println!("parent = {:?}", Story::fmt_doub_opt(&self.parent));
        println!("poll = {:?}", Story::fmt_doub_opt(&self.poll));
        println!("kids = {:?}", Story::fmt_doub_opt(&self.kids));
        println!("url = {:?}", Story::fmt_doub_opt(&self.url));
        println!("score = {:?}", Story::fmt_doub_opt(&self.score));
        println!("title = {:?}", Story::fmt_doub_opt(&self.title));
        println!("parts = {:?}", Story::fmt_doub_opt(&self.parts));
        println!("descendants = {:?}", Story::fmt_doub_opt(&self.descendants));
    }
}

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
    pub fn get_story_by_id(&self, id: &str) -> Result<Story, Box<Error>> {
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
