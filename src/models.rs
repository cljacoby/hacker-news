use serde;
use serde::Deserialize;
use serde::Serialize;

// Id used to reference Items is a sequentially generated, non-negative
// numeric Id. As of 10/31/2020 (boo!) 8:40 AM Eastern, the maximum ID value is 24'950_932.
// It's conceivable that this Id may eventually need an u64.
pub type Id = u32;

// Can score be negative?
// Currently leaving as i64 to handle potential negative case.
// Check the dump files for a negative score.
pub type Score = i64;

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
    pub score: Option<Score>,
    /// The title of the story, poll or job.
    pub title: Option<String>,
    /// The URL of the story.
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    // Fields directly obtained from the response payload
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
    pub score: Option<Score>,
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
}
