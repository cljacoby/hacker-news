use std::error::Error;
use std::fmt;

use reqwest::Method;
use serde::{self, Deserialize, Serialize};

// TODO: Can score be negative?
// Currently leaving as i64 to handle potential negative case.
// Check the dump files for a negative score. 

#[derive(Debug)]
pub struct HNError {
    msg: String,
}

impl fmt::Display for HNError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "msg: {}", self.msg) 
    }
}

impl Error for HNError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
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


    pub fn get_by_id(&self, id: Id) -> Result<Item, Box<dyn Error>> {
        let url = format!("{base}/v0/item/{id}.json?print=pretty", base = HNClient::BASE_URL, id = id);
        let text = self.client
            .request(Method::GET, &url)
            .send()?
            .text()?;
        let story = serde_json::from_str(&text)?;
        
        Ok(story)
    }
}

// Id used to reference Items is a sequentially generated, non-negative
// numeric Id. As of 10/31/2020 (boo!) 8:40 AM Eastern, the maximum ID value is 24'950_932.
// It's conceivable that this Id may eventually need an u64. 
pub type Id = u32;

#[derive(Serialize, Deserialize, Debug)]
    pub struct Job {
    /// The item's unique id.
    pub id: Id,
    /// true if the item is deleted.
    pub deleted: Option<bool>,
    /// The username of the item's author.
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    pub dead: Option<bool>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// The comment, story or poll text. HTML.
    pub text: Option<String>,
    /// The URL of the story.
    pub url: Option<String>,
    /// The title of the story, poll or job.
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    /// The item's unique id.
    pub id: Id,
    /// True if the item is deleted.
    pub deleted: Option<bool>,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// True if the item is dead.
    pub dead: Option<bool>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// In the case of stories or polls, the total comment count.
    pub descendants: Option<u32>,
    /// The story's score, or the votes for a pollopt.
    pub score: Option<i64>,
    /// The title of the story, poll or job.
    pub title: Option<String>,
    /// The URL of the story.
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    /// The item's unique id.
    pub id: Id,
    /// true if the item is deleted.
    pub deleted: Option<bool>,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    pub dead: Option<bool>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// The comment's parent: either another comment or the relevant story.
    pub parent: Option<Id>,
    /// The comment, story or poll text. HTML.
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    /// The item's unique id.
    pub id: Id,
    /// true if the item is deleted.
    pub deleted: Option<bool>,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    pub dead: Option<bool>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// A list of related pollopts, in display order.
    pub parts: Option<Vec<Id>>,
    /// In the case of stories or polls, the total comment count.
    pub descendants: Option<u32>,
    /// The story's score, or the votes for a pollopt.
    pub score: Option<i64>,
    /// The title of the story, poll or job.
    pub title: Option<String>,
    /// The comment, story or poll text. HTML.
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PollOption {
    /// The item's unique id.
    pub id: Id,
    /// true if the item is deleted.
    pub deleted: Option<bool>,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    pub dead: Option<bool>,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// The comment's parent: either another comment or the relevant story.
    pub parent: Option<Id>,
    /// The story's score, or the votes for a pollopt.
    pub score: Option<i64>,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Item {
    #[serde(rename = "job")]
    Job(Job),
    #[serde(rename = "story")]
    Story(Story),
    #[serde(rename = "comment")]
    Comment(Comment),
    #[serde(rename = "poll")]
    Poll(Poll),
    #[serde(rename = "pollopt")]
    PollOption(PollOption),
}


#[cfg(test)]
mod tests {

    use super::*;

}
