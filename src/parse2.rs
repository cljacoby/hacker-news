use std::error::Error;
// use std::convert::TryFrom;
// use crate::model::Comment;
// use crate::error::HNError;
use crate::model::Id;
use scraper::Html;
use scraper::Selector;
use lazy_static::lazy_static;


/// ```rust 
/// #[derive(Debug, Serialize, Deserialize)]
/// pub struct Comment {
///     pub user: String,
///     pub id: Id,
///     pub text: String,
///     pub indent: i32,
///     // pub bool: deleted,
///     pub children: Vec<Comment>,
/// }
/// ```

// impl TryFrom<&Html> for Comment {
//     type Error = HNError;
//     fn try_from(html: &Html) -> Result<Self, Self::Error> {
//         unimplemented!()
//     }
// }
// pub(crate) trait FromHtml {
//     fn from_html(html: &Html) -> Result<Self, Box<dyn Error>> where Self: Sized;
// }

pub mod CommentParser {

    use super::*;

    lazy_static! {
        // Applied to root of HTML document
        static ref qs_comment_tree: Selector = Selector::parse("table.comment-tree").unwrap();
        // Applied to comment tree root (i.e. node `table.comment-tree`)
        static ref qs_comment: Selector = Selector::parse("tr.athing.comtr").unwrap();
        // Applied to comment node (i.e. node `tr.athing.comtr`)
        static ref qs_comment_text: Selector = Selector::parse("span.commtext").unwrap();
        static ref qs_comment_user: Selector = Selector::parse("a.hnuser").unwrap();
        static ref qs_indent: Selector = Selector::parse("td.ind img").unwrap();
    }

    fn extract_id(html: &Html) -> Result<Id, Box<dyn Error>> {
        unimplemented!()
    }
    
    fn extract_text(html: &Html) -> Result<String, Box<dyn Error>> {
        unimplemented!()
    }

    fn extract_user(html: &Html) -> Result<String, Box<dyn Error>> {
        unimplemented!()
    }
    
    fn extract_indent(html: &Html) -> Result<u32, Box<dyn Error>> {
        unimplemented!()
    }
}


