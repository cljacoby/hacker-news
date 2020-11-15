use std::collections::VecDeque;
use std::error::Error;

use crate::HNClient;
use crate::HNError;
use crate::models::Id;
use crate::models::Item;
use crate::models::Comment;
use crate::models::Story;

// pub trait Replies {}
// impl Replies for Comment {}
// impl Replies for Story {}

pub struct RepliesIter {
    queue: VecDeque<Id>,
    client: HNClient,
}

impl RepliesIter {
    pub fn new(queue: VecDeque<Id>, client: HNClient) -> Self {
        Self { queue, client }
    } 
}

impl Iterator for RepliesIter {
    type Item = Result<Comment, Box<dyn Error>>;

    // Breadth First Search iteration
    fn next(&mut self) -> Option<Self::Item> {

        // Pop Id from queue, return sentinal None if queue is exhausted
        let id = match self.queue.pop_front() {
            None => return None,
            Some(id) => id,
        };

        // Make request to HackerNews API to get this Id's Comment data
        let resp = self.client.get_by_id(id);
        let comment = match resp {

            // If API request failed, return Error
            Err(src) => {
                let err = HNError::new(
                    format!("Request to HackerNews API failed"),
                    Some(src),
                );
                return Some(Err(Box::new(err))); 
            },

            // If API we got an Item, verify that it's a Comment variant, and extract inner Commnent
            Ok(item) => match item {
                Item::Comment(comment) => comment,
                _ => {
                    let err = HNError::new(
                        format!("Thread iteration got non-Comment Item variant"),
                        None,
                    );
                    return Some(Err(Box::new(err))); 
                }
            } 
        };

        // Enqueue this comment's childern comments 
        if let Some(ref kids) = comment.kids {
            for kid in kids {
                self.queue.push_back(*kid);
            }
        }

        Some(Ok(comment))
    }
}

