
pub type Score=u32;
pub type Id=u32;

pub struct Date(pub u16, pub u8, pub u8);

#[derive(Debug)]
pub struct Listing {
    pub title: String,
    pub id: Id,
    pub score: Option<Score>,
    pub user: Option<String>,
    // comments: u32,
    pub url: String,
}

#[derive(Debug)]
pub struct Comment {
    pub user: String,
    pub id: Id,
    pub text: String,
    pub indent: i32,
    // pub bool: deleted,
}