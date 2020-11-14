use std::collections::VecDeque;

use crate::models::Id;
use crate::models::Job;
use crate::models::Comment;

pub trait Thread {
    fn kids(&self) -> &Option<Vec<Id>>;
}

#[derive(Debug)]
enum NodeState {
    Unfilled,
    Filled,
}

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
    queue: VecDeque<&'node Node<&'node N>>
}

impl<'node, N> Iterator for NodeIter<'node, N>
    where N: Thread, &'node N: Thread
{
    type Item = &'node N;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.queue.pop_front() {
            for child in node.children.iter() {
                self.queue.push_back(child);
            }
            Some(node.data)
        } else {
            None
        }
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



