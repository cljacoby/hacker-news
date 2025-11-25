use serde::Deserialize;
use serde::Serialize;

pub mod firebase;

pub type Score = u32;
pub type Id = u32;

#[derive(Debug)]
pub struct Date(pub u16, pub u8, pub u8);

#[derive(Debug, Serialize, Deserialize)]
pub struct Thread {
    // todo: top should be generalized to support other item's, i.e. poll, askhn
    pub top: firebase::Story,
    pub comments: Vec<Comment>,
}

// todo: what is the purpose of this struct? I think this might have come when I was
// parsing the HTML, and I was parsing each row of html on the front page listings.
#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub title: String,
    pub id: Id,
    pub score: Option<Score>,
    pub user: Option<String>,
    // comments: u32,
    pub url: String,
    pub text: Option<String>,
}

// impl From<Item> for Listing {
//     fn from(item: Item) -> Self {
//         match item {
//             Item::Story(x) => Listing {
//                 title: x.title.unwrap(),
//                 id: x.id,
//                 score: x.score,
//                 user: x.by,
//                 url: x.url.unwrap(),
//                 text: None,
//             },
//             Item::Job(_x) => unimplemented!(),
//             // Listing {
//             //     title: x.title,
//             //     id: x.id,
//             //     score: x.score,
//             //     user: x.by,
//             //     url: x.url,
//             //     text: x.text,
//             // },
//             Item::Comment(_x) => unimplemented!(),
//             // Listing {
//             //     title: x.title,
//             //     id: x.id,
//             //     score: x.score,
//             //     user: x.by,
//             //     url: x.url,
//             //     text: x.text,
//             // },
//             Item::Poll(_x) => unimplemented!(),
//             Item::PollOption(_x) => unimplemented!(),
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub user: String,
    pub id: Id,
    pub text: Option<String>,
    pub indent: u32,
    pub dead: bool,
    pub children: Vec<Comment>,
}
