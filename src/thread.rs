use std::collections::VecDeque;

use crate::models::Id;
use crate::models::Item;
use crate::models::Job;
use crate::models::Comment;
use crate::HNClient;

pub trait Thread {

    /*
     * NOTE: `kids` returns an Option based on the HackerNews API model.
     * When a Comment or Story has no replies, the `kids` API response
     * field is omitted entirely, rather than passing a blank JSON array.
     * For `kids` to return None indicates that this Comment or Story
     * had no replies.
     * */

    fn kids(&self) -> &Option<Vec<Id>>;
}

#[derive(Debug)]
enum NodeState {
    Unfilled,
    Filled,
}

/*
 * NOTE: This code was implemented Generic over N. This allows creating
 * a Node where `data` is either a Story or a Comment, which are the
 * two HackerNews models which have comments (a.k.a. the `kids` attributre).
 * The field `children` is also generic over N; however, it could just
 * be the Concrete type `Comment`.
 * */

/*
 * TODO: Consider using references + lifetimes vs. using a pointer or
 * container, such as Box or Cell respectively. I'm currently not sure
 * of the exact performance details of each decision. I think that
 * using references and lifetime annotations if possible will provide
 * the best performance; however, it's also kind of pain in the butt.
 * If a simple Box is all that's needed to keep the borrow checker
 * calm, I'd take that. 
 * */

#[derive(Debug)]
struct Node<N>
    where N: Thread
{
    data: N,
    state: NodeState,
    children: Vec<Node<N>>,
}

struct NodeIter <'node, N>
    where N: Thread, &'node N: Thread
{
    // TODO: Can I do BFS and DFS using this single queue-based struct?
    client: Box<HNClient>,
    queue: VecDeque<&'node Node<&'node N>>
}

impl<'node, N> Iterator for NodeIter<'node, N>
    where N: Thread, &'node N: Thread
{
    type Item = &'node N;

    fn next(&mut self) -> Option<Self::Item> {

        // Pop a node; return sential None if queue is exhausted
        let node = match self.queue.pop_front() {
            None => return None,
            Some(node) => node,
        };

        match node.state {

            // If node is Uniflled, retrieve each child's data 
            NodeState::Unfilled => {
            },
            NodeState::Filled => {},
        }
        for child in node.children.iter() {
            self.queue.push_back(child);
        }
        Some(node.data)
    }

}


// **********************************************
// **********************************************


impl Thread for Job {
    fn kids(&self) -> &Option<Vec<Id>> {
        &self.kids
    }
}

impl Thread for &Job {
    fn kids(&self) -> &Option<Vec<Id>> {
        &self.kids
    }
}

impl Thread for Comment {
    fn kids(&self) -> &Option<Vec<Id>> {
        &self.kids
    }
}

impl Thread for &Comment {
    fn kids(&self) -> &Option<Vec<Id>> {
        &self.kids
    }
}



