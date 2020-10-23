use std::error::Error;

use reqwest::blocking::Client;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use serde_with::rust::double_option;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Story {
    /// The item's unique id.
    pub id: i64,

    /// true if the item is deleted.
    pub deleted: bool,

    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
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

    /// The URL of the story.
    pub url: String,

    /// The story's score, or the votes for a pollopt.
    pub score: i64,

    /// The title of the story, poll or job. HTML.
    pub title: String,

    /// A list of related pollopts, in display order.
    pub parts: String,

    /// In the case of stories or polls, the total comment count/
    pub descendants: i64,
}

impl Story {
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_full_init() {
        let story = Story {
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
            url:        "url".to_string(),
            score:      0,
            title:      "title".to_string(),
            parts:      "parts".to_string(),
            descendants: 2,
        };
        println!("story = {:#?}", story);
        let s = serde_json::to_string(&story);
        println!("s = {:#?}", s);
    }

    #[test]
    fn test_default() {
        let story = Story {
            id: 100,
            ..Story::default()
        };
        println!("story = {:#?}", story);
        let s = serde_json::to_string(&story);
        println!("s = {:#?}", s);
    }
}
