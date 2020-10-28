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


    pub fn get_by_id(&self, id: &str) -> Result<Item, Box<dyn Error>> {
        let url = format!("{base}/v0/item/{id}.json?print=pretty", base = HNClient::BASE_URL, id = id);
        let text = self.client
            .request(Method::GET, &url)
            .send()?
            .text()?;
        println!("text:");
        println!("{:#?}", text);

        let story = serde_json::from_str(&text)?;
        
        Ok(story)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename = "job")]
    Job {
        /// The item's unique id.
        id: i64,
        /// true if the item is deleted.
        deleted: Option<bool>,
        /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
        //#[serde(rename = "type")]
        //hn_type: String,
        /// The username of the item's author.
        by: String,
        /// Creation date of the item, in Unix Time.
        time: i64,
        /// true if the item is dead.
        dead: Option<bool>,
        /// The ids of the item's comments, in ranked display order.
        kids: Option<Vec<i64>>,
        /// The comment, story or poll text. HTML.
        /// Optional response key; may be omitted.
        text: Option<String>,
        /// The URL of the story.
        url: Option<String>,
        /// The title of the story, poll or job.
        title: String,
    },
    #[serde(rename = "story")]
    Story {
        /// The item's unique id.
        id: i64,
        /// True if the item is deleted.
        /// Optional response key; may be omitted.
        deleted: Option<bool>,
        /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
        //#[serde(rename = "type")]
        //hn_type: String,
        /// The username of the item's author.
        by: String,
        /// Creation date of the item, in Unix Time.
        time: i64,
        /// True if the item is dead.
        /// Optional response key; may be omitted.
        dead: Option<bool>,
        /// The ids of the item's comments, in ranked display order.
        kids: Option<Vec<i64>>,
        /// In the case of stories or polls, the total comment count.
        /// Optional response key; may be omitted.
        descendants: Option<i64>,
        /// The story's score, or the votes for a pollopt.
        score: i64,
        /// The title of the story, poll or job.
        title: String,
        /// The URL of the story.
        /// Optional response key; may be omitted.
        url: Option<String>,
    },
    #[serde(rename = "comment")]
    Comment {
        /// The item's unique id.
        id: i64,
        /// true if the item is deleted.
        deleted: Option<bool>,
        /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
        //#[serde(rename = "type")]
        //hn_type: String,
        /// The username of the item's author.
        by: String,
        /// Creation date of the item, in Unix Time.
        time: i64,
        /// true if the item is dead.
        dead: Option<bool>,
        /// The ids of the item's comments, in ranked display order.
        kids: Option<Vec<i64>>,
        /// The comment's parent: either another comment or the relevant story.
        parent: i64,
        /// The comment, story or poll text. HTML.
        /// Optional response key; may be omitted.
        text: Option<String>,
    },
    #[serde(rename = "poll")]
    Poll {
        /// The item's unique id.
        id: i64,
        /// true if the item is deleted.
        deleted: Option<bool>,
        /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
        //#[serde(rename = "type")]
        //hn_type: String,
        /// The username of the item's author.
        by: String,
        /// Creation date of the item, in Unix Time.
        time: i64,
        /// true if the item is dead.
        dead: Option<bool>,
        /// The ids of the item's comments, in ranked display order.
        kids: Option<Vec<i64>>,
        /// A list of related pollopts, in display order.
        parts: Vec<i64>,
        /// In the case of stories or polls, the total comment count.
        /// Optional response key; may be omitted.
        descendants: Option<i64>,
        /// The story's score, or the votes for a pollopt.
        score: i64,
        /// The title of the story, poll or job.
        title: String,
        /// The comment, story or poll text. HTML.
        text: String,
    },
    #[serde(rename = "pollopt")]
    PollOption {
        /// The item's unique id.
        id: i64,
        /// true if the item is deleted.
        deleted: Option<bool>,
        /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
        //#[serde(rename = "type")]
        //hn_type: String,
        /// The username of the item's author.
        by: String,
        /// Creation date of the item, in Unix Time.
        time: i64,
        /// true if the item is dead.
        dead: Option<bool>,
        /// The ids of the item's comments, in ranked display order.
        /// Optional response key; may be omitted.
        kids: Option<Vec<i64>>,
        /// The comment's parent: either another comment or the relevant story.
        parent: Option<i64>,
        /// The story's score, or the votes for a pollopt.
        score: i64,
    },
}


#[cfg(test)]
mod tests {

    use super::*;


}
