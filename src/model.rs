use serde::Deserialize;
use serde::Serialize;

pub type Score=u32;
pub type Id=u32;

#[derive(Debug)]
pub struct Date(pub u16, pub u8, pub u8);

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub title: String,
    pub id: Id,
    pub score: Option<Score>,
    pub user: Option<String>,
    // comments: u32,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub user: String,
    pub id: Id,
    pub text: String,
    pub indent: u32,
    // pub bool: deleted,
    pub children: Vec<Comment>,
}

// TODO: This is essentially a Listing, at least with respect to what it represents in the data
// model. There should be some sort of unification in the API.
#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    by: String,
    descendants: u64,
    id: Id,
    kids: Vec<Id>,
    score: Score,
    time: u64,
    title: String,
    // type: String,
    url: String,
}
