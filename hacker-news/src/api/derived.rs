
use serde::{Serialize, Deserialize};

use crate::api::{Id, Item};
use crate::error::HnError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
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

impl TryFrom<Item> for Listing {
    type Error=crate::error::HnError;

    fn try_from(item: Item) -> Result<Listing, Self::Error> {
        match item {
            Item::Job(j) => Ok(Listing {
                id: j.id,
                deleted: j.deleted,
                by: j.by,
                time: j.time,
                dead: j.dead,
                kids: j.kids,
                text: j.text,
                url: j.url,
                title: j.title,
            }),
            Item::Story(s) => Ok(Listing {
                id: s.id,
                deleted: s.deleted,
                by: s.by,
                time: s.time,
                dead: s.dead,
                kids: s.kids,
                text: s.text,
                url: s.url,
                title: s.title,
            }),
            Item::Comment(_) => Err(HnError::ListingError(Some("A Comment cannot be a top level listing"))),
            Item::Poll(p) => Ok(Listing {
                id: p.id,
                deleted: p.deleted,
                by: p.by,
                time: p.time,
                dead: p.dead,
                kids: p.kids,
                text: p.text,
                url: None,
                title: p.title,
            }),
            Item::PollOption(_) =>  Err(HnError::ListingError(Some("A PollOption cannot be a top level listing"))),
        }
    }

}