use std::collections::VecDeque;
use std::error::Error;
use std::iter::Iterator;
use std::iter::IntoIterator;

use log::debug;
use reqwest::Method;
use serde::{self, Deserialize, Serialize};

pub mod error;
use error::HNError;


pub struct HNClient {
    // Using blocking for now becase its quick and easy. Consider adding async client in the "future"
    // (pun unintended but not unwelcome).
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
    // TODO: Do I want to put fields like this in the type used for
    // serialization/deserialization? Or instead, create a type with convenaint
    // From/Into impls.

    // Not part of actual API data. A container which can be filled
    // by calling `fetch_kids`
    // pub nodes: Vec<Comment>,
}

impl Story {
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
pub enum CommentNodeState {
    Unfilled,
    Filled,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommentNode {
    pub comment: Comment,
    pub state: CommentNodeState,
    children: Vec<CommentNode>,
}

impl From<Comment> for CommentNode {
    fn from(comment: Comment) -> Self {
        Self {
            comment,
            state: CommentNodeState::Unfilled,
            children: vec![],
        }
    }
}

pub struct CommentNodeIter<'node> {
    root: &'node CommentNode,
    queue: VecDeque<&'node CommentNode>,
}

impl<'node> CommentNodeIter<'node> {

    fn new(node: &'node CommentNode) -> CommentNodeIter {
        
        // TODO: It would be nice if the enum variant defined the data structure
        // itself, rathen than just existing as a field

        // TODO: How to handle when state is Unfilled?
        // match node.state {
        //     CommentNodeState::Unfilled => {
        //         panic!("Not supporting iterating over Unfilled CommentNodes");
        //     },
        //     _ => {}
        // }

        let queue: VecDeque<&CommentNode> = node.children().collect();

        Self {
            root: node,
            queue,
        }
    }
}

impl<'node> Iterator for CommentNodeIter<'node> {
    type Item = &'node CommentNode;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.queue.pop_front() {
            for child in node.children() {
                // TODO: How does each child call `fill` to get its own children
                self.queue.push_back(child);
            } 
            Some(node)
        } else {
            None
        }
    }
    
}

// impl IntoIterator for CommentNode {
//     type Item = &CommentNode;
//     type IntoIter = CommentNodeIter;
// 
//     fn into_iter(self) -> Self::IntoIter {
//         CommentNodeIter::new(&self)
//     }
// }


impl CommentNode {
    fn fill_comment_node(
        node: &mut CommentNode,
        client: &HNClient,
        recurse: bool,
    ) -> Result<(), Box<dyn Error>> {

        // If this node's children have already been populated, return early
        match node.state {
            CommentNodeState::Filled => {
                return Ok(())
            },
            _ => {},
        }

        // For each comment, retrieve the comment data via an API request,
        // and create a CommentNode. If `recurse` flag is true, also do the
        // same for all descendants
        if let Some(ref kids) = node.comment.kids {
            for id in kids.iter() {
                
                let comment = match client.get_by_id(*id)? {
                    Item::Comment(comment) => Ok(comment),
                    _ => Err(HNError::new(
                        "`fill_childern` got Item variant other than Item::Comment".to_string(),
                        None,
                    )),
                }?;
                
                let mut child_node = CommentNode::from(comment);
                if recurse {
                    CommentNode::fill_comment_node(&mut child_node, client, recurse)?;
                }
                node.children.push(child_node);
            }
        }
        node.state = CommentNodeState::Filled;

        Ok(())
    }

    // Instance method to populate a CommentNode's children nodes 
    pub fn fill_children(&mut self, client: &HNClient) -> Result<(), Box<dyn Error>> {
        CommentNode::fill_comment_node(self, client, false)?;
        Ok(())
    }

    // Instance method to recursively populate all of a CommentNode's descendant nodes
    pub fn fill_descendants(&mut self, client: &HNClient) -> Result<(), Box<dyn Error>> {
        CommentNode::fill_comment_node(self, client, true)?;
        Ok(())
    }

    pub fn children(&self) -> std::slice::Iter<'_, CommentNode> {
        self.children.iter()
    }

    pub fn nodes(&self) -> CommentNodeIter {
        CommentNodeIter::new(self)
    }

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

#[cfg(test)]
mod tests {

    use super::*;
   
    fn get_comment() -> Result<Comment, Box<dyn Error>> {
        // https://news.ycombinator.com/reply?id=25017340
        let id = 25017340;
        let client = HNClient::new();
        let comment = match client.get_by_id(id)? {
            Item::Comment(c) => c,
            _ => unreachable!("Id `10000` corresponds to a comment"),
        };

        Ok(comment)
    }

    #[test]
    fn test_comment_node() -> Result<(), Box<dyn Error>> {
        let client = HNClient::new();
        let comment = get_comment()?;
        let mut node = CommentNode::from(comment);
        node.fill_descendants(&client);
        println!("{:#?}", node);
        // TODO: Add actual assertion logic

        Ok(())
    }

    #[test]
    fn test_comment_node_children() -> Result<(), Box<dyn Error>>{
        let client = HNClient::new();
        let comment = get_comment()?;
        let mut node = CommentNode::from(comment);
        node.fill_children(&client);
        for child in node.children() {
            println!("{:#?}", child);
            // TODO: Add actual assertion logic
        }

        Ok(())
    }

    #[test]
    fn test_comment_node_iterator() -> Result<(), Box<dyn Error>>{
        let client = HNClient::new();
        let comment = get_comment()?;
        let mut node = CommentNode::from(comment);
        node.fill_children(&client);
        for child in node.nodes() {
            println!("{:#?}", child);
            // TODO: Add actual assertion logic
        }

        Ok(())
    }



}
