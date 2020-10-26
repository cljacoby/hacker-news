use std::error::Error;

use reqwest::Method;
use serde::{self, Deserialize, Serialize};

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


    pub fn get_by_id() {}


    // #[rustfmt::skip]
    // pub fn get_story_by_id(&self, id: &str) -> Result<Base, Box<dyn Error>> {
    //     let url = format!("{base}/v0/item/{id}.json?print=pretty", base = HNClient::BASE_URL, id = id);
    //     let story = self.client
    //         .request(Method::GET, &url)
    //         .send()?
    //         .json()?;
    //     
    //     Ok(story)
    // }
}


/*
 * TODO:
 * The struct `Base` was defined by directly referencing the model outlined in this
 * documentation:
 *
 * https://hackernews.api-docs.io/v0/items/base
 *
 * I'm not sure if the struct is itself that useful. What may be useful is having a
 * catch-all superset of all the Item variants. This could also use the Serde double_option,
 * and have the varaints have try_into methods to deive them.
 * */


#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Base {
    /// The item's unique id.
    pub id: i64,
    /// true if the item is deleted.
    pub deleted: bool,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    #[serde(rename = "type")]
    pub hn_type: String,
    /// The username of the item's author.
    pub by: String,
    /// Creation date of the item, in Unix Time.
    pub time: i64,
    /// The comment, story or poll text. HTML.
    pub text: String,
    /// true if the item is dead.
    pub dead: bool,
    /// The comment's parent: either another comment or the relevant story.
    pub parent: i64,
    /// The pollopt's associated poll.
    pub poll: String,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Job {
    /// The item's unique id.
    pub id: i64,
    /// true if the item is deleted.
    pub deleted: bool,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    #[serde(rename = "type")]
    pub hn_type: String,
    /// The username of the item's author.
    pub by: String,
    /// Creation date of the item, in Unix Time.
    pub time: i64,
    /// true if the item is dead.
    pub dead: bool,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Vec<i64>,
    /// The comment, story or poll text. HTML.
    pub text: String,
    /// The URL of the story.
    pub url: String,
    /// The title of the story, poll or job.
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Story {
    /// The item's unique id.
    pub id: i64,
    /// true if the item is deleted.
    pub deleted: bool,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    #[serde(rename = "type")]
    pub hn_type: String,
    /// The username of the item's author.
    pub by: String,
    /// Creation date of the item, in Unix Time.
    pub time: i64,
    /// true if the item is dead.
    pub dead: bool,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Vec<i64>,
    /// In the case of stories or polls, the total comment count.
    pub descendants: i64,
    /// The story's score, or the votes for a pollopt.
    pub score: i64,
    /// The title of the story, poll or job.
    pub title: String,
    /// The URL of the story.
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Comment {
    /// The item's unique id.
    pub id: i64,
    /// true if the item is deleted.
    pub deleted: bool,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    #[serde(rename = "type")]
    pub hn_type: String,
    /// The username of the item's author.
    pub by: String,
    /// Creation date of the item, in Unix Time.
    pub time: i64,
    /// true if the item is dead.
    pub dead: bool,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Vec<i64>,
    /// The comment's parent: either another comment or the relevant story.
    pub parent: i64,
    /// The comment, story or poll text. HTML.
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Poll {
    /// The item's unique id.
    pub id: i64,
    /// true if the item is deleted.
    pub deleted: bool,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    #[serde(rename = "type")]
    pub hn_type: String,
    /// The username of the item's author.
    pub by: String,
    /// Creation date of the item, in Unix Time.
    pub time: i64,
    /// true if the item is dead.
    pub dead: bool,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Vec<i64>,
    /// A list of related pollopts, in display order.
    pub parts: Vec<i64>,
    /// In the case of stories or polls, the total comment count.
    pub descendants: i64,
    /// The story's score, or the votes for a pollopt.
    pub score: i64,
    /// The title of the story, poll or job.
    pub title: String,
    /// The comment, story or poll text. HTML.
    pub text: String,

}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PollOption {
    /// The item's unique id.
    pub id: i64,
    /// true if the item is deleted.
    pub deleted: bool,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    #[serde(rename = "type")]
    pub hn_type: String,
    /// The username of the item's author.
    pub by: String,
    /// Creation date of the item, in Unix Time.
    pub time: i64,
    /// true if the item is dead.
    pub dead: bool,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Vec<i64>,
    /// The comment's parent: either another comment or the relevant story.
    pub parent: i64,
    /// The story's score, or the votes for a pollopt.
    pub score: i64,
    
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_full_init() {
        let story = Base {
            id:         100,
            deleted:    false,
            hn_type:    "hn_type".to_string(),
            by:         "author".to_string(),
            time:       0,
            text:       "text".to_string(),
            dead:       false,
            parent:     0,
            poll:       "poll".to_string(),
            kids:       vec![1],
        };
        println!("story = {:#?}", story);
        let s = serde_json::to_string(&story);
        println!("s = {:#?}", s);
    }

    #[test]
    fn test_default() {
        let story = Base {
            id: 100,
            ..Base::default()
        };
        println!("story = {:#?}", story);
        let s = serde_json::to_string(&story);
        println!("s = {:#?}", s);
    }
}
