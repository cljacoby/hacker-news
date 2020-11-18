use std::collections::VecDeque;
use std::error::Error;

use crate::HNClient;
use crate::HNError;
use crate::models::Id;
use crate::models::Item;
use crate::models::Comment;

/*
 * FIXME:
 *  
 *  - RepliesIter is obtained from the HNClient methods
 *    `walk_story_replies`, and `walk_comment_replies`. Ideally,
 *    this could be a single method generalized over both types
 *  
 *  - When using `walk_story_replies` and `walk_comment_replies`,
 *    the RepliesIter's current implementation wants to own its own
 *    HNClient, and therefore the HNClient duplicates itself. Can
 *    this be done with a reference, box, or container like Cell,
 *    RefCell, etc.?
 *
 *  - In the original hacked up version in `main.rs` I had a timeout
 *    between API request. Should there also be a timeout here?
 *
 *  - As a matter of understand-ability, the `next()` method is kind of
 *    hard to visually follow. It would be nicer if I could define the
 *    Error handling a little better so its less verbose
 * */

pub struct RepliesIter<'clnt> {
    queue: VecDeque<Id>,
    client: &'clnt HNClient,
}

impl<'clnt> RepliesIter<'clnt> {
    pub fn new(queue: VecDeque<Id>, client: &'clnt HNClient) -> Self {
        Self { queue, client }
    } 
}

impl<'clnt> Iterator for RepliesIter<'clnt> {
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
