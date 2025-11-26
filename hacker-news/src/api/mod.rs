use serde::{Serialize, Deserialize};

pub mod derived;

pub type Score = u64;
pub type Id = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// The user's unique username. Case-sensitive.
    id: String,
    /// Delay in minutes between a comment's creation and its visibility to other users.
    delay: Option<u32>,
    /// Creation date of the user, in Unix Time.
    created: u32,
    /// The user's karma.
    karma: Score,
    /// The user's optional self-description. HTML.
    about: Option<String>,
    /// List of the user's stories, polls and comments.
    submitted: Option<Vec<Id>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Job {
    /// The item's unique id.
    pub id: Id,
    /// true if the item is deleted.
    #[serde(default)]
    pub deleted: bool,
    /// The username of the item's author.
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    #[serde(default)]
    pub dead: bool,
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
    #[serde(default)]
    pub deleted: bool,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// True if the item is dead.
    #[serde(default)]
    pub dead: bool,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// In the case of stories or polls, the total comment count.
    pub descendants: Option<u32>,
    /// The story's score, or the votes for a pollopt.
    pub score: Option<Score>,
    /// The title of the story, poll or job.
    pub title: String,
    /// The URL of the story.
    pub url: Option<String>,
    /// The comment, story or poll text. HTML.
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    // Fields directly obtained from the response payload
    /// The item's unique id.
    pub id: Id,
    /// True if the item is deleted.
    #[serde(default)]
    pub deleted: bool,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    #[serde(default)]
    pub dead: bool,
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
    /// True if the item is deleted.
    #[serde(default)]
    pub deleted: bool,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    #[serde(default)]
    pub dead: bool,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// A list of related pollopts, in display order.
    pub parts: Option<Vec<Id>>,
    /// In the case of stories or polls, the total comment count.
    pub descendants: Option<u32>,
    /// The story's score, or the votes for a pollopt.
    pub score: Option<Score>,
    /// The title of the story, poll or job.
    pub title: String,
    /// The comment, story or poll text. HTML.
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PollOption {
    /// The item's unique id.
    pub id: Id,
    /// True if the item is deleted.
    #[serde(default)]
    pub deleted: bool,
    /// The type of item. One of "job", "story", "comment", "poll", or "pollopt".
    pub by: Option<String>,
    /// Creation date of the item, in Unix Time.
    pub time: u64,
    /// true if the item is dead.
    #[serde(default)]
    pub dead: bool,
    /// The ids of the item's comments, in ranked display order.
    pub kids: Option<Vec<Id>>,
    /// The comment's parent: either another comment or the relevant story.
    pub parent: Option<Id>,
    /// The story's score, or the votes for a pollopt.
    pub score: Option<Score>,
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

impl Item {
    pub fn id(&self) -> Id {
        match self {
            Item::Job(j) => j.id,
            Item::Story(s) => s.id,
            Item::Comment(c) => c.id,
            Item::Poll(p) => p.id,
            Item::PollOption(po) => po.id,
        }
    }

    pub fn deleted(&self) -> bool {
        match self {
            Item::Job(j) => j.deleted,
            Item::Story(s) => s.deleted,
            Item::Comment(c) => c.deleted,
            Item::Poll(p) => p.deleted,
            Item::PollOption(po) => po.deleted,
        }
    }

    pub fn by(&self) -> Option<&str> {
        match self {
            Item::Job(j) => j.by.as_deref(),
            Item::Story(s) => s.by.as_deref(),
            Item::Comment(c) => c.by.as_deref(),
            Item::Poll(p) => p.by.as_deref(),
            Item::PollOption(po) => po.by.as_deref(),
        }
    }

    pub fn time(&self) -> u64 {
        match self {
            Item::Job(j) => j.time,
            Item::Story(s) => s.time,
            Item::Comment(c) => c.time,
            Item::Poll(p) => p.time,
            Item::PollOption(po) => po.time,
        }
    }

    pub fn dead(&self) -> bool {
        match self {
            Item::Job(j) => j.dead,
            Item::Story(s) => s.dead,
            Item::Comment(c) => c.dead,
            Item::Poll(p) => p.dead,
            Item::PollOption(po) => po.dead,
        }
    }

    pub fn kids(&self) -> Option<&[Id]> {
        match self {
            Item::Job(j) => j.kids.as_deref(),
            Item::Story(s) => s.kids.as_deref(),
            Item::Comment(c) => c.kids.as_deref(),
            Item::Poll(p) => p.kids.as_deref(),
            Item::PollOption(po) => po.kids.as_deref(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ItemsAndProfiles {
    pub items: Vec<Id>,
    pub profiles: Vec<String>,
}

impl Item {
    pub fn is_job(&self) -> bool {
        matches!(self, Self::Job(_job))
    }

    pub fn is_story(&self) -> bool {
        matches!(self, Self::Story(_story))
    }

    pub fn is_comment(&self) -> bool {
        matches!(self, Self::Comment(_comment))
    }

    pub fn is_poll(&self) -> bool {
        matches!(self, Self::Poll(_poll))
    }

    pub fn is_poll_option(&self) -> bool {
        matches!(self, Self::PollOption(_poll_opt))
    }
}

#[cfg(test)]
mod tests {

    use super::Item;
    use super::Story;

    #[test]
    fn test_item_type() {
        let story = Item::Story(Story {
            id: 27476206,
            deleted: false,
            by: Some("what_ever".to_string()),
            time: 1623432780,
            dead: false,
            kids: Some(vec![27488169, 27478163, 27488195, 27477211, 27488706, 27477425, 27477221, 27489125, 27490162, 27489280, 27487982, 27479605, 27490009, 27488234, 27491642, 27489141, 27477380, 27489264]),
            descendants: Some(314),
            score: Some(529),
            title: "Apple admits it ranked its Files app ahead of competitor Dropbox".to_string(),
            url: Some("https://www.theverge.com/2021/6/11/22528701/apple-rank-own-app-over-competitor-files-dropbox-wwdc-2017".to_string()),
            text: None,
        });

        assert!(story.is_story());
    }
}
