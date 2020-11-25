use std::collections::VecDeque;
use std::error::Error;

use crate::models::Comment;
use crate::models::Id;
use crate::models::Item;
use crate::models::Job;
use crate::models::Poll;
use crate::models::PollOption;
use crate::models::Story;
use crate::HNClient;
use crate::HNError;

/*
 * TODO:
 *  - Can/should the BFS order be optioned to also do DFS
 *  - In the original hacked up version in `main.rs` I had a timeout
 *    between API request. Should there also be a timeout here?
 * */

pub trait Replyable {
    // I originally tried to define this method as returning either `dyn Iterator`
    // or `impl Iterator`, but was getting compiler errors about not-being Sized
    fn kids(&self) -> &Option<Vec<Id>>;
}

impl Replyable for Story {
    fn kids(&self) -> &Option<Vec<Id>> {
        &self.kids
    }
}
impl Replyable for Comment {
    fn kids(&self) -> &Option<Vec<Id>> {
        &self.kids
    }
}
impl Replyable for Poll {
    fn kids(&self) -> &Option<Vec<Id>> {
        &self.kids
    }
}
impl Replyable for PollOption {
    fn kids(&self) -> &Option<Vec<Id>> {
        &self.kids
    }
}
impl Replyable for Job {
    fn kids(&self) -> &Option<Vec<Id>> {
        &self.kids
    }
}

#[derive(Debug)]
pub struct RepliesIter<'clnt> {
    queue: VecDeque<Id>,
    client: &'clnt HNClient,
    // root: Box<dyn Replyable>,
}

// impl<'clnt> fmt::Debug for RepliesIter<'clnt> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("RepliesIter")
//          .field("queue", &self.queue)
//          .field("client", &self.client)
//          .finish()
//     }
// }

impl<'clnt> RepliesIter<'clnt> {
    pub fn new(root: Id, client: &'clnt HNClient) -> Result<Self, Box<dyn Error>> {
        let item = client.get_by_id(root)?;
        let root = match item {
            Item::Job(j) => Box::new(j) as Box<dyn Replyable>,
            Item::Story(s) => Box::new(s) as Box<dyn Replyable>,
            Item::Comment(c) => Box::new(c) as Box<dyn Replyable>,
            Item::Poll(p) => Box::new(p) as Box<dyn Replyable>,
            Item::PollOption(o) => Box::new(o) as Box<dyn Replyable>,
        };

        let mut queue = VecDeque::new();
        if let Some(kids) = root.kids() {
            for kid in kids {
                queue.push_back(*kid);
            }
        }

        // Ok(Self { queue, client, root })
        Ok(Self { queue, client })
    }
}

impl<'clnt> Iterator for RepliesIter<'clnt> {
    type Item = Result<Comment, Box<dyn Error>>;

    // Breadth First Search iteration
    fn next(&mut self) -> Option<Self::Item> {
        // Pop Id from queue; return Iterator's sentinal None if queue is exhausted
        let id = match self.queue.pop_front() {
            None => return None,
            Some(id) => id,
        };

        // Make request to HN API to get this Id's Comment data
        let resp = self.client.get_by_id(id);
        let comment = match resp {
            // If API request failed, return Error
            Err(src) => {
                let err = HNError::new(format!("Request to HackerNews API failed"), Some(src));
                return Some(Err(Box::new(err)));
            }

            // The API method returns a generic item; verify that it's a Comment variant,
            // and extract inner Comment instance
            Ok(item) => match item {
                Item::Comment(comment) => comment,
                _ => {
                    let err = HNError::new(
                        format!("Thread iteration got non-Comment Item variant"),
                        None,
                    );
                    return Some(Err(Box::new(err)));
                }
            },
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
