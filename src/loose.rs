use std::error::Error;

use reqwest::blocking::Client;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use serde_with::rust::double_option;

/**
 * This struct will Deserialize with any valid JSON.
 * */
#[derive(Serialize, Deserialize, Debug, Default)]
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

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_default() {
        let story = Story {
            id: Some(Some(100)),
            ..Story::default()
        };
        println!("story = {:#?}", story);
        let s = serde_json::to_string(&story);
        println!("s = {:#?}", s);
    }

    #[test]
    fn test_full_init() {
        let story = Story {
            id:         Some(Some(100)),
            deleted:    Some(Some(false)),
            hn_type:    Some(Some("hn_type".to_string())),
            by:         Some(Some("author".to_string())),
            time:       Some(Some(0)),
            text:       Some(Some("text".to_string())),
            dead:       Some(Some(false)),
            parent:     Some(Some(0)),
            poll:       Some(Some("poll".to_string())),
            kids:       Some(Some(vec![1])),
            url:        Some(Some("url".to_string())),
            score:      Some(Some(0)),
            title:      Some(Some("title".to_string())),
            parts:      Some(Some("parts".to_string())),
            descendants: Some(Some(2)),
        };
        println!("story = {:#?}", story);
        let s = serde_json::to_string(&story);
        println!("s = {:#?}", s);
    }


    /// This test shows how implementing the struct this way will parse,
    /// any valid JSON without throwing an error. However, if the JSON fields
    /// have no overlap with the actual struct definition, the result will be
    /// an empty JSON object.
    #[test]
    fn test_invalid() -> Result<(), Box<Error>> {
        let data = json!({
            "this": "is",
            "an": "invalid",
            "story": "payload",
        });
        println!("input json = {:#?}", data);
        let story: Story = serde_json::from_value(data)?;
        let story = serde_json::to_string(&story);
        println!("story = {:#?}", story);

        Ok(())
    }

}
